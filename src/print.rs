use clap::ArgMatches;
use rayon::prelude::*;
use std::io::{self, Write};

use crate::config::Config;
use crate::context::Context;
use crate::module::Module;
use crate::modules;

const PROMPT_ORDER: &[&str] = &[
    "username",
    "directory",
    "git_branch",
    "git_status",
    "package",
    "nodejs",
    "rust",
    "python",
    "golang",
    "cmd_duration",
    "line_break",
    "jobs",
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

    let modules = PROMPT_ORDER
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
