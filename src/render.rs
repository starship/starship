//! Prompt module rendering.

use std::collections::BTreeSet;
use std::fmt;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::context::Context;
use crate::plan::{ModuleName, ModuleUse, Plan, PromptState};
use crate::segment::Segment;

/// Whether a resolution belongs to the first render or a later poll.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ResolutionKind {
    Initial,
}

/// One resolved module.
pub struct Resolution<'plan> {
    module: &'plan ModuleUse,
    segments: Vec<Segment>,
    elapsed: Duration,
    kind: ResolutionKind,
}

impl fmt::Debug for Resolution<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: String = self.segments.iter().map(Segment::value).collect();
        formatter
            .debug_struct("Resolution")
            .field("module", &self.module.module.as_str())
            .field("value", &value)
            .field("elapsed", &self.elapsed)
            .field("kind", &self.kind)
            .finish()
    }
}

impl<'plan> Resolution<'plan> {
    fn initial(module: &'plan ModuleUse, segments: Vec<Segment>, elapsed: Duration) -> Self {
        Self {
            module,
            segments,
            elapsed,
            kind: ResolutionKind::Initial,
        }
    }

    pub fn store_in(self, state: &mut PromptState<'plan>) {
        state
            .record(self.module, self.segments)
            .expect("a resolution always belongs to its prompt state");
    }
}

/// A module subset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selection {
    EveryModule,
}

impl Selection {
    fn admits(self, _module: &str) -> bool {
        match self {
            Self::EveryModule => true,
        }
    }
}

/// The modules of `plan` that `selection` covers, in first-paint order.
///
/// The plan already reports each of its modules exactly once, however many
/// positions name it — see [`Plan::module_uses`] — so a module is run once and
/// its value stands in all of them, which is what the formatter did when it
/// cached one value per variable name.
fn selected_modules(plan: &Plan, selection: Selection) -> impl Iterator<Item = &ModuleUse> {
    plan.module_uses()
        .iter()
        .filter(move |module_use| selection.admits(module_use.module.as_str()))
}

/// What taking from a [`Resolutions`] produced.
#[derive(Debug)]
pub enum Arrival<'plan> {
    /// A module finished.
    Resolved(Resolution<'plan>),
    /// Every module has finished; there will be no further arrivals.
    Finished,
}

/// Modules that are running, whose output can be taken as it arrives.
///
/// A caller wanting a *time-based* draw policy — collecting several results
/// that land close together rather than drawing each the instant it lands —
/// does not go through this type. [`stream`]'s event loop owns its own
/// channel and calls `recv_timeout` on it directly instead, which is what lets
/// it wait on a module arriving *or* a repaint deadline expiring *or* a
/// dynamic module falling due, all at once.
pub struct Resolutions<'plan> {
    receiver: mpsc::Receiver<Resolution<'plan>>,
}

impl<'plan> Resolutions<'plan> {
    /// Waits for the next module to finish, however long that takes.
    pub fn next_arrival(&self) -> Arrival<'plan> {
        match self.receiver.recv() {
            Ok(resolution) => Arrival::Resolved(resolution),
            Err(mpsc::RecvError) => Arrival::Finished,
        }
    }
}

/// Runs the selected modules and lets `consume` take their output as it
/// arrives.
///
/// Every selected module produces exactly one arrival, including one that
/// produced nothing: "this module resolved to nothing" is information a caller
/// repainting a prompt needs just as much as a value is.
///
/// `consume` runs on the calling thread and returns when it stops taking; the
/// modules still running are waited for before this returns, so nothing outlives
/// the call.
pub fn with_resolutions<'plan, T>(
    plan: &'plan Plan,
    context: &Context,
    selection: Selection,
    consume: impl FnOnce(&Resolutions<'plan>) -> T,
) -> T {
    let selected = selected_modules(plan, selection);
    let referenced_modules = plan.referenced_modules();
    let (sender, receiver) = mpsc::channel::<Resolution<'plan>>();

    // `in_place_scope` rather than `scope`: the body runs on the calling thread
    // instead of being migrated into the pool, which is what lets `consume` be
    // an ordinary closure borrowing whatever the caller is filling in. `scope`
    // would demand a `Send` closure and a shared, locked accumulator for no
    // benefit — the receiving side is not the work.
    rayon::in_place_scope(|scope| {
        for module_use in selected {
            let sender = sender.clone();
            scope.spawn(move |_| {
                let started = Instant::now();
                let segments = render_module(module_use, context, referenced_modules);
                // The receiver lives as long as this scope, so a send can only
                // fail once `consume` has returned — in which case there is
                // nothing left to report the result to.
                let _ = sender.send(Resolution::initial(module_use, segments, started.elapsed()));
            });
        }
        // Every remaining sender is owned by a spawned worker, so the channel
        // disconnects — and [`Arrival::Finished`] appears — exactly when the
        // last of them has finished. Dropping this one is what makes that true.
        drop(sender);

        consume(&Resolutions { receiver })
    })
}

