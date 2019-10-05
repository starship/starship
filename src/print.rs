use clap::ArgMatches;
use rayon::prelude::*;
use std::io::{self, Write};

use crate::context::Context;
use crate::module::Module;
use crate::module::ALL_MODULES;
use crate::modules;

pub fn prompt(args: ArgMatches) {
    let context = Context::new(args);
    let config = context.config.get_root_config();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Write a new line before the prompt
    if config.add_newline {
        writeln!(handle).unwrap();
    }

    let mut prompt_order: Vec<&str> = Vec::new();

    // Write out a custom prompt order
    for module in config.prompt_order {
        if ALL_MODULES.contains(&module) {
            prompt_order.push(module);
        } else {
            log::debug!(
                "Expected prompt_order to contain value from {:?}. Instead received {}",
                ALL_MODULES,
                module,
            );
        }
    }

    let modules = &prompt_order
        .par_iter()
        .filter(|module| context.is_module_enabled(module))
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
