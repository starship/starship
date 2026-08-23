//! Streaming prompt execution.

pub(crate) mod bus;
pub(crate) mod latency;
pub(crate) mod schedule;

pub use latency::LatencyEstimates;

use std::fmt;
use std::io::{self, Write};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::context::{Context, Properties, Target};
#[cfg(test)]
use crate::frame::Patch;
use crate::frame::{
    FrameEncoding, ProcessId, PromptVariablePayload, RawTerminalPayload, ServerEvent, Timings,
};
use crate::module::painted::{Painted, TerminalWidth};
use crate::plan::{Plan, PromptState};
use crate::print::prompt_configuration;
use crate::render::{self, DynamicModule, Resolution, ResolutionKind, Selection, Spawner};
use crate::segment::Segment;
use crate::transport::{RefinementTier, StreamingTransport, Tier, TransportMismatch};

use bus::{Bus, BusWindow, Reflow, Verdict};
use schedule::{ArrivalSchedule, PredictedArrival};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);
const FAR_FUTURE_DELAY: Duration = Duration::from_secs(24 * 60 * 60);

/// Streams a prompt to the terminal output.
pub fn stream(
    properties: Properties,
    target: Target,
    latency_estimates: &LatencyEstimates,
    streaming_transport: StreamingTransport,
    frame_encoding: FrameEncoding,
) -> Result<(), StreamError> {
    let context = Context::new(properties, target).rendering_for_the_terminal();
    let standard_output = io::stdout();
    let mut output_writer = standard_output.lock();

    let transport_tier = streaming_transport.tier(context.shell, &context.target)?;
    run_at_tier(
        &context,
        latency_estimates,
        transport_tier,
        frame_encoding,
        &mut output_writer,
    )
    .map_err(StreamError::InputOutput)
}

#[derive(Debug)]
pub enum StreamError {
    TransportMismatch(TransportMismatch),
    InputOutput(io::Error),
}

impl StreamError {
    pub fn is_broken_pipe(&self) -> bool {
        matches!(self, Self::InputOutput(error) if error.kind() == io::ErrorKind::BrokenPipe)
    }
}

impl fmt::Display for StreamError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransportMismatch(error) => error.fmt(formatter),
            Self::InputOutput(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for StreamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TransportMismatch(error) => Some(error),
            Self::InputOutput(error) => Some(error),
        }
    }
}

impl From<TransportMismatch> for StreamError {
    fn from(error: TransportMismatch) -> Self {
        Self::TransportMismatch(error)
    }
}

#[cfg(test)]
fn run(
    context: &Context,
    latency_estimates: &LatencyEstimates,
    output_writer: &mut impl Write,
) -> io::Result<()> {
    let transport_tier = StreamingTransport::Auto
        .tier(context.shell, &context.target)
        .expect("The automatic transport accepts every shell environment.");
    run_at_tier(
        context,
        latency_estimates,
        transport_tier,
        FrameEncoding::Compact,
        output_writer,
    )
}

fn run_at_tier(
    context: &Context,
    latency_estimates: &LatencyEstimates,
    transport_tier: Tier,
    frame_encoding: FrameEncoding,
    output_writer: &mut impl Write,
) -> io::Result<()> {
    if crate::print::is_dumb_terminal() {
        return handle_dumb_terminal(context, frame_encoding, output_writer);
    }

    let prompt_config = prompt_configuration(context);
    let execution_plan = Plan::build(&prompt_config);
    let asynchronous_config = &context.root_config.asynchronous;
    let optional_refinement = transport_tier.refinement();

    let mut measured_timings = Timings::default();
    let mut prompt_state = PromptState::empty(&execution_plan);

    let is_asynchronous = asynchronous_config.enabled && optional_refinement.is_some();

    if !is_asynchronous {
        render::stream(
            &execution_plan,
            context,
            Selection::EveryModule,
            |resolution| {
                measured_timings.record(resolution.module().as_str(), resolution.elapsed());
                resolution.store_in(&mut prompt_state);
            },
        );

        let final_painted_prompt = paint_prompt(&prompt_state, context);
        write_server_event(
            create_ready_event(&final_painted_prompt, context),
            frame_encoding,
            output_writer,
        )?;

        let final_timings = latency_estimates
            .updated_with(&measured_timings)
            .timings()
            .clone();
        return write_server_event(
            ServerEvent::Complete(final_timings),
            frame_encoding,
            output_writer,
        );
    }

    render::stream_instant(&execution_plan, context, |resolution| {
        measured_timings.record(resolution.module().as_str(), resolution.elapsed());
        resolution.store_in(&mut prompt_state);
    });

    let initial_painted_prompt = paint_prompt(&prompt_state, context);
    write_server_event(
        create_ready_event(&initial_painted_prompt, context),
        frame_encoding,
        output_writer,
    )?;

    let refinement_tier = optional_refinement
        .expect("A static transport tier returns early; async logic requires a refinement tier.");

    let terminal_canvas = TerminalCanvas {
        prompt_state,
        painted_output: initial_painted_prompt,
        terminal_width: TerminalWidth(context.width),
        refinement_tier,
    };

    let bus_window = BusWindow::from_milliseconds(asynchronous_config.bus);
    let active_scheduler = Scheduler::new(&execution_plan, context, asynchronous_config);
    let active_progress =
        ProgressTracker::new(render::modules(&execution_plan, Selection::DeferredOnly).count());
    let active_bus = initialize_bus(
        &execution_plan,
        latency_estimates,
        asynchronous_config.adaptive,
        bus_window,
    );

    let mut streaming_session = StreamingSession {
        context,
        canvas: terminal_canvas,
        bus: active_bus,
        measured_timings,
        latency_estimates,
        progress: active_progress,
        scheduler: active_scheduler,
        frame_encoding,
        last_written_timestamp: Instant::now(),
    };

    let (arrival_sender, arrival_receiver) = mpsc::channel::<Resolution>();
    render::while_running(
        &execution_plan,
        context,
        Selection::DeferredOnly,
        &arrival_sender,
        |process_spawner| {
            streaming_session.serve(&arrival_receiver, process_spawner, output_writer)
        },
    )
}

