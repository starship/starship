// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
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
use std::time::Instant;

/// How a module produces its output.
pub trait Render {
    /// The module's real value.
    fn full<'context>(&self, context: &'context Context<'_>) -> Option<Module<'context>>;
}

/// The renderer of a module dispatched purely by name.
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
}
