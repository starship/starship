use clap::ArgMatches;
use rayon::prelude::*;
use unicode_width::UnicodeWidthChar;
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

    let modules = compute_modules(&context);

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

pub fn explainer(args: ArgMatches) {
    let context = Context::new(args);

    let dont_print = vec!["line_break", "character"];

    let modules = compute_modules(&context)
        .into_iter()
        .filter(|module| !dont_print.contains(&module.get_name().as_str()))
        .map(|module| {
            let ansi_strings = module.ansi_strings();
            let desc = module.get_description();
            (
                ansi_term::ANSIStrings(&ansi_strings[1..ansi_strings.len() - 1]).to_string(),
                desc.to_owned(),
            )
        })
        .collect::<Vec<(String, String)>>();

    let max_module_width = modules.iter().fold(0, |acc, (m, _)| {
        std::cmp::max(acc, m.chars().count() + count_wide_chars(&m))
    });

    println!("\nHere's a breakdown of your prompt:");
    for (module, desc) in modules {
        let wide_chars = count_wide_chars(&module);
        println!(
            " {:width$}  -  {}",
            module,
            desc,
            width = max_module_width - wide_chars
        );
    }
}

fn compute_modules<'a>(context: &'a Context) -> Vec<Module<'a>> {
    let mut prompt_order: Vec<&str> = Vec::new();

    // Write out a custom prompt order
    for module in context.config.get_root_config().prompt_order {
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

    prompt_order
        .par_iter()
        .filter(|module| !context.is_module_disabled_in_config(module))
        .map(|module| modules::handle(module, &context)) // Compute modules
        .flatten() // Remove segments set to `None`
        .collect::<Vec<Module<'a>>>()
}

fn count_wide_chars(value: &str) -> usize {
    value.chars().filter(|c| c.width().unwrap_or(0) > 1).count()
}
