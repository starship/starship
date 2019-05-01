use clap::ArgMatches;
use std::io::{self, Write};

use crate::context::Context;
use crate::module::Module;
use crate::modules;

pub fn prompt(args: ArgMatches) {
    let prompt_order = vec![
        "directory",
        "git_branch",
        "package",
        "nodejs",
        "rust",
        "python",
        "line_break",
        "character",
    ];
    let context = Context::new(args);

    // TODO:
    // - List files in directory
    // - Index binaries in PATH

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Write a new line before the prompt
    writeln!(handle).unwrap();

    let modules = prompt_order
        .iter()
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