struct StreamingSession<'plan, 'context> {
    context: &'plan Context<'context>,
    canvas: TerminalCanvas<'plan>,
    bus: Bus,
    measured_timings: Timings,
    latency_estimates: &'plan LatencyEstimates,
    progress: ProgressTracker,
    scheduler: Scheduler<'plan>,
    frame_encoding: FrameEncoding,
    last_written_timestamp: Instant,
}

impl<'plan, 'context> StreamingSession<'plan, 'context> {
    fn serve(
        &mut self,
        resolutions_receiver: &mpsc::Receiver<Resolution<'plan>>,
        process_spawner: &Spawner<'_, 'plan, 'context>,
        output_writer: &mut impl Write,
    ) -> io::Result<()> {
        loop {
            if self.progress.check_and_take_ready() {
                self.flush_held_bus_events(output_writer)?;
                let final_timings = self
                    .latency_estimates
                    .updated_with(&self.measured_timings)
                    .timings()
                    .clone();
                self.write_server_event(ServerEvent::Complete(final_timings), output_writer)?;
            }

            if self.progress.is_completed() && !self.scheduler.has_active_polls() {
                return Ok(());
            }

            self.scheduler
                .spawn_due_tasks(Instant::now(), process_spawner);

            if self.last_written_timestamp.elapsed() >= HEARTBEAT_INTERVAL {
                self.write_server_event(ServerEvent::Heartbeat, output_writer)?;
            }

            let next_wakeup_time = self.calculate_next_wakeup();
            let timeout_duration = next_wakeup_time.saturating_duration_since(Instant::now());

            match resolutions_receiver.recv_timeout(timeout_duration) {
                Ok(resolution) => self.process_resolution(resolution, output_writer)?,
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    let current_time = Instant::now();
                    if self
                        .bus
                        .deadline()
                        .is_some_and(|deadline| deadline <= current_time)
                    {
                        self.flush_held_bus_events(output_writer)?;
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    unreachable!(
                        "The streaming session holds a channel sender, so the receiver cannot disconnect."
                    )
                }
            }
        }
    }

    fn process_resolution(
        &mut self,
        resolution: Resolution<'plan>,
        output_writer: &mut impl Write,
    ) -> io::Result<()> {
        self.measured_timings
            .record(resolution.module().as_str(), resolution.elapsed());
        self.scheduler.register_resolution(&resolution);
        self.progress.register_resolution(&resolution);

        resolution.store_in(&mut self.canvas.prompt_state);

        let next_painted_prompt = paint_prompt(&self.canvas.prompt_state, self.context);
        let reflow = Reflow::between(&self.canvas.painted_output, &next_painted_prompt);

        if self.bus.admit(reflow, Instant::now()) == Verdict::DrawNow {
            self.draw_and_transmit(next_painted_prompt, output_writer)?;
        }
        Ok(())
    }

    fn flush_held_bus_events(&mut self, output_writer: &mut impl Write) -> io::Result<()> {
        if !self.bus.release() {
            return Ok(());
        }
        let next_painted_prompt = paint_prompt(&self.canvas.prompt_state, self.context);
        self.draw_and_transmit(next_painted_prompt, output_writer)
    }

    fn draw_and_transmit(
        &mut self,
        next_painted_prompt: Painted,
        output_writer: &mut impl Write,
    ) -> io::Result<()> {
        let patch_payload = crate::transport::patch(
            &self.canvas.painted_output,
            &next_painted_prompt,
            self.canvas.terminal_width,
            self.canvas.refinement_tier,
            self.context.shell,
        );

        self.canvas.painted_output = next_painted_prompt;

        if let Some(payload) = patch_payload {
            self.write_server_event(ServerEvent::Patch(payload), output_writer)?;
        }
        Ok(())
    }

    fn write_server_event(
        &mut self,
        event: ServerEvent,
        output_writer: &mut impl Write,
    ) -> io::Result<()> {
        event.write(self.frame_encoding, output_writer)?;
        self.last_written_timestamp = Instant::now();
        Ok(())
    }

    fn calculate_next_wakeup(&self) -> Instant {
        let heartbeat_deadline = self.last_written_timestamp + HEARTBEAT_INTERVAL;
        let bus_deadline = self.bus.deadline();
        let scheduler_deadline = self.scheduler.next_wakeup();

        [Some(heartbeat_deadline), bus_deadline, scheduler_deadline]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(heartbeat_deadline)
    }
}

struct TerminalCanvas<'plan> {
    prompt_state: PromptState<'plan>,
    painted_output: Painted,
    terminal_width: TerminalWidth,
    refinement_tier: RefinementTier,
}

struct ProgressTracker {
    pending_initial_resolutions: usize,
    is_reported: bool,
}

impl ProgressTracker {
    fn new(total_deferred_modules: usize) -> Self {
        Self {
            pending_initial_resolutions: total_deferred_modules,
            is_reported: false,
        }
    }

    fn register_resolution(&mut self, resolution: &Resolution) {
        if matches!(resolution.kind(), ResolutionKind::Initial) {
            assert!(
                self.pending_initial_resolutions > 0,
                "Every deferred module must have exactly one initial resolution."
            );
            self.pending_initial_resolutions -= 1;
        }
    }

    fn check_and_take_ready(&mut self) -> bool {
        if self.pending_initial_resolutions == 0 && !self.is_reported {
            self.is_reported = true;
            true
        } else {
            false
        }
    }

    fn is_completed(&self) -> bool {
        self.is_reported
    }
}

