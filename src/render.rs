//! Prompt module rendering.

use std::collections::BTreeSet;
use std::fmt;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::context::Context;
use crate::module::{ALL_MODULES, Module};
use crate::modules::{Cadence, cadence};
use crate::plan::{ModuleName, ModuleSlot, ModuleUse, Plan, PromptState};
use crate::segment::Segment;

/// Whether a resolution belongs to the first render or a later poll.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ResolutionKind {
    Initial,
    Refresh,
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

    pub fn module(&self) -> &'plan ModuleName {
        &self.module.module
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub(crate) fn kind(&self) -> ResolutionKind {
        self.kind
    }

    pub(crate) fn slot(&self) -> ModuleSlot {
        self.module.slot()
    }

    pub fn store_in(self, state: &mut PromptState<'plan>) {
        state.record(self.module, self.segments);
    }
}

/// A module subset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selection {
    EveryModule,
    InstantOnly,
    DeferredOnly,
}

impl Selection {
    fn admits(self, module: &str) -> bool {
        let is_instant = cadence(module) == Some(Cadence::Instant);
        match self {
            Self::EveryModule => true,
            Self::InstantOnly => is_instant,
            Self::DeferredOnly => !is_instant,
        }
    }
}

pub fn modules(plan: &Plan, selection: Selection) -> impl Iterator<Item = &ModuleName> {
    selected_modules(plan, selection).map(|module_use| &module_use.module)
}

/// A dynamic module and its refresh period.
#[derive(Clone)]
pub struct DynamicModule<'plan> {
    module: &'plan ModuleUse,
    referenced_modules: &'plan BTreeSet<ModuleName>,
    period: Duration,
}

impl<'plan> DynamicModule<'plan> {
    pub fn name(&self) -> &'plan ModuleName {
        &self.module.module
    }

    pub fn period(&self) -> Duration {
        self.period
    }

    pub(crate) fn slot(&self) -> ModuleSlot {
        self.module.slot()
    }

    #[must_use]
    pub fn every(mut self, period: Duration) -> Self {
        self.period = period;
        self
    }

    pub fn resolve(&self, context: &Context) -> Resolution<'plan> {
        let started = Instant::now();
        let segments = render_module(self.module, context, self.referenced_modules);
        Resolution {
            module: self.module,
            segments,
            elapsed: started.elapsed(),
            kind: ResolutionKind::Refresh,
        }
    }
}

/// Returns enabled dynamic modules in prompt order.
pub fn dynamic_modules<'plan>(plan: &'plan Plan, context: &Context) -> Vec<DynamicModule<'plan>> {
    selected_modules(plan, Selection::DeferredOnly)
        .filter(|module_use| !is_switched_off(&module_use.module, context))
        .filter_map(|module_use| match cadence(module_use.module.as_str()) {
            Some(Cadence::Dynamic { period }) => Some(DynamicModule {
                module: module_use,
                referenced_modules: plan.referenced_modules(),
                period,
            }),
            _ => None,
        })
        .collect()
}

