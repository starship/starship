use std::env;

use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};

use crate::configs::nix_shell::NixShellConfig;
use crate::segment::Segment;

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

    let shell_type = env::var("IN_NIX_SHELL").ok()?;

    match shell_type.as_ref() {
        "impure" | "1" | "pure" => {}
        _ => return None,
    };

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "name" => {
                let name = env::var("name").ok()?;
                Some(Segment {
                    _name: "name".to_string(),
                    value: name.to_string(),
                    style,
                })
            }
            "impure_msg" => match shell_type.as_ref() {
                "1" | "impure" => Some(Segment {
                    _name: "impure_msg".to_string(),
                    value: config.impure_msg.to_string(),
                    style,
                }),
                _ => None,
            },
            "pure_msg" => match shell_type.as_ref() {
                "pure" => Some(Segment {
                    _name: "pure_msg".to_string(),
                    value: config.pure_msg.to_string(),
                    style,
                }),
                _ => None,
            },
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}
