//! Prompts streamed to a shell: drawn as soon as what renders quickly has,
//! and refined as the rest does.

mod bus;
mod frame;

use std::io;
use std::time::Instant;

use crate::context::{Context, Properties, Target};
use crate::plan::Plan;
use crate::print::{self, DUMB_TERMINAL_PROMPT};
use crate::segment::Segment;
use crate::workers::{self, Rendered, Workers};

use bus::Bus;
use frame::{Frame, Sink};

/// Streams the main and right prompts to standard output.
pub fn stream(properties: Properties) -> io::Result<()> {
    let context = Context::new(properties, Target::Main);
    run(&context, &mut io::stdout().lock())
}

fn run(context: &Context, output: &mut impl Sink) -> io::Result<()> {
    output.send(Frame::Process)?;
    if print::is_dumb_terminal() {
        log::error!("Under a 'dumb' terminal (TERM=dumb).");
        output.send(Frame::Prompt {
            main: DUMB_TERMINAL_PROMPT,
            right: DUMB_TERMINAL_PROMPT,
        })?;
        return output.send(Frame::Complete);
    }

    let plan = Plan::new(context, [Target::Main, Target::Right]);
    if !context.root_config.asynchronous.enabled {
        let rendered = workers::render_all(context, &plan.modules);
        let [main, right] = plan.prompts(&|position| &rendered[position], context);
        output.send(Frame::Prompt {
            main: &main,
            right: &right,
        })?;
        return output.send(Frame::Complete);
    }
    workers::with_workers(context, &plan.modules, None, |mut workers| {
        Session::new(context, &plan, Instant::now()).run(&mut workers, output)
    })
}

/// Where the modules a session shows render.
trait Renderer {
    /// Starts rendering the module at `position` in the plan.
    fn render(&mut self, position: usize);

    /// Waits for a module to land, or for `deadline` to pass if there is one.
    /// Returns the time then, and every module that has landed by then.
    fn wait(&mut self, deadline: Option<Instant>) -> (Instant, Vec<Rendered>);
}

impl Renderer for &Workers<'_> {
    fn render(&mut self, position: usize) {
        Workers::render(self, position);
    }

    fn wait(&mut self, deadline: Option<Instant>) -> (Instant, Vec<Rendered>) {
        let landed = Workers::wait(self, deadline);
        (Instant::now(), landed)
    }
}

/// A stream under way: what each module shows, and what has been drawn.
struct Session<'a> {
    context: &'a Context<'a>,
    plan: &'a Plan<'a, 2>,
    /// When the stream started.
    start: Instant,
    /// One for each of the plan's modules, by position.
    slots: Vec<Slot>,
    bus: Bus,
    /// The main and right prompts as last drawn.
    drawn: Option<[String; 2]>,
    /// Whether the stream has reported that every module has rendered.
    complete: bool,
}

/// One module of the plan, as a stream tracks it.
struct Slot {
    /// What the module last rendered.
    segments: Vec<Segment>,
    state: State,
}

#[derive(Clone, Copy)]
enum State {
    Rendering,
    Rendered,
}

impl<'a> Session<'a> {
    fn new(context: &'a Context<'a>, plan: &'a Plan<'a, 2>, start: Instant) -> Self {
        let slots = plan
            .modules
            .iter()
            .map(|_| Slot {
                segments: Vec::new(),
                state: State::Rendering,
            })
            .collect();
        Self {
            context,
            plan,
            bus: Bus::new(start, context.root_config.asynchronous.bus.fallback()),
            start,
            slots,
            drawn: None,
            complete: false,
        }
    }

    fn run(mut self, renderer: &mut impl Renderer, output: &mut impl Sink) -> io::Result<()> {
        for position in 0..self.slots.len() {
            renderer.render(position);
        }
        let mut now = self.start;
        loop {
            let deadline = self.advance(now, output)?;
            if self.complete {
                return Ok(());
            }
            let (time, landed) = renderer.wait(deadline);
            now = time;
            for rendered in landed {
                self.land(rendered, now);
            }
        }
    }

