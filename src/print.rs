use clap::ArgMatches;
use rayon::prelude::*;
use std::io::{self, Write};

use crate::config::Config;
use crate::context::Context;
use crate::module::Module;
use crate::module::ALL_MODULES;
use crate::modules;

// List of default prompt order
// NOTE: If this const value is changed then Default prompt order subheading inside
// prompt heading of config docs needs to be updated according to changes made here.
const DEFAULT_PROMPT_ORDER: &[&str] = &[
    "username",
    "directory",
    "git_branch",
    "git_status",
    "package",
    "nodejs",
    "ruby",
    "rust",
    "python",
    "golang",
    "nix_shell",
    "cmd_duration",
    "line_break",
    "jobs",
    #[cfg(feature = "battery")]
    "battery",
    "character",
];

pub fn prompt(args: ArgMatches) {
    let context = Context::new(args);
    let config = &context.config;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Write a new line before the prompt
    if config.get_as_bool("add_newline") != Some(false) {
        writeln!(handle).unwrap();
    }

    let mut prompt_order: Vec<&str> = Vec::new();

    // Write out a custom prompt order
    if let Some(modules) = config.get_as_array("prompt_order") {
        // if prompt_order = [] use default_prompt_order
        if !modules.is_empty() {
            for module in modules {
                let str_value = module.as_str();

                if let Some(value) = str_value {
                    if ALL_MODULES.contains(&value) {
                        prompt_order.push(value);
                    } else {
                        log::debug!(
                            "Expected prompt_order to contain value from {:?}. Instead received {}",
                            ALL_MODULES,
                            value,
                        );
                    }
                } else {
                    log::debug!(
                        "Expected prompt_order to be an array of strings. Instead received {} of type {}",
                        module,
                        module.type_str()
                    );
                }
            }
        } else {
            prompt_order = DEFAULT_PROMPT_ORDER.to_vec();
        }
    } else {
        prompt_order = DEFAULT_PROMPT_ORDER.to_vec();
    }

    let modules = &prompt_order
        .par_iter()
        .map(|module| modules::handle(module, &context)) // Compute modules
        .flatten()
        .collect::<Vec<Module>>(); // Remove segments set to `None`

    let mut printable = modules.iter();

    // Print the first module without its prefix
    if let Some(first_module) = printable.next() {
        let module_without_prefix = first_module.to_string_without_prefix();
        write!(handle, "{}", module_without_prefix).unwrap()
    }

    // Print all remaining modules
    printable.for_each(|module| write!(handle, "{}", module).unwrap());
}

pub fn module(module_name: &str, args: ArgMatches) {
    let context = Context::new(args);

    // If the module returns `None`, print an empty string
    let module = modules::handle(module_name, &context)
        .map(|m| m.to_string())
        .unwrap_or_default();

    print!("{}", module);
}
