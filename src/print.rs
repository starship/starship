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

    let max_len = context
        .properties
        .get("max_length")
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(0);

    let modules = &prompt_order
        .par_iter()
        .filter(|module| !context.is_module_disabled_in_config(module))
        .map(|module| modules::handle(module, &context)) // Compute modules
        .flatten()
        .collect::<Vec<Module>>(); // Remove segments set to `None`

    let mut print_without_prefix = true;
    let printable = modules.iter();

    let mut segments_len = 0usize;

    for module in printable {
        // Skip printing the prefix of a module after the line_break
        segments_len += if print_without_prefix {
            segments_len = 0;
            module.segments_len_without_prefix()
        } else {
            module.segments_len()
        };

        if max_len > 0 && segments_len > max_len - 1 {
            write!(buf, "{}", modules::handle("line_break", &context).unwrap()).unwrap();
            segments_len = module.segments_len_without_prefix();
            print_without_prefix = true;
        }

        if print_without_prefix {
            write!(buf, "{}", module.to_string_without_prefix()).unwrap();
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
