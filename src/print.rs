use ansi_term::ANSIStrings;
use clap::ArgMatches;
use rayon::prelude::*;
use std::fmt::{self, Debug, Write as FmtWrite};
use std::io::{self, Write};
use unicode_width::UnicodeWidthChar;

use crate::context::{Context, Shell};
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

    // A workaround for a fish bug (see #739,#279). Applying it to all shells
    // breaks things (see #808,#824,#834). Should only be printed in fish.
    if let Shell::Fish = context.shell {
        buf.push_str("\x1b[J"); // An ASCII control code to clear screen
    }

    let modules = compute_modules(&context);

    let mut print_without_prefix = true;
    let printable = modules.iter();

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
    enum Mod<'a> {
        Builtin(&'a str),
        Custom(&'a str),
    }

    struct DebugCustomModules<'tmp>(&'tmp toml::value::Table);

    impl Debug for DebugCustomModules<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.0.keys()).finish()
        }
    }

    let mut prompt_order: Vec<Mod> = Vec::new();

    // Write out a custom prompt order
    let config_prompt_order = context.config.get_root_config().prompt_order;

    for module in &config_prompt_order {
        if ALL_MODULES.contains(module) {
            // Write out a module if it isn't disabled
            if !context.is_module_disabled_in_config(*module) {
                prompt_order.push(Mod::Builtin(module));
            }
        } else if *module == "custom" {
            // Write out all custom modules, except for those that are explicitly set
            if let Some(custom_modules) = context.config.get_custom_modules() {
                for (custom_module, config) in custom_modules {
                    if should_add_implicit_custom_module(
                        custom_module,
                        config,
                        &config_prompt_order,
                    ) {
                        prompt_order.push(Mod::Custom(custom_module));
                    }
                }
            }
        } else if module.starts_with("custom.") {
            // Write out a custom module if it isn't disabled (and it exists...)
            match context.is_custom_module_disabled_in_config(&module[7..]) {
                Some(true) => (), // Module is disabled, we don't add it to the prompt
                Some(false) => prompt_order.push(Mod::Custom(&module[7..])),
                None => match context.config.get_custom_modules() {
                    Some(modules) => log::debug!(
                        "prompt_order contains custom module \"{}\", but no configuration was provided. Configuration for the following modules were provided: {:?}",
                        module,
                        DebugCustomModules(modules),
                    ),
                    None => log::debug!(
                        "prompt_order contains custom module \"{}\", but no configuration was provided.",
                        module,
                    ),
                },
            }
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
        .map(|module| match module {
            Mod::Builtin(builtin) => modules::handle(builtin, context),
            Mod::Custom(custom) => modules::custom::module(custom, context),
        }) // Compute segments
        .flatten() // Remove segments set to `None`
        .collect::<Vec<Module<'a>>>()
}

fn should_add_implicit_custom_module(
    custom_module: &str,
    config: &toml::Value,
    config_prompt_order: &[&str],
) -> bool {
    let is_explicitly_specified = config_prompt_order.iter().any(|x| {
        x.len() == 7 + custom_module.len() && &x[..7] == "custom." && &x[7..] == custom_module
    });

    if is_explicitly_specified {
        // The module is already specified explicitly, so we skip it
        return false;
    }

    let false_value = toml::Value::Boolean(false);

    !config
        .get("disabled")
        .unwrap_or(&false_value)
        .as_bool()
        .unwrap_or(false)
}

fn count_wide_chars(value: &str) -> usize {
    value.chars().filter(|c| c.width().unwrap_or(0) > 1).count()
}