struct Scheduler<'plan> {
    polls: Vec<Option<ScheduledPoll<'plan>>>,
}

impl<'plan> Scheduler<'plan> {
    fn new(
        plan: &'plan Plan,
        context: &Context,
        asynchronous_config: &crate::configs::asynchronous::AsynchronousConfig,
    ) -> Self {
        let mut polls = std::iter::repeat_with(|| None)
            .take(plan.module_uses().len())
            .collect::<Vec<_>>();

        for module in configured_dynamic_modules(plan, context, asynchronous_config) {
            let slot_index = module.slot().index();
            polls[slot_index] = Some(ScheduledPoll {
                module,
                state: PollState::AwaitingInitial,
            });
        }
        Self { polls }
    }

    fn register_resolution(&mut self, resolution: &Resolution) {
        let slot_index = resolution.slot().index();
        let optional_poll = self.polls[slot_index].as_mut();

        let poll = match resolution.kind() {
            ResolutionKind::Initial => optional_poll,
            ResolutionKind::Refresh => Some(
                optional_poll.expect("A refresh resolution was received for a non-dynamic module."),
            ),
        };
        if let Some(poll) = poll {
            poll.state = PollState::Due(calculate_next_due_time(poll.module.period()));
        }
    }

    fn spawn_due_tasks(&mut self, current_time: Instant, process_spawner: &Spawner<'_, 'plan, '_>) {
        for poll in self.polls.iter_mut().flatten() {
            if let PollState::Due(due_time) = poll.state
                && due_time <= current_time
            {
                process_spawner.poll(poll.module.clone());
                poll.state = PollState::Running;
            }
        }
    }

    fn next_wakeup(&self) -> Option<Instant> {
        self.polls
            .iter()
            .flatten()
            .filter_map(|poll| {
                if let PollState::Due(due_time) = poll.state {
                    Some(due_time)
                } else {
                    None
                }
            })
            .min()
    }

    fn has_active_polls(&self) -> bool {
        self.polls.iter().any(Option::is_some)
    }
}

struct ScheduledPoll<'plan> {
    module: DynamicModule<'plan>,
    state: PollState,
}

enum PollState {
    AwaitingInitial,
    Due(Instant),
    Running,
}

fn initialize_bus(
    plan: &Plan,
    latency_estimates: &LatencyEstimates,
    is_adaptive: bool,
    window: BusWindow,
) -> Bus {
    if !is_adaptive {
        return Bus::fixed(window);
    }

    let schedule = calculate_expected_arrivals(plan, latency_estimates, window);
    if schedule.is_empty() {
        Bus::fixed(window)
    } else {
        Bus::scheduled(window, schedule, Instant::now())
    }
}

fn calculate_expected_arrivals(
    plan: &Plan,
    latency_estimates: &LatencyEstimates,
    window: BusWindow,
) -> ArrivalSchedule {
    let predicted_arrivals = render::modules(plan, Selection::DeferredOnly)
        .filter_map(|module| latency_estimates.of(module.as_str()))
        .map(PredictedArrival::after);

    ArrivalSchedule::of(predicted_arrivals, window)
}

fn handle_dumb_terminal(
    context: &Context,
    frame_encoding: FrameEncoding,
    output_writer: &mut impl Write,
) -> io::Result<()> {
    log::error!("Environment configured as a 'dumb' terminal (TERM=dumb).");

    let text_segment = Segment::from_text(None, crate::print::DUMB_TERMINAL_PROMPT);
    let notice_painted = Painted::paint(&text_segment, None);

    write_server_event(
        create_ready_event(&notice_painted, context),
        frame_encoding,
        output_writer,
    )?;
    write_server_event(
        ServerEvent::Complete(Timings::default()),
        frame_encoding,
        output_writer,
    )
}

fn paint_prompt(state: &PromptState<'_>, context: &Context) -> Painted {
    let rendered_state = state.render();
    let mut segments = Vec::with_capacity(rendered_state.len() + 1);

    if has_leading_blank_line(context) {
        segments.push(Segment::LineTerm);
    }

    segments.extend(rendered_state);
    Painted::paint(&segments, Some(TerminalWidth(context.width)))
}

fn has_leading_blank_line(context: &Context) -> bool {
    context.root_config.add_newline && context.target == Target::Main
}

fn create_ready_event(painted_output: &Painted, context: &Context) -> ServerEvent {
    let terminal_payload = RawTerminalPayload::prompt(painted_output);
    let escaped_payload = PromptVariablePayload::escaped_for(&terminal_payload, context.shell);

    ServerEvent::Ready {
        prompt: escaped_payload,
        process_id: ProcessId::of_this_process(),
    }
}

fn write_server_event(
    event: ServerEvent,
    frame_encoding: FrameEncoding,
    output_writer: &mut impl Write,
) -> io::Result<()> {
    event.write(frame_encoding, output_writer)
}

fn calculate_next_due_time(period: Duration) -> Instant {
    Instant::now()
        .checked_add(period)
        .unwrap_or_else(calculate_far_future_fallback)
}

fn calculate_far_future_fallback() -> Instant {
    Instant::now() + FAR_FUTURE_DELAY
}