// Use effective defaults: disabled dynamic modules must not keep streams alive.
fn is_switched_off(module: &ModuleName, context: &Context) -> bool {
    use crate::config::ModuleConfig;

    let table = context.config.get_module_config(module.as_str());
    match module.as_str() {
        "battery" => crate::configs::battery::BatteryConfig::try_load(table).disabled,
        "localip" => crate::configs::localip::LocalipConfig::try_load(table).disabled,
        "memory_usage" => crate::configs::memory_usage::MemoryConfig::try_load(table).disabled,
        "time" => crate::configs::time::TimeConfig::try_load(table).disabled,
        other => context.is_module_disabled_in_config(other),
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
/// does not go through this type. [`crate::stream`]'s event loop owns its own
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

/// Somewhere to start a module rendering that is not on the calling thread.
///
/// Handed to the body of [`while_running`] so that a caller draining
/// resolutions can start *more* work without waiting for what is already
/// running to finish. That is what a dynamic module needs: its period expires
/// while the prompt's slow modules are still resolving, and re-rendering it on
/// the draining thread would stall every other refinement behind it.
pub struct Spawner<'borrow, 'scope, 'context> {
    scope: &'borrow rayon::Scope<'scope>,
    sender: mpsc::Sender<Resolution<'scope>>,
    context: &'scope Context<'context>,
}

impl<'scope, 'context> Spawner<'_, 'scope, 'context> {
    /// Renders `module` again, off this thread, delivering the result to the
    /// same stream of arrivals as everything else.
    ///
    /// Several of these run at once, which is the point: a battery service that
    /// takes half a second to answer must not hold up a clock that only wants
    /// to tick.
    pub fn poll(&self, module: DynamicModule<'scope>) {
        let sender = self.sender.clone();
        let context = self.context;
        self.scope.spawn(move |_| {
            // The receiver outlives this scope, so a failed send would mean the
            // caller had stopped draining — in which case there is nobody left
            // to tell.
            let _ = sender.send(module.resolve(context));
        });
    }
}

