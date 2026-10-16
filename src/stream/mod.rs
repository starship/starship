//! Stream the main and right prompts as their modules render, and keep them
//! current while the shell waits for input.
//!
//! A shell reads `main\0right\0` each time the prompts change, until the
//! stream ends: once nothing will change them again, or the moment the shell
//! stops reading, whatever is still rendering.

mod bus;
#[cfg(unix)]
mod snapshot;
#[cfg(test)]
mod testing;
mod timings;

use std::io::{self, Write};
use std::time::{Duration, Instant, SystemTime};

use crate::context::{Context, Properties, Target};
use crate::modules;
use crate::plan::Plan;
use crate::print::{self, DUMB_TERMINAL_PROMPT};
use crate::segment::Segment;
use crate::workers::{self, Rendered, Workers};

use bus::{Bus, Estimate, Rendering, Span};

#[cfg(unix)]
pub use snapshot::detach;

/// The main prompt and the right one.
type Prompts = [String; 2];

/// Streams both prompts to standard output, redrawing them at most `redraws`
/// times after the first paint if that is limited.
pub fn stream(properties: Properties, redraws: Option<usize>) -> io::Result<()> {
    #[cfg(unix)]
    end_with_reader();
    let context = Context::new(properties, Target::Main);
    run(&context, redraws, &mut io::stdout().lock())
}

/// Streams both prompts to fish, which it signals as `shell`; see [`snapshot`].
#[cfg(unix)]
pub fn publish(properties: Properties, shell: i32) -> io::Result<()> {
    let context = Context::new(properties, Target::Main);
    let mut snapshot = snapshot::Snapshot::for_fish(nix::unistd::Pid::from_raw(shell))?;
    run(&context, None, &mut snapshot)?;
    snapshot.linger()
}

/// Ends the process the moment nothing reads the pipe on its standard output
/// any more, rather than when it next writes, which could be long after: it
/// renders on threads that may never finish, and has nothing left to do.
#[cfg(unix)]
fn end_with_reader() {
    use nix::errno::Errno;
    use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
    use nix::sys::stat::{SFlag, fstat};
    use std::os::fd::AsFd;

    let stdout = io::stdout();
    let is_pipe = fstat(stdout.as_fd()).is_ok_and(|status| {
        SFlag::from_bits_truncate(status.st_mode) & SFlag::S_IFMT == SFlag::S_IFIFO
    });
    let watching = is_pipe.then(|| {
        std::thread::Builder::new()
            .name("stream-reader".to_owned())
            .spawn(move || {
                // The writing end of a pipe polls readable, or in error, once
                // nothing reads it.
                let mut reader = [PollFd::new(stdout.as_fd(), PollFlags::POLLIN)];
                while poll(&mut reader, PollTimeout::NONE) == Err(Errno::EINTR) {}
                std::process::exit(0)
            })
    });
    if let Some(Err(error)) = watching {
        log::debug!("Unable to watch for the shell to stop reading: {error}");
    }
}

fn run(context: &Context, redraws: Option<usize>, output: &mut impl Sink) -> io::Result<()> {
    if print::is_dumb_terminal(context) {
        log::error!("Under a 'dumb' terminal (TERM=dumb).");
        return output.draw(&[DUMB_TERMINAL_PROMPT, DUMB_TERMINAL_PROMPT].map(str::to_owned));
    }

    let plan = Plan::new(context, [Target::Main, Target::Right]);
    let timings = timings::path(context).and_then(timings::Writer::claim);
    let estimates = timings::load(timings.as_ref(), &plan.modules);
    let waiting = estimates
        .iter()
        .filter(|&&estimate| Estimate::waits(estimate));
    let threads = crate::num_module_threads(plan.modules.len(), Some(waiting.count()));
    workers::with_workers(context, &plan.modules, threads, |workers| {
        Session::new(context, &plan, estimates, redraws, Clock::now(), timings)
            .run(&mut &*workers, output)
    })
}

/// Where a stream draws its prompts.
trait Sink {
    /// Draws `prompts` in place of those drawn before.
    fn draw(&mut self, prompts: &Prompts) -> io::Result<()>;
}

impl<W: Write> Sink for W {
    fn draw(&mut self, prompts: &Prompts) -> io::Result<()> {
        self.write_all(encode(prompts).as_bytes())?;
        self.flush()
    }
}

/// `prompts` as a shell reads them: each followed by a NUL, so that any NUL
/// within one, which a terminal draws as nothing anyway, is left out.
fn encode(prompts: &Prompts) -> String {
    prompts
        .iter()
        .flat_map(|prompt| prompt.split('\0').chain(["\0"]))
        .collect()
}

/// What renders modules for a stream: workers, or a simulation of them.
trait Renderer {
    fn render(&mut self, position: usize);

    /// Waits until a module renders, or until `deadline` if there is one, and
    /// returns when it stopped waiting and the modules rendered by then; or
    /// nothing, once the stream is to end, as a simulation does at its horizon.
    fn wait(&mut self, deadline: Option<Instant>) -> Option<(Instant, Vec<Rendered>)>;
}

impl Renderer for &Workers<'_> {
    fn render(&mut self, position: usize) {
        Workers::render(self, position);
    }

    fn wait(&mut self, deadline: Option<Instant>) -> Option<(Instant, Vec<Rendered>)> {
        let rendered = Workers::wait(self, deadline);
        Some((Instant::now(), rendered))
    }
}

/// The wall clock as of the start, followed on the monotonic one, so that
/// refreshes fall due as the wall clock turns over.
struct Clock {
    start: Instant,
    /// Since the Unix epoch, at the start.
    wall: Duration,
}

