use ansi_term::Color;
use std::env;

use super::{Context, Module};

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
    let mut module = context.new_module("nix_shell")?;

    env::var("IN_NIX_SHELL")
        .ok()
        .and_then(|shell_type| {
            if shell_type == "1" || shell_type == "impure" {
                Some(module.config_value_str("impure_msg").unwrap_or("impure"))
            } else if shell_type == "pure" {
                Some(module.config_value_str("pure_msg").unwrap_or("pure"))
            } else {
                None
            }
        })
        .map(|shell_type| {
            if module.config_value_bool("use_name").unwrap_or(false) {
                match env::var("name").ok() {
                    Some(name) => format!("{} ({})", name, shell_type),
                    None => shell_type.to_string(),
                }
            } else {
                shell_type.to_string()
            }
        })
        .map(|segment| {
            let module_style = module
                .config_value_style("style")
                .unwrap_or_else(|| Color::Red.bold());
            module.set_style(module_style);
            module.new_segment("nix_shell", &segment);
            module
        })
}
