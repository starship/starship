mod aws;
mod azure;
mod buf;
mod bun;
mod c;
mod cc;
mod character;
mod claude_context;
mod claude_cost;
mod claude_model;
mod cmake;
mod cmd_duration;
mod cobol;
mod conda;
mod container;
mod cpp;
mod crystal;
pub mod custom;
mod daml;
mod dart;
mod deno;
mod directory;
mod direnv;
mod docker_context;
mod dotnet;
mod elixir;
mod elm;
mod env_var;
mod erlang;
mod fennel;
mod fill;
mod fortran;
mod fossil_branch;
mod fossil_metrics;
mod gcloud;
mod git_branch;
mod git_commit;
mod git_metrics;
mod git_state;
pub mod git_status;
mod gleam;
mod golang;
mod gradle;
mod guix_shell;
mod haskell;
mod haxe;
mod helm;
mod hg_branch;
mod hg_state;
mod hostname;
mod java;
mod jj_bookmark;
mod jobs;
mod julia;
mod kotlin;
mod kubernetes;
mod line_break;
mod localip;
mod lua;
mod maven;
mod memory_usage;
mod meson;
mod mise;
mod mojo;
mod nats;
mod netns;
mod nim;
mod nix_shell;
mod nodejs;
mod ocaml;
mod odin;
mod opa;
mod openstack;
mod os;
mod package;
mod perl;
mod php;
mod pijul_channel;
mod pixi;
mod pulumi;
mod purescript;
mod python;
mod quarto;
mod raku;
mod red;
mod rlang;
mod ruby;
mod rust;
mod scala;
mod shell;
mod shlvl;
mod singularity;
mod solidity;
mod spack;
mod status;
mod sudo;
mod swift;
mod terraform;
mod time;
mod username;
mod utils;
mod vagrant;
mod vcs;
mod vcsh;
mod vlang;
mod xmake;
mod zig;

#[cfg(feature = "battery")]
mod battery;
mod typst;

pub(crate) mod registry;

#[cfg(feature = "battery")]
pub use self::battery::{BatteryInfoProvider, BatteryInfoProviderImpl};

use crate::config::ModuleConfig;
use crate::context::{Context, Detected, Shell};
use crate::module::Module;
use std::time::{Duration, Instant};

/// How a module behaves over time, and how expensive it is to produce.
///
/// The prompt is rendered incrementally: cheap modules are painted immediately
/// and expensive ones stream in afterwards. Every module therefore has to
/// declare which of these it is, so the renderer can decide what to wait for.
/// See [`cadence`] for the classification of each module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cadence {
    /// No unbounded input/output: no subprocess, no repository discovery, no
    /// directory scan. Cheap system calls such as `gethostname` are
    /// acceptable, and so is reading a single file whose path is already
    /// known *and* is guaranteed to resolve fast — a virtual filesystem such
    /// as `/proc` or `/run`. A path under the user's home directory is not
    /// guaranteed fast: it may be a slow or network mount (NFS, SMB, a
    /// stalled FUSE mount), and a single `open` or `stat` against one can
    /// block for as long as the mount is stuck, unbounded in exactly the way
    /// this variant promises not to be.
    ///
    /// These modules can be rendered on the critical path of the first paint.
    Instant,
    /// Performs unbounded input/output: spawns a subprocess, discovers a
    /// repository, or scans a directory. The cost is set by the machine and the
    /// working directory rather than by the module, and a subprocess may run
    /// all the way to `command_timeout`.
    Deferred,
    /// Intrinsically time-varying; a rendered value goes stale on its own and
    /// must be re-polled to stay correct, even when nothing else changed.
    Dynamic {
        /// How long a rendered value stays correct.
        period: Duration,
    },
}

/// The clock shown by the `time` module advances every second at its default
/// (and finest useful) granularity.
const TIME_PERIOD: Duration = Duration::from_secs(1);

#[cfg(feature = "battery")]
const BATTERY_PERIOD: Duration = Duration::from_secs(30);

/// Memory pressure is the fastest-moving of these values: it can swing across a
/// configured threshold within seconds of a build starting.
const MEMORY_USAGE_PERIOD: Duration = Duration::from_secs(5);

/// The local address changes only when the machine changes network, which is
/// rare but must not go unnoticed for the length of a shell session.
const LOCALIP_PERIOD: Duration = Duration::from_secs(30);

/// The [`Cadence`] of a module, or `None` if the module is unknown.
///
/// A module missing from the registry table has no declared cadence and the
/// renderer cannot schedule it; `all_modules_have_a_cadence` fails if a module
/// is added to `ALL_MODULES` without being classified there.
///
/// The names accepted here are exactly the names accepted by [`handle`],
/// including the `env_var.` and `custom.` prefixed forms. See
/// [`registry::entry_for`] for the classification of each built-in module and
/// why it is what it is.
#[must_use]
pub fn cadence(module: &str) -> Option<Cadence> {
    registry::cadence(module)
}