impl Clock {
    fn now() -> Self {
        Self {
            start: Instant::now(),
            wall: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default(),
        }
    }

    /// The first instant after `now` at which the wall clock reads a multiple
    /// of `period`, unless it has none, being zero.
    fn next(&self, now: Instant, period: Span) -> Option<Instant> {
        let period = period.get().as_nanos();
        let wall = (self.wall + (now - self.start)).as_nanos();
        let left = period - wall.checked_rem(period)?;
        Some(now + Duration::from_nanos(u64::try_from(left).ok()?))
    }

    fn next_refresh(&self, now: Instant, refresh: &modules::Refresh) -> Option<Instant> {
        match refresh {
            modules::Refresh::Every(period) => self.next(now, Span::from(*period)),
            modules::Refresh::Time(schedule) => {
                let wall = self.wall + (now - self.start);
                let timestamp = jiff::Timestamp::new(
                    i64::try_from(wall.as_secs()).ok()?,
                    wall.subsec_nanos() as i32,
                )
                .ok()?;
                now.checked_add(schedule.delay(timestamp)?)
            }
        }
    }
}

/// How the bus plans.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Planning {
    /// From what each module did for the previous prompt.
    Adaptive,
    /// Waiting for every module as for one never measured.
    Fallback,
}

/// A module of the plan, as the stream shows it.
struct Slot {
    shown: Vec<Segment>,
    /// When it renders again while the prompt is shown, if it does.
    refresh: Option<modules::Refresh>,
    /// What it is expected to do: what it did for the previous prompt, until
    /// it renders for this one.
    estimate: Option<Estimate>,
    state: State,
}

#[derive(Clone, Copy)]
enum State {
    /// Rendering for this prompt, since then.
    Rendering(Instant),
    /// Rendering again, since then.
    Refreshing(Instant),
    /// Rendered, to render again then, if it ever does.
    Rendered(Option<Instant>),
}

impl Slot {
    /// Since when it has been rendering, if it is.
    fn since(&self) -> Option<Instant> {
        match self.state {
            State::Rendering(since) | State::Refreshing(since) => Some(since),
            State::Rendered(_) => None,
        }
    }

    /// When it renders again, if it has rendered and ever does.
    fn due(&self) -> Option<Instant> {
        match self.state {
            State::Rendered(due) => due,
            State::Rendering(_) | State::Refreshing(_) => None,
        }
    }

    /// Starts rendering again if it is due to by `now`.
    fn refresh(&mut self, now: Instant) -> bool {
        let due = self.due().is_some_and(|due| due <= now);
        if due {
            self.state = State::Refreshing(now);
        }
        due
    }

    /// Shows `segments`, rendered by `now`, and returns how long they took if
    /// they change what it shows. Only its render for this prompt measures
    /// what it does for the next.
    fn land(&mut self, segments: Vec<Segment>, now: Instant, clock: &Clock) -> Option<Span> {
        let since = self.since()?;
        let measured = if self.shown == segments {
            Estimate::Unchanged
        } else {
            self.shown = segments;
            Estimate::Changed(Span::from(now - since))
        };
        if let State::Rendering(_) = self.state {
            self.estimate = Some(Estimate::after(self.estimate, measured));
        }
        self.state = State::Rendered(
            self.refresh
                .as_ref()
                .and_then(|refresh| clock.next_refresh(now, refresh)),
        );
        match measured {
            Estimate::Changed(took) => Some(took),
            Estimate::Unchanged => None,
        }
    }
}

struct Session<'context> {
    context: &'context Context<'context>,
    plan: &'context Plan<'context, 2>,
    clock: Clock,
    planning: Planning,
    slots: Vec<Slot>,
    bus: Bus,
    /// The prompts last drawn, once any have been.
    drawn: Option<Prompts>,
    /// How many more times the shell can draw them, if that is limited.
    draws: Option<usize>,
    /// Where to keep what the modules did, for the next prompt.
    timings: Option<timings::Writer>,
}

impl<'context> Session<'context> {
    fn new(
        context: &'context Context<'context>,
        plan: &'context Plan<'context, 2>,
        estimates: Vec<Option<Estimate>>,
        redraws: Option<usize>,
        clock: Clock,
        timings: Option<timings::Writer>,
    ) -> Self {
        let configuration = &context.root_config.asynchronous;
        let planning = if configuration.enabled && configuration.bus.adaptive {
            Planning::Adaptive
        } else {
            Planning::Fallback
        };
        let slots = plan
            .modules
            .iter()
            .zip(estimates)
            .map(|(module, estimate)| Slot {
                shown: Vec::new(),
                refresh: modules::refresh(module, context).filter(|_| configuration.refresh),
                estimate,
                state: State::Rendering(clock.start),
            })
            .collect();
        Self {
            context,
            plan,
            bus: Bus::new(
                clock.start,
                configuration.bus.fallback().into(),
                plan.modules.len(),
            ),
            clock,
            planning,
            slots,
            drawn: None,
            draws: redraws.map(|redraws| redraws.saturating_add(1)),
            timings,
        }
    }

