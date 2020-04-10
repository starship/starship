use ansi_term::ANSIStrings;
use clap::ArgMatches;
use rayon::prelude::*;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};
use unicode_width::UnicodeWidthChar;

use crate::configs::PROMPT_ORDER;
use crate::context::{Context, Shell};
use crate::formatter::StringFormatter;
use crate::module::Module;
use crate::module::ALL_MODULES;
use crate::modules;
use crate::segment::Segment;

pub fn prompt(args: ArgMatches) {
    let context = Context::new(args);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", get_prompt(context)).unwrap();
}

pub fn get_prompt(context: Context) -> String {
    let config = context.config.get_root_config();
    let mut buf = String::new();

    // A workaround for a fish bug (see #739,#279). Applying it to all shells
    // breaks things (see #808,#824,#834). Should only be printed in fish.
    if let Shell::Fish = context.shell {
        buf.push_str("\x1b[J"); // An ASCII control code to clear screen
    }

    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter
    } else {
        log::error!("Error parsing `format`");
        buf.push_str(">");
        return buf;
    };
    let formatter = formatter.map_variables_to_segments(|module| {
        // Make $all display all modules
        if module == "all" {
            Some(
                PROMPT_ORDER
                    .par_iter()
                    .flat_map(|module| match *module {
                        "\n" => {
                            let mut line_break = Segment::new("line_break");
                            line_break.set_value("\n");
                            Some(vec![line_break])
                        }
                        _ => {
                            if context.is_module_disabled_in_config(&module) {
                                None
                            } else {
                                modules::handle(module, &context).map(|module| module.segments)
                            }
                        }
                    })
                    .flatten()
                    .collect::<Vec<_>>(),
            )
        } else if context.is_module_disabled_in_config(&module) {
            None
        } else {
            // Get segments from module
            modules::handle(module, &context).map(|module| module.segments)
        }
    });

    // Creates a root module and prints it.
    let mut root_module = Module::new("Starship Root", "The root module", None);
    root_module.get_prefix().set_value("");
    root_module.get_suffix().set_value("");
    root_module.set_segments(formatter.parse(None));

    let module_strings = root_module.ansi_strings_for_shell(context.shell.clone());
    write!(buf, "{}", ANSIStrings(&module_strings)).unwrap();

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

    let dont_print = vec!["character"];

    let modules = compute_modules(&context)
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

fn compute_modules<'a>(context: &'a Context) -> Vec<Module<'a>> {
    let mut prompt_order: Vec<&str> = Vec::new();

    let config = context.config.get_root_config();
    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter
    } else {
        log::error!("Error parsing `format`");
        return Vec::new();
    };
    let modules = formatter.get_variables();

    // Write out a custom prompt order
    for module in modules {
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