    /// Does what is due by `now`: draws what is aboard the bus if it leaves,
    /// and reports once every module has rendered and been drawn. Returns
    /// when the bus leaves, if no module lands first.
    fn advance(&mut self, now: Instant, output: &mut impl Sink) -> io::Result<Option<Instant>> {
        if self
            .departure(now)
            .is_some_and(|departure| departure <= now)
        {
            self.draw(output)?;
        }
        if !self.complete && self.bus.is_empty() && !self.rendering() {
            output.send(Frame::Complete)?;
            self.complete = true;
        }
        Ok(self.departure(now))
    }

    fn draw(&mut self, output: &mut impl Sink) -> io::Result<()> {
        self.bus.leave();
        let prompts = self
            .plan
            .prompts(&|position| &self.slots[position].segments, self.context);
        if self.drawn.as_ref() == Some(&prompts) {
            return Ok(());
        }
        let [main, right] = &prompts;
        output.send(Frame::Prompt { main, right })?;
        self.drawn = Some(prompts);
        Ok(())
    }

    /// Takes in a module that landed at `now`: what it shows boards the bus
    /// if it changed.
    fn land(&mut self, rendered: Rendered, now: Instant) {
        let slot = &mut self.slots[rendered.position];
        // Only a module that is rendering lands.
        let State::Rendering = slot.state else {
            return;
        };
        if slot.segments != rendered.segments {
            slot.segments = rendered.segments;
            self.bus.board(now, now - self.start);
        }
        slot.state = State::Rendered;
    }

    /// When the bus leaves, given whether any module is still rendering.
    fn departure(&self, now: Instant) -> Option<Instant> {
        self.bus.departure(now, self.rendering())
    }

    /// Whether any module is still rendering.
    fn rendering(&self) -> bool {
        self.slots
            .iter()
            .any(|slot| matches!(slot.state, State::Rendering))
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
    use std::rc::Rc;
    use std::time::Duration;

    fn context(configuration: toml::Table) -> Context<'static> {
        let mut context = default_context().set_config(configuration);
        context.width = 20;
        context
    }

    fn milliseconds(milliseconds: u64) -> Duration {
        Duration::from_millis(milliseconds)
    }

