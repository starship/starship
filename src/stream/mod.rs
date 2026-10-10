//! Stream the main and right prompts as modules finish rendering, then refresh
//! them while the shell waits for input.

mod bus;
mod frame;
#[cfg(test)]
mod testing;
mod timings;

use std::io;
use std::mem;
use std::time::{Duration, Instant, SystemTime};

use crate::context::{Context, Properties, Target};
use crate::modules;
use crate::plan::Plan;
use crate::print::{self, DUMB_TERMINAL_PROMPT};
use crate::segment::Segment;
use crate::workers::{self, Rendered, Workers};

use bus::{Bus, DeparturePlanner, Estimate, Rendering};
use frame::{Frame, Sink};

const PROMPT_COUNT: usize = 2;
const PROMPT_TARGETS: [Target; PROMPT_COUNT] = [Target::Main, Target::Right];
const NANOSECONDS_PER_SECOND: u128 = Duration::from_secs(1).as_nanos();

type PromptPlan<'context> = Plan<'context, PROMPT_COUNT>;

/// Stream both prompts to standard output using the previous stream's timings.
pub fn stream(properties: Properties, timings: &str) -> io::Result<()> {
    let context = Context::new(properties, Target::Main);
    run(&context, timings, &mut io::stdout().lock())
}

fn run(context: &Context, timings: &str, output: &mut impl Sink) -> io::Result<()> {
    output.send(Frame::Process)?;

    if print::is_dumb_terminal() {
        return write_fallback_prompt(output);
    }

    let plan = Plan::new(context, PROMPT_TARGETS);
    if context.root_config.asynchronous.enabled {
        run_asynchronously(context, &plan, timings, output)
    } else {
        run_synchronously(context, &plan, timings, output)
    }
}

fn write_fallback_prompt(output: &mut impl Sink) -> io::Result<()> {
    log::error!("Under a 'dumb' terminal (TERM=dumb).");
    output.send(Frame::Prompt {
        main: DUMB_TERMINAL_PROMPT,
        right: DUMB_TERMINAL_PROMPT,
    })?;
    output.send(Frame::Complete(""))
}

fn run_synchronously(
    context: &Context,
    plan: &PromptPlan<'_>,
    timings: &str,
    output: &mut impl Sink,
) -> io::Result<()> {
    let rendered = workers::render_all(context, &plan.modules);
    let [main, right] = plan.prompts(&|position| &rendered[position], context);
    output.send(Frame::Prompt {
        main: &main,
        right: &right,
    })?;
    output.send(Frame::Complete(timings))
}

fn run_asynchronously<'context>(
    context: &'context Context<'context>,
    plan: &'context PromptPlan<'context>,
    timings: &str,
    output: &mut impl Sink,
) -> io::Result<()> {
    let estimates = timings::read(timings, &plan.modules);
    let waiting_module_count = estimates
        .iter()
        .filter(|&&estimate| Estimate::waits(estimate))
        .count();

    workers::with_workers(
        context,
        &plan.modules,
        Some(waiting_module_count),
        |mut workers| {
            Session::new(context, plan, estimates, Clock::now()).run(&mut workers, output)
        },
    )
}

/// Submit renders and wait for completed modules or the next deadline.
trait Renderer {
    fn render(&mut self, position: usize);

    /// Return the observation time and every module completed by then.
    fn wait(&mut self, deadline: Option<Instant>) -> (Instant, Vec<Rendered>);
}

impl Renderer for &Workers<'_> {
    fn render(&mut self, position: usize) {
        Workers::render(self, position);
    }

    fn wait(&mut self, deadline: Option<Instant>) -> (Instant, Vec<Rendered>) {
        let completed = Workers::wait(self, deadline);
        (Instant::now(), completed)
    }
}

/// Anchor wall-clock refresh boundaries to the stream's monotonic clock.
struct Clock {
    started_at: Instant,
    wall_time_at_start: Duration,
}

impl Clock {
    fn now() -> Self {
        Self {
            started_at: Instant::now(),
            wall_time_at_start: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default(),
        }
    }

    /// Find the first wall-clock multiple of `period` strictly after `now`.
    /// Zero periods and unrepresentable deadlines have no next refresh.
    fn next_refresh_at(&self, now: Instant, period: Duration) -> Option<Instant> {
        let wall_time = self.wall_time_at_start.checked_add(now - self.started_at)?;
        let elapsed_nanoseconds = wall_time.as_nanos().checked_rem(period.as_nanos())?;
        let remaining_nanoseconds = period.as_nanos() - elapsed_nanoseconds;
        now.checked_add(duration_from_nanoseconds(remaining_nanoseconds)?)
    }
}

fn duration_from_nanoseconds(nanoseconds: u128) -> Option<Duration> {
    let seconds = u64::try_from(nanoseconds / NANOSECONDS_PER_SECOND).ok()?;
    let nanoseconds = u32::try_from(nanoseconds % NANOSECONDS_PER_SECOND).ok()?;
    Some(Duration::new(seconds, nanoseconds))
}

struct ModuleSlot {
    segments: Vec<Segment>,
    refresh_period: Option<Duration>,
    estimate: Option<Estimate>,
    state: RenderState,
}

