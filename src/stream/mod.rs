//! Stream the main and right prompts as their modules render, and keep them
//! current while the shell waits for input.
//!
//! A shell reads `main\0right\0` each time the prompts change, until the
//! stream ends: once nothing will change them again, or the moment the shell
//! stops reading, whatever is still rendering. Each render runs on a thread of
//! its own, which the stream never waits for: it ends the process instead.

mod bus;
#[cfg(unix)]
mod snapshot;
#[cfg(test)]
mod testing;
mod timings;

use std::io::{self, Write};
use std::process;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, Scope};
use std::time::{Duration, Instant, SystemTime};

use crate::context::{Context, Properties, Target};
use crate::modules;

use crate::print::{self, DUMB_TERMINAL_PROMPT, Plan};
use crate::segment::Segment;

use bus::{Bus, Estimate, Rendering, Span};
use timings::Timings;

#[cfg(unix)]
pub use snapshot::detach;

/// The main prompt and the right one.
type Prompts = [String; 2];

/// Streams both prompts to standard output, redrawing them at most `redraws`
/// times after the first paint if that is limited.
pub fn stream(properties: Properties, redraws: Option<usize>) -> ! {
    #[cfg(unix)]
    end_with_reader();
    let context = Context::new(properties, Target::Main);
    run(&context, redraws, &mut io::stdout().lock(), end)
}

/// Streams both prompts to fish, which it signals as `shell`; see [`snapshot`].
#[cfg(unix)]
pub fn publish(properties: Properties, shell: i32) -> ! {
    let context = Context::new(properties, Target::Main);
    match snapshot::Snapshot::for_fish(nix::unistd::Pid::from_raw(shell)) {
        Ok(mut snapshot) => run(&context, None, &mut snapshot, |streamed| match streamed {
            Ok(()) => snapshot::linger(),
            Err(error) => end(Err(error)),
        }),
        Err(error) => end(Err(error)),
    }
}

