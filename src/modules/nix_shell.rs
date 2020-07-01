use std::env;

use super::{Context, Module, RootModuleConfig};

use crate::configs::nix_shell::NixShellConfig;
use crate::formatter::StringFormatter;

/// Creates a module showing if inside a nix-shell
///
/// The module will use the `$IN_NIX_SHELL` and `$name` environment variable to
/// determine if it's inside a nix-shell and the name of it.
///
/// The following options are availables:
///     - impure_msg (string) // change the impure msg
///     - pure_msg (string)   // change the pure msg
///
/// Will display the following:
///     - pure (name)    // $name == "name" in a pure nix-shell
///     - impure (name)  // $name == "name" in an impure nix-shell
///     - pure           // $name == "" in a pure nix-shell
///     - impure         // $name == "" in an impure nix-shell
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("nix_shell");
    let config: NixShellConfig = NixShellConfig::try_load(module.config);

    let shell_name = env::var("name").ok();
    let shell_type = env::var("IN_NIX_SHELL").ok()?;
    let shell_type_format = match shell_type.as_ref() {
        "impure" => config.impure_msg,
        "pure" => config.pure_msg,
        _ => {
            return None;
        }
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                "state" => Some(shell_type_format),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "name" => shell_name.as_ref().map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `nix_shell`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