/// Runs the selected modules, handing each one's output to `receive` as soon as
/// that module finishes.
pub fn stream<'plan>(
    plan: &'plan Plan,
    context: &Context,
    selection: Selection,
    mut receive: impl FnMut(Resolution<'plan>),
) {
    with_resolutions(plan, context, selection, |resolutions| {
        while let Arrival::Resolved(resolution) = resolutions.next_arrival() {
            receive(resolution);
        }
    });
}

/// Runs every module the plan asks for and takes their output into one render.
pub fn fill_slots<'plan>(plan: &'plan Plan, context: &Context) -> PromptState<'plan> {
    let mut state = PromptState::empty(plan);
    stream(plan, context, Selection::EveryModule, |resolution| {
        resolution.store_in(&mut state);
    });
    state
}

/// The segments one module contributes to the prompt.
fn render_module(
    module: &ModuleUse,
    context: &Context,
    referenced_modules: &BTreeSet<ModuleName>,
) -> Vec<Segment> {
    crate::print::handle_module(module.module.as_str(), context, referenced_modules)
        .into_iter()
        .flat_map(|module| module.segments)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::PromptConfiguration;
    use crate::test::default_context;

    /// The plan a context's main prompt would be built from.
    fn plan_of(context: &Context) -> Plan {
        Plan::build(&PromptConfiguration::new(
            &context.config,
            &context.root_config,
            context.destination(),
            &context.target,
        ))
    }

    #[test]
    fn a_module_named_more_than_once_fills_every_position() {
        let context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$character$character"
            [character]
            format = ">"
        });
        let plan = plan_of(&context);
        let state = fill_slots(&plan, &context);

        assert_eq!(
            ">>",
            crate::module::painted::Painted::paint(&state.render(), None).to_string()
        );
    }

    #[test]
    fn distinct_custom_modules_fill_their_own_slots() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let shell = if cfg!(windows) {
            vec![
                "powershell".to_owned(),
                "-NoProfile".to_owned(),
                "-Command".to_owned(),
                "-".to_owned(),
            ]
        } else {
            vec!["/bin/sh".to_owned()]
        };
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${custom.fast}${custom.slow}$character"
            [custom.fast]
            command = "echo fast"
            when = true
            format = "$output"
            use_stdin = true
            ignore_timeout = true
            shell = (shell.clone())
            [custom.slow]
            command = "echo slow"
            when = true
            format = "$output"
            use_stdin = true
            ignore_timeout = true
            shell = shell
            [character]
            format = ">"
        });
        context.current_dir = directory.path().to_path_buf();
        context.logical_dir = directory.path().to_path_buf();
        let plan = plan_of(&context);
        let state = fill_slots(&plan, &context);

        assert_eq!(
            "fastslow>",
            crate::module::painted::Painted::paint(&state.render(), None).to_string()
        );
    }

    #[test]
    fn streaming_and_filling_agree() {
        let context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$status$character"
            [character]
            format = "[>](green)"
        });
        let plan = plan_of(&context);

        let filled = fill_slots(&plan, &context);
        let mut streamed = PromptState::empty(&plan);
        stream(&plan, &context, Selection::EveryModule, |resolution| {
            resolution.store_in(&mut streamed);
        });

        assert_eq!(
            crate::module::painted::Painted::paint(&filled.render(), None).to_markup(),
            crate::module::painted::Painted::paint(&streamed.render(), None).to_markup(),
        );
    }
}