    fn run(&mut self, renderer: &mut impl Renderer, output: &mut impl Sink) -> io::Result<()> {
        // When modules outnumber workers, those never measured render first,
        // then those expected to change the prompt soonest.
        let mut order: Vec<usize> = (0..self.slots.len()).collect();
        order.sort_by_key(|&position| self.slots[position].estimate);
        for position in order {
            renderer.render(position);
        }
        // Shown until they render, and worked out while they do.
        for (slot, module) in self.slots.iter_mut().zip(&self.plan.modules) {
            if let Some(approximation) = modules::approximate(module, self.context) {
                slot.shown = approximation.segments;
            }
        }

        let mut now = self.clock.start;
        loop {
            if self
                .departure(now)
                .is_some_and(|departure| departure <= now)
            {
                self.draw(output)?;
            }
            // A shell that can draw only so often is done with them complete.
            if self.draws.is_some() && self.rendered() && self.bus.is_empty() {
                return Ok(());
            }
            for (position, slot) in self.slots.iter_mut().enumerate() {
                if slot.refresh(now) {
                    renderer.render(position);
                }
            }

            let departure = self.departure(now);
            let deadline = self
                .slots
                .iter()
                .filter_map(Slot::due)
                .chain(departure)
                .min();
            // Nothing will change the prompts again.
            if deadline.is_none() && self.slots.iter().all(|slot| slot.since().is_none()) {
                return Ok(());
            }
            let Some((then, rendered)) = renderer.wait(deadline) else {
                return Ok(());
            };
            now = then;
            let unrendered = !self.rendered();
            for Rendered { position, segments } in rendered {
                if let Some(took) = self.slots[position].land(segments, now, &self.clock) {
                    self.bus.board(now, took);
                }
            }
            if unrendered
                && self.rendered()
                && let Some(timings) = &self.timings
            {
                timings.save(&self.report());
            }
        }
    }

    /// When the bus leaves, if anything is aboard.
    fn departure(&mut self, now: Instant) -> Option<Instant> {
        // Without async, the first paint waits indefinitely for every module,
        // including timeout-exempt commands. A shell's last draw is complete too.
        if (!self.context.root_config.asynchronous.enabled || self.draws == Some(1))
            && !self.rendered()
        {
            return None;
        }
        let adaptive = self.planning == Planning::Adaptive;
        let rendering = self.slots.iter().filter_map(|slot| {
            Some(Rendering {
                since: slot.since()?,
                estimate: slot.estimate.filter(|_| adaptive),
            })
        });
        self.bus.departure(now, rendering)
    }

    fn draw(&mut self, output: &mut impl Sink) -> io::Result<()> {
        self.bus.leave();
        let prompts = self
            .plan
            .prompts(&|position| &self.slots[position].shown, self.context);
        // Modules can change without changing what they compose.
        if self.drawn.as_ref() != Some(&prompts) {
            output.draw(&prompts)?;
            self.drawn = Some(prompts);
            self.draws = self.draws.map(|draws| draws.saturating_sub(1));
        }
        Ok(())
    }

    /// Whether every module has rendered for this prompt.
    fn rendered(&self) -> bool {
        self.slots
            .iter()
            .all(|slot| !matches!(slot.state, State::Rendering(_)))
    }

