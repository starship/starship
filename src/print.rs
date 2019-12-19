use clap::ArgMatches;
use rayon::prelude::*;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

use crate::context::Context;
use crate::module::Module;
use crate::module::ALL_MODULES;
use crate::modules;

pub fn prompt(args: ArgMatches) {
    let context = Context::new(args);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", get_prompt(context)).unwrap();
}

pub fn get_prompt(context: Context) -> String {
    let config = context.config.get_root_config();
    let mut buf = String::new();

    // Write a new line before the prompt
    if config.add_newline {
        writeln!(buf).unwrap();
    }

    buf.push_str("\x1b[J");

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
        .filter(|module| !context.is_module_disabled_in_config(module))
        .map(|module| modules::handle(module, &context)) // Compute modules
        .flatten()
        .collect::<Vec<Module>>(); // Remove segments set to `None`

    let mut print_without_prefix = true;
    let printable = modules.iter();

    for module in printable {
        // Skip printing the prefix of a module after the line_break
        if print_without_prefix {
            let module_without_prefix = module.to_string_without_prefix();
            write!(buf, "{}", module_without_prefix).unwrap()
        } else {
            write!(buf, "{}", module).unwrap();
        }

        print_without_prefix = module.get_name() == "line_break"
    }

    buf
}

pub fn module(module_name: &str, args: ArgMatches) {
    let context = Context::new(args);

    // If the module returns `None`, print an empty string
    let module = modules::handle(module_name, &context)
        .map(|m| m.to_string())
        .unwrap_or_default();

    print!("{}", module);
}
