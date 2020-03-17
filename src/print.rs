use ansi_term::ANSIStrings;
use clap::ArgMatches;
use rayon::prelude::*;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};
use unicode_width::UnicodeWidthChar;

use crate::context::{Context, Shell};
use crate::module::Module;
use crate::module::ALL_MODULES;
use crate::modules;

struct ModuleOrder<'a> {
    prompt_order: Vec<Module<'a>>,
    character_prompt_order: Option<Vec<Module<'a>>>,
}

pub fn prompt(args: ArgMatches) {
    let context = Context::new(args);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if !context.character_only {
        write!(handle, "{}", get_prompt(context)).unwrap();
    } else if context.shell != Shell::PowerShell {
        write!(handle, "{}", get_character_prompt(context)).unwrap();
    } else {
        write!(handle, "{}", get_prompt(context)).unwrap();
    }
}

pub fn get_prompt(context: Context) -> String {
    let config = context.config.get_root_config();
    let mut buf = String::new();

    // Write a new line before the prompt
    if config.add_newline && (!config.split_prompt || context.shell == Shell::PowerShell) {
        writeln!(buf).unwrap();
    }

    // A workaround for a fish bug (see #739,#279). Applying it to all shells
    // breaks things (see #808,#824,#834). Should only be printed in fish.
    if let Shell::Fish = context.shell {
        buf.push_str("\x1b[J"); // An ASCII control code to clear screen
    }

    let modules = compute_modules(&context, config.split_prompt);

    let mut print_without_prefix = true;
    let printable = modules.prompt_order.iter();

    for module in printable {
        // Skip printing the prefix of a module after the line_break
        if print_without_prefix {
            let module_without_prefix = module.to_string_without_prefix(context.shell.clone());
            write!(buf, "{}", module_without_prefix).unwrap()
        } else {
            let module = module.ansi_strings_for_shell(context.shell.clone());
            write!(buf, "{}", ANSIStrings(&module)).unwrap();
        }

        print_without_prefix = module.get_name() == "line_break"
    }

    buf
}

fn get_character_prompt(context: Context) -> String {
    let config = context.config.get_root_config();
    let mut buf = String::new();

    // Write a new line before the prompt
    if config.add_newline {
        writeln!(buf).unwrap();
    }

    // A workaround for a fish bug (see #739,#279). Applying it to all shells
    // breaks things (see #808,#824,#834). Should only be printed in fish.
    if let Shell::Fish = context.shell {
        buf.push_str("\x1b[J"); // An ASCII control code to clear screen
    }

    let modules = compute_modules(&context, config.split_prompt);

    let mut print_without_prefix = true;
    let character_prompt_order = modules.character_prompt_order;
    let chatacter_prompt_order_unwrapped = character_prompt_order.expect(
        "Please dont run this command while `split_prompt` is disabled."
    );
    let printable = chatacter_prompt_order_unwrapped.iter();

    for module in printable {
        // Skip printing the prefix of a module after the line_break
        if print_without_prefix {
            let module_without_prefix = module.to_string_without_prefix(context.shell.clone());
            write!(buf, "{}", module_without_prefix).unwrap()
        } else {
            let module = module.ansi_strings_for_shell(context.shell.clone());
            write!(buf, "{}", ANSIStrings(&module)).unwrap();
        }

        print_without_prefix = module.get_name() == "line_break"
    }

    buf
}

pub fn module(module_name: &str, args: ArgMatches) {
    let context = Context::new(args);
    let module = get_module(module_name, context).unwrap_or_default();
    print!("{}", module);
}

pub fn get_module(module_name: &str, context: Context) -> Option<String> {
    modules::handle(module_name, &context).map(|m| m.to_string())
}