/// How a module produces its output.
///
/// Most modules produce one value: the value they exist to show, however long
/// it takes to work out. A module whose real value needs unbounded work may
/// additionally offer an *instant* one — a value that is correct as far as it
/// goes, cheap enough to appear in the very first paint, and replaced by the
/// real value as soon as that resolves.
///
/// The default [`Render::instant`] returns nothing, which is what "this module
/// contributes nothing until it has really resolved" means. Overriding it is
/// only worthwhile where a blank would be conspicuous; today that is
/// [`directory::Directory`] alone, because a first paint with no path in it is
/// not a prompt anyone would recognise.
pub trait Render {
    /// The module's real value.
    fn full<'context>(&self, context: &'context Context<'_>) -> Option<Module<'context>>;

    /// A value cheap enough to paint before anything slow has resolved, or
    /// nothing if the module has no such value.
    ///
    /// An implementation must not perform unbounded input/output — no
    /// subprocess, no repository discovery, no directory scan — since this runs
    /// on the critical path between starting the process and drawing a prompt.
    fn instant<'context>(&self, context: &'context Context<'_>) -> Option<Module<'context>> {
        let _ = context;
        None
    }
}

/// The renderer of a module that has no instant approximation of its own.
///
/// It inherits [`Render::instant`]'s default. There is deliberately no way to
/// give a module an instant approximation from here: a module that wants one
/// declares it next to the rest of its own code, the way `directory` does.
struct DispatchedByName<'name>(&'name str);

impl Render for DispatchedByName<'_> {
    fn full<'context>(&self, context: &'context Context<'_>) -> Option<Module<'context>> {
        dispatch(self.0, context)
    }
}

/// Runs `action` with the renderer of `module`.
fn with_renderer<T>(module: &str, action: impl FnOnce(&dyn Render) -> T) -> T {
    match module {
        "directory" => action(&directory::Directory),
        other => action(&DispatchedByName(other)),
    }
}

/// Renders `module` in full, recording how long it took.
pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    timed(module, context, || {
        with_renderer(module, |renderer| renderer.full(context))
    })
}

/// Renders `module`'s instant approximation, if it has one, recording how long
/// it took.
///
/// Returns `None` both for a module with no approximation and for one whose
/// approximation resolved to nothing; the two are the same thing to a caller
/// painting a prompt, which is why they are not distinguished.
pub fn instant<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    timed(module, context, || {
        with_renderer(module, |renderer| renderer.instant(context))
    })
}

/// Runs one rendering of `module` and records its duration on the result.
///
/// A module that took less than a millisecond keeps the default duration of
/// zero, and one that produced nothing in under a millisecond stays `None`
/// rather than becoming an empty module that exists only to carry a duration.
fn timed<'context>(
    module: &str,
    context: &'context Context<'_>,
    render: impl FnOnce() -> Option<Module<'context>>,
) -> Option<Module<'context>> {
    let start: Instant = Instant::now();
    let mut rendered = render();
    let elapsed = start.elapsed();

    log::trace!("Took {elapsed:?} to compute module {module:?}");
    if elapsed.as_millis() >= 1 {
        rendered
            .get_or_insert_with(|| context.new_module(module))
            .duration = elapsed;
    }

    rendered
}

/// Dispatches a module by the name a format string spells it with.
///
/// What `module` dispatches to is decided entirely by [`registry::ModuleId`]:
/// a name that does not parse as one is not a module at all, and everything
/// else — the closed set of built-ins and the two open-ended prefixed
/// families — is [`registry::render`]'s job.
fn dispatch<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    match registry::render(module, context) {
        Some(rendered) => rendered,
        None => {
            eprintln!(
                "Error: Unknown module {module}. Use starship module --list to list out all supported modules."
            );
            None
        }
    }
}

/// `module`'s description, as shown by `starship module --list` and in the
/// generated configuration schema.
#[must_use]
pub fn description(module: &str) -> &'static str {
    registry::description(module)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::module::ALL_MODULES;

    #[test]
    fn all_modules_have_description() {
        for module in ALL_MODULES {
            println!("Checking if {module:?} has a description");
            assert_ne!(description(module), "<no description>");
        }
    }

    #[test]
    fn all_modules_have_a_cadence() {
        // A module the renderer cannot classify is a module it cannot schedule,
        // so adding one to `ALL_MODULES` without a cadence must fail here rather
        // than silently fall back to some default.
        for module in ALL_MODULES {
            assert!(
                cadence(module).is_some(),
                "module {module:?} has no declared cadence; add it to `cadence`"
            );
        }
    }

    #[test]
    fn dynamically_named_modules_have_a_cadence() {
        // These two are dispatched by prefix rather than by name, so they never
        // appear in `ALL_MODULES` and the check above cannot see them.
        assert_eq!(cadence("env_var"), Some(Cadence::Instant));
        assert_eq!(cadence("env_var.SHELL"), Some(Cadence::Instant));
        assert_eq!(cadence("custom.git_hash"), Some(Cadence::Deferred));
    }

    #[test]
    fn an_unknown_module_has_no_cadence() {
        assert_eq!(cadence("not_a_module"), None);
        // The prefixed forms only classify the prefixed forms.
        assert_eq!(cadence("custom"), None);
    }

    #[test]
    fn every_dynamic_period_is_usable() {
        // A zero period would ask the renderer to re-poll without ever making
        // progress.
        for module in ALL_MODULES {
            if let Some(Cadence::Dynamic { period }) = cadence(module) {
                assert!(
                    !period.is_zero(),
                    "module {module:?} declares a zero refresh period"
                );
            }
        }
    }
}