    /// What each module did for this prompt, for the next.
    fn report(&self) -> String {
        timings::report(
            self.plan
                .modules
                .iter()
                .zip(&self.slots)
                .filter_map(|(module, slot)| Some((module.as_str(), slot.estimate?))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Shell;
    use crate::test::default_context;
    use proptest::prelude::*;
    use std::cell::Cell;
    use std::collections::VecDeque;
    use std::path::PathBuf;

    const FALLBACK: u64 = 50;
    const EVERY_WORKER: usize = usize::MAX;

    fn context(configuration: toml::Table) -> Context<'static> {
        let mut context = default_context().set_config(configuration);
        context.width = 20;
        context
    }

    fn milliseconds(milliseconds: u64) -> Duration {
        Duration::from_millis(milliseconds)
    }

    fn elapsed(since: Instant, now: Instant) -> u64 {
        (now - since).as_millis().try_into().unwrap()
    }

    /// Each module's renders, as how many milliseconds each takes and what it
    /// shows; the last one repeats.
    type Script<'a> = &'a [(&'a str, &'a [(u64, &'a str)])];

    /// Workers rendering on a simulated clock, as many at once as there are,
    /// until a horizon.
    struct Simulation<'a> {
        renders: Vec<&'a [(u64, &'a str)]>,
        workers: usize,
        now: &'a Cell<Instant>,
        start: Instant,
        horizon: Instant,
        rendering: Vec<(Instant, usize, &'a str)>,
        queued: VecDeque<usize>,
        /// When each module first rendered.
        first: Vec<Option<u64>>,
        /// Whether the stream was still going at the horizon.
        reached_horizon: bool,
    }

    impl<'a> Simulation<'a> {
        fn new(
            plan: &Plan<'_, 2>,
            script: Script<'a>,
            now: &'a Cell<Instant>,
            settings: &Settings,
        ) -> Self {
            let renders = plan
                .modules
                .iter()
                .map(|module| {
                    let (_, renders) = script
                        .iter()
                        .rfind(|(name, _)| name == module)
                        .unwrap_or_else(|| panic!("no script for {module}"));
                    assert!(!renders.is_empty(), "an empty script for {module}");
                    *renders
                })
                .collect();
            Self {
                renders,
                workers: settings.workers,
                now,
                start: now.get(),
                horizon: now.get() + milliseconds(settings.horizon),
                rendering: Vec::new(),
                queued: VecDeque::new(),
                first: vec![None; plan.modules.len()],
                reached_horizon: false,
            }
        }

        fn start_queued(&mut self) {
            while self.rendering.len() < self.workers
                && let Some(position) = self.queued.pop_front()
            {
                let renders = &mut self.renders[position];
                let &(took, text) = renders.first().expect("a render");
                if renders.len() > 1 {
                    *renders = &renders[1..];
                }
                self.rendering
                    .push((self.now.get() + milliseconds(took), position, text));
            }
        }
    }

    impl Renderer for Simulation<'_> {
        fn render(&mut self, position: usize) {
            self.queued.push_back(position);
            self.start_queued();
        }

        fn wait(&mut self, deadline: Option<Instant>) -> Option<(Instant, Vec<Rendered>)> {
            let next = self
                .rendering
                .iter()
                .map(|&(lands, ..)| lands)
                .chain(deadline)
                .min()
                .expect("the stream waits for nothing at all");
            if next > self.horizon {
                self.reached_horizon = true;
                return None;
            }
            let now = next.max(self.now.get());
            self.now.set(now);
            let (landed, rendering) = self
                .rendering
                .drain(..)
                .partition::<Vec<_>, _>(|&(lands, ..)| lands <= now);
            self.rendering = rendering;
            let rendered = landed
                .into_iter()
                .map(|(lands, position, text)| {
                    self.first[position].get_or_insert(elapsed(self.start, lands));
                    let segments = if text.is_empty() {
                        Vec::new()
                    } else {
                        Segment::from_text(None, text)
                    };
                    Rendered { position, segments }
                })
                .collect();
            self.start_queued();
            Some((now, rendered))
        }
    }

    /// What a shell is drawn, and when.
    struct Recording<'a> {
        start: Instant,
        now: &'a Cell<Instant>,
        drawn: Vec<(u64, Prompts)>,
    }

    impl Sink for Recording<'_> {
        fn draw(&mut self, prompts: &Prompts) -> io::Result<()> {
            self.drawn
                .push((elapsed(self.start, self.now.get()), prompts.clone()));
            Ok(())
        }
    }

    struct Settings {
        /// The wall clock at the start, in milliseconds since the Unix epoch.
        wall: u64,
        horizon: u64,
        workers: usize,
        redraws: Option<usize>,
        timings: Option<PathBuf>,
    }

    impl Default for Settings {
        fn default() -> Self {
            Self {
                wall: 0,
                horizon: 60_000,
                workers: EVERY_WORKER,
                redraws: None,
                timings: None,
            }
        }
    }

    struct Simulated {
        drawn: Vec<(u64, Prompts)>,
        /// When each module first rendered, by position in the plan.
        first: Vec<Option<u64>>,
        report: String,
        /// Whether the stream ended before the horizon.
        ended: bool,
    }

    impl Simulated {
        /// When each main prompt was drawn, and what it was.
        fn mains(&self) -> Vec<(u64, &str)> {
            self.drawn
                .iter()
                .map(|(drawn, [main, _])| (*drawn, main.as_str()))
                .collect()
        }
    }

    fn simulate_in(
        settings: Settings,
        context: &Context,
        timings: &str,
        script: Script,
    ) -> Simulated {
        let plan = Plan::new(context, [Target::Main, Target::Right]);
        let start = Instant::now();
        let now = Cell::new(start);
        let mut simulation = Simulation::new(&plan, script, &now, &settings);
        let mut recording = Recording {
            start,
            now: &now,
            drawn: Vec::new(),
        };
        let clock = Clock {
            start,
            wall: milliseconds(settings.wall),
        };
        let estimates = timings::read(timings, &plan.modules);
        let mut session = Session::new(
            context,
            &plan,
            estimates,
            settings.redraws,
            clock,
            settings.timings.and_then(timings::Writer::claim),
        );
        session.run(&mut simulation, &mut recording).unwrap();
        Simulated {
            drawn: recording.drawn,
            first: simulation.first,
            report: session.report(),
            ended: !simulation.reached_horizon,
        }
    }

    fn three_modules(mut configuration: toml::Table) -> Context<'static> {
        configuration.insert("add_newline".into(), false.into());
        configuration.insert("format".into(), "${custom.a}${custom.b}$character".into());
        context(configuration)
    }

    #[test]
    fn the_prompt_is_drawn_as_configured_for_what_the_previous_prompt_says() {
        let slow: Script = &[
            ("character", &[(0, ">")]),
            ("custom.a", &[(10, "a")]),
            ("custom.b", &[(2000, "b")]),
        ];
        let blank: Script = &[
            ("character", &[(0, ">")]),
            ("custom.a", &[(10, "a")]),
            ("custom.b", &[(15, "")]),
        ];
        let measured = "character=0,custom.a=10,custom.b=2000";
        let cases = [
            (
                "b, known to be slow, holds nothing back",
                toml::Table::new(),
                measured,
                slow,
                vec![(10, "a>"), (2000, "ab>")],
            ),
            (
                "b, never measured, is waited for until the fallback",
                toml::Table::new(),
                "",
                slow,
                vec![(50, "a>"), (2000, "ab>")],
            ),
            (
                "without adaptive, whatever the previous prompt says",
                toml::toml! { async.bus.adaptive = false },
                measured,
                slow,
                vec![(50, "a>"), (2000, "ab>")],
            ),
            (
                "the fallback is how long the first paint waits at most",
                toml::toml! { async.bus.fallback = 30 },
                "",
                slow,
                vec![(30, "a>"), (2000, "ab>")],
            ),
            (
                "without async, the first paint waits for every module",
                toml::toml! { async.enabled = false },
                measured,
                slow,
                vec![(2000, "ab>")],
            ),
            (
                "b, which changed nothing, is not waited for",
                toml::Table::new(),
                "character=0,custom.a=10,custom.b",
                blank,
                vec![(10, "a>")],
            ),
            (
                "unless the previous prompt says it changes the prompt",
                toml::Table::new(),
                "character=0,custom.a=10,custom.b=15",
                blank,
                vec![(15, "a>")],
            ),
        ];
        for (why, configuration, timings, script, expected) in cases {
            let simulated = simulate_in(
                Settings::default(),
                &three_modules(configuration),
                timings,
                script,
            );
            assert_eq!(expected, simulated.mains(), "{why}");
        }
    }