#[derive(Clone, Copy)]
enum RenderState {
    Rendering {
        started_at: Instant,
        kind: RenderKind,
    },
    Rendered {
        refresh_at: Option<Instant>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RenderKind {
    Initial,
    Refresh,
}

impl ModuleSlot {
    fn new(
        started_at: Instant,
        refresh_period: Option<Duration>,
        estimate: Option<Estimate>,
    ) -> Self {
        Self {
            segments: Vec::new(),
            refresh_period,
            estimate,
            state: RenderState::Rendering {
                started_at,
                kind: RenderKind::Initial,
            },
        }
    }

    fn next_refresh_at(&self) -> Option<Instant> {
        match self.state {
            RenderState::Rendered { refresh_at } => refresh_at,
            RenderState::Rendering { .. } => None,
        }
    }

    fn start_refresh_if_due(&mut self, now: Instant) -> bool {
        if !self
            .next_refresh_at()
            .is_some_and(|deadline| deadline <= now)
        {
            return false;
        }

        self.state = RenderState::Rendering {
            started_at: now,
            kind: RenderKind::Refresh,
        };
        true
    }

    /// Commit a render and return its actual change/duration for the bus.
    /// Only the initial render updates the estimate reported to the next stream.
    fn finish_render(
        &mut self,
        segments: Vec<Segment>,
        now: Instant,
        clock: &Clock,
    ) -> Option<Estimate> {
        let RenderState::Rendering { started_at, kind } = self.state else {
            return None;
        };

        let measured = if self.segments == segments {
            Estimate::Unchanged
        } else {
            self.segments = segments;
            Estimate::Changed(now - started_at)
        };

        if kind == RenderKind::Initial {
            self.estimate = Some(Estimate::after(self.estimate, measured));
        }
        self.state = RenderState::Rendered {
            refresh_at: self
                .refresh_period
                .and_then(|period| clock.next_refresh_at(now, period)),
        };
        Some(measured)
    }

    fn rendering(&self, use_estimates: bool) -> Option<Rendering> {
        match self.state {
            RenderState::Rendering { started_at, .. } => Some(Rendering {
                since: started_at,
                estimate: self.estimate.filter(|_| use_estimates),
            }),
            RenderState::Rendered { .. } => None,
        }
    }
}

struct Session<'context> {
    context: &'context Context<'context>,
    plan: &'context PromptPlan<'context>,
    clock: Clock,
    use_estimates: bool,
    modules: Vec<ModuleSlot>,
    bus: Bus,
    departure_planner: DeparturePlanner,
    last_prompts: Option<[String; PROMPT_COUNT]>,
    heartbeat_pending: bool,
    completion_reported: bool,
}

impl<'context> Session<'context> {
    fn new(
        context: &'context Context<'context>,
        plan: &'context PromptPlan<'context>,
        estimates: Vec<Option<Estimate>>,
        clock: Clock,
    ) -> Self {
        let configuration = &context.root_config.asynchronous;
        let modules = plan
            .modules
            .iter()
            .zip(estimates)
            .map(|(module, estimate)| {
                let refresh_period =
                    modules::period(module, context).filter(|_| configuration.refresh);
                ModuleSlot::new(clock.started_at, refresh_period, estimate)
            })
            .collect();

        Self {
            context,
            plan,
            bus: Bus::new(clock.started_at, configuration.bus.fallback()),
            departure_planner: DeparturePlanner::with_capacity(plan.modules.len()),
            use_estimates: configuration.bus.adaptive,
            clock,
            modules,
            last_prompts: None,
            heartbeat_pending: false,
            completion_reported: false,
        }
    }

    fn run(mut self, renderer: &mut impl Renderer, output: &mut impl Sink) -> io::Result<()> {
        self.start_initial_renders(renderer);

        let mut now = self.clock.started_at;
        loop {
            self.advance(now, renderer, output)?;
            if self.is_finished() {
                return Ok(());
            }

            let (completed_at, completed) = renderer.wait(self.next_deadline(now));
            now = completed_at;
            for rendered in completed {
                self.accept_render(rendered, now);
            }
        }
    }

    /// Unmeasured modules first, followed by the earliest expected changes.
    /// Preserve plan order for equal estimates, including worker queue delays.
    fn start_initial_renders(&self, renderer: &mut impl Renderer) {
        let mut positions: Vec<usize> = (0..self.modules.len()).collect();
        positions.sort_by_key(|&position| self.modules[position].estimate);
        for position in positions {
            renderer.render(position);
        }
    }

    /// Output and completion must precede new refresh work, including when a
    /// failed write tells us the shell has stopped reading.
    fn advance(
        &mut self,
        now: Instant,
        renderer: &mut impl Renderer,
        output: &mut impl Sink,
    ) -> io::Result<()> {
        self.write_pending_output(now, output)?;
        self.report_completion_if_ready(output)?;
        self.start_due_refreshes(now, renderer);
        Ok(())
    }

    fn write_pending_output(&mut self, now: Instant, output: &mut impl Sink) -> io::Result<()> {
        let heartbeat_pending = mem::take(&mut self.heartbeat_pending);
        if self
            .next_departure(now)
            .is_some_and(|deadline| deadline <= now)
        {
            return self.draw(output);
        }
        if heartbeat_pending {
            output.send(Frame::Heartbeat)?;
        }
        Ok(())
    }

    fn draw(&mut self, output: &mut impl Sink) -> io::Result<()> {
        self.bus.leave();
        let prompts = self
            .plan
            .prompts(&|position| &self.modules[position].segments, self.context);

        if self.last_prompts.as_ref() == Some(&prompts) {
            return Ok(());
        }

        let [main, right] = &prompts;
        output.send(Frame::Prompt { main, right })?;
        self.last_prompts = Some(prompts);
        Ok(())
    }

    fn report_completion_if_ready(&mut self, output: &mut impl Sink) -> io::Result<()> {
        if self.completion_reported
            || !self.bus.is_empty()
            || self.rendering_modules().next().is_some()
        {
            return Ok(());
        }

        output.send(Frame::Complete(&self.timing_report()))?;
        self.completion_reported = true;
        Ok(())
    }

    fn start_due_refreshes(&mut self, now: Instant, renderer: &mut impl Renderer) {
        for (position, module) in self.modules.iter_mut().enumerate() {
            if module.start_refresh_if_due(now) {
                renderer.render(position);
            }
        }
    }

    fn accept_render(&mut self, rendered: Rendered, now: Instant) {
        match self.modules[rendered.position].finish_render(rendered.segments, now, &self.clock) {
            Some(Estimate::Changed(duration)) => {
                self.bus.board(now, duration);
            }
            Some(Estimate::Unchanged) => self.heartbeat_pending |= self.completion_reported,
            None => {}
        }
    }

