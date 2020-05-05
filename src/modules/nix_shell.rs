use std::env;

use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::nix_shell::NixShellConfig;

// IN_NIX_SHELL should be "pure" or "impure" but lorri uses "1" for "impure"
// https://github.com/target/lorri/issues/140

/// Creates a module showing if inside a nix-shell
///
/// The module will use the `$IN_NIX_SHELL` and `$name` environment variable to
/// determine if it's inside a nix-shell and the name of it.
///
/// The following options are availables:
///     - use_name   (bool)   // print the name of the nix-shell
///     - impure_msg (string) // change the impure msg
///     - pure_msg (string)   // change the pure msg
///
/// Will display the following:
///     - name (pure)    // use_name == true in a pure nix-shell
///     - name (impure)  // use_name == true in an impure nix-shell
///     - pure           // use_name == false in a pure nix-shell
///     - impure         // use_name == false in an impure nix-shell
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("nix_shell");
    let config: NixShellConfig = NixShellConfig::try_load(module.config);

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    let shell_type = env::var("IN_NIX_SHELL").ok()?;
    let shell_type_segment: SegmentConfig = match shell_type.as_ref() {
        "1" | "impure" => config.impure_msg,
        "pure" => config.pure_msg,
        _ => {
            return None;
        }
    };

    if config.use_name {
        if let Ok(name) = env::var("name") {
            module.create_segment(
                "nix_shell",
                &shell_type_segment.with_value(&format!("{} ({})", name, shell_type_segment.value)),
            );
        } else {
            module.create_segment("nix_shell", &shell_type_segment);
        }
    } else {
        module.create_segment("nix_shell", &shell_type_segment);
    }

    Some(module)
}
