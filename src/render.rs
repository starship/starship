//! Prompt module rendering.

use std::collections::BTreeSet;
use std::fmt;
use std::sync::mpsc;
use std::thread;
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
        let stringified_value: String = self.segments.iter().map(Segment::value).collect();
        formatter
            .debug_struct("Resolution")
            .field("module", &self.module.module.as_str())
            .field("value", &stringified_value)
            .field("elapsed", &self.elapsed)
            .field("kind", &self.kind)
            .finish()
    }
}

impl<'plan> Resolution<'plan> {
    #[inline]
    fn initial(module: &'plan ModuleUse, segments: Vec<Segment>, elapsed: Duration) -> Self {
        Self {
            module,
            segments,
            elapsed,
            kind: ResolutionKind::Initial,
        }
    }

    #[inline]
    pub fn module(&self) -> &'plan ModuleName {
        &self.module.module
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    #[inline]
    pub(crate) fn kind(&self) -> ResolutionKind {
        self.kind
    }

    #[inline]
    pub(crate) fn slot(&self) -> ModuleSlot {
        self.module.slot()
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    #[inline]
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
    InstantOnly,
    DeferredOnly,
}

impl Selection {
    fn admits(self, module_name: &str) -> bool {
        let is_instant = cadence(module_name) == Some(Cadence::Instant);
        matches!(
            (self, is_instant),
            (Self::EveryModule, _) | (Self::InstantOnly, true) | (Self::DeferredOnly, false)
        )
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
    #[inline]
    pub fn name(&self) -> &'plan ModuleName {
        &self.module.module
    }

    #[inline]
    pub fn period(&self) -> Duration {
        self.period
    }

    #[inline]
    pub(crate) fn slot(&self) -> ModuleSlot {
        self.module.slot()
    }

    #[must_use]
    pub fn every(mut self, period: Duration) -> Self {
        self.period = period;
        self
    }

    pub fn resolve(&self, context: &Context) -> Resolution<'plan> {
        let start_time = Instant::now();
        let segments = render_module(self.module, context, self.referenced_modules);

        Resolution {
            module: self.module,
            segments,
            elapsed: start_time.elapsed(),
            kind: ResolutionKind::Refresh,
        }
    }
}

/// The plan's dynamic modules, in prompt order, skipping any that are disabled.
pub fn dynamic_modules<'plan>(plan: &'plan Plan, context: &Context) -> Vec<DynamicModule<'plan>> {
    let referenced_modules = plan.referenced_modules();

    selected_modules(plan, Selection::DeferredOnly)
        .filter(|module_use| !is_switched_off(&module_use.module, context))
        .filter_map(|module_use| {
            let Some(Cadence::Dynamic { period }) = cadence(module_use.module.as_str()) else {
                return None;
            };
            Some(DynamicModule {
                module: module_use,
                referenced_modules,
                period,
            })
        })
        .collect()
}

// A disabled dynamic module must not be polled forever.
fn is_switched_off(module_name: &ModuleName, context: &Context) -> bool {
    let string_name = module_name.as_str();
    context.is_module_disabled_in_config(string_name)
}

/// The modules of `plan` that `selection` covers, in first-paint order.
fn selected_modules(plan: &Plan, selection: Selection) -> impl Iterator<Item = &ModuleUse> {
    plan.module_uses()
        .iter()
        .filter(move |module_use| selection.admits(module_use.module.as_str()))
}

/// What taking from a [`Resolutions`] produced.
#[derive(Debug)]
pub enum Arrival<'plan> {
    Resolved(Resolution<'plan>),
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
    pub fn next_arrival(&self) -> Arrival<'plan> {
        self.receiver
            .recv()
            .map_or(Arrival::Finished, Arrival::Resolved)
    }
}