    fn next_departure(&mut self, now: Instant) -> Option<Instant> {
        let rendering = self
            .modules
            .iter()
            .filter_map(|module| module.rendering(self.use_estimates));
        self.bus
            .departure_with_planner(now, rendering, &mut self.departure_planner)
    }

    fn next_deadline(&mut self, now: Instant) -> Option<Instant> {
        self.next_departure(now)
            .into_iter()
            .chain(self.modules.iter().filter_map(ModuleSlot::next_refresh_at))
            .min()
    }

    fn rendering_modules(&self) -> impl Iterator<Item = Rendering> + '_ {
        self.modules
            .iter()
            .filter_map(|module| module.rendering(self.use_estimates))
    }

    fn timing_report(&self) -> String {
        timings::report(
            self.plan
                .modules
                .iter()
                .zip(&self.modules)
                .filter_map(|(name, module)| {
                    module.estimate.map(|estimate| (name.as_str(), estimate))
                }),
        )
    }

    fn is_finished(&self) -> bool {
        self.completion_reported
            && self.bus.is_empty()
            && self
                .modules
                .iter()
                .all(|module| matches!(module.state, RenderState::Rendered { refresh_at: None }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Shell;
    use crate::test::default_context;
    use frame::read_frames;
    use proptest::prelude::*;
    use std::cell::Cell;
    use std::collections::{HashMap, VecDeque};
    use std::io::Write;

    const TEST_TERMINAL_WIDTH: usize = 20;
    const FRAME_FIELD_COUNT: usize = 3;
    const DEFAULT_HORIZON_MILLISECONDS: u64 = 60_000;
    const SILENT_DISCONNECT_TIMEOUT: Duration = Duration::from_secs(60);
    const UNLIMITED_WORKERS: usize = usize::MAX;
    const DEFAULT_FALLBACK_MILLISECONDS: u64 = 50;
    const PROPERTY_CASE_COUNT: u32 = 512;
    const MAXIMUM_RENDER_MILLISECONDS: u64 = 3_000;
    const MAXIMUM_TIMETABLE_MILLISECONDS: u64 = 300;
    const MAXIMUM_UNMEASURED_MILLISECONDS: u64 = 120;

    fn context(configuration: toml::Table) -> Context<'static> {
        let mut context = default_context().set_config(configuration);
        context.width = TEST_TERMINAL_WIDTH;
        context
    }

    fn milliseconds(value: u64) -> Duration {
        Duration::from_millis(value)
    }

    fn elapsed_milliseconds(elapsed: Duration) -> u64 {
        elapsed
            .as_millis()
            .try_into()
            .expect("test duration fits in u64")
    }

    /// Fixture entries name a module and list its (duration in milliseconds,
    /// output) pairs. The last pair repeats after the sequence is exhausted.
    type Script<'script> = &'script [(&'script str, &'script [(u64, &'script str)])];

    struct ModuleScript<'script> {
        remaining: &'script [(u64, &'script str)],
        first_completion_milliseconds: Option<u64>,
    }

    impl<'script> ModuleScript<'script> {
        fn next_render(&mut self) -> (u64, &'script str) {
            let (&render, remaining) = self
                .remaining
                .split_first()
                .expect("every simulated module has at least one render");
            if !remaining.is_empty() {
                self.remaining = remaining;
            }
            render
        }
    }

    struct ScheduledRender<'script> {
        completes_at: Instant,
        position: usize,
        text: &'script str,
    }

    impl ScheduledRender<'_> {
        fn result(&self) -> Rendered {
            Rendered {
                position: self.position,
                segments: if self.text.is_empty() {
                    Vec::new()
                } else {
                    Segment::from_text(None, self.text)
                },
            }
        }
    }

    struct Simulation<'simulation> {
        module_scripts: Vec<ModuleScript<'simulation>>,
        worker_count: usize,
        now: &'simulation Cell<Instant>,
        rendering: Vec<ScheduledRender<'simulation>>,
        queued: VecDeque<usize>,
        started_at: Instant,
        horizon: Duration,
    }

    impl<'simulation> Simulation<'simulation> {
        fn new(
            plan: &PromptPlan<'_>,
            script: Script<'simulation>,
            now: &'simulation Cell<Instant>,
            settings: &SimulationSettings,
        ) -> Self {
            let module_scripts = plan
                .modules
                .iter()
                .map(|module| {
                    let remaining = script
                        .iter()
                        .rev()
                        .find(|(name, _)| *name == module.as_str())
                        .map(|(_, renders)| *renders)
                        .unwrap_or_else(|| panic!("missing script for {module}"));
                    assert!(!remaining.is_empty(), "empty script for {module}");
                    ModuleScript {
                        remaining,
                        first_completion_milliseconds: None,
                    }
                })
                .collect();
            Self {
                module_scripts,
                worker_count: settings.worker_count,
                now,
                rendering: Vec::with_capacity(plan.modules.len().min(settings.worker_count)),
                queued: VecDeque::with_capacity(plan.modules.len()),
                started_at: now.get(),
                horizon: milliseconds(settings.horizon_milliseconds),
            }
        }

        fn start_queued(&mut self) {
            while self.rendering.len() < self.worker_count {
                let Some(position) = self.queued.pop_front() else {
                    break;
                };
                let (duration_milliseconds, text) = self.module_scripts[position].next_render();
                self.rendering.push(ScheduledRender {
                    completes_at: self.now.get() + milliseconds(duration_milliseconds),
                    position,
                    text,
                });
            }
        }

        fn advance_clock(&self, deadline: Option<Instant>) -> Instant {
            let next = self
                .rendering
                .iter()
                .map(|render| render.completes_at)
                .chain(deadline)
                .min()
                .expect("the stream waits for a module that never lands");
            let now = next.max(self.now.get());
            assert!(
                now - self.started_at <= self.horizon + SILENT_DISCONNECT_TIMEOUT,
                "the stream exceeded its silence timeout after its shell stopped reading"
            );
            self.now.set(now);
            now
        }