/// Ends the process once its stream has, at once: whatever still renders,
/// nothing needs any more.
fn end(streamed: io::Result<()>) -> ! {
    match streamed {
        // A closed pipe is a shell that no longer wants this prompt.
        Err(error) if error.kind() != io::ErrorKind::BrokenPipe => {
            eprintln!("Unable to stream the prompt: {error}");
            process::exit(1)
        }
        _ => process::exit(0),
    }
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

/// Streams the prompts of `context` to `output`, then hands how that went to
/// `end`, while renders may still be running: the stream ends the process
/// there, a test returns and waits for them.
fn run<T>(
    context: &Context,
    redraws: Option<usize>,
    output: &mut impl Sink,
    end: impl FnOnce(io::Result<()>) -> T,
) -> T {
    if print::is_dumb_terminal(context) {
        log::error!("Under a 'dumb' terminal (TERM=dumb).");
        return end(output.draw(&[DUMB_TERMINAL_PROMPT, DUMB_TERMINAL_PROMPT].map(str::to_owned)));
    }

    let plan = Plan::new(context, [Target::Main, Target::Right]);
    let timings = Timings::open(context);
    let estimates = timings::read(
        timings.as_ref().map_or("", Timings::previous),
        &plan.modules,
    );
    let (sender, receiver) = mpsc::channel();
    let render = |position: usize, context: &Context<'_>| {
        let segments = print::render(&plan.modules[position], context);
        // Sent to a stream that may have ended without it.
        let _ = sender.send(Rendered { position, segments });
    };
    let mut session = Session::new(context, &plan, estimates, redraws, Clock::now(), timings);
    thread::scope(|scope| {
        let mut threads = Threads {
            scope,
            context,
            render: &render,
            receiver,
        };
        end(session.run(&mut threads, output))
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

/// What a module showed once it rendered.
struct Rendered {
    position: usize,
    segments: Vec<Segment>,
}

/// What renders modules for a stream: threads, or a simulation of them.
trait Renderer {
    /// Starts rendering every module.
    fn start(&mut self, modules: usize);

    /// Starts rendering the module at `position` again.
    fn render(&mut self, position: usize);

    /// Waits until a module renders, or until `deadline` if there is one, and
    /// returns when it stopped waiting and the modules rendered by then; or
    /// nothing, once the stream is to end, as a simulation does at its horizon.
    fn wait(
        &mut self,
        deadline: Option<Instant>,
    ) -> Option<(Instant, impl Iterator<Item = Rendered> + '_)>;
}

/// Renders each module on a thread of its own, sending what it shows. Every
/// module's first render shares what `context` observes of the filesystem;
/// a module rendering again observes it anew.
struct Threads<'scope, 'environment, F> {
    scope: &'scope Scope<'scope, 'environment>,
    context: &'environment Context<'environment>,
    render: &'environment F,
    receiver: Receiver<Rendered>,
}

impl<'environment, F> Renderer for Threads<'_, 'environment, F>
where
    F: Fn(usize, &Context<'environment>) + Sync,
{
    fn start(&mut self, modules: usize) {
        let (render, context) = (self.render, self.context);
        print::fan_out(self.scope, modules, move |position| {
            render(position, context)
        });
    }

    fn render(&mut self, position: usize) {
        let (render, context) = (self.render, self.context);
        let again = move || render(position, &context.renewed());
        if let Err(error) = thread::Builder::new().spawn_scoped(self.scope, again) {
            log::debug!("Unable to start rendering on a thread of its own: {error}");
            again();
        }
    }

    fn wait(
        &mut self,
        deadline: Option<Instant>,
    ) -> Option<(Instant, impl Iterator<Item = Rendered> + '_)> {
        let first = match deadline {
            Some(deadline) => self
                .receiver
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .ok(),
            // The sender outlives every render, so this waits for one.
            None => self.receiver.recv().ok(),
        };
        Some((
            Instant::now(),
            first.into_iter().chain(self.receiver.try_iter()),
        ))
    }
}

/// How long a stream waiting for a refresh goes without reading the wall
/// clock, which can be set, or skip a machine's sleep, meanwhile.
const LOOK_AT_THE_WALL: Duration = Duration::from_secs(1);

/// The monotonic clock as of the start, and the wall clock, as refreshes fall
/// due when it turns over.
struct Clock {
    start: Instant,
    wall: Wall,
}

enum Wall {
    /// The system's, read anew each time, however it has been set since.
    System,
    /// One that read this, since the Unix epoch, at the start, and has run
    /// with the monotonic clock since.
    #[cfg(test)]
    Simulated(Duration),
}

impl Clock {
    fn now() -> Self {
        Self {
            start: Instant::now(),
            wall: Wall::System,
        }
    }

    /// What the wall clock read at `now`, since the Unix epoch.
    fn wall(&self, now: Instant) -> Duration {
        match self.wall {
            Wall::System => {
                let wall = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
                let since = Instant::now().saturating_duration_since(now);
                wall.unwrap_or_default().saturating_sub(since)
            }
            #[cfg(test)]
            Wall::Simulated(start) => start + (now - self.start),
        }
    }

    /// The first reading after the wall clock's at `now` that is a multiple
    /// of `period`, unless it has none, being zero.
    fn next(&self, now: Instant, period: Duration) -> Option<Duration> {
        let wall = self.wall(now);
        let left = period.as_nanos() - wall.as_nanos().checked_rem(period.as_nanos())?;
        Some(wall + Duration::from_nanos(u64::try_from(left).ok()?))
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
    /// How often it renders again while the prompt is shown, if it does.
    period: Option<Duration>,
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
    /// Rendered, to render again once the wall clock reads this, if it ever
    /// does.
    Rendered(Option<Duration>),
}

impl Slot {
    /// Since when it has been rendering, if it is.
    fn since(&self) -> Option<Instant> {
        match self.state {
            State::Rendering(since) | State::Refreshing(since) => Some(since),
            State::Rendered(_) => None,
        }
    }

    /// When the wall clock reads what it renders again at, if it has
    /// rendered and ever does.
    fn due(&self) -> Option<Duration> {
        match self.state {
            State::Rendered(due) => due,
            State::Rendering(_) | State::Refreshing(_) => None,
        }
    }

    /// Starts rendering again at `now` if the wall clock, reading `wall`, has
    /// reached its time, or reads more than a period before it, having been
    /// set back.
    fn refresh(&mut self, now: Instant, wall: Duration) -> bool {
        let due = self
            .due()
            .zip(self.period)
            .is_some_and(|(due, period)| due <= wall || due > wall + period);
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
        self.state = State::Rendered(self.period.and_then(|period| clock.next(now, period)));
        match measured {
            Estimate::Changed(took) => Some(took),
            Estimate::Unchanged => None,
        }
    }
}

/// How many more times a shell can draw the prompts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Draws {
    Unlimited,
    /// Only so many, the last of them once every module has rendered.
    Left(usize),
}

impl Draws {
    /// The first paint, and `redraws` more if that is limited.
    fn after(redraws: Option<usize>) -> Self {
        redraws.map_or(Self::Unlimited, |redraws| {
            Self::Left(redraws.saturating_add(1))
        })
    }

    fn spend(&mut self) {
        if let Self::Left(left) = self {
            *left = left.saturating_sub(1);
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
    draws: Draws,
    /// Where to keep what the modules did for the next prompt, until kept.
    timings: Option<Timings>,
}

impl<'context> Session<'context> {
    fn new(
        context: &'context Context<'context>,
        plan: &'context Plan<'context, 2>,
        estimates: Vec<Option<Estimate>>,
        redraws: Option<usize>,
        clock: Clock,
        timings: Option<Timings>,
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
                period: configuration
                    .refresh
                    .then(|| modules::period(module, context))
                    .flatten(),
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
            draws: Draws::after(redraws),
            timings,
        }
    }

    fn run(&mut self, renderer: &mut impl Renderer, output: &mut impl Sink) -> io::Result<()> {
        renderer.start(self.slots.len());
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
            if self.finished() {
                return Ok(());
            }
            let wall = self.clock.wall(now);
            for (position, slot) in self.slots.iter_mut().enumerate() {
                if slot.refresh(now, wall) {
                    renderer.render(position);
                }
            }

            let departure = self.departure(now);
            let deadline = self
                .slots
                .iter()
                .filter_map(Slot::due)
                .map(|due| now + due.saturating_sub(wall).min(LOOK_AT_THE_WALL))
                .chain(departure)
                .min();
            let Some((then, rendered)) = renderer.wait(deadline) else {
                return Ok(());
            };
            now = then;
            for Rendered { position, segments } in rendered {
                if let Some(took) = self.slots[position].land(segments, now, &self.clock) {
                    self.bus.board(now, took);
                }
            }
            // Kept once, as soon as every module has rendered for this prompt.
            if self.rendered()
                && let Some(timings) = self.timings.take()
            {
                timings.keep(&self.report());
            }
        }
    }

    /// When the bus leaves, if anything is aboard.
    fn departure(&mut self, now: Instant) -> Option<Instant> {
        // Without async, the first paint waits indefinitely for every module,
        // including timeout-exempt commands. A shell's last draw is complete too.
        if (!self.context.root_config.asynchronous.enabled || self.draws == Draws::Left(1))
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
            self.draws.spend();
        }
        Ok(())
    }

    /// Whether nothing the shell is to draw will change: every module has
    /// rendered and been drawn, and either the shell draws only so often, or
    /// no module will ever render again.
    fn finished(&self) -> bool {
        self.bus.is_empty()
            && match self.draws {
                Draws::Left(_) => self.rendered(),
                Draws::Unlimited => self
                    .slots
                    .iter()
                    .all(|slot| matches!(slot.state, State::Rendered(None))),
            }
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
    use std::cell::{Cell, RefCell};
    use std::convert::identity;
    use std::path::PathBuf;

    fn milliseconds(milliseconds: u64) -> Duration {
        Duration::from_millis(milliseconds)
    }

    fn context(configuration: toml::Table) -> Context<'static> {
        let mut context = default_context().set_config(configuration);
        context.width = 20;
        context
    }

    /// What happened in a stream, in the order it happened, each at a time
    /// since the stream started.
    #[derive(Clone, Debug, PartialEq)]
    enum Event {
        /// A module, by position in the plan, started rendering.
        Started(Duration, usize),
        /// A module rendered, and shows this.
        Landed(Duration, usize, String),
        Drawn(Duration, Prompts),
    }

    /// A simulated clock, and what happened by it.
    struct World {
        start: Instant,
        now: Cell<Instant>,
        events: RefCell<Vec<Event>>,
    }

    impl World {
        fn since(&self) -> Duration {
            self.now.get() - self.start
        }

        fn log(&self, event: Event) {
            self.events.borrow_mut().push(event);
        }
    }

    /// Each module's renders, as how many milliseconds each takes and what it
    /// shows, in turn, over and over.
    type Script<'a> = &'a [(&'a str, &'a [(u64, &'a str)])];

    /// Modules rendering on the simulated clock, until a horizon.
    struct Simulation<'a> {
        world: &'a World,
        renders: Vec<std::iter::Cycle<std::slice::Iter<'a, (u64, &'a str)>>>,
        horizon: Instant,
        rendering: Vec<(Instant, usize, &'a str)>,
        /// How many times running the stream has waited for nothing.
        idle: usize,
        reached_horizon: bool,
    }

    impl Renderer for Simulation<'_> {
        fn start(&mut self, modules: usize) {
            for position in 0..modules {
                self.render(position);
            }
        }

        fn render(&mut self, position: usize) {
            let &(took, text) = self.renders[position].next().expect("a render");
            self.world.log(Event::Started(self.world.since(), position));
            let lands = self.world.now.get() + milliseconds(took);
            self.rendering.push((lands, position, text));
        }

        fn wait(
            &mut self,
            deadline: Option<Instant>,
        ) -> Option<(Instant, impl Iterator<Item = Rendered> + '_)> {
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
            let now = next.max(self.world.now.get());
            let (landed, rendering) = self
                .rendering
                .drain(..)
                .partition::<Vec<_>, _>(|&(lands, ..)| lands <= now);
            self.rendering = rendering;
            self.idle = if landed.is_empty() && now == self.world.now.get() {
                self.idle + 1
            } else {
                0
            };
            assert!(self.idle < 3, "the stream spins without anything happening");
            self.world.now.set(now);
            let rendered = landed
                .into_iter()
                .map(|(_, position, text)| {
                    self.world
                        .log(Event::Landed(self.world.since(), position, text.to_owned()));
                    let segments = if text.is_empty() {
                        Vec::new()
                    } else {
                        Segment::from_text(None, text)
                    };
                    Rendered { position, segments }
                })
                .collect::<Vec<_>>();
            Some((now, rendered.into_iter()))
        }
    }

    impl Sink for &World {
        fn draw(&mut self, prompts: &Prompts) -> io::Result<()> {
            self.log(Event::Drawn(self.since(), prompts.clone()));
            Ok(())
        }
    }

    struct Settings {
        /// The wall clock at the start, in milliseconds since the Unix epoch.
        wall: u64,
        horizon: u64,
        redraws: Option<usize>,
        /// Where the session keeps its timings, if anywhere.
        timings: Option<PathBuf>,
    }

    impl Default for Settings {
        fn default() -> Self {
            Self {
                wall: 0,
                horizon: 60_000,
                redraws: None,
                timings: None,
            }
        }
    }

    struct Simulated {
        events: Vec<Event>,
        /// The modules of the plan, by position.
        modules: Vec<String>,
        report: String,
        /// Whether the stream ended before the horizon.
        ended: bool,
    }

    impl Simulated {
        fn drawn(&self) -> impl Iterator<Item = (Duration, &Prompts)> {
            self.events.iter().filter_map(|event| match event {
                Event::Drawn(at, prompts) => Some((*at, prompts)),
                Event::Started(..) | Event::Landed(..) => None,
            })
        }

        /// When each main prompt was drawn, in milliseconds, and what it was.
        fn mains(&self) -> Vec<(u64, &str)> {
            self.drawn()
                .map(|(at, [main, _])| (at.as_millis().try_into().unwrap(), main.as_str()))
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
        let world = World {
            start,
            now: Cell::new(start),
            events: RefCell::new(Vec::new()),
        };
        let renders = plan
            .modules
            .iter()
            .map(|module| {
                let (_, renders) = script
                    .iter()
                    .rfind(|(name, _)| name == module)
                    .unwrap_or_else(|| panic!("no script for {module}"));
                assert!(!renders.is_empty(), "an empty script for {module}");
                renders.iter().cycle()
            })
            .collect();
        let mut simulation = Simulation {
            world: &world,
            renders,
            horizon: start + milliseconds(settings.horizon),
            rendering: Vec::new(),
            idle: 0,
            reached_horizon: false,
        };
        let clock = Clock {
            start,
            wall: Wall::Simulated(milliseconds(settings.wall)),
        };
        let estimates = timings::read(timings, &plan.modules);
        let timings = settings
            .timings
            .map(|directory| Timings::at(directory, "key", SystemTime::now()));
        let mut session = Session::new(context, &plan, estimates, settings.redraws, clock, timings);
        session.run(&mut simulation, &mut &world).unwrap();
        Simulated {
            events: world.events.take(),
            modules: plan.modules.iter().cloned().collect(),
            report: session.report(),
            ended: !simulation.reached_horizon,
        }
    }

    /// What generated modules show. Shared among them, so that modules can
    /// change what they show without changing what they compose.
    const OUTPUTS: [&str; 4] = ["", "a", "b", "ab"];

    /// Outputs that no two modules share, for tests that tell modules apart
    /// by what they show.
    const DISTINCT: [&str; 5] = ["a", "b", "c", "d", "e"];

    /// A render that takes longer than any span a stream plans with.
    const DAY_AND_A_MILLISECOND: u64 = 86_400_001;

    /// A module of a generated prompt.
    #[derive(Clone, Debug)]
    struct Module {
        previous: Option<Estimate>,
        /// How often it renders again, in milliseconds, if it does.
        period: Option<u64>,
        /// Whether it is in the right prompt rather than the main one.
        right: bool,
        /// How long each render takes, in milliseconds, and what it shows,
        /// in turn.
        renders: Vec<(u64, &'static str)>,
    }

    /// A generated prompt, its configuration, and how long it is watched.
    #[derive(Clone, Debug)]
    struct Scenario {
        modules: Vec<Module>,
        fallback: u64,
        asynchronous: bool,
        adaptive: bool,
        refresh: bool,
        redraws: Option<usize>,
        wall: u64,
        horizon: u64,
    }

    impl Scenario {
        /// A prompt of `modules`, under the default configuration but for
        /// `fallback`, watched until long after every module has rendered.
        fn plain(modules: Vec<Module>, fallback: u64) -> Self {
            let mut scenario = Self {
                modules,
                fallback,
                asynchronous: true,
                adaptive: true,
                refresh: true,
                redraws: None,
                wall: 0,
                horizon: 0,
            };
            scenario.horizon = scenario.complete() + fallback + 1_000;
            scenario
        }

        fn name(index: usize) -> String {
            format!("custom.{index}")
        }

        fn context(&self) -> Context<'static> {
            let format = |right: bool| -> String {
                (0..self.modules.len())
                    .filter(|&index| self.modules[index].right == right)
                    .map(|index| format!("${{{}}}", Self::name(index)))
                    .collect()
            };
            let custom: toml::Table = self
                .modules
                .iter()
                .enumerate()
                .map(|(index, module)| {
                    let refresh = i64::try_from(module.period.unwrap_or(0)).unwrap();
                    let configuration = toml::toml! { command = "generated" refresh = refresh };
                    (index.to_string(), configuration.into())
                })
                .collect();
            let (enabled, refresh, adaptive) = (self.asynchronous, self.refresh, self.adaptive);
            let fallback = i64::try_from(self.fallback).unwrap();
            let mut configuration = toml::toml! {
                add_newline = false
                [async]
                enabled = enabled
                refresh = refresh
                [async.bus]
                adaptive = adaptive
                fallback = fallback
            };
            configuration.insert("format".into(), format(false).into());
            configuration.insert("right_format".into(), format(true).into());
            configuration.insert("custom".into(), custom.into());
            context(configuration)
        }

        /// When every module has rendered once, in milliseconds.
        fn complete(&self) -> u64 {
            let first = self.modules.iter().map(|module| module.renders[0].0);
            first.max().unwrap_or(0)
        }

        fn simulate(&self, timings: Option<PathBuf>) -> Simulated {
            let names: Vec<String> = (0..self.modules.len()).map(Self::name).collect();
            let script: Vec<(&str, &[(u64, &str)])> = names
                .iter()
                .zip(&self.modules)
                .map(|(name, module)| (name.as_str(), module.renders.as_slice()))
                .collect();
            let previous = timings::report(
                names
                    .iter()
                    .zip(&self.modules)
                    .filter_map(|(name, module)| Some((name.as_str(), module.previous?))),
            );
            let settings = Settings {
                wall: self.wall,
                horizon: self.horizon,
                redraws: self.redraws,
                timings,
            };
            simulate_in(settings, &self.context(), &previous, &script)
        }

        /// The prompts the modules compose, given which output each shows.
        fn compose(&self, shown: &[&str]) -> Prompts {
            let side = |right: bool| -> String {
                (0..self.modules.len())
                    .filter(|&index| self.modules[index].right == right)
                    .map(|index| shown[index])
                    .collect()
            };
            [side(false), side(true)]
        }
    }

    fn estimate() -> impl Strategy<Value = Option<Estimate>> {
        prop_oneof![
            Just(None),
            Just(Some(Estimate::Unchanged)),
            (0_u64..400).prop_map(|took| Some(Estimate::Changed(Span::from(milliseconds(took))))),
        ]
    }

    /// A module showing one of `outputs`, which refreshes every `period`
    /// milliseconds if `live` would have it, or else whose first render may
    /// take longer than a day.
    fn module(
        live: bool,
        outputs: &'static [&'static str],
        period: u64,
    ) -> impl Strategy<Value = Module> {
        let first = if live {
            (0_u64..400).boxed()
        } else {
            prop_oneof![9 => 0_u64..400, 1 => Just(DAY_AND_A_MILLISECOND)].boxed()
        };
        // Refreshes that take one of a few durations land together, and some
        // take longer than the stream is watched.
        let again = prop_oneof![
            4 => prop::sample::select(&[0_u64, 5, 20][..]),
            4 => 0_u64..400,
            1 => Just(60_000_u64),
        ];
        let period = if live {
            prop::option::of(Just(period)).boxed()
        } else {
            Just(None).boxed()
        };
        let output = prop::sample::select(outputs);
        (
            estimate(),
            period,
            any::<bool>(),
            (first, output.clone()),
            prop::collection::vec((again, output), 0..3),
        )
            .prop_map(|(previous, period, right, first, again)| Module {
                previous,
                period,
                right,
                renders: std::iter::once(first).chain(again).collect(),
            })
    }

    /// A prompt that refreshes, or one that may take longer than a day, its
    /// modules refreshing together, if at all: every hundred milliseconds or
    /// more, or sooner than a bus leaves. Some show only nothing or `a`, so
    /// that two of them swapping what they show compose the same prompt.
    fn scenario() -> impl Strategy<Value = Scenario> {
        (
            any::<bool>(),
            prop_oneof![Just(&OUTPUTS[..]), Just(&OUTPUTS[..2])],
            prop_oneof![1_u64..40, 100_u64..1_500],
        )
            .prop_flat_map(|(live, outputs, period)| {
                (
                    prop::collection::vec(module(live, outputs, period), 1..=DISTINCT.len()),
                    10_u64..100,
                    any::<bool>(),
                    any::<bool>(),
                    any::<bool>(),
                    prop::option::of(0_usize..3),
                    0_u64..2_000,
                    prop::bool::weighted(0.1),
                )
                    .prop_map(
                        move |(
                            modules,
                            fallback,
                            asynchronous,
                            adaptive,
                            refresh,
                            redraws,
                            wall,
                            short,
                        )| {
                            let mut scenario = Scenario {
                                modules,
                                fallback,
                                asynchronous,
                                adaptive,
                                refresh: refresh && live,
                                redraws,
                                wall,
                                horizon: 0,
                            };
                            let complete = scenario.complete();
                            scenario.horizon = if short && complete > 0 {
                                complete - 1
                            } else if live {
                                1_500
                            } else {
                                complete + fallback + 1_000
                            };
                            scenario
                        },
                    )
            })
    }

    /// What a stream must do, whatever its timetable: draw the prompts just
    /// as its modules show them, never the same twice running, at most a
    /// fallback late; render again exactly as the wall clock turns over each
    /// period; end exactly when nothing will change the prompts again, or
    /// once complete for a shell that can draw only so often; and keep what
    /// each module did for this prompt as soon as each has rendered once.
    fn check(scenario: &Scenario, simulated: &Simulated, kept: &str) -> Result<(), TestCaseError> {
        let index = |position: usize| -> usize {
            simulated.modules[position]
                .strip_prefix("custom.")
                .and_then(|index| index.parse().ok())
                .expect("a generated module")
        };
        let count = scenario.modules.len();
        let fallback = milliseconds(scenario.fallback);
        let complete = milliseconds(scenario.complete());
        let horizon = milliseconds(scenario.horizon);
        let finished = complete <= horizon;
        let period = |module: usize| {
            scenario.modules[module]
                .period
                .filter(|_| scenario.refresh)
                .map(milliseconds)
        };

        let mut shown = vec![""; count];
        // What the modules composed, from when.
        let mut composed = vec![(Duration::ZERO, scenario.compose(&shown))];
        let mut drawn: Vec<(Duration, &Prompts)> = Vec::new();
        let mut landed: Vec<Option<Duration>> = vec![None; count];
        let mut due: Vec<Option<Duration>> = vec![None; count];
        let mut first: Vec<Option<(Duration, &str)>> = vec![None; count];
        for event in &simulated.events {
            match event {
                Event::Started(at, position) => {
                    let module = index(*position);
                    match landed[module] {
                        None => prop_assert_eq!(Duration::ZERO, *at, "every module starts at once"),
                        Some(_) => {
                            prop_assert_eq!(due[module], Some(*at), "rendered again when due")
                        }
                    }
                    due[module] = None;
                }
                Event::Landed(at, position, text) => {
                    let module = index(*position);
                    shown[module] = text;
                    landed[module] = Some(*at);
                    first[module].get_or_insert((*at, text));
                    // The first instant after it at which the wall clock
                    // reads a whole number of periods.
                    due[module] = period(module).map(|period| {
                        let wall = (milliseconds(scenario.wall) + *at).as_nanos();
                        let left = period.as_nanos() - wall % period.as_nanos();
                        *at + Duration::from_nanos(left.try_into().unwrap())
                    });
                    composed.push((*at, scenario.compose(&shown)));
                }
                Event::Drawn(at, prompts) => {
                    prop_assert_eq!(
                        &scenario.compose(&shown),
                        prompts,
                        "drawn as shown at {:?}",
                        at
                    );
                    if let Some(&(_, previous)) = drawn.last() {
                        prop_assert_ne!(previous, prompts, "drawn twice running");
                    }
                    drawn.push((*at, prompts));
                }
            }
        }
        let last = scenario.compose(&shown);

        // Every refresh that fell due while the stream went on started.
        if !simulated.ended {
            for (module, due) in due.iter().enumerate() {
                prop_assert!(
                    due.is_none_or(|due| due >= horizon),
                    "custom.{} never refreshed",
                    module
                );
            }
        }

        let ends_by_horizon = complete + fallback <= horizon;
        match scenario.redraws {
            Some(redraws) => {
                prop_assert!(drawn.len() <= redraws + 1, "drawn {} times", drawn.len());
                if finished {
                    prop_assert!(simulated.ended || !ends_by_horizon, "ends once complete");
                } else {
                    prop_assert!(
                        drawn.len() <= redraws,
                        "the last draw waits for every module"
                    );
                    prop_assert!(!simulated.ended);
                }
            }
            None if scenario.refresh
                && scenario
                    .modules
                    .iter()
                    .any(|module| module.period.is_some()) =>
            {
                prop_assert!(!simulated.ended, "a refreshing prompt never settles");
            }
            None => {
                prop_assert!(simulated.ended || !ends_by_horizon, "nothing renders again");
                prop_assert!(finished || !simulated.ended);
            }
        }
        if simulated.ended {
            prop_assert_eq!(
                drawn.last().map(|&(_, prompts)| prompts),
                Some(&last),
                "ends as shown"
            );
        }

        if scenario.asynchronous {
            if scenario.redraws.is_none() {
                // Whenever the modules last showed something a fallback ago,
                // the shell shows that or something newer.
                let then =
                    std::iter::once(Duration::ZERO).chain(composed.iter().map(|&(at, _)| at));
                for at in then.map(|at| at + fallback).filter(|&at| at <= horizon) {
                    let shows = drawn.iter().rev().find(|&&(drawn, _)| drawn <= at);
                    let current = composed
                        .iter()
                        .enumerate()
                        .filter(|&(position, &(from, _))| {
                            let until = composed
                                .get(position + 1)
                                .map_or(Duration::MAX, |&(until, _)| until);
                            from <= at && until.saturating_add(fallback) >= at
                        });
                    let mut current = current.map(|(_, (_, prompts))| prompts);
                    prop_assert!(
                        shows.is_some_and(|&(_, shows)| current.any(|prompts| prompts == shows)),
                        "at {:?} the shell shows {:?}, over a fallback late",
                        at,
                        shows
                    );
                }
            }
        } else {
            prop_assert!(
                drawn.iter().all(|&(at, _)| at >= complete),
                "drawn before every module rendered"
            );
            if complete.max(fallback) <= horizon && finished {
                prop_assert!(
                    drawn
                        .first()
                        .is_some_and(|&(at, _)| at <= complete.max(fallback))
                );
            }
        }

        let report = timings::report(simulated.modules.iter().enumerate().map(
            |(position, name)| {
                let module = index(position);
                let (took, text) = first[module].unwrap_or_default();
                let measured = if text.is_empty() {
                    Estimate::Unchanged
                } else {
                    Estimate::Changed(Span::from(took))
                };
                (
                    name.as_str(),
                    Estimate::after(scenario.modules[module].previous, measured),
                )
            },
        ));
        if finished {
            prop_assert_eq!(&report, kept, "kept once every module rendered");
            prop_assert_eq!(&report, &simulated.report, "refreshes measure nothing");
        } else {
            prop_assert_eq!("", kept, "kept before every module rendered");
        }
        Ok(())
    }

    /// What `modules` show and how long each took to: those that show
    /// something, rendering once.
    fn visible(modules: &[Module]) -> Vec<(Duration, &'static str)> {
        modules
            .iter()
            .map(|module| (milliseconds(module.renders[0].0), module.renders[0].1))
            .filter(|(_, output)| !output.is_empty())
            .collect()
    }

    /// Modules that each render once, telling themselves apart.
    fn distinct(renders: impl IntoIterator<Item = (Option<Estimate>, u64, bool)>) -> Vec<Module> {
        renders
            .into_iter()
            .zip(DISTINCT)
            .map(|((previous, took, shows), output)| Module {
                previous,
                period: None,
                right: false,
                renders: vec![(took, if shows { output } else { "" })],
            })
            .collect()
    }

    /// What the timetable drawn cost: the wait for the first paint, and for
    /// each redraw, the size of the bus its first change opened and the wait.
    fn drawn_cost(
        drawn: &[(Duration, &str)],
        visible: &[(Duration, &str)],
        fallback: Span,
    ) -> Duration {
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
            bus::size(fallback, Span::from(first)).get() + (*at - first)
        });
        drawn[0].0 + redraws.sum::<Duration>()
    }

    fn cheapest_cost(visible: &[(Duration, &str)], fallback: Span) -> Option<Duration> {
        let mut landings: Vec<_> = visible
            .iter()
            .map(|&(took, _)| bus::Landing {
                after: took,
                size: bus::size(fallback, Span::from(took)),
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

        #[test]
        fn a_stream_shows_its_modules_faithfully_and_promptly(scenario in scenario()) {
            let directory = tempfile::tempdir().unwrap();
            let simulated = scenario.simulate(Some(directory.path().to_path_buf()));
            let kept = Timings::at(directory.path().to_path_buf(), "key", SystemTime::now());
            check(&scenario, &simulated, kept.previous())?;

            // Without adaptive planning, or without async, what any module
            // did for the previous prompt makes no difference.
            if !(scenario.asynchronous && scenario.adaptive) {
                let mut unmeasured = scenario.clone();
                for module in &mut unmeasured.modules {
                    module.previous = None;
                }
                prop_assert_eq!(&simulated.events, &unmeasured.simulate(None).events);
            }
        }

        /// When every module does what it did before, the prompt is drawn on
        /// the cheapest timetable there is.
        #[test]
        fn true_estimates_make_the_cheapest_timetable(
            modules in prop::collection::vec((0_u64..300, any::<bool>()), 1..=DISTINCT.len()),
            fallback in 10_u64..100,
        ) {
            let modules = distinct(modules.into_iter().map(|(took, shows)| {
                let previous = if shows {
                    Estimate::Changed(Span::from(milliseconds(took)))
                } else {
                    Estimate::Unchanged
                };
                (Some(previous), took, shows)
            }));
            let simulated = Scenario::plain(modules.clone(), fallback).simulate(None);
            let drawn: Vec<_> = simulated.drawn().map(|(at, [main, _])| (at, main.as_str())).collect();
            let visible = visible(&modules);
            let fallback = Span::from(milliseconds(fallback));

            prop_assert_eq!(cheapest_cost(&visible, fallback), Some(drawn_cost(&drawn, &visible, fallback)), "{:?}", drawn);
        }

        /// Modules never measured are waited for until the fallback, and the
        /// first paint shows all of them that have rendered by then.
        #[test]
        fn modules_never_measured_are_waited_for_until_the_fallback(
            took in prop::collection::vec(0_u64..120, 1..=DISTINCT.len()),
            fallback in 10_u64..100,
        ) {
            let modules = distinct(took.iter().map(|&took| (None, took, true)));
            let simulated = Scenario::plain(modules.clone(), fallback).simulate(None);
            let (first_paint, [prompt, _]) = simulated.drawn().next().unwrap();
            let rendered: String = visible(&modules)
                .into_iter()
                .filter(|&(took, _)| took <= first_paint)
                .map(|(_, output)| output)
                .collect();

            prop_assert_eq!(milliseconds(*took.iter().max().unwrap()).min(milliseconds(fallback)), first_paint);
            prop_assert_eq!(&rendered, prompt);
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

    #[test]
    fn a_refresh_falls_due_by_the_wall_clock_however_it_is_set() {
        let now = Instant::now();
        let at = Duration::from_secs;
        let due_at_100 = || Slot {
            shown: Vec::new(),
            period: Some(at(1)),
            estimate: None,
            state: State::Rendered(Some(at(100))),
        };
        for (wall, due, why) in [
            (99, false, "before its time"),
            (100, true, "at its time"),
            (
                3_600,
                true,
                "the clock set forward, or the machine woken from sleep",
            ),
            (98, true, "the clock set back by more than a period"),
        ] {
            assert_eq!(due, due_at_100().refresh(now, at(wall)), "{why}");
        }
    }

    /// Too rare to generate: modules that refresh together and swap what they
    /// show compose the same prompt again, which is not drawn again.
    #[test]
    fn modules_that_swap_what_they_show_draw_nothing_new() {
        let swapping = |shown: [&'static str; 2]| Module {
            previous: None,
            period: Some(1_000),
            right: false,
            renders: vec![(1, shown[0]), (1, shown[1])],
        };
        let mut scenario = Scenario::plain(vec![swapping(["a", ""]), swapping(["", "a"])], 50);
        scenario.horizon = 3_500;
        let simulated = scenario.simulate(None);

        assert_eq!(vec![(1, "a")], simulated.mains());
        assert!(!simulated.ended, "both run again each second");
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
    fn a_dumb_terminal_gets_the_standard_fallback() {
        let mut context = context(toml::toml! { format = "$time" });
        context.env.insert("TERM", "dumb".to_owned());
        let mut output = Vec::new();
        run(&context, None, &mut output, identity).unwrap();

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
            deadline: Instant::now() + milliseconds(300),
        };

        let error = run(&context, None, &mut output, identity).unwrap_err();
        assert_eq!(io::ErrorKind::BrokenPipe, error.kind());
        let mains: Vec<_> = read_prompts(&output.written)
            .into_iter()
            .map(|[main, _]| main)
            .collect();
        assert!(mains.len() >= 2, "the time turned over: {mains:?}");
        assert!(mains.windows(2).all(|pair| pair[0] != pair[1]));
    }

    /// A shell that creates a marker once drawn the first prompts, and stops
    /// reading once drawn prompts showing `until`, or at a deadline.
    struct Watching {
        written: Vec<u8>,
        marker: PathBuf,
        until: &'static str,
        deadline: Instant,
    }

    impl Write for Watching {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let shown = String::from_utf8_lossy(&self.written).contains(self.until);
            if shown || Instant::now() >= self.deadline {
                return Err(io::ErrorKind::BrokenPipe.into());
            }
            self.written.extend_from_slice(bytes);
            std::fs::write(&self.marker, "")?;
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_module_rendering_again_finds_what_appeared_since() {
        let directory = tempfile::tempdir().unwrap();
        let mut context = context(toml::toml! {
            add_newline = false
            format = "${custom.found}$time"
            time.disabled = false
            time.time_format = "%.3f"
            [custom.found]
            command = "printf found"
            detect_files = ["marker"]
            when = false
            format = "$output"
            refresh = 50
        });
        context.current_dir = directory.path().to_path_buf();
        let mut shell = Watching {
            written: Vec::new(),
            marker: directory.path().join("marker"),
            until: "found",
            deadline: Instant::now() + Duration::from_secs(3),
        };

        let error = run(&context, None, &mut shell, identity).unwrap_err();
        assert_eq!(io::ErrorKind::BrokenPipe, error.kind());
        let prompts = read_prompts(&shell.written);
        assert!(!prompts[0][0].contains("found"), "{:?}", prompts[0]);
        assert!(
            prompts.iter().any(|[main, _]| main.contains("found")),
            "a refresh kept what the first render observed, to {:?}",
            prompts.last()
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_stream_ends_without_waiting_for_renders_it_no_longer_needs() {
        let directory = tempfile::tempdir().unwrap();
        let mut context = context(toml::toml! {
            add_newline = false
            format = "${custom.again}${custom.slow}"
            command_timeout = 10_000
            [custom.again]
            // At once the first time, and for two seconds every time after.
            command = "test -f rendered && sleep 2; touch rendered; printf again"
            when = true
            shell = ["/bin/sh"]
            format = "$output"
            refresh = 10
            [custom.slow]
            command = "sleep 0.3; printf slow"
            when = true
            shell = ["/bin/sh"]
            format = "$output"
        });
        context.current_dir = directory.path().to_path_buf();
        let start = Instant::now();
        let mut output = Vec::new();

        let ended = run(&context, Some(0), &mut output, |streamed| {
            streamed.unwrap();
            start.elapsed()
        });
        assert!(
            ended < Duration::from_millis(1_500),
            "ended {ended:?} in, waiting on a refresh nothing needed"
        );
        assert_eq!(
            vec![["againslow".to_owned(), String::new()]],
            read_prompts(&output)
        );
    }

    #[test]
    fn a_stream_ends_on_the_prompts_starship_prints() {
        let directory = tempfile::tempdir().unwrap();
        for asynchronous in [true, false] {
            let mut configuration = toml::toml! {
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
            configuration.insert(
                "async".to_owned(),
                toml::toml! { enabled = asynchronous }.into(),
            );
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
                run(&context_for(Target::Main), None, &mut output, identity).unwrap();

                let mut drawn = read_prompts(&output);
                assert_eq!(
                    Some([
                        print::get_prompt(&context_for(Target::Main)),
                        print::get_prompt(&context_for(Target::Right))
                    ]),
                    drawn.pop(),
                    "under {shell:?}"
                );
                assert!(
                    asynchronous || drawn.is_empty(),
                    "without async, drawn once complete"
                );
            }
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
}