/// Hands work to the running scope.
///
/// The plan and the threads need separate lifetimes here, where the rayon scope
/// this replaced needed only one. `thread::Scope` is invariant in its own
/// lifetime, so tying the two together would demand that the plan live exactly
/// as long as the scope rather than merely outlive it, and no caller can
/// promise that. `'plan: 'scope` says the true thing instead.
pub struct Spawner<'scope, 'env, 'plan, 'context> {
    scope: &'scope thread::Scope<'scope, 'env>,
    sender: mpsc::Sender<Resolution<'plan>>,
    context: &'plan Context<'context>,
}

impl<'scope, 'plan: 'scope, 'context> Spawner<'scope, '_, 'plan, 'context> {
    pub fn poll(&self, module: DynamicModule<'plan>) {
        let sender = self.sender.clone();
        let context = self.context;
        // Taken before the module moves into the thread; it borrows the plan,
        // not the module, so it outlives both.
        let name = module.name().as_str();

        spawn_named(self.scope, name, move || {
            let _ = sender.send(module.resolve(context));
        });
    }
}

/// What runs while the modules do.
///
/// A trait rather than a closure parameter because the method has to be generic
/// over the scope's lifetime. `thread::Scope` is invariant in that lifetime, so
/// a closure argument would have to name one exact scope, and the only scope a
/// caller outside [`while_running`] can name is `'static` — which would demand
/// a `'static` plan. A trait method can be higher-ranked where a closure
/// argument cannot, and it stays a direct call: nothing boxed, no vtable, and
/// no allocation per module.
pub trait WhileRunning<'plan, 'context> {
    type Output;

    fn run<'scope>(self, spawner: &Spawner<'scope, '_, 'plan, 'context>) -> Self::Output
    where
        'plan: 'scope;
}

/// Work that runs on the calling thread if it is dropped without being taken.
///
/// `spawn_scoped` takes the closure by value and drops it when it cannot make a
/// thread, so there is no way to ask for it back. Expressing the fallback as a
/// destructor sidesteps that: the successful path takes the work out and the
/// `None` left behind makes dropping a no-op, while the failing path drops the
/// closure still holding it and the work runs right here. Nothing is allocated
/// either way, and no caller can forget to handle the failure.
struct RunUnlessTaken<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> RunUnlessTaken<F> {
    fn take_and_run(mut self) {
        if let Some(body) = self.0.take() {
            body();
        }
    }
}

impl<F: FnOnce()> Drop for RunUnlessTaken<F> {
    fn drop(&mut self) {
        if let Some(body) = self.0.take() {
            body();
        }
    }
}

/// Puts one module's render on its own thread.
///
/// A module spends nearly all of its time blocked — waiting on a subprocess, a
/// file, or the network — so the threads are there to overlap those waits, not
/// to use more processors. One thread per module is what that costs: a prompt
/// pays for the modules it actually defers instead of for a worker pool sized
/// against the machine, and a thread that is never needed is never made.
///
/// Naming them makes a wedged prompt legible in a debugger or a crash report,
/// which a pool of interchangeable workers cannot be.
fn spawn_named<'scope, F>(scope: &'scope thread::Scope<'scope, '_>, name: &str, body: F)
where
    F: FnOnce() + Send + 'scope,
{
    let work = RunUnlessTaken(Some(body));
    let builder = thread::Builder::new().name(format!("starship:{name}"));

    // Out of threads. Dropping `work` renders the module on this thread, which
    // costs the overlap it would have had — a slow prompt rather than one
    // missing a module.
    if let Err(error) = builder.spawn_scoped(scope, move || work.take_and_run()) {
        log::debug!("Rendering {name} on the calling thread; no thread to spare: {error}");
    }
}

/// Spawns `module_use`'s render on `scope`, sending the result through `sender`.
fn spawn_resolution<'scope, 'plan: 'scope, 'context>(
    scope: &'scope thread::Scope<'scope, '_>,
    sender: mpsc::Sender<Resolution<'plan>>,
    module_use: &'plan ModuleUse,
    context: &'plan Context<'context>,
    referenced_modules: &'plan BTreeSet<ModuleName>,
) {
    spawn_named(scope, module_use.module.as_str(), move || {
        let start_time = Instant::now();
        let segments = render_module(module_use, context, referenced_modules);
        let _ = sender.send(Resolution::initial(
            module_use,
            segments,
            start_time.elapsed(),
        ));
    });
}