/// Runs the selected modules, delivering each one's output to `arrivals` as it
/// finishes, while `body` runs on the calling thread.
///
/// Unlike [`with_resolutions`] the channel is the caller's, so it stays open
/// after the last selected module has finished and the caller can keep using it
/// for work it starts itself through the [`Spawner`]. A caller that works this
/// way cannot learn that everything has finished by watching the channel
/// disconnect, and must count what it is waiting for instead.
pub fn while_running<'scope, 'context, T>(
    plan: &'scope Plan,
    context: &'scope Context<'context>,
    selection: Selection,
    arrivals: &mpsc::Sender<Resolution<'scope>>,
    body: impl FnOnce(&Spawner<'_, 'scope, 'context>) -> T,
) -> T {
    let selected = selected_modules(plan, selection);
    let referenced_modules = plan.referenced_modules();

    rayon::in_place_scope(|scope| {
        for module_use in selected {
            let sender = arrivals.clone();
            scope.spawn(move |_| {
                let started = Instant::now();
                let segments = render_module(module_use, context, referenced_modules);
                let _ = sender.send(Resolution::initial(module_use, segments, started.elapsed()));
            });
        }

        body(&Spawner {
            scope,
            sender: arrivals.clone(),
            context,
        })
    })
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

/// Runs everything the very first paint of a prompt may show, handing each
/// result to `receive` as it becomes available.
///
/// That is two things:
///
/// * the instant approximation of every module that has one
///   ([`crate::modules::Render::instant`]), run on this thread, in paint order.
///   An approximation is by definition too cheap to be worth handing to a
///   thread, and running them here keeps the pool free for the work that is
///   not;
/// * every [`Cadence::Instant`] module, run for real and in parallel.
///
/// Both are bounded, so this returns in the time the slowest *instant* module
/// takes rather than the time the slowest module takes.
pub fn stream_instant<'plan>(
    plan: &'plan Plan,
    context: &Context,
    mut receive: impl FnMut(Resolution<'plan>),
) {
    for module_use in selected_modules(plan, Selection::DeferredOnly) {
        let started = Instant::now();
        let Some(module) = approximate_module(&module_use.module, context) else {
            continue;
        };
        receive(Resolution::initial(
            module_use,
            module.segments,
            started.elapsed(),
        ));
    }

    stream(plan, context, Selection::InstantOnly, receive);
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

/// The instant approximation of one module, if it has one.
///
/// Unlike [`render_module`] this never expands `custom` or `env_var` into their
/// children: those are dispatched by prefix and have no approximation, so there
/// would be nothing to expand them for.
fn approximate_module<'context>(
    module: &ModuleName,
    context: &'context Context<'_>,
) -> Option<Module<'context>> {
    // Only a module that exists can have an approximation, and a disabled
    // module has nothing to approximate. Both are checks `handle_module`
    // applies to the full render whichever origin the slot has, so the first
    // paint and the refinement agree about which modules are in the prompt at
    // all.
    if !ALL_MODULES.contains(&module.as_str())
        || context.is_module_disabled_in_config(module.as_str())
    {
        return None;
    }

    crate::modules::instant(module.as_str(), context)
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

    /// The names of the modules a selection runs, in completion order.
    fn resolved_modules(context: &Context, selection: Selection) -> Vec<String> {
        let plan = plan_of(context);
        let mut resolved = Vec::new();
        stream(&plan, context, selection, |resolution| {
            resolved.push(resolution.module().as_str().to_owned());
        });
        resolved
    }

    #[test]
    fn every_module_of_the_plan_is_reported_exactly_once() {
        let context = default_context().set_config(toml::toml! {
            format = "$character$character$status$hostname"
        });
        let mut resolved = resolved_modules(&context, Selection::EveryModule);
        resolved.sort_unstable();

        // `character` fills two slots but is run once.
        assert_eq!(vec!["character", "hostname", "status"], resolved);
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
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${custom.fast}${custom.slow}$character"
            [custom.fast]
            command = "printf fast"
            when = true
            format = "$output"
            shell = ["/bin/sh"]
            [custom.slow]
            command = "printf slow"
            when = true
            format = "$output"
            shell = ["/bin/sh"]
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
    fn the_instant_selection_and_its_complement_partition_the_plan() {
        let context = default_context().set_config(toml::toml! {
            format = "$character$directory$status$git_branch"
        });

        let mut instant = resolved_modules(&context, Selection::InstantOnly);
        let mut deferred = resolved_modules(&context, Selection::DeferredOnly);
        instant.sort_unstable();
        deferred.sort_unstable();

        assert_eq!(vec!["character", "status"], instant);
        assert_eq!(vec!["directory", "git_branch"], deferred);
    }

    #[test]
    fn an_empty_selection_runs_nothing() {
        let context = default_context().set_config(toml::toml! {
            format = "$directory"
        });
        assert!(resolved_modules(&context, Selection::InstantOnly).is_empty());
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

    #[test]
    fn a_dynamic_module_is_found_among_the_deferred_ones() {
        let context = default_context().set_config(toml::toml! {
            format = "$time$character"
            [time]
            disabled = false
            format = "$time"
        });
        let plan = plan_of(&context);

        let found = dynamic_modules(&plan, &context);
        assert_eq!(1, found.len());
        assert_eq!("time", found[0].name().as_str());
        assert_eq!(Duration::from_secs(1), found[0].period());
    }

    #[test]
    fn a_prompt_with_nothing_dynamic_in_it_has_no_dynamic_modules() {
        let context = default_context().set_config(toml::toml! {
            format = "$character$directory$git_branch"
        });
        assert!(dynamic_modules(&plan_of(&context), &context).is_empty());
    }

    #[test]
    fn a_dynamic_module_can_be_given_a_different_period() {
        let context = default_context().set_config(toml::toml! {
            format = "$time"
            [time]
            disabled = false
        });
        let plan = plan_of(&context);
        let module = dynamic_modules(&plan, &context)
            .into_iter()
            .next()
            .expect("time is dynamic")
            .every(Duration::from_millis(250));

        assert_eq!(Duration::from_millis(250), module.period());
    }

    #[test]
    fn resolving_a_dynamic_module_produces_a_value_the_prompt_renders() {
        let context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$time"
            [time]
            disabled = false
            format = "the-time"
        });
        let plan = plan_of(&context);
        let module = dynamic_modules(&plan, &context)
            .into_iter()
            .next()
            .expect("time is dynamic");

        let resolution = module.resolve(&context);
        assert_eq!("time", resolution.module().as_str());
        assert_eq!(ResolutionKind::Refresh, resolution.kind());
        let mut state = PromptState::empty(&plan);
        resolution.store_in(&mut state);

        assert_eq!(
            "the-time",
            crate::module::painted::Painted::paint(&state.render(), None).to_string()
        );
    }
}