pub fn explain(args: ArgMatches) {
    let context = Context::new(args);

    struct ModuleInfo {
        value: String,
        value_len: usize,
        desc: String,
    }

    let dont_print = vec!["line_break", "character"];

    let modules = compute_modules(&context, false).prompt_order
        .into_iter()
        .filter(|module| !dont_print.contains(&module.get_name().as_str()))
        .map(|module| {
            let ansi_strings = module.ansi_strings();
            let value = module.get_segments().join("");
            ModuleInfo {
                value: ansi_term::ANSIStrings(&ansi_strings[1..ansi_strings.len() - 1]).to_string(),
                value_len: value.chars().count() + count_wide_chars(&value),
                desc: module.get_description().to_owned(),
            }
        })
        .collect::<Vec<ModuleInfo>>();

    let mut max_ansi_module_width = 0;
    let mut max_module_width = 0;

    for info in &modules {
        max_ansi_module_width = std::cmp::max(
            max_ansi_module_width,
            info.value.chars().count() + count_wide_chars(&info.value),
        );
        max_module_width = std::cmp::max(max_module_width, info.value_len);
    }

    let desc_width = term_size::dimensions()
        .map(|(w, _)| w)
        .map(|width| width - std::cmp::min(width, max_ansi_module_width));

    println!("\n Here's a breakdown of your prompt:");
    for info in modules {
        let wide_chars = count_wide_chars(&info.value);

        if let Some(desc_width) = desc_width {
            let wrapped = textwrap::fill(&info.desc, desc_width);
            let mut lines = wrapped.split('\n');
            println!(
                " {:width$}  -  {}",
                info.value,
                lines.next().unwrap(),
                width = max_ansi_module_width - wide_chars
            );

            for line in lines {
                println!("{}{}", " ".repeat(max_module_width + 6), line.trim());
            }
        } else {
            println!(
                " {:width$}  -  {}",
                info.value,
                info.desc,
                width = max_ansi_module_width - wide_chars
            );
        };
    }
}

fn compute_modules<'a>(context: &'a Context, split_prompt: bool) -> ModuleOrder<'a> {
    let mut prompt_order: Vec<&str> = Vec::new();
    let mut character_prompt_order: Vec<&str> = Vec::new();
    let mut hit_line_break = false;

    // Write out a custom prompt order
    for module in context.config.get_root_config().prompt_order {
        if ALL_MODULES.contains(&module) {
            if !split_prompt {
                prompt_order.push(module);
            } else if ((split_prompt && module == "line_break") || hit_line_break) && context.shell != Shell::PowerShell {
                if hit_line_break {
                    character_prompt_order.push(&module);
                }
                hit_line_break = true;
            } else {
                prompt_order.push(module);
            }
        } else {
            log::debug!(
                "Expected prompt_order to contain value from {:?}. Instead received {}",
                ALL_MODULES,
                module,
            );
        }
    }

    if !split_prompt {
        ModuleOrder {
            prompt_order:
                prompt_order
                    .par_iter()
                    .filter(|module| !context.is_module_disabled_in_config(module))
                    .map(|module| modules::handle(module, &context)) // Compute modules
                    .flatten() // Remove segments set to `None`
                    .collect::<Vec<Module<'a>>>(),
            
            character_prompt_order: None
        }
    } else if context.shell != Shell::PowerShell {
        ModuleOrder {
            prompt_order:
                prompt_order
                    .par_iter()
                    .filter(|module| !context.is_module_disabled_in_config(module))
                    .map(|module| modules::handle(module, &context)) // Compute modules
                    .flatten() // Remove segments set to `None`
                    .collect::<Vec<Module<'a>>>(),
            
            character_prompt_order: 
                Some(
                    character_prompt_order
                        .par_iter()
                        .filter(|module| !context.is_module_disabled_in_config(module))
                        .map(|module| modules::handle(module, &context)) // Compute modules
                        .flatten() // Remove segments set to `None`
                        .collect::<Vec<Module<'a>>>(),
                )
        }
    } else {
        ModuleOrder {
            prompt_order:
                prompt_order
                    .par_iter()
                    .filter(|module| !context.is_module_disabled_in_config(module))
                    .map(|module| modules::handle(module, &context)) // Compute modules
                    .flatten() // Remove segments set to `None`
                    .collect::<Vec<Module<'a>>>(),
            
            character_prompt_order: None
        }
    }
}

fn count_wide_chars(value: &str) -> usize {
    value.chars().filter(|c| c.width().unwrap_or(0) > 1).count()
}