fn configured_dynamic_modules<'plan>(
    plan: &'plan Plan,
    context: &Context,
    asynchronous_config: &crate::configs::asynchronous::AsynchronousConfig,
) -> Vec<DynamicModule<'plan>> {
    render::dynamic_modules(plan, context)
        .into_iter()
        .map(|module| {
            let override_period = asynchronous_config
                .dynamic
                .period_for(module.name().as_str());
            match override_period {
                Some(period) => module.every(period),
                None => module,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Shell;
    use crate::test::default_context;
    use std::collections::BTreeSet;
    use std::io::Cursor;
    use std::time::Duration;

    /// zsh: the only shell at [`Tier::CellPrecise`], so a stream built for it produces every event kind.
    fn refinable(mut context: Context<'static>) -> Context<'static> {
        context.shell = Shell::Zsh;
        context
    }

    /// Every event one streaming prompt writes, for a session with nothing measured yet.
    fn events_of(context: &Context) -> Vec<ServerEvent> {
        events_of_a_session(context, &LatencyEstimates::none())
    }

    /// Every event one streaming prompt writes; only for a session that finishes (this writes into
    /// an unbounded buffer) — use [`ClosesAfter`] for one that keeps re-polling forever.
    fn events_of_a_session(context: &Context, estimates: &LatencyEstimates) -> Vec<ServerEvent> {
        let mut written: Vec<u8> = Vec::new();
        run(context, estimates, &mut written).expect("writing to a vector cannot fail");

        let mut reader = Cursor::new(written);
        let mut events = Vec::new();
        while let Some(event) =
            ServerEvent::read_from(&mut reader).expect("the engine writes well-formed events")
        {
            events.push(event);
        }
        events
    }

    /// Event kind, ignoring payload — the protocol itself has no `kind()`; only tests want this.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum EventKind {
        Ready,
        Patch,
        Complete,
        Heartbeat,
    }

    fn kind_of(event: &ServerEvent) -> EventKind {
        match event {
            ServerEvent::Ready { .. } => EventKind::Ready,
            ServerEvent::Patch(_) => EventKind::Patch,
            ServerEvent::Complete(_) => EventKind::Complete,
            ServerEvent::Heartbeat => EventKind::Heartbeat,
        }
    }

    fn kinds_of(events: &[ServerEvent]) -> Vec<EventKind> {
        events.iter().map(kind_of).collect()
    }

    /// The prompt-variable bytes of `event`, as text.
    fn prompt_text(event: &ServerEvent) -> String {
        let bytes = match event {
            ServerEvent::Ready { prompt, .. } => prompt.as_bytes(),
            ServerEvent::Patch(patch) => patch.prompt().as_bytes(),
            other => panic!("{other:?} carries no prompt"),
        };
        String::from_utf8(bytes.to_vec()).expect("prompt bytes are text")
    }

    /// A writer that fails every write once `deadline` passes, standing in for a reader that
    /// has gone away — a deadline rather than a write count so a test need not predict how
    /// many writes one poll cycle costs.
    struct ClosesAfter {
        buffer: Vec<u8>,
        deadline: Instant,
    }

    impl Write for ClosesAfter {
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            if Instant::now() >= self.deadline {
                return Err(io::Error::from(io::ErrorKind::BrokenPipe));
            }
            self.buffer.extend_from_slice(data);
            Ok(data.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn the_first_event_is_always_ready() {
        let context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$character"
            [character]
            format = ">"
        });
        let events = events_of(&context);

        assert_eq!(Some(EventKind::Ready), events.first().map(kind_of));
        assert_eq!(">", prompt_text(&events[0]));
    }

    #[test]
    fn the_last_event_is_always_complete_when_nothing_dynamic_is_running() {
        let context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$character"
        });
        let events = events_of(&context);

        assert_eq!(Some(EventKind::Complete), events.last().map(kind_of));
        let timings = match events.last().expect("there is an event") {
            ServerEvent::Complete(timings) => timings,
            other => panic!("the last event was {other:?}, not a completion"),
        };
        assert!(timings.get("character").is_some());
    }

    /// A shell that can't be refined must get its prompt already complete, or it keeps a
    /// stale one for the command line's life.
    #[test]
    fn a_shell_that_cannot_be_refined_is_sent_one_finished_prompt() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${custom.slow}$character"
            [character]
            format = ">"
            [custom.slow]
            when = true
            command = "echo refined"
            format = "[$output](cyan)"
        });
        context.current_dir = directory.path().to_path_buf();
        context.logical_dir = directory.path().to_path_buf();
        // The default context's shell is `Unknown`, which is `Tier::Static`.
        assert!(!Tier::of(context.shell).can_refine());

        let events = events_of(&context);
        assert_eq!(
            vec![EventKind::Ready, EventKind::Complete],
            kinds_of(&events),
            "a static shell must be sent nothing after its prompt"
        );
        assert!(
            prompt_text(&events[0]).contains("refined"),
            "the one prompt a static shell is sent must be the finished one, \
             but was {:?}",
            prompt_text(&events[0])
        );
        assert!(
            match &events[1] {
                ServerEvent::Complete(timings) => timings.get("custom.slow").is_some(),
                other => panic!("the last event was {other:?}, not a completion"),
            },
            "a static stream still reports what each module cost"
        );
    }

    #[test]
    fn a_prompt_of_instant_modules_alone_needs_no_refinement() {
        let context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$status$character"
            [character]
            format = ">"
        });

        // Nothing deferred, so nothing can change after the paint (deliberately
        // `Shell::Unknown`, not a refinable shell).
        assert_eq!(
            vec![EventKind::Ready, EventKind::Complete],
            kinds_of(&events_of(&context))
        );
    }

    #[test]
    fn a_deferred_module_that_resolves_to_nothing_changes_nothing() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let mut context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "$rust$character"
            [character]
            format = ">"
        });
        context.current_dir = directory.path().to_path_buf();

        // `rust` is deferred and resolves to nothing here, so the paint was already
        // right — no refinement follows.
        assert_eq!(
            vec![EventKind::Ready, EventKind::Complete],
            kinds_of(&events_of(&context))
        );
    }

    #[test]
    fn the_paint_leaves_deferred_slots_empty() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let mut context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "[$git_branch](red)$character"
            [character]
            format = ">"
        });
        context.current_dir = directory.path().to_path_buf();

        let events = events_of(&context);
        assert_eq!(">", prompt_text(&events[0]));
    }

    #[test]
    fn add_newline_puts_the_blank_line_inside_the_painted_prompt() {
        let context = default_context().set_config(toml::toml! {
            add_newline = true
            format = "$character"
            [character]
            format = ">"
        });
        let events = events_of(&context);

        assert_eq!("\n>", prompt_text(&events[0]));
    }

    #[test]
    fn a_right_prompt_never_leads_with_a_blank_line() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = true
            right_format = "$character"
            [character]
            format = ">"
        });
        context.target = Target::Right;

        assert_eq!(">", prompt_text(&events_of(&context)[0]));
    }

    /// A prompt whose deferred `custom` modules genuinely resolve to something controllable,
    /// so its stream carries real refinements rather than only a paint.
    fn context_with_deferred_modules(shell_command: &str) -> (Context<'static>, tempfile::TempDir) {
        // A `custom` module runs in the working directory, so it needs one that exists.
        let directory = tempfile::tempdir().expect("a temporary directory");
        let mut context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "${custom.first} ${custom.second}$line_break$character"
            [async]
            bus = GENEROUS_WINDOW
            [character]
            format = "[>](green) "
            [custom.first]
            when = true
            command = shell_command
            format = "[$output](bold cyan)"
            [custom.second]
            when = true
            command = shell_command
            format = "[$output](yellow)"
        });
        context.current_dir = directory.path().to_path_buf();
        context.logical_dir = directory.path().to_path_buf();
        (context, directory)
    }

    /// Replays `events` into a live terminal emulator, kept live so a caller can keep feeding
    /// it (see `a_patch_that_carries_a_repaint_carries_the_prompt_that_matches_it`). Mirrors the
    /// shell: `Ready` and a prompt-only `Patch` are expanded and redrawn; a `Patch` with a
    /// repaint is fed the repaint alone, never also redrawn from its prompt.
    fn terminal_after(
        events: &[ServerEvent],
        width: TerminalWidth,
        shell: Shell,
    ) -> crate::damage::terminal::EmulatedTerminal {
        let mut terminal = crate::damage::terminal::EmulatedTerminal::blank(width);
        for event in events {
            match event {
                ServerEvent::Ready { prompt, .. } => {
                    terminal.redraw(prompt.as_terminal_bytes_under(shell).as_bytes());
                }
                ServerEvent::Patch(patch) => match patch.repaint() {
                    Some(repaint) => terminal.feed(repaint.as_bytes()),
                    None => {
                        terminal.redraw(patch.prompt().as_terminal_bytes_under(shell).as_bytes());
                    }
                },
                ServerEvent::Complete(_) | ServerEvent::Heartbeat => {}
            }
        }
        terminal
    }

    /// Replays a whole event stream and returns the resulting screen; expanding (rather than
    /// feeding raw escaped bytes) means this also tests the escaping, not just the repaints.
    fn replay(
        events: &[ServerEvent],
        width: TerminalWidth,
        shell: Shell,
    ) -> crate::damage::terminal::Screen {
        terminal_after(events, width, shell).screen()
    }

    #[test]
    fn replaying_the_events_reproduces_a_full_render() {
        let (context, _directory) = context_with_deferred_modules("echo refined");
        let width = TerminalWidth(context.width);

        let events = events_of(&context);
        assert!(
            events
                .iter()
                .any(|event| matches!(event, ServerEvent::Patch(_))),
            "the stream refined nothing, so replaying it proves nothing: {events:?}"
        );

        // What a plain synchronous `starship prompt` would have put on screen.
        let plan = Plan::build(&prompt_configuration(&context));
        let expected = paint_prompt(&render::fill_slots(&plan, &context), &context);

        assert_eq!(
            crate::damage::terminal::fully_rendered(&expected, width),
            replay(&events, width, context.shell),
            "replaying the event stream must leave the terminal exactly as a \
             synchronous render would"
        );
    }

    #[test]
    fn replaying_a_multi_line_prompt_reproduces_a_full_render() {
        // Modules resolve to multi-line text — the case the length prefix exists for, and
        // where a repaint's row arithmetic can get it wrong.
        let (context, _directory) = context_with_deferred_modules("printf 'one\\ntwo'");
        let width = TerminalWidth(context.width);

        let events = events_of(&context);
        let plan = Plan::build(&prompt_configuration(&context));
        let expected = paint_prompt(&render::fill_slots(&plan, &context), &context);

        assert_eq!(
            crate::damage::terminal::fully_rendered(&expected, width),
            replay(&events, width, context.shell)
        );
    }

    /// Long enough that two `echo`s can't straddle it, so this tests the bus, not machine load.
    const GENEROUS_WINDOW: u64 = 5_000;

    /// The patches between the paint and the completion: the refinements.
    fn refinements(events: &[ServerEvent]) -> Vec<&Patch> {
        events
            .iter()
            .filter_map(|event| match event {
                ServerEvent::Patch(patch) => Some(patch),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn width_stable_refinements_are_drawn_as_they_land() {
        // A fill absorbs both modules' growth, so neither changes line width or has reason
        // to wait, despite the long window.
        let directory = tempfile::tempdir().expect("a temporary directory");
        let mut context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "${custom.first}$fill${custom.second}$line_break$character"
            [async]
            bus = GENEROUS_WINDOW
            [character]
            format = "[>](green) "
            [fill]
            symbol = "."
            [custom.first]
            when = true
            command = "echo one"
            format = "[$output](cyan)"
            [custom.second]
            when = true
            command = "echo two"
            format = "[$output](yellow)"
        });
        context.current_dir = directory.path().to_path_buf();
        context.logical_dir = directory.path().to_path_buf();

        let events = events_of(&context);
        let refinements = refinements(&events);
        assert_eq!(
            2,
            refinements.len(),
            "a refinement that moves nothing sideways must not wait for the \
             bus, but got {refinements:?}"
        );
        assert!(
            refinements.iter().all(|patch| patch.repaint().is_some()),
            "a width-stable refinement must repaint cells rather than redraw \
             the whole prompt: {refinements:?}"
        );
    }

    #[test]
    fn width_changing_refinements_are_collapsed_into_one_reflow() {
        // No fill this time, so each module's output shifts everything after it and both
        // join one window.
        let (context, _directory) = context_with_deferred_modules("echo refined");

        assert_eq!(
            1,
            refinements(&events_of(&context)).len(),
            "two reflows landing in one window must be drawn once"
        );
    }

    /// A patch is atomic (see `crate::frame`); this asserts the substance behind that shape —
    /// the prompt a patch carries actually describes the screen its repaint draws.
    #[test]
    fn a_patch_that_carries_a_repaint_carries_the_prompt_that_matches_it() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let mut context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "${custom.first}$fill${custom.second}$line_break$character"
            [async]
            bus = GENEROUS_WINDOW
            [character]
            format = "[>](green) "
            [fill]
            symbol = "."
            [custom.first]
            when = true
            command = "echo one"
            format = "[$output](cyan)"
            [custom.second]
            when = true
            command = "echo two"
            format = "[$output](yellow)"
        });
        context.current_dir = directory.path().to_path_buf();
        context.logical_dir = directory.path().to_path_buf();
        let width = TerminalWidth(context.width);

        let events = events_of(&context);
        let repainting_indices: Vec<usize> = events
            .iter()
            .enumerate()
            .filter_map(|(index, event)| match event {
                ServerEvent::Patch(patch) if patch.repaint().is_some() => Some(index),
                _ => None,
            })
            .collect();
        assert!(
            !repainting_indices.is_empty(),
            "the fixture must produce at least one repainting patch or it \
             proves nothing: {events:?}"
        );

        for index in repainting_indices {
            let ServerEvent::Patch(patch) = &events[index] else {
                unreachable!("filtered to patch events with a repaint above");
            };

            // What was on screen right before this patch, with only its own repaint applied.
            let mut terminal = terminal_after(&events[..index], width, context.shell);
            terminal.feed(patch.repaint().expect("filtered above").as_bytes());
            let screen_from_repaint = terminal.screen();

            // What the patch's own prompt says the whole screen should be.
            let mut fresh = crate::damage::terminal::EmulatedTerminal::blank(width);
            fresh.redraw(
                patch
                    .prompt()
                    .as_terminal_bytes_under(context.shell)
                    .as_bytes(),
            );

            assert_eq!(
                fresh.screen(),
                screen_from_repaint,
                "the patch at event {index} carries a prompt that disagrees \
                 with its own repaint"
            );
        }
    }

    /// Three slow modules separated by `$fill`, sleeps spaced to land in one group without
    /// depending on scheduler noise; the fill keeps every refinement width-stable.
    ///
    /// The gaps between sleeps (300ms) are wide relative to process-spawn and
    /// scheduling jitter under a loaded CI runner — narrower gaps (previously
    /// 20ms) let that jitter push the "informed" run's predicted arrivals far
    /// enough off the "unaided" run's actual ones that they landed in
    /// different repaint groups, intermittently failing a test whose whole
    /// point is that they group. `command_timeout` is raised well past the
    /// longest sleep since the default (500ms) would otherwise kill these
    /// modules before they resolve.
    fn context_with_three_width_stable_modules() -> (Context<'static>, tempfile::TempDir) {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let mut context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "${custom.first}$fill${custom.second} ${custom.third}$line_break$character"
            command_timeout = 10_000
            [async]
            bus = GENEROUS_WINDOW
            [character]
            format = "[>](green) "
            [fill]
            symbol = "."
            [custom.first]
            when = true
            command = "sleep 0.30 && echo one"
            format = "[$output](cyan)"
            [custom.second]
            when = true
            command = "sleep 0.60 && echo two"
            format = "[$output](yellow)"
            [custom.third]
            when = true
            command = "sleep 0.90 && echo three"
            format = "[$output](red)"
        });
        context.current_dir = directory.path().to_path_buf();
        context.logical_dir = directory.path().to_path_buf();
        (context, directory)
    }

    /// What the shell hands to the next prompt: the completion payload's actual round-tripped
    /// text, not the engine's in-memory value.
    fn estimates_from(events: &[ServerEvent]) -> LatencyEstimates {
        let completion = events
            .iter()
            .find(|event| matches!(event, ServerEvent::Complete(_)))
            .expect("every stream reports what its modules cost");

        let mut written: Vec<u8> = Vec::new();
        completion
            .write_to(&mut written)
            .expect("writing to a vector cannot fail");
        let mut fields = written.split(|&byte| byte == 0);
        assert_eq!(Some(b"COMPLETE".as_slice()), fields.next());
        let payload =
            String::from_utf8_lossy(fields.next().expect("a timing payload")).into_owned();

        LatencyEstimates::parse_argument(&payload).expect("parsing never fails")
    }

    /// The same prompt drawn twice, the second time knowing what the first measured.
    #[test]
    fn measured_latencies_collapse_repaints_a_fixed_window_draws_one_at_a_time() {
        let (context, _directory) = context_with_three_width_stable_modules();

        // First prompt of the session: nothing measured, so every width-stable refinement
        // draws immediately.
        let unaided = events_of(&context);
        assert_eq!(
            3,
            refinements(&unaided).len(),
            "three width-stable refinements should be three repaints without a \
             prediction, but were {:?}",
            refinements(&unaided)
        );

        // The next prompt of the same session, handed what the first measured.
        let estimates = estimates_from(&unaided);
        let informed = events_of_a_session(&context, &estimates);

        assert!(
            refinements(&informed).len() < refinements(&unaided).len(),
            "knowing when the modules resolve must group their repaints: \
             {:?} against {:?}",
            refinements(&informed),
            refinements(&unaided)
        );
    }

    /// Fewer repaints are only worth having if the screen ends up the same.
    #[test]
    fn scheduling_repaints_does_not_change_what_is_finally_on_screen() {
        let (context, _directory) = context_with_three_width_stable_modules();
        let width = TerminalWidth(context.width);

        let estimates = estimates_from(&events_of(&context));
        let informed = events_of_a_session(&context, &estimates);

        let plan = Plan::build(&prompt_configuration(&context));
        let expected = paint_prompt(&render::fill_slots(&plan, &context), &context);

        assert_eq!(
            crate::damage::terminal::fully_rendered(&expected, width),
            replay(&informed, width, context.shell),
            "a scheduled stream must leave the terminal exactly as a \
             synchronous render would"
        );
    }

    #[test]
    fn a_prompt_whose_modules_have_never_been_measured_draws_to_the_fixed_window() {
        // Estimates for modules this prompt lacks predict nothing about it, so it behaves
        // as if it had none.
        let (context, _directory) = context_with_three_width_stable_modules();
        let mut unrelated = Timings::default();
        unrelated.record("git_status", Duration::from_millis(80));
        let estimates = LatencyEstimates::none().updated_with(&unrelated);

        assert_eq!(
            3,
            refinements(&events_of_a_session(&context, &estimates)).len()
        );
    }

    #[test]
    fn the_timing_frame_carries_the_session_estimate_rather_than_one_measurement() {
        let (context, _directory) = context_with_deferred_modules("echo refined");
        let measured = estimates_from(&events_of(&context));

        // A second prompt folds its own measurement into what it was handed; what comes
        // back is neither run alone.
        let mut inflated = Timings::default();
        inflated.record("custom.first", Duration::from_secs(10));
        let handed_in = measured.updated_with(&inflated);
        let handed_back = estimates_from(&events_of_a_session(&context, &handed_in));

        let estimate = handed_back
            .of("custom.first")
            .expect("the module was measured");
        assert!(
            estimate < handed_in.of("custom.first").expect("it was handed in"),
            "a fast prompt must pull the estimate down, but it stayed at {estimate:?}"
        );
        assert!(
            estimate > measured.of("custom.first").expect("it was measured"),
            "one fast prompt must not erase the history either, but it fell to {estimate:?}"
        );
    }

    #[test]
    fn disabling_async_renders_one_finished_prompt_even_for_a_refinable_shell() {
        let (context, _directory) = context_with_deferred_modules("echo refined");
        let mut disabled = context;
        disabled = disabled.set_config(toml::toml! {
            add_newline = false
            format = "${custom.first} ${custom.second}$line_break$character"
            [async]
            enabled = false
            [character]
            format = "[>](green) "
            [custom.first]
            when = true
            command = "echo refined"
            format = "[$output](bold cyan)"
            [custom.second]
            when = true
            command = "echo refined"
            format = "[$output](yellow)"
        });

        let events = events_of(&disabled);
        assert_eq!(
            vec![EventKind::Ready, EventKind::Complete],
            kinds_of(&events),
            "a shell that could otherwise be refined must be sent one finished \
             prompt when [async] is disabled, but was {:?}",
            kinds_of(&events)
        );
        assert!(
            prompt_text(&events[0]).contains("refined"),
            "the one prompt sent must already be the finished one, but was {:?}",
            prompt_text(&events[0])
        );
    }

    #[test]
    fn disabling_adaptive_ignores_measured_latencies_and_keeps_the_fixed_window() {
        let (context, _directory) = context_with_three_width_stable_modules();
        let estimates = estimates_from(&events_of(&context));

        let mut not_adaptive = context;
        not_adaptive.root_config.asynchronous.adaptive = false;

        assert_eq!(
            3,
            refinements(&events_of_a_session(&not_adaptive, &estimates)).len(),
            "with adaptive off, measured latencies must not change how repaints \
             are grouped"
        );
    }

    #[test]
    fn async_dynamic_configuration_overrides_a_module_own_period() {
        let context = refinable(default_context()).set_config(toml::toml! {
            format = "$time"
            [time]
            disabled = false
        });
        let plan = Plan::build(&prompt_configuration(&context));

        let mut configured = context.root_config.asynchronous.dynamic.clone();
        configured.time = crate::configs::asynchronous::RefreshPeriod::try_from(250)
            .expect("250ms is within range");
        let mut asynchronous = context.root_config.asynchronous.clone();
        asynchronous.dynamic = configured;

        let modules = configured_dynamic_modules(&plan, &context, &asynchronous);
        assert_eq!(1, modules.len());
        assert_eq!(Duration::from_millis(250), modules[0].period());
    }

    #[test]
    fn a_dynamic_slot_is_not_due_until_its_initial_arrival() {
        let context = refinable(default_context()).set_config(toml::toml! {
            format = "$time"
            [time]
            disabled = false
        });
        let plan = Plan::build(&prompt_configuration(&context));
        let asynchronous = &context.root_config.asynchronous;
        let mut scheduler = Scheduler::new(&plan, &context, asynchronous);

        let slot_index = scheduler
            .polls
            .iter()
            .position(Option::is_some)
            .expect("time is dynamic");
        assert!(matches!(
            scheduler.polls[slot_index]
                .as_ref()
                .expect("time is dynamic")
                .state,
            PollState::AwaitingInitial
        ));

        let dynamic_module = configured_dynamic_modules(&plan, &context, asynchronous)
            .into_iter()
            .next()
            .expect("time is dynamic");
        let resolution = dynamic_module.resolve(&context);
        scheduler.register_resolution(&resolution);

        assert!(matches!(
            scheduler.polls[slot_index]
                .as_ref()
                .expect("time is dynamic")
                .state,
            PollState::Due(_)
        ));
    }

    #[test]
    fn the_directory_module_is_painted_before_it_has_really_resolved() {
        // The whole reason `Render::instant` exists: `directory` is deferred, so without
        // it the first paint shows no path.
        let home = tempfile::tempdir().expect("a temporary directory");
        let working = home.path().join("code");
        std::fs::create_dir_all(&working).expect("a working directory");

        let mut context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "$directory"
            [directory]
            format = "[$path]($style)"
            truncate_to_repo = true
        });
        context.current_dir = working.clone();
        context.logical_dir = working;
        context
            .env
            .insert("HOME", home.path().display().to_string());

        let events = events_of(&context);
        assert!(
            prompt_text(&events[0]).contains("code"),
            "the first paint should already show a path, but was {:?}",
            prompt_text(&events[0])
        );
    }

    // Guards the process-leak regression: `render::dynamic_modules` must skip switched-off
    // modules, or the streaming process never exits (see its doc comment in `src/render.rs`).

    /// The untouched default `$all` prompt must reach `Complete` with nothing left running;
    /// `battery` alone defaults to enabled, so it's pinned off here to keep that unrelated
    /// default from failing this test.
    #[test]
    fn the_default_configuration_leaves_nothing_running() {
        let context = refinable(default_context()).set_config(toml::toml! {
            [battery]
            disabled = true
        });

        let mut writer = ClosesAfter {
            buffer: Vec::new(),
            deadline: Instant::now() + Duration::from_secs(2),
        };
        let result = run(&context, &LatencyEstimates::none(), &mut writer);

        assert!(
            result.is_ok(),
            "a default prompt must run to completion rather than still be \
             writing when the deadline arrives, but got {result:?}"
        );

        let mut reader = Cursor::new(writer.buffer);
        let mut saw_complete = false;
        while let Some(event) = ServerEvent::read_from(&mut reader).expect("a well-formed event") {
            if matches!(event, ServerEvent::Complete(_)) {
                saw_complete = true;
            }
        }
        assert!(saw_complete, "a default prompt must report completion");
    }

    /// `time` defaults to `disabled = true`; naming it in `format` alone must not schedule
    /// re-polling, since a switched-off module can never change the screen.
    #[test]
    fn a_statically_disabled_dynamic_module_does_not_keep_the_session_alive() {
        let context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "$time"
        });

        let mut writer = ClosesAfter {
            buffer: Vec::new(),
            deadline: Instant::now() + Duration::from_secs(2),
        };
        let result = run(&context, &LatencyEstimates::none(), &mut writer);

        assert!(
            result.is_ok(),
            "a disabled `time` module must not keep the session alive, but \
             the stream was still writing when the deadline arrived: {result:?}"
        );
    }

    /// An enabled dynamic module is genuinely re-rendered while live, and its on-screen
    /// value actually changes between polls.
    #[test]
    fn an_enabled_dynamic_module_is_repolled_and_its_value_advances_on_screen() {
        let context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "$time"
            [time]
            disabled = false
            // Nanosecond precision so polls a few milliseconds apart are certain to disagree,
            // without waiting out a real clock second.
            time_format = "%N"
            [async.dynamic]
            time = 5
        });

        let mut writer = ClosesAfter {
            buffer: Vec::new(),
            deadline: Instant::now() + Duration::from_millis(200),
        };
        let result = run(&context, &LatencyEstimates::none(), &mut writer);

        assert!(
            matches!(&result, Err(error) if error.kind() == io::ErrorKind::BrokenPipe),
            "an enabled clock must keep the session alive past the deadline, \
             but the stream returned {result:?}"
        );

        let mut reader = Cursor::new(writer.buffer);
        let mut values = Vec::new();
        while let Ok(Some(event)) = ServerEvent::read_from(&mut reader) {
            match event {
                ServerEvent::Ready { prompt, .. } => {
                    values.push(String::from_utf8_lossy(prompt.as_bytes()).into_owned());
                }
                ServerEvent::Patch(patch) => {
                    values.push(String::from_utf8_lossy(patch.prompt().as_bytes()).into_owned());
                }
                ServerEvent::Complete(_) | ServerEvent::Heartbeat => {}
            }
        }

        assert!(
            values.len() >= 2,
            "expected at least two prompts in 200 milliseconds of 5-millisecond \
             polling, got {}",
            values.len()
        );
        let distinct: BTreeSet<&String> = values.iter().collect();
        assert!(
            distinct.len() > 1,
            "the time module's value never changed across repaints: {values:?}"
        );
    }

    /// A quiet poll cycle still writes a heartbeat before `HEARTBEAT` is up — the only way
    /// to notice a closed pipe with nothing else due. Takes just over one second to run;
    /// there is no faster way to observe it.
    #[test]
    fn a_quiet_interval_still_produces_a_heartbeat() {
        let context = refinable(default_context()).set_config(toml::toml! {
            add_newline = false
            format = "$time"
            [time]
            disabled = false
            [async.dynamic]
            time = 30_000
        });

        let mut writer = ClosesAfter {
            buffer: Vec::new(),
            deadline: Instant::now() + HEARTBEAT_INTERVAL + Duration::from_millis(200),
        };
        let result = run(&context, &LatencyEstimates::none(), &mut writer);

        assert!(
            matches!(&result, Err(error) if error.kind() == io::ErrorKind::BrokenPipe),
            "the session must still be running past one heartbeat for this to \
             prove anything, but returned {result:?}"
        );

        let mut reader = Cursor::new(writer.buffer);
        let mut saw_a_heartbeat = false;
        while let Ok(Some(event)) = ServerEvent::read_from(&mut reader) {
            if matches!(event, ServerEvent::Heartbeat) {
                saw_a_heartbeat = true;
            }
        }
        assert!(
            saw_a_heartbeat,
            "waiting past a whole HEARTBEAT produced no heartbeat event at all"
        );
    }
}