    /// How each module renders in a simulation: every render of it takes the
    /// time, in milliseconds, and shows the text, of the next of its renders,
    /// and once those run out, of the last.
    type Script<'a> = &'a [(&'a str, &'a [(u64, &'a str)])];

    /// Modules that render as a script says, on a clock of the test's own,
    /// on as many workers as `workers` says.
    struct Simulation<'a> {
        plan: &'a Plan<'a, 2>,
        script: HashMap<&'a str, &'a [(u64, &'a str)]>,
        workers: usize,
        /// How many times each module has started rendering.
        started: HashMap<usize, usize>,
        now: Rc<Cell<Instant>>,
        /// When each module rendering lands, and what it shows.
        landing: Vec<(Instant, usize, &'a str)>,
        /// The modules waiting for a worker, in the order asked for.
        queued: VecDeque<usize>,
        /// When each module first landed, in milliseconds.
        landed: HashMap<String, u64>,
        start: Instant,
        /// When the shell stops reading.
        horizon: Duration,
    }

    impl Simulation<'_> {
        /// Starts rendering what is queued, while there are workers free.
        fn start_queued(&mut self) {
            while self.landing.len() < self.workers
                && let Some(position) = self.queued.pop_front()
            {
                let renders = self.script[self.plan.modules[position].as_str()];
                let count = self.started.entry(position).or_default();
                let (took, text) = renders[(*count).min(renders.len() - 1)];
                *count += 1;
                self.landing
                    .push((self.now.get() + milliseconds(took), position, text));
            }
        }
    }

    impl Renderer for Simulation<'_> {
        fn render(&mut self, position: usize) {
            self.queued.push_back(position);
            self.start_queued();
        }

        fn wait(&mut self, deadline: Option<Instant>) -> (Instant, Vec<Rendered>) {
            let next = self.landing.iter().map(|&(at, ..)| at).min();
            let now = match (next, deadline) {
                (Some(next), Some(deadline)) => next.min(deadline),
                (Some(next), None) => next,
                (None, Some(deadline)) => deadline,
                (None, None) => panic!("the stream waits for a module that never lands"),
            };
            self.now.set(now.max(self.now.get()));
            assert!(
                self.now.get() - self.start <= self.horizon + Duration::from_secs(60),
                "the stream ran on for a minute after its shell stopped reading, \
                 without writing anything that would have told it so"
            );
            let (landed, landing): (Vec<_>, _) = self
                .landing
                .drain(..)
                .partition(|&(at, ..)| at <= self.now.get());
            self.landing = landing;
            self.start_queued();
            for &(at, position, _) in &landed {
                let milliseconds = u64::try_from((at - self.start).as_millis()).unwrap();
                self.landed
                    .entry(self.plan.modules[position].clone())
                    .or_insert(milliseconds);
            }
            let landed = landed
                .into_iter()
                .map(|(_, position, text)| Rendered {
                    position,
                    // A module that shows nothing renders no segments at all.
                    segments: if text.is_empty() {
                        Vec::new()
                    } else {
                        Segment::from_text(None, text)
                    },
                })
                .collect();
            (self.now.get(), landed)
        }
    }

    /// The frames a simulated stream writes, each with the milliseconds into
    /// the stream it was written at. A stream that keeps its prompt current
    /// is stopped as a shell would stop it, by no longer reading, once
    /// `horizon` milliseconds have passed.
    struct Recording {
        start: Instant,
        now: Rc<Cell<Instant>>,
        horizon: Duration,
        frames: Vec<(u64, [String; 3])>,
    }

    impl Sink for Recording {
        fn send(&mut self, frame: Frame) -> io::Result<()> {
            let elapsed = self.now.get() - self.start;
            if elapsed > self.horizon {
                return Err(io::ErrorKind::BrokenPipe.into());
            }
            let mut bytes = Vec::new();
            frame.write_to(&mut bytes)?;
            let milliseconds = u64::try_from(elapsed.as_millis()).unwrap();
            self.frames.extend(
                read_frames(&bytes)
                    .into_iter()
                    .map(|frame| (milliseconds, frame)),
            );
            Ok(())
        }
    }

    /// The circumstances of a simulated stream.
    struct Setting {
        /// When the shell stops reading, in milliseconds.
        horizon: u64,
        workers: usize,
    }

    impl Default for Setting {
        fn default() -> Self {
            Self {
                horizon: 60_000,
                workers: usize::MAX,
            }
        }
    }

    /// What a simulated stream wrote, each frame with the milliseconds into
    /// the stream it was written at, and when each module first landed.
    struct Simulated {
        frames: Vec<(u64, [String; 3])>,
        landed: HashMap<String, u64>,
    }

    /// Runs a stream of the prompts `context` configures, with the modules
    /// rendering as `script` says.
    fn simulate_in(setting: Setting, context: &Context, script: Script) -> Simulated {
        let plan = Plan::new(context, [Target::Main, Target::Right]);
        let start = Instant::now();
        let now = Rc::new(Cell::new(start));
        let mut simulation = Simulation {
            plan: &plan,
            script: script.iter().copied().collect(),
            workers: setting.workers,
            started: HashMap::new(),
            now: Rc::clone(&now),
            landing: Vec::new(),
            queued: VecDeque::new(),
            landed: HashMap::new(),
            start,
            horizon: milliseconds(setting.horizon),
        };
        let mut recording = Recording {
            start,
            now,
            horizon: milliseconds(setting.horizon),
            frames: Vec::new(),
        };
        recording.send(Frame::Process).unwrap();
        let session = Session::new(context, &plan, start);
        match session.run(&mut simulation, &mut recording) {
            Ok(()) => {}
            Err(error) => assert_eq!(io::ErrorKind::BrokenPipe, error.kind()),
        }
        Simulated {
            frames: recording.frames,
            landed: simulation.landed,
        }
    }

    fn simulate(context: &Context, script: Script) -> Vec<(u64, [String; 3])> {
        simulate_in(Setting::default(), context, script).frames
    }

    /// Each main prompt drawn, with the time it was drawn at.
    fn prompts(frames: &[(u64, [String; 3])]) -> Vec<(u64, &str)> {
        frames
            .iter()
            .filter(|(_, [keyword, ..])| keyword == "PROMPT")
            .map(|(at, [_, main, _])| (*at, main.as_str()))
            .collect()
    }

    fn three_modules() -> Context<'static> {
        context(toml::toml! {
            add_newline = false
            format = "${custom.a}${custom.b}$character"
        })
    }

    #[test]
    fn a_module_is_waited_for_until_the_fallback() {
        let frames = simulate(
            &three_modules(),
            &[
                ("character", &[(0, ">")]),
                ("custom.a", &[(10, "a")]),
                ("custom.b", &[(2000, "b")]),
            ],
        );

        assert_eq!(vec![(50, "a>"), (2000, "ab>")], prompts(&frames));
    }

    #[test]
    fn the_fallback_is_how_long_the_first_paint_waits_at_most() {
        let frames = simulate(
            &context(toml::toml! {
                add_newline = false
                format = "${custom.a}${custom.b}$character"
                async.bus.fallback = 30
            }),
            &[
                ("character", &[(0, ">")]),
                ("custom.a", &[(6, "a")]),
                ("custom.b", &[(2000, "b")]),
            ],
        );

        assert_eq!(vec![(30, "a>"), (2000, "ab>")], prompts(&frames));
    }

    #[test]
    fn the_later_a_change_lands_the_less_it_waits_for_others() {
        let script = |b: &'static [(u64, &'static str)]| -> Vec<(&'static str, &'static [(u64, &'static str)])> {
            vec![
                ("character", &[(0, ">")]),
                ("custom.a", &[(300, "a")]),
                ("custom.b", b),
            ]
        };

        assert_eq!(
            vec![(50, ">"), (305, "ab>")],
            prompts(&simulate(&three_modules(), &script(&[(305, "b")]))),
            "a, landing three hundred milliseconds in, waits for b five later"
        );
        assert_eq!(
            vec![(50, ">"), (307, "a>"), (320, "ab>")],
            prompts(&simulate(&three_modules(), &script(&[(320, "b")]))),
            "but not twenty"
        );
    }

    #[test]
    fn modules_that_change_nothing_do_not_hurry_the_bus() {
        let frames = simulate(
            &context(toml::toml! {
                add_newline = false
                format = "${custom.a}${custom.b}${custom.c}${custom.d}$character"
            }),
            &[
                ("character", &[(0, ">")]),
                ("custom.a", &[(20, "a")]),
                ("custom.b", &[(1, "")]),
                ("custom.c", &[(1, "")]),
                ("custom.d", &[(1, "")]),
            ],
        );

        assert_eq!(vec![(20, "a>")], prompts(&frames));
    }

    #[test]
    fn the_right_prompt_is_drawn_with_the_main_one() {
        let frames = simulate(
            &context(toml::toml! {
                add_newline = false
                format = "$character"
                right_format = "${custom.right}"
            }),
            &[
                ("character", &[(0, ">")]),
                ("custom.right", &[(500, "right")]),
            ],
        );

        let drawn: Vec<_> = frames
            .iter()
            .filter(|(_, [keyword, ..])| keyword == "PROMPT")
            .map(|(at, [_, main, right])| (*at, main.as_str(), right.as_str()))
            .collect();
        assert_eq!(vec![(50, ">", ""), (500, ">", "right")], drawn);
    }

    const LETTERS: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

    /// How a module renders in a generated stream: when it lands, and
    /// whether it shows anything.
    #[derive(Clone, Debug)]
    struct Generated {
        landing: u64,
        shows: bool,
    }

    /// A stream of up to six modules, `custom.0` to `custom.5`, each showing
    /// its letter, `a` to `f`, if it shows anything, on as many workers as
    /// `workers` says.
    fn simulate_generated(modules: &[Generated], workers: usize) -> Simulated {
        let names: Vec<String> = (0..modules.len())
            .map(|index| format!("custom.{index}"))
            .collect();
        let format: String = names.iter().map(|name| format!("${{{name}}}")).collect();
        let renders: Vec<[(u64, &str); 1]> = modules
            .iter()
            .zip(LETTERS)
            .map(|(module, letter)| [(module.landing, if module.shows { letter } else { "" })])
            .collect();
        let script: Vec<(&str, &[(u64, &str)])> = names
            .iter()
            .zip(&renders)
            .map(|(name, render)| (name.as_str(), render.as_slice()))
            .collect();
        let mut configuration = toml::Table::new();
        configuration.insert("add_newline".into(), false.into());
        configuration.insert("format".into(), format.into());
        simulate_in(
            Setting {
                workers,
                ..Setting::default()
            },
            &context(configuration),
            &script,
        )
    }

    /// The letters of the modules that show something, in order.
    fn everything(modules: &[Generated]) -> String {
        modules
            .iter()
            .zip(LETTERS)
            .filter(|(module, _)| module.shows)
            .map(|(_, letter)| letter)
            .collect()
    }

    fn generated(landing: std::ops::Range<u64>) -> impl Strategy<Value = Generated> {
        (landing, any::<bool>()).prop_map(|(landing, shows)| Generated { landing, shows })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]

        /// However few the workers, every change is drawn within the
        /// fallback of landing, and the first paint within the fallback of the
        /// start. Every prompt drawn differs from the last, and the stream ends
        /// on everything drawn and reports so once, last.
        #[test]
        fn what_the_bus_promises(
            modules in prop::collection::vec(generated(0..3000), 1..=6),
            workers in 1_usize..=6,
        ) {
            let Simulated { frames, landed } = simulate_generated(&modules, workers);
            let drawn = prompts(&frames);

            prop_assert_eq!(Some("PROCESS"), frames.first().map(|(_, [keyword, ..])| keyword.as_str()));
            prop_assert_eq!(Some("COMPLETE"), frames.last().map(|(_, [keyword, ..])| keyword.as_str()));
            prop_assert_eq!(1, frames.iter().filter(|(_, [keyword, ..])| keyword == "COMPLETE").count());
            prop_assert!(drawn[0].0 <= 50, "first drawn at {}", drawn[0].0);
            for pair in drawn.windows(2) {
                prop_assert_ne!(pair[0].1, pair[1].1);
            }
            for (index, (module, letter)) in modules.iter().zip(LETTERS).enumerate() {
                if !module.shows {
                    continue;
                }
                let landed = landed[&format!("custom.{index}")];
                let shown = drawn
                    .iter()
                    .find(|(_, prompt)| prompt.contains(letter))
                    .map(|&(at, _)| at);
                prop_assert!(
                    shown.is_some_and(|at| at <= landed + 50),
                    "{letter} landed at {landed}, shown at {shown:?}: {drawn:?}"
                );
            }
            let everything = everything(&modules);
            prop_assert_eq!(Some(everything.as_str()), drawn.last().map(|&(_, prompt)| prompt));
        }

        /// The first paint waits until every module has landed, or for the
        /// fallback, and shows what has landed by then.
        #[test]
        fn modules_are_waited_for_until_the_fallback(
            landings in prop::collection::vec(0_u64..120, 1..=6),
        ) {
            let modules: Vec<Generated> = landings
                .iter()
                .map(|&landing| Generated { landing, shows: true })
                .collect();
            let frames = simulate_generated(&modules, usize::MAX).frames;
            let (first, prompt) = prompts(&frames)[0];

            let landed: String = modules
                .iter()
                .zip(LETTERS)
                .filter(|(module, _)| module.landing <= first)
                .map(|(_, letter)| letter)
                .collect();
            prop_assert_eq!(landings.iter().max().map(|&last| last.min(50)), Some(first));
            prop_assert_eq!(landed.as_str(), prompt);
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
        run(&context, &mut output).unwrap();

        let keywords: Vec<String> = read_frames(&output)
            .into_iter()
            .map(|[keyword, ..]| keyword)
            .collect();
        assert_eq!(["PROCESS", "PROMPT", "COMPLETE"], keywords.as_slice());
    }

    /// Whatever a stream draws first, the prompts it leaves a shell showing
    /// are the ones `starship prompt` prints.
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
            let prompt_of = |target| {
                let mut context = context(configuration.clone());
                context.shell = shell;
                context.target = target;
                context.current_dir = directory.path().to_path_buf();
                context.properties.status_code = Some("1".to_owned());
                context.properties.jobs = 2;
                context
            };
            let mut output = Vec::new();
            run(&prompt_of(Target::Main), &mut output).unwrap();
            let frames = read_frames(&output);

            let [keyword, main, right] = frames
                .iter()
                .rfind(|[keyword, ..]| keyword == "PROMPT")
                .expect("a prompt is drawn");
            assert_eq!("PROMPT", keyword);
            assert_eq!(
                &print::get_prompt(&prompt_of(Target::Main)),
                main,
                "main prompt under {shell:?}"
            );
            assert_eq!(
                &print::get_prompt(&prompt_of(Target::Right)),
                right,
                "right prompt under {shell:?}"
            );
            assert_eq!(
                Some("COMPLETE"),
                frames.last().map(|[keyword, ..]| keyword.as_str())
            );
        }
    }
}