        fn take_completed(&mut self, now: Instant) -> Vec<Rendered> {
            let started_at = self.started_at;
            let module_scripts = &mut self.module_scripts;
            let mut completed = Vec::new();
            self.rendering.retain(|render| {
                if render.completes_at > now {
                    return true;
                }
                module_scripts[render.position]
                    .first_completion_milliseconds
                    .get_or_insert_with(|| elapsed_milliseconds(render.completes_at - started_at));
                completed.push(render.result());
                false
            });
            completed
        }
    }

    impl Renderer for Simulation<'_> {
        fn render(&mut self, position: usize) {
            self.queued.push_back(position);
            self.start_queued();
        }

        fn wait(&mut self, deadline: Option<Instant>) -> (Instant, Vec<Rendered>) {
            let now = self.advance_clock(deadline);
            let completed = self.take_completed(now);
            self.start_queued();
            (now, completed)
        }
    }

    #[derive(Debug)]
    struct RecordedFrame {
        elapsed_milliseconds: u64,
        fields: [String; FRAME_FIELD_COUNT],
    }

    impl RecordedFrame {
        fn keyword(&self) -> &str {
            let [keyword, ..] = &self.fields;
            keyword
        }

        fn prompt(&self) -> Option<(&str, &str)> {
            let [keyword, main, right] = &self.fields;
            (keyword == "PROMPT").then_some((main.as_str(), right.as_str()))
        }

        fn timings(&self) -> Option<&str> {
            let [keyword, timings, _] = &self.fields;
            (keyword == "COMPLETE").then_some(timings.as_str())
        }
    }

    struct Recording<'clock> {
        started_at: Instant,
        now: &'clock Cell<Instant>,
        horizon: Duration,
        bytes: Vec<u8>,
        frames: Vec<RecordedFrame>,
    }

    impl Sink for Recording<'_> {
        fn send(&mut self, frame: Frame) -> io::Result<()> {
            let elapsed = self.now.get() - self.started_at;
            if elapsed > self.horizon {
                return Err(io::ErrorKind::BrokenPipe.into());
            }
            self.bytes.clear();
            frame.write_to(&mut self.bytes)?;
            let elapsed_milliseconds = elapsed_milliseconds(elapsed);
            self.frames.extend(
                read_frames(&self.bytes)
                    .into_iter()
                    .map(|fields| RecordedFrame {
                        elapsed_milliseconds,
                        fields,
                    }),
            );
            Ok(())
        }
    }

    struct SimulationSettings {
        wall_time_milliseconds: u64,
        horizon_milliseconds: u64,
        worker_count: usize,
    }

    impl Default for SimulationSettings {
        fn default() -> Self {
            Self {
                wall_time_milliseconds: 0,
                horizon_milliseconds: DEFAULT_HORIZON_MILLISECONDS,
                worker_count: UNLIMITED_WORKERS,
            }
        }
    }

    struct SimulationResult {
        frames: Vec<RecordedFrame>,
        first_completions: HashMap<String, u64>,
    }

    fn simulate_in(
        settings: SimulationSettings,
        context: &Context,
        timings: &str,
        script: Script,
    ) -> SimulationResult {
        let plan = Plan::new(context, PROMPT_TARGETS);
        let started_at = Instant::now();
        let now = Cell::new(started_at);
        let mut simulation = Simulation::new(&plan, script, &now, &settings);
        let mut recording = Recording {
            started_at,
            now: &now,
            horizon: milliseconds(settings.horizon_milliseconds),
            bytes: Vec::new(),
            frames: Vec::new(),
        };
        let clock = Clock {
            started_at,
            wall_time_at_start: milliseconds(settings.wall_time_milliseconds),
        };
        recording.send(Frame::Process).unwrap();
        let session = Session::new(context, &plan, timings::read(timings, &plan.modules), clock);
        if let Err(error) = session.run(&mut simulation, &mut recording) {
            assert_eq!(io::ErrorKind::BrokenPipe, error.kind());
        }
        let first_completions = plan
            .modules
            .iter()
            .zip(simulation.module_scripts)
            .filter_map(|(name, module)| {
                Some((name.clone(), module.first_completion_milliseconds?))
            })
            .collect();
        SimulationResult {
            frames: recording.frames,
            first_completions,
        }
    }

    fn simulate(context: &Context, timings: &str, script: Script) -> Vec<RecordedFrame> {
        simulate_in(SimulationSettings::default(), context, timings, script).frames
    }

    fn keywords(frames: &[RecordedFrame]) -> Vec<(u64, &str)> {
        frames
            .iter()
            .map(|frame| (frame.elapsed_milliseconds, frame.keyword()))
            .collect()
    }

    fn prompts(frames: &[RecordedFrame]) -> Vec<(u64, &str)> {
        frames
            .iter()
            .filter_map(|frame| {
                frame
                    .prompt()
                    .map(|(main, _)| (frame.elapsed_milliseconds, main))
            })
            .collect()
    }

    fn three_modules() -> Context<'static> {
        context(toml::toml! {
            add_newline = false
            format = "${custom.a}${custom.b}$character"
        })
    }

    #[test]
    fn the_prompt_is_drawn_once_where_drawing_it_sooner_would_flash() {
        let context = context(toml::toml! {
            add_newline = false
            format = "${custom.a}${custom.b}${custom.c}$character"
        });

        fn simulate_last_module(
            context: &Context,
            timings: &str,
            duration_milliseconds: u64,
        ) -> Vec<RecordedFrame> {
            simulate(
                context,
                timings,
                &[
                    ("character", &[(0, ">")]),
                    ("custom.a", &[(1, "a")]),
                    ("custom.b", &[(7, "b")]),
                    ("custom.c", &[(duration_milliseconds, "c")]),
                ],
            )
        }

        assert_eq!(
            vec![(16, "abc>")],
            prompts(&simulate_last_module(
                &context,
                "character=0,custom.a=1,custom.b=7,custom.c=16",
                16,
            )),
            "drawn sooner, the prompt would flash as c lands"
        );
        assert_eq!(
            vec![(7, "ab>"), (300, "abc>")],
            prompts(&simulate_last_module(
                &context,
                "character=0,custom.a=1,custom.b=7,custom.c=300",
                300,
            )),
            "but c, slow enough to look like a change of its own, gets a redraw"
        );
    }

    #[test]
    fn the_later_a_module_lands_the_less_the_bus_waits_for_others() {
        let context = three_modules();

        fn simulate_last_module(
            context: &Context,
            timings: &str,
            duration_milliseconds: u64,
        ) -> Vec<RecordedFrame> {
            simulate(
                context,
                timings,
                &[
                    ("character", &[(0, ">")]),
                    ("custom.a", &[(300, "a")]),
                    ("custom.b", &[(duration_milliseconds, "b")]),
                ],
            )
        }

        assert_eq!(
            vec![(0, ">"), (300, "a>"), (320, "ab>")],
            prompts(&simulate_last_module(
                &context,
                "character=0,custom.a=300,custom.b=320",
                320,
            )),
            "twenty milliseconds apart, three hundred in, a and b are drawn apart"
        );
        assert_eq!(
            vec![(0, ">"), (303, "ab>")],
            prompts(&simulate_last_module(
                &context,
                "character=0,custom.a=300,custom.b=303",
                303,
            )),
            "three apart, together"
        );
    }

    #[test]
    fn a_module_never_measured_is_waited_for_until_the_fallback() {
        let script: Script = &[
            ("character", &[(0, ">")]),
            ("custom.a", &[(10, "a")]),
            ("custom.b", &[(2000, "b")]),
        ];
        let not_adaptive = context(toml::toml! {
            add_newline = false
            format = "${custom.a}${custom.b}$character"
            async.bus.adaptive = false
        });
        let timings = "character=0,custom.a=10,custom.b=2000";

        assert_eq!(
            vec![(10, "a>"), (2000, "ab>")],
            prompts(&simulate(&three_modules(), timings, script)),
            "b, known to be slow, holds nothing back"
        );
        let unmeasured = vec![(50, "a>"), (2000, "ab>")];
        assert_eq!(unmeasured, prompts(&simulate(&three_modules(), "", script)));
        assert_eq!(
            unmeasured,
            prompts(&simulate(&not_adaptive, timings, script)),
            "without adaptive, whatever the previous prompt says"
        );
    }

    #[test]
    fn the_fallback_is_how_long_the_first_paint_waits_at_most() {
        let frames = simulate(
            &context(toml::toml! {
                add_newline = false
                format = "${custom.a}${custom.b}$character"
                async.bus.fallback = 30
            }),
            "",
            &[
                ("character", &[(0, ">")]),
                ("custom.a", &[(6, "a")]),
                ("custom.b", &[(2000, "b")]),
            ],
        );

        assert_eq!(vec![(30, "a>"), (2000, "ab>")], prompts(&frames));
    }

    #[test]
    fn a_module_that_changed_nothing_is_not_waited_for() {
        let script: Script = &[
            ("character", &[(0, ">")]),
            ("custom.a", &[(10, "a")]),
            ("custom.b", &[(15, "")]),
        ];

        assert_eq!(
            vec![(10, "a>")],
            prompts(&simulate(
                &three_modules(),
                "character=0,custom.a=10,custom.b",
                script
            ))
        );
        assert_eq!(
            vec![(15, "a>")],
            prompts(&simulate(
                &three_modules(),
                "character=0,custom.a=10,custom.b=15",
                script
            )),
            "unless the previous prompt says it changes the prompt"
        );
    }

    #[test]
    fn with_fewer_workers_than_modules_the_quickest_render_first() {
        let frames = simulate_in(
            SimulationSettings {
                worker_count: 1,
                ..SimulationSettings::default()
            },
            &three_modules(),
            "character=0,custom.a=1000,custom.b=5",
            &[
                ("character", &[(0, ">")]),
                ("custom.a", &[(1000, "a")]),
                ("custom.b", &[(5, "b")]),
            ],
        )
        .frames;

        assert_eq!(
            vec![(5, "b>"), (1005, "ab>")],
            prompts(&frames),
            "the plan puts a, the slowest, before b"
        );
    }

    #[test]
    fn the_right_prompt_is_drawn_with_the_main_one() {
        let frames = simulate(
            &context(toml::toml! {
                add_newline = false
                format = "$character"
                right_format = "${custom.right}"
            }),
            "character=0,custom.right=500",
            &[
                ("character", &[(0, ">")]),
                ("custom.right", &[(500, "right")]),
            ],
        );

        let drawn: Vec<_> = frames
            .iter()
            .filter_map(|frame| {
                frame
                    .prompt()
                    .map(|(main, right)| (frame.elapsed_milliseconds, main, right))
            })
            .collect();
        assert_eq!(vec![(0, ">", ""), (500, ">", "right")], drawn);
    }

    #[test]
    fn refreshes_fall_due_on_the_wall_clock_and_are_drawn_together() {
        let frames = simulate_in(
            SimulationSettings {
                wall_time_milliseconds: 300,
                horizon_milliseconds: 5_000,
                ..SimulationSettings::default()
            },
            &context(toml::toml! {
                add_newline = false
                format = "$time $memory_usage"
                time.disabled = false
                memory_usage.disabled = false
            }),
            "time=1,memory_usage=2",
            &[
                (
                    "time",
                    &[
                        (1, "t0"),
                        (1, "t1"),
                        (1, "t2"),
                        (1, "t3"),
                        (1, "t4"),
                        (1, "t5"),
                    ],
                ),
                ("memory_usage", &[(2, "m0"), (2, "m1")]),
            ],
        )
        .frames;

        assert_eq!(
            vec![
                (2, "t0 m0"),
                (701, "t1 m0"),
                (1701, "t2 m0"),
                (2701, "t3 m0"),
                (3701, "t4 m0"),
                (4702, "t5 m1")
            ],
            prompts(&frames),
            "the time turns over each second, and memory every five, both on \
             the wall clock, which reads whole seconds 700 milliseconds after one"
        );
    }

    #[test]
    fn a_refresh_that_changes_nothing_still_shows_the_stream_is_alive() {
        let frames = simulate(
            &context(toml::toml! {
                add_newline = false
                format = "$time"
                time.disabled = false
            }),
            "time=0",
            &[("time", &[(0, "12:00")])],
        );

        assert_eq!(
            vec![
                (0, "PROCESS"),
                (0, "PROMPT"),
                (0, "COMPLETE"),
                (1000, "HEARTBEAT"),
                (2000, "HEARTBEAT")
            ],
            keywords(&frames)[..5]
        );
    }

    #[test]
    fn only_the_first_render_of_a_module_is_reported() {
        let frames = simulate_in(
            SimulationSettings {
                horizon_milliseconds: 5_000,
                ..SimulationSettings::default()
            },
            &context(toml::toml! {
                add_newline = false
                format = "$time${custom.slow}"
                time.disabled = false
            }),
            "time=5,custom.slow=3500",
            &[("time", &[(1, "t0")]), ("custom.slow", &[(2500, "s")])],
        )
        .frames;

        let report = frames.iter().find_map(RecordedFrame::timings);
        assert_eq!(
            Some("custom.slow=3250,time=4"),
            report,
            "the time, rendering again unchanged while the slow module rendered, \
             is reported as it first rendered"
        );
    }

    #[test]
    fn a_stream_finishes_once_nothing_renders_again() {
        for (configuration, finishes) in [
            (toml::toml! { format = "$time" }, true),
            (
                toml::toml! { format = "$time"
                time.disabled = false },
                false,
            ),
            (
                toml::toml! { format = "$time"
                time.disabled = false
                time.time_format = "today" },
                true,
            ),
            (
                toml::toml! { format = "$time"
                time.disabled = false
                time.refresh = false },
                true,
            ),
            (
                toml::toml! { format = "$time"
                time.disabled = false
                async.refresh = false },
                true,
            ),
        ] {
            let frames = simulate_in(
                SimulationSettings {
                    horizon_milliseconds: 5_000,
                    ..SimulationSettings::default()
                },
                &context(configuration.clone()),
                "",
                &[("time", &[(0, "")])],
            )
            .frames;
            let last = frames
                .last()
                .map(|frame| (frame.elapsed_milliseconds, frame.keyword()));

            assert_eq!(
                finishes,
                last == Some((0, "COMPLETE")),
                "{configuration}: {last:?}"
            );
        }
    }

    #[test]
    fn a_custom_module_runs_again_as_often_as_it_is_configured_to() {
        let frames = simulate_in(
            SimulationSettings {
                horizon_milliseconds: 2_500,
                ..SimulationSettings::default()
            },
            &context(toml::toml! {
                add_newline = false
                format = "${custom.song}"
                [custom.song]
                command = "now-playing"
                when = true
                refresh = 1000
            }),
            "custom.song=3",
            &[("custom.song", &[(3, "first"), (3, "first"), (3, "second")])],
        )
        .frames;

        assert_eq!(
            vec![(3, "first"), (2003, "second")],
            prompts(&frames),
            "run again each second, it is redrawn once its output changes"
        );
        assert!(keywords(&frames).contains(&(1003, "HEARTBEAT")));
    }

    const GENERATED_MODULE_NAMES: [&str; 6] = [
        "custom.0", "custom.1", "custom.2", "custom.3", "custom.4", "custom.5",
    ];
    const MODULE_OUTPUTS: [&str; GENERATED_MODULE_NAMES.len()] = ["a", "b", "c", "d", "e", "f"];

    #[derive(Clone, Debug)]
    struct GeneratedModule {
        render_duration_milliseconds: u64,
        shows_output: bool,
        previous_estimate: Option<Estimate>,
    }

    fn generated_prompt_format(names: &[&str]) -> String {
        let capacity = names.iter().map(|name| name.len() + "${}".len()).sum();
        let mut format = String::with_capacity(capacity);
        for name in names {
            format.push_str("${");
            format.push_str(name);
            format.push('}');
        }
        format
    }

    fn simulate_generated(modules: &[GeneratedModule], worker_count: usize) -> SimulationResult {
        let names = &GENERATED_MODULE_NAMES[..modules.len()];
        let renders: Vec<_> = modules
            .iter()
            .zip(MODULE_OUTPUTS)
            .map(|(module, output)| {
                let output = if module.shows_output { output } else { "" };
                [(module.render_duration_milliseconds, output)]
            })
            .collect();
        let script: Vec<_> = names
            .iter()
            .zip(&renders)
            .map(|(&name, render)| (name, render.as_slice()))
            .collect();
        let timings = timings::report(
            names
                .iter()
                .zip(modules)
                .filter_map(|(&name, module)| Some((name, module.previous_estimate?))),
        );
        let mut configuration = toml::Table::new();
        configuration.insert("add_newline".into(), false.into());
        configuration.insert("format".into(), generated_prompt_format(names).into());
        simulate_in(
            SimulationSettings {
                worker_count,
                ..SimulationSettings::default()
            },
            &context(configuration),
            &timings,
            &script,
        )
    }

    struct VisibleModule {
        render_duration_milliseconds: u64,
        output: &'static str,
    }

    fn visible_modules(modules: &[GeneratedModule]) -> impl Iterator<Item = VisibleModule> + '_ {
        modules
            .iter()
            .zip(MODULE_OUTPUTS)
            .filter(|(module, _)| module.shows_output)
            .map(|(module, output)| VisibleModule {
                render_duration_milliseconds: module.render_duration_milliseconds,
                output,
            })
    }

    fn complete_prompt(modules: &[GeneratedModule]) -> String {
        visible_modules(modules)
            .map(|module| module.output)
            .collect()
    }

    fn any_estimate() -> impl Strategy<Value = Option<Estimate>> {
        prop_oneof![
            Just(None),
            Just(Some(Estimate::Unchanged)),
            (0_u64..MAXIMUM_RENDER_MILLISECONDS)
                .prop_map(|duration| Some(Estimate::Changed(milliseconds(duration)))),
        ]
    }

    fn generated_modules(
        durations: std::ops::Range<u64>,
    ) -> impl Strategy<Value = GeneratedModule> {
        (durations, any::<bool>(), any_estimate()).prop_map(
            |(render_duration_milliseconds, shows_output, previous_estimate)| GeneratedModule {
                render_duration_milliseconds,
                shows_output,
                previous_estimate,
            },
        )
    }

    /// Charge the first paint's wait and each redraw's bus size plus waiting time.
    fn drawn_timetable_cost(drawn: &[(u64, &str)], visible: &[VisibleModule]) -> Duration {
        let redraw_costs = drawn.windows(2).map(|pair| {
            let [(_, before), (drawn_at, after)] = pair else {
                unreachable!("adjacent prompts")
            };
            let first_change = visible
                .iter()
                .filter(|module| after.contains(module.output) && !before.contains(module.output))
                .map(|module| module.render_duration_milliseconds)
                .min()
                .expect("a redraw draws a change");
            bus::size(
                milliseconds(DEFAULT_FALLBACK_MILLISECONDS),
                milliseconds(first_change),
            ) + milliseconds(drawn_at - first_change)
        });
        milliseconds(drawn[0].0) + redraw_costs.sum::<Duration>()
    }

    fn minimum_timetable_cost(visible: &[VisibleModule]) -> Option<Duration> {
        let fallback = milliseconds(DEFAULT_FALLBACK_MILLISECONDS);
        let mut landings: Vec<_> = visible
            .iter()
            .map(|module| {
                let after = milliseconds(module.render_duration_milliseconds);
                testing::Landing {
                    after,
                    size: bus::size(fallback, after),
                }
            })
            .collect();
        landings.sort_unstable_by_key(|landing| landing.after);
        testing::every_timetable(bus::Passengers::Prompt, fallback, &landings)
            .into_iter()
            .map(|(_, cost)| cost)
            .min()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(PROPERTY_CASE_COUNT))]

        #[test]
        fn redraws_preserve_content_and_bound_visible_change_latency(
            modules in prop::collection::vec(
                generated_modules(0..MAXIMUM_RENDER_MILLISECONDS),
                1..=GENERATED_MODULE_NAMES.len(),
            ),
            worker_count in 1_usize..=GENERATED_MODULE_NAMES.len(),
        ) {
            let SimulationResult {
                frames,
                first_completions,
            } = simulate_generated(&modules, worker_count);
            let drawn = prompts(&frames);

            prop_assert_eq!(Some("COMPLETE"), frames.last().map(RecordedFrame::keyword));
            prop_assert_eq!(
                1,
                frames
                    .iter()
                    .filter(|frame| frame.keyword() == "COMPLETE")
                    .count()
            );
            prop_assert!(
                drawn[0].0 <= DEFAULT_FALLBACK_MILLISECONDS,
                "first drawn at {}",
                drawn[0].0
            );
            for pair in drawn.windows(2) {
                prop_assert_ne!(pair[0].1, pair[1].1);
            }
            for (index, (module, output)) in modules.iter().zip(MODULE_OUTPUTS).enumerate() {
                if !module.shows_output {
                    continue;
                }
                let completed_at = first_completions[GENERATED_MODULE_NAMES[index]];
                let shown_at = drawn
                    .iter()
                    .find(|(_, prompt)| prompt.contains(output))
                    .map(|&(drawn_at, _)| drawn_at);
                prop_assert!(
                    shown_at.is_some_and(|drawn_at| {
                        (completed_at..=completed_at + DEFAULT_FALLBACK_MILLISECONDS).contains(&drawn_at)
                    }),
                    "{output} completed at {completed_at}, shown at {shown_at:?}: {drawn:?}"
                );
            }
            let expected = complete_prompt(&modules);
            prop_assert_eq!(
                Some(expected.as_str()),
                drawn.last().map(|&(_, prompt)| prompt)
            );
        }

        #[test]
        fn true_estimates_make_the_cheapest_timetable(
            modules in prop::collection::vec(
                (0_u64..MAXIMUM_TIMETABLE_MILLISECONDS, any::<bool>()),
                1..=GENERATED_MODULE_NAMES.len(),
            ),
        ) {
            let modules: Vec<_> = modules
                .into_iter()
                .map(|(render_duration_milliseconds, shows_output)| GeneratedModule {
                    render_duration_milliseconds,
                    shows_output,
                    previous_estimate: Some(if shows_output {
                        Estimate::Changed(milliseconds(render_duration_milliseconds))
                    } else {
                        Estimate::Unchanged
                    }),
                })
                .collect();
            let frames = simulate_generated(&modules, UNLIMITED_WORKERS).frames;
            let drawn = prompts(&frames);
            let visible: Vec<_> = visible_modules(&modules).collect();
            prop_assert_eq!(
                minimum_timetable_cost(&visible),
                Some(drawn_timetable_cost(&drawn, &visible)),
                "{:?}",
                drawn
            );
        }

        #[test]
        fn modules_never_measured_are_waited_for_until_the_fallback(
            durations in prop::collection::vec(
                0_u64..MAXIMUM_UNMEASURED_MILLISECONDS,
                1..=GENERATED_MODULE_NAMES.len(),
            ),
        ) {
            let modules: Vec<_> = durations
                .iter()
                .map(|&render_duration_milliseconds| GeneratedModule {
                    render_duration_milliseconds,
                    shows_output: true,
                    previous_estimate: None,
                })
                .collect();
            let frames = simulate_generated(&modules, UNLIMITED_WORKERS).frames;
            let (first_paint, prompt) = prompts(&frames)[0];
            let completed: String = visible_modules(&modules)
                .filter(|module| module.render_duration_milliseconds <= first_paint)
                .map(|module| module.output)
                .collect();
            let expected_first_paint = durations
                .iter()
                .max()
                .map(|&last| last.min(DEFAULT_FALLBACK_MILLISECONDS));
            prop_assert_eq!(expected_first_paint, Some(first_paint));
            prop_assert_eq!(completed.as_str(), prompt);
        }
    }

    #[test]
    fn without_async_the_prompt_is_drawn_once_every_module_renders() {
        let context = context(toml::toml! {
            add_newline = false
            format = "$character"
            right_format = "$jobs"
            async.enabled = false
        });
        let mut output = Vec::new();
        run(&context, "character=3", &mut output).unwrap();

        assert_eq!(
            vec![
                [
                    "PROCESS".to_owned(),
                    std::process::id().to_string(),
                    String::new()
                ],
                [
                    "PROMPT".to_owned(),
                    print::get_prompt(&context),
                    String::new()
                ],
                [
                    "COMPLETE".to_owned(),
                    "character=3".to_owned(),
                    String::new()
                ],
            ],
            read_frames(&output),
        );
    }

    /// Simulate a reader disconnecting at a real-time deadline.
    struct ClosesAt {
        written: Vec<u8>,
        deadline: Instant,
    }

    impl Write for ClosesAt {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if Instant::now() >= self.deadline {
                return Err(io::ErrorKind::BrokenPipe.into());
            }
            self.written.write(bytes)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn a_stream_keeps_its_prompt_current_until_the_shell_stops_reading() {
        let context = context(toml::toml! {
            add_newline = false
            format = "$time"
            time.disabled = false
            time.time_format = "%T%.f"
        });
        let mut output = ClosesAt {
            written: Vec::new(),
            deadline: Instant::now() + milliseconds(1100),
        };

        let error = run(&context, "", &mut output).unwrap_err();
        assert_eq!(io::ErrorKind::BrokenPipe, error.kind());
        let prompts: Vec<String> = read_frames(&output.written)
            .into_iter()
            .filter(|[keyword, ..]| keyword == "PROMPT")
            .map(|[_, prompt, _]| prompt)
            .collect();
        assert!(prompts.len() >= 2, "the time turned over: {prompts:?}");
        assert!(prompts.windows(2).all(|pair| pair[0] != pair[1]));
    }

    #[test]
    fn a_stream_ends_on_the_prompts_starship_prints() {
        let directory = tempfile::tempdir().unwrap();
        let configuration = toml::toml! {
            format = "[$directory](bold)${custom.slow}$fill${custom.raw}$line_break$status$character"
            right_format = "${custom.slow}[$jobs](red)"
            [directory]
            format = "$path "
            [custom.slow]
            command = "echo '50% $HOME `tick`'"
            when = true
            format = "[$output](cyan) "
            [custom.raw]
            command = "echo '%~'"
            when = true
            unsafe_no_escape = true
            [status]
            disabled = false
        };
        for shell in [
            Shell::Zsh,
            Shell::Bash,
            Shell::Fish,
            Shell::Tcsh,
            Shell::Xonsh,
            Shell::Unknown,
        ] {
            let context_for_target = |target| {
                let mut context = context(configuration.clone());
                context.shell = shell;
                context.target = target;
                context.current_dir = directory.path().to_path_buf();
                context.properties.status_code = Some("1".to_owned());
                context.properties.jobs = 2;
                context
            };
            let mut output = Vec::new();
            run(&context_for_target(Target::Main), "", &mut output).unwrap();
            let frames = read_frames(&output);

            let [_, main, right] = frames
                .iter()
                .rfind(|[keyword, ..]| keyword == "PROMPT")
                .expect("a prompt is drawn");
            assert_eq!(
                &print::get_prompt(&context_for_target(Target::Main)),
                main,
                "main prompt under {shell:?}"
            );
            assert_eq!(
                &print::get_prompt(&context_for_target(Target::Right)),
                right,
                "right prompt under {shell:?}"
            );
            assert_eq!(
                Some("COMPLETE"),
                frames.last().map(|[keyword, ..]| keyword.as_str())
            );
        }
    }
    #[test]
    fn zero_refresh_period_has_no_deadline() {
        let started_at = Instant::now();
        let clock = Clock {
            started_at,
            wall_time_at_start: Duration::ZERO,
        };
        assert_eq!(None, clock.next_refresh_at(started_at, Duration::ZERO));
    }

    #[test]
    fn refresh_deadlines_follow_wall_clock_boundaries_strictly_after_now() {
        let started_at = Instant::now();
        let period = Duration::from_secs(1);
        let clock = Clock {
            started_at,
            wall_time_at_start: milliseconds(300),
        };
        let first_boundary = started_at + milliseconds(700);
        assert_eq!(
            Some(first_boundary),
            clock.next_refresh_at(started_at, period)
        );
        assert_eq!(
            Some(first_boundary + period),
            clock.next_refresh_at(first_boundary, period),
            "a refresh on a boundary advances to the following boundary"
        );
    }

    #[test]
    fn nanosecond_conversion_accepts_the_entire_duration_range() {
        let maximum_nanoseconds = Duration::MAX.as_nanos();
        assert_eq!(
            Some(Duration::MAX),
            duration_from_nanoseconds(maximum_nanoseconds)
        );
        assert_eq!(None, duration_from_nanoseconds(maximum_nanoseconds + 1));
    }

    #[test]
    fn overflowing_wall_time_has_no_refresh_deadline() {
        let started_at = Instant::now();
        let clock = Clock {
            started_at,
            wall_time_at_start: Duration::MAX,
        };
        assert_eq!(
            None,
            clock.next_refresh_at(started_at + Duration::from_nanos(1), Duration::from_secs(1),)
        );
    }

    #[test]
    fn long_refresh_periods_preserve_nearby_deadlines() {
        let started_at = Instant::now();
        let period = Duration::from_secs(u64::MAX);
        let one_nanosecond = Duration::from_nanos(1);
        let clock = Clock {
            started_at,
            wall_time_at_start: period - one_nanosecond,
        };
        assert_eq!(
            Some(started_at + one_nanosecond),
            clock.next_refresh_at(started_at, period),
            "the wall-clock remainder may exceed u64 nanoseconds"
        );
    }
}