/// Runs the selected modules, delivering each one's output to `arrivals` as it finishes.
pub fn while_running<'plan, 'context, B>(
    plan: &'plan Plan,
    context: &'plan Context<'context>,
    selection: Selection,
    arrivals: &mpsc::Sender<Resolution<'plan>>,
    body: B,
) -> B::Output
where
    B: WhileRunning<'plan, 'context>,
{
    let referenced_modules = plan.referenced_modules();

    thread::scope(|scope| {
        for module_use in selected_modules(plan, selection) {
            spawn_resolution(
                scope,
                arrivals.clone(),
                module_use,
                context,
                referenced_modules,
            );
        }

        body.run(&Spawner {
            scope,
            sender: arrivals.clone(),
            context,
        })
    })
}

/// Runs the selected modules and lets `consume` take their output as it arrives.
pub fn with_resolutions<'plan, T>(
    plan: &'plan Plan,
    context: &'plan Context<'_>,
    selection: Selection,
    consume: impl FnOnce(&Resolutions<'plan>) -> T,
) -> T {
    let referenced_modules = plan.referenced_modules();
    let (sender, receiver) = mpsc::channel::<Resolution<'plan>>();

    thread::scope(|scope| {
        for module_use in selected_modules(plan, selection) {
            spawn_resolution(
                scope,
                sender.clone(),
                module_use,
                context,
                referenced_modules,
            );
        }

        // Drop our sender so the receiver ends once every worker's clone is dropped too.
        drop(sender);

        consume(&Resolutions { receiver })
    })
}

/// Runs the selected modules, handing each one's output to `receive` as soon as it finishes.
pub fn stream<'plan>(
    plan: &'plan Plan,
    context: &'plan Context<'_>,
    selection: Selection,
    mut receive: impl FnMut(Resolution<'plan>),
) {
    with_resolutions(plan, context, selection, |resolutions| {
        while let Arrival::Resolved(resolution) = resolutions.next_arrival() {
            receive(resolution);
        }
    });
}

/// Runs everything the very first paint of a prompt may show.
pub fn stream_instant<'plan>(
    plan: &'plan Plan,
    context: &'plan Context<'_>,
    mut receive: impl FnMut(Resolution<'plan>),
) {
    // Approximations are Instant by contract, so they run synchronously rather
    // than paying for a thread each.
    selected_modules(plan, Selection::DeferredOnly)
        .filter_map(|module_use| {
            let start_time = Instant::now();
            let approximation = approximate_module(&module_use.module, context)?;
            Some(Resolution::initial(
                module_use,
                approximation.segments,
                start_time.elapsed(),
            ))
        })
        .for_each(&mut receive);

    stream(plan, context, Selection::InstantOnly, receive);
}

/// Runs every module the plan asks for and takes their output into one render.
pub fn fill_slots<'plan>(plan: &'plan Plan, context: &'plan Context<'_>) -> PromptState<'plan> {
    let mut state = PromptState::empty(plan);
    stream(plan, context, Selection::EveryModule, |resolution| {
        resolution.store_in(&mut state);
    });
    state
}

/// The segments one module contributes to the prompt.
fn render_module(
    module_use: &ModuleUse,
    context: &Context,
    referenced_modules: &BTreeSet<ModuleName>,
) -> Vec<Segment> {
    crate::print::handle_module(module_use.module.as_str(), context, referenced_modules)
        .into_iter()
        .flat_map(|module| module.segments)
        .collect()
}

/// The instant approximation of one module, or `None` if it has none, is
/// unrecognised, or is disabled.
fn approximate_module<'context>(
    module_name: &ModuleName,
    context: &'context Context<'_>,
) -> Option<Module<'context>> {
    let string_name = module_name.as_str();

    let is_enabled =
        ALL_MODULES.contains(&string_name) && !context.is_module_disabled_in_config(string_name);

    is_enabled
        .then(|| crate::modules::instant(string_name, context))
        .flatten()
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