    #[test]
    fn without_async_the_first_paint_waits_beyond_the_largest_planning_span() {
        let after_a_day = Span::MAX.get().as_millis() as u64 + 1;
        let simulated = simulate_in(
            Settings {
                horizon: after_a_day + 1,
                ..Settings::default()
            },
            &three_modules(toml::toml! { async.enabled = false }),
            "character=0,custom.a=10,custom.b=2000",
            &[
                ("character", &[(0, ">")]),
                ("custom.a", &[(10, "a")]),
                ("custom.b", &[(after_a_day, "b")]),
            ],
        );

        assert_eq!(vec![(after_a_day, "ab>")], simulated.mains());
        assert!(simulated.ended);
    }

    #[test]
    fn without_async_an_unfinished_render_never_exposes_a_partial_prompt() {
        let after_a_day = Span::MAX.get().as_millis() as u64 + 1;
        let simulated = simulate_in(
            Settings {
                horizon: after_a_day,
                ..Settings::default()
            },
            &three_modules(toml::toml! { async.enabled = false }),
            "",
            &[
                ("character", &[(0, ">")]),
                ("custom.a", &[(10, "a")]),
                ("custom.b", &[(after_a_day + 1, "b")]),
            ],
        );

        assert!(simulated.drawn.is_empty());
        assert!(!simulated.ended, "still waiting for the unfinished module");
    }

    #[test]
    fn without_async_refreshes_still_use_the_configured_waiting_budget() {
        let simulated = simulate_in(
            Settings {
                horizon: 199,
                ..Settings::default()
            },
            &context(toml::toml! {
                add_newline = false
                format = "${custom.fast}${custom.slow}"
                async.enabled = false
                async.bus.fallback = 10
                [custom.fast]
                command = "fast"
                when = true
                refresh = 100
                [custom.slow]
                command = "slow"
                when = true
                refresh = 100
            }),
            "",
            &[
                ("custom.fast", &[(1, "a"), (1, "A")]),
                ("custom.slow", &[(1, "b"), (45, "B")]),
            ],
        );

        assert_eq!(vec![(1, "ab"), (110, "Ab"), (145, "AB")], simulated.mains());
        assert!(!simulated.ended, "both refresh again at two hundred");
    }

    #[test]
    fn with_fewer_workers_than_modules_the_quickest_render_first() {
        let simulated = simulate_in(
            Settings {
                workers: 1,
                ..Settings::default()
            },
            &three_modules(toml::Table::new()),
            "character=0,custom.a=1000,custom.b=5",
            &[
                ("character", &[(0, ">")]),
                ("custom.a", &[(1000, "a")]),
                ("custom.b", &[(5, "b")]),
            ],
        );

        assert_eq!(
            vec![(5, "b>"), (1005, "ab>")],
            simulated.mains(),
            "the plan puts a, the slowest, before b"
        );
    }

    #[test]
    fn the_right_prompt_is_drawn_with_the_main_one() {
        let simulated = simulate_in(
            Settings::default(),
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

        assert_eq!(
            vec![
                (0, [">".to_owned(), String::new()]),
                (500, [">".to_owned(), "right".to_owned()])
            ],
            simulated.drawn
        );
    }

    #[test]
    fn refreshes_fall_due_on_the_wall_clock_and_are_drawn_together() {
        let simulated = simulate_in(
            Settings {
                wall: 300,
                horizon: 5_000,
                ..Settings::default()
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
        );

        assert_eq!(
            vec![
                (2, "t0 m0"),
                (701, "t1 m0"),
                (1701, "t2 m0"),
                (2701, "t3 m0"),
                (3701, "t4 m0"),
                (4702, "t5 m1")
            ],
            simulated.mains(),
            "the time turns over each second, and memory every five, both on \
             the wall clock, which reads whole seconds 700 milliseconds after one"
        );
    }

    #[test]
    fn time_refreshes_follow_configured_zone_boundaries_and_dst() {
        for (format, zone, wall, before, after) in [
            (
                "%F",
                "-4",
                "2026-10-16T03:59:59.750Z",
                "2026-10-15",
                "2026-10-16",
            ),
            ("%H", "+5.75", "2026-10-16T06:14:59.750Z", "11", "12"),
            (
                "%Z",
                "America/New_York",
                "2026-11-01T05:59:59.750Z",
                "EDT",
                "EST",
            ),
            (
                "%:z",
                "America/New_York",
                "2026-11-01T05:59:59.750Z",
                "-04:00",
                "-05:00",
            ),
        ] {
            let mut configuration = toml::toml! {
                add_newline = false
                format = "$time"
                time.disabled = false
            };
            let time = configuration["time"].as_table_mut().unwrap();
            time.insert("time_format".to_owned(), format.into());
            time.insert("utc_time_offset".to_owned(), zone.into());
            let wall: jiff::Timestamp = wall.parse().unwrap();
            let renders = [(1, before), (1, after)];
            let simulated = simulate_in(
                Settings {
                    wall: u64::try_from(wall.as_millisecond()).unwrap(),
                    horizon: 500,
                    ..Settings::default()
                },
                &context(configuration),
                "time=1",
                &[("time", &renders)],
            );
            assert_eq!(
                simulated.mains(),
                vec![(1, before), (251, after)],
                "{format} in {zone}"
            );
        }
    }

    #[test]
    fn modules_that_compose_the_same_prompt_again_draw_nothing() {
        let simulated = simulate_in(
            Settings {
                horizon: 2_500,
                ..Settings::default()
            },
            &context(toml::toml! {
                add_newline = false
                format = "${custom.left}${custom.right}"
                [custom.left]
                command = "left"
                when = true
                refresh = 1000
                [custom.right]
                command = "right"
                when = true
                refresh = 1000
            }),
            "custom.left=1,custom.right=1",
            &[
                ("custom.left", &[(1, "a"), (1, ""), (1, "a")]),
                ("custom.right", &[(1, "b"), (1, "ab"), (1, "b")]),
            ],
        );

        assert_eq!(vec![(1, "ab")], simulated.mains());
        assert!(!simulated.ended, "both run again each second");
    }

    #[test]
    fn only_the_first_render_of_a_module_is_reported() {
        let simulated = simulate_in(
            Settings {
                horizon: 5_000,
                ..Settings::default()
            },
            &context(toml::toml! {
                add_newline = false
                format = "$time${custom.slow}"
                time.disabled = false
            }),
            "time=5,custom.slow=3500",
            &[("time", &[(1, "t0")]), ("custom.slow", &[(2500, "s")])],
        );

        assert_eq!(
            "custom.slow=3250,time=4", simulated.report,
            "the time, rendering again unchanged while the slow module rendered, \
             is reported as it first rendered"
        );
    }

    #[test]
    fn the_report_is_kept_once_every_module_has_rendered_whatever_still_refreshes() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("timings");
        simulate_in(
            Settings {
                horizon: 1_000,
                timings: Some(path.clone()),
                ..Settings::default()
            },
            &context(toml::toml! {
                add_newline = false
                format = "${custom.refreshing}${custom.slow}"
                [custom.refreshing]
                command = "refreshing"
                when = true
                refresh = 100
                [custom.slow]
                command = "slow"
                when = true
            }),
            "",
            &[
                ("custom.refreshing", &[(0, "r"), (60_000, "never")]),
                ("custom.slow", &[(300, "s")]),
            ],
        );

        assert_eq!(
            "custom.refreshing=0,custom.slow=300",
            timings::body(&std::fs::read_to_string(path).unwrap()),
            "kept at three hundred milliseconds, while the refresh started at one hundred never lands"
        );
    }

    #[test]
    fn a_stream_ends_once_nothing_renders_again() {
        for (configuration, ends) in [
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
            let simulated = simulate_in(
                Settings {
                    horizon: 5_000,
                    ..Settings::default()
                },
                &context(configuration.clone()),
                "",
                &[("time", &[(0, "")])],
            );

            assert_eq!(ends, simulated.ended, "{configuration}");
        }
    }

    #[test]
    fn a_custom_module_runs_again_as_often_as_it_is_configured_to() {
        let simulated = simulate_in(
            Settings {
                horizon: 2_500,
                ..Settings::default()
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
        );

        assert_eq!(
            vec![(3, "first"), (2003, "second")],
            simulated.mains(),
            "run again each second, it is redrawn once its output changes"
        );
    }

    #[test]
    fn a_shell_that_draws_once_more_is_drawn_the_complete_prompt_and_nothing_after() {
        let context = context(toml::toml! {
            add_newline = false
            format = "$time${custom.a}${custom.b}$character"
            time.disabled = false
        });
        let script: Script = &[
            ("time", &[(0, "t")]),
            ("character", &[(0, ">")]),
            ("custom.a", &[(10, "a")]),
            ("custom.b", &[(20, "b"), (100, "B")]),
        ];
        let timings = "time=0,character=0,custom.a=10,custom.b=2000";
        let with_redraws = |redraws| {
            simulate_in(
                Settings {
                    redraws: Some(redraws),
                    ..Settings::default()
                },
                &context,
                timings,
                script,
            )
        };

        let once = with_redraws(1);
        assert_eq!(vec![(10, "ta>"), (20, "tab>")], once.mains());
        assert!(once.ended, "the time is not refreshed");
        let never = with_redraws(0);
        assert_eq!(vec![(20, "tab>")], never.mains());
        assert!(never.ended);
    }

    const GENERATED: [&str; 6] = [
        "custom.0", "custom.1", "custom.2", "custom.3", "custom.4", "custom.5",
    ];
    const OUTPUTS: [&str; GENERATED.len()] = ["a", "b", "c", "d", "e", "f"];

    #[derive(Clone, Debug)]
    struct Generated {
        took: u64,
        shows: bool,
        previous: Option<Estimate>,
    }

    fn simulate_generated(modules: &[Generated], settings: Settings) -> Simulated {
        let names = &GENERATED[..modules.len()];
        let renders: Vec<_> = modules
            .iter()
            .zip(OUTPUTS)
            .map(|(module, output)| [(module.took, if module.shows { output } else { "" })])
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
                .filter_map(|(&name, module)| Some((name, module.previous?))),
        );
        let format: String = names.iter().map(|name| format!("${{{name}}}")).collect();
        let mut configuration = toml::Table::new();
        configuration.insert("add_newline".into(), false.into());
        configuration.insert("format".into(), format.into());
        simulate_in(settings, &context(configuration), &timings, &script)
    }

    /// What each module that shows something shows, and how long it takes to.
    fn visible(modules: &[Generated]) -> impl Iterator<Item = (u64, &'static str)> + '_ {
        modules
            .iter()
            .zip(OUTPUTS)
            .filter(|(module, _)| module.shows)
            .map(|(module, output)| (module.took, output))
    }

    fn generated(took: std::ops::Range<u64>) -> impl Strategy<Value = Generated> {
        let previous = prop_oneof![
            Just(None),
            Just(Some(Estimate::Unchanged)),
            (0_u64..3_000).prop_map(|took| Some(Estimate::Changed(Span::from(milliseconds(took))))),
        ];
        (took, any::<bool>(), previous).prop_map(|(took, shows, previous)| Generated {
            took,
            shows,
            previous,
        })
    }

    /// What the timetable drawn cost: the wait for the first paint, and for
    /// each redraw, the size of the bus its first change opened and the wait.
    fn drawn_cost(drawn: &[(u64, &str)], visible: &[(u64, &str)]) -> Duration {
        let redraws = drawn.windows(2).map(|pair| {
            let [(_, before), (at, after)] = pair else {
                unreachable!("a pair")
            };
            let first = visible
                .iter()
                .filter(|(_, output)| after.contains(output) && !before.contains(output))
                .map(|&(took, _)| took)
                .min()
                .expect("a redraw draws a change");
            bus::size(
                Span::from(milliseconds(FALLBACK)),
                Span::from(milliseconds(first)),
            )
            .get()
                + milliseconds(at - first)
        });
        milliseconds(drawn[0].0) + redraws.sum::<Duration>()
    }

    fn cheapest_cost(visible: &[(u64, &str)]) -> Option<Duration> {
        let fallback = Span::from(milliseconds(FALLBACK));
        let mut landings: Vec<_> = visible
            .iter()
            .map(|&(took, _)| bus::Landing {
                after: milliseconds(took),
                size: bus::size(fallback, Span::from(milliseconds(took))),
            })
            .collect();
        landings.sort_unstable_by_key(|landing| landing.after);
        testing::every_timetable(bus::Passengers::Prompt, fallback.get(), &landings)
            .into_iter()
            .map(|(_, cost)| cost)
            .min()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]

        /// Whatever modules take, show, and did before, every module that
        /// shows something is drawn within a fallback of rendering, never a
        /// prompt the same as the one before, and the last drawn is complete.
        #[test]
        fn redraws_show_every_module_promptly_and_end_complete(
            modules in prop::collection::vec(generated(0..3_000), 1..=GENERATED.len()),
            workers in 1_usize..=GENERATED.len(),
        ) {
            let simulated = simulate_generated(&modules, Settings { workers, ..Settings::default() });
            let drawn = simulated.mains();

            prop_assert!(simulated.ended);
            prop_assert!(drawn[0].0 <= FALLBACK, "first drawn at {}", drawn[0].0);
            for pair in drawn.windows(2) {
                prop_assert_ne!(pair[0].1, pair[1].1);
            }
            for (position, (module, output)) in modules.iter().zip(OUTPUTS).enumerate() {
                if !module.shows {
                    continue;
                }
                let rendered = simulated.first[position].expect("every module renders");
                let shown = drawn.iter().find(|(_, prompt)| prompt.contains(output)).map(|&(at, _)| at);
                prop_assert!(
                    shown.is_some_and(|at| (rendered..=rendered + FALLBACK).contains(&at)),
                    "{output} rendered at {rendered}, shown at {shown:?}: {drawn:?}"
                );
            }
            let complete: String = visible(&modules).map(|(_, output)| output).collect();
            prop_assert_eq!(Some(complete.as_str()), drawn.last().map(|&(_, prompt)| prompt));
        }

        /// A shell that can draw only so often is drawn no more than that, the
        /// last time the complete prompt, and then the stream ends.
        #[test]
        fn a_shell_is_drawn_no_more_often_than_it_can_be(
            modules in prop::collection::vec(generated(0..3_000), 1..=GENERATED.len()),
            redraws in 0_usize..3,
        ) {
            let simulated = simulate_generated(
                &modules,
                Settings { redraws: Some(redraws), ..Settings::default() },
            );
            let drawn = simulated.mains();

            prop_assert!(simulated.ended);
            prop_assert!(drawn.len() <= redraws + 1, "{drawn:?}");
            let complete: String = visible(&modules).map(|(_, output)| output).collect();
            prop_assert_eq!(Some(complete.as_str()), drawn.last().map(|&(_, prompt)| prompt));
        }

        /// When every module does what it did before, the prompt is drawn on
        /// the cheapest timetable there is.
        #[test]
        fn true_estimates_make_the_cheapest_timetable(
            modules in prop::collection::vec((0_u64..300, any::<bool>()), 1..=GENERATED.len()),
        ) {
            let modules: Vec<_> = modules
                .into_iter()
                .map(|(took, shows)| Generated {
                    took,
                    shows,
                    previous: Some(if shows {
                        Estimate::Changed(Span::from(milliseconds(took)))
                    } else {
                        Estimate::Unchanged
                    }),
                })
                .collect();
            let simulated = simulate_generated(&modules, Settings::default());
            let drawn = simulated.mains();
            let visible: Vec<_> = visible(&modules).collect();

            prop_assert_eq!(cheapest_cost(&visible), Some(drawn_cost(&drawn, &visible)), "{:?}", drawn);
        }

        /// Modules never measured are waited for until the fallback, and the
        /// first paint shows all of them that have rendered by then.
        #[test]
        fn modules_never_measured_are_waited_for_until_the_fallback(
            took in prop::collection::vec(0_u64..120, 1..=GENERATED.len()),
        ) {
            let modules: Vec<_> = took
                .iter()
                .map(|&took| Generated { took, shows: true, previous: None })
                .collect();
            let simulated = simulate_generated(&modules, Settings::default());
            let (first_paint, prompt) = simulated.mains()[0];
            let rendered: String = visible(&modules)
                .filter(|&(took, _)| took <= first_paint)
                .map(|(_, output)| output)
                .collect();

            prop_assert_eq!(took.iter().max().map(|&last| last.min(FALLBACK)), Some(first_paint));
            prop_assert_eq!(rendered.as_str(), prompt);
        }

        /// Prompts arrive as they were drawn, but for any NUL within them.
        #[test]
        fn prompts_survive_encoding_but_for_nuls(
            main in prop_oneof![any::<String>(), "[\\x00-\\x7f]{0,80}"],
            right in prop_oneof![any::<String>(), "[\\x00-\\x7f]{0,80}"],
        ) {
            prop_assert_eq!(
                vec![[main.replace('\0', ""), right.replace('\0', "")]],
                read_prompts(encode(&[main, right]).as_bytes())
            );
        }
    }

    /// The prompts drawn as `bytes`, in order.
    fn read_prompts(bytes: &[u8]) -> Vec<Prompts> {
        let fields: Vec<String> = std::str::from_utf8(bytes)
            .expect("prompts are text")
            .split_terminator('\0')
            .map(str::to_owned)
            .collect();
        assert_eq!(0, fields.len() % 2, "prompts come in pairs: {fields:?}");
        fields
            .chunks(2)
            .map(|pair| pair.to_vec().try_into().expect("a pair"))
            .collect()
    }

    #[test]
    fn without_async_the_prompts_are_drawn_once_every_module_renders() {
        let context = context(toml::toml! {
            add_newline = false
            format = "$character"
            right_format = "$jobs"
            async.enabled = false
        });
        let mut output = Vec::new();
        run(&context, None, &mut output).unwrap();

        assert_eq!(
            vec![[print::get_prompt(&context), String::new()]],
            read_prompts(&output)
        );
    }

    #[test]
    fn a_dumb_terminal_gets_the_standard_fallback() {
        let mut context = context(toml::toml! { format = "$time" });
        context.env.insert("TERM", "dumb".to_owned());
        let mut output = Vec::new();
        run(&context, None, &mut output).unwrap();

        assert_eq!(DUMB_TERMINAL_PROMPT, print::get_prompt(&context));
        assert_eq!(
            vec![[DUMB_TERMINAL_PROMPT, DUMB_TERMINAL_PROMPT].map(str::to_owned)],
            read_prompts(&output)
        );
    }

    /// A shell that stops reading at a deadline.
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

        let error = run(&context, None, &mut output).unwrap_err();
        assert_eq!(io::ErrorKind::BrokenPipe, error.kind());
        let mains: Vec<_> = read_prompts(&output.written)
            .into_iter()
            .map(|[main, _]| main)
            .collect();
        assert!(mains.len() >= 2, "the time turned over: {mains:?}");
        assert!(mains.windows(2).all(|pair| pair[0] != pair[1]));
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
            let context_for = |target| {
                let mut context = context(configuration.clone());
                context.shell = shell;
                context.target = target;
                context.current_dir = directory.path().to_path_buf();
                context.properties.status_code = Some("1".to_owned());
                context.properties.jobs = 2;
                context
            };
            let mut output = Vec::new();
            run(&context_for(Target::Main), None, &mut output).unwrap();

            assert_eq!(
                Some([
                    print::get_prompt(&context_for(Target::Main)),
                    print::get_prompt(&context_for(Target::Right))
                ]),
                read_prompts(&output).pop(),
                "under {shell:?}"
            );
        }
    }

    #[test]
    fn the_first_paint_shows_the_directory_before_its_repository_is_found() {
        let repository = tempfile::tempdir().unwrap();
        crate::utils::create_command("git")
            .unwrap()
            .arg("init")
            .current_dir(repository.path())
            .output()
            .unwrap();
        let working_directory = repository.path().join("src");
        std::fs::create_dir(&working_directory).unwrap();
        let mut context = context(toml::toml! {
            add_newline = false
            format = "$directory$character"
            directory.format = "$path"
            directory.truncation_length = 8
        });
        context.current_dir = working_directory.clone();
        context.logical_dir = working_directory;

        let simulated = simulate_in(
            Settings::default(),
            &context,
            "character=0,directory=500",
            &[("character", &[(0, ">")]), ("directory", &[(500, "found")])],
        );
        let drawn = simulated.mains();

        let repository_name = repository.path().file_name().unwrap().to_string_lossy();
        let [(0, first), (500, refined)] = drawn[..] else {
            panic!("a first paint and one refinement, not {drawn:?}");
        };
        assert!(
            first.ends_with(&format!(
                "{repository_name}{}src>",
                std::path::MAIN_SEPARATOR
            )),
            "{first}"
        );
        assert_eq!("found>", refined);
    }

    #[test]
    fn a_refresh_falls_due_strictly_after_now_on_the_wall_clock() {
        let start = Instant::now();
        let second = Span::from(Duration::from_secs(1));
        let clock = Clock {
            start,
            wall: milliseconds(300),
        };
        let boundary = start + milliseconds(700);

        assert_eq!(Some(boundary), clock.next(start, second));
        assert_eq!(
            Some(boundary + second),
            clock.next(boundary, second),
            "a refresh on a boundary falls due on the next"
        );
        assert_eq!(
            clock.next(boundary, Span::MAX),
            clock.next_refresh(
                boundary,
                &modules::Refresh::Every(Span::MAX.get() + Duration::from_secs(1)),
            ),
            "generic refreshes retain their bounded planning span"
        );
        assert_eq!(
            None,
            clock.next(start, Span::default()),
            "nor ever without a period"
        );
    }
}
