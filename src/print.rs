use ansi_term::ANSIStrings;
use clap::ArgMatches;
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::fmt::{self, Debug, Write as FmtWrite};
use std::io::{self, Write};
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

use crate::configs::PROMPT_ORDER;
use crate::context::{Context, Shell};
use crate::formatter::{StringFormatter, VariableHolder};
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
        buf.push('>');
        return buf;
    };
    let modules = formatter.get_variables();
    let formatter = formatter.map_variables_to_segments(|module| {
        // Make $all display all modules
        if module == "all" {
            Some(Ok(PROMPT_ORDER
                .par_iter()
                .flat_map(|module| {
                    handle_module(module, &context, &modules)
                        .into_iter()
                        .flat_map(|module| module.segments)
                        .collect::<Vec<Segment>>()
                })
                .collect::<Vec<_>>()))
        } else if context.is_module_disabled_in_config(&module) {
            None
        } else {
            // Get segments from module
            Some(Ok(handle_module(module, &context, &modules)
                .into_iter()
                .flat_map(|module| module.segments)
                .collect::<Vec<Segment>>()))
        }
    });

    // Creates a root module and prints it.
    let mut root_module = Module::new("Starship Root", "The root module", None);
    root_module.set_segments(
        formatter
            .parse(None)
            .expect("Unexpected error returned in root format variables"),
    );

    let module_strings = root_module.ansi_strings_for_shell(context.shell);
    if config.add_newline {
        writeln!(buf).unwrap();
    }
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

pub fn timings(args: ArgMatches) {
    let context = Context::new(args);

    struct ModuleTiming {
        name: String,
        name_len: usize,
        value: String,
        duration: Duration,
        duration_len: usize,
    }

    let mut modules = compute_modules(&context)
        .iter()
        .filter(|module| !module.is_empty() || module.duration.as_millis() > 0)
        .map(|module| ModuleTiming {
            name: String::from(module.get_name().as_str()),
            name_len: better_width(module.get_name().as_str()),
            value: ansi_term::ANSIStrings(&module.ansi_strings())
                .to_string()
                .replace('\n', "\\n"),
            duration: module.duration,
            duration_len: better_width(format_duration(&module.duration).as_str()),
        })
        .collect::<Vec<ModuleTiming>>();

    modules.sort_by(|a, b| b.duration.cmp(&a.duration));

    let max_name_width = modules.iter().map(|i| i.name_len).max().unwrap_or(0);
    let max_duration_width = modules.iter().map(|i| i.duration_len).max().unwrap_or(0);

    println!("\n Here are the timings of modules in your prompt (>=1ms or output):");

    // for now we do not expect a wrap around at the end... famous last words
    // Overall a line looks like this: " {module name}  -  {duration}  -  {module value}".
    for timing in &modules {
        println!(
            " {}{}  -  {}{}  -   {}",
            timing.name,
            " ".repeat(max_name_width - (timing.name_len)),
            " ".repeat(max_duration_width - (timing.duration_len)),
            format_duration(&timing.duration),
            timing.value
        );
    }
}

pub fn explain(args: ArgMatches) {
    let context = Context::new(args);

    struct ModuleInfo {
        value: String,
        value_len: usize,
        desc: String,
        duration: String,
    }

    let dont_print = vec!["line_break"];

    let modules = compute_modules(&context)
        .into_iter()
        .filter(|module| !dont_print.contains(&module.get_name().as_str()))
        // this contains empty modules which should not print
        .filter(|module| !module.is_empty())
        .map(|module| {
            let value = module.get_segments().join("");
            ModuleInfo {
                value: ansi_term::ANSIStrings(&module.ansi_strings()).to_string(),
                value_len: better_width(value.as_str())
                    + better_width(format_duration(&module.duration).as_str()),
                desc: module.get_description().to_owned(),
                duration: format_duration(&module.duration),
            }
        })
        .collect::<Vec<ModuleInfo>>();

    let max_module_width = modules.iter().map(|i| i.value_len).max().unwrap_or(0);

    // In addition to the module width itself there are also 9 padding characters in each line.
    // Overall a line looks like this: " {module value} ({xxxms})  -  {description}".
    const PADDING_WIDTH: usize = 9;

    let desc_width = term_size::dimensions()
        .map(|(w, _)| w)
        // Add padding length to module length to avoid text overflow. This line also assures desc_width >= 0.
        .map(|width| width - std::cmp::min(width, max_module_width + PADDING_WIDTH));

    println!("\n Here's a breakdown of your prompt:");
    for info in modules {
        if let Some(desc_width) = desc_width {
            // Custom Textwrapping!
            let mut current_pos = 0;
            let mut escaping = false;
            // Print info
            print!(
                " {} ({}){}  -  ",
                info.value,
                info.duration,
                " ".repeat(max_module_width - (info.value_len))
            );
            for g in info.desc.graphemes(true) {
                // Handle ANSI escape sequnces
                if g == "\x1B" {
                    escaping = true;
                }
                if escaping {
                    print!("{}", g);
                    escaping = !("a" <= g && "z" >= g || "A" <= g && "Z" >= g);
                    continue;
                }

                // Handle normal wrapping
                current_pos += grapheme_width(g);
                // Wrap when hitting max width or newline
                if g == "\n" || current_pos > desc_width {
                    // trim spaces on linebreak
                    if g == " " && desc_width > 1 {
                        continue;
                    }

                    print!("\n{}", " ".repeat(max_module_width + PADDING_WIDTH));
                    if g == "\n" {
                        current_pos = 0;
                        continue;
                    }

                    current_pos = 1;
                }
                print!("{}", g);
            }
            println!();
        } else {
            println!(
                " {}{}  -  {}",
                info.value,
                " ".repeat(max_module_width - info.value_len),
                info.desc,
            );
        };
    }
}

fn compute_modules<'a>(context: &'a Context) -> Vec<Module<'a>> {
    let mut prompt_order: Vec<Module<'a>> = Vec::new();

    let config = context.config.get_root_config();
    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter
    } else {
        log::error!("Error parsing `format`");
        return Vec::new();
    };
    let modules = formatter.get_variables();

    for module in &modules {
        // Manually add all modules if `$all` is encountered
        if module == "all" {
            for module in PROMPT_ORDER.iter() {
                let modules = handle_module(module, &context, &modules);
                prompt_order.extend(modules.into_iter());
            }
        } else {
            let modules = handle_module(module, &context, &modules);
            prompt_order.extend(modules.into_iter());
        }
    }

    prompt_order
}

fn handle_module<'a>(
    module: &str,
    context: &'a Context,
    module_list: &BTreeSet<String>,
) -> Vec<Module<'a>> {
    struct DebugCustomModules<'tmp>(&'tmp toml::value::Table);

    impl Debug for DebugCustomModules<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.0.keys()).finish()
        }
    }

    let mut modules: Vec<Option<Module>> = Vec::new();

    if ALL_MODULES.contains(&module) {
        // Write out a module if it isn't disabled
        if !context.is_module_disabled_in_config(module) {
            modules.push(modules::handle(module, &context));
        }
    } else if module == "custom" {
        // Write out all custom modules, except for those that are explicitly set
        if let Some(custom_modules) = context.config.get_custom_modules() {
            let custom_modules = custom_modules
                .iter()
                .map(|(custom_module, config)| {
                    if should_add_implicit_custom_module(custom_module, config, &module_list) {
                        modules::custom::module(custom_module, &context)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Option<Module<'a>>>>();
            modules.extend(custom_modules)
        }
    } else if let Some(module) = module.strip_prefix("custom.") {
        // Write out a custom module if it isn't disabled (and it exists...)
        match context.is_custom_module_disabled_in_config(&module) {
            Some(true) => (), // Module is disabled, we don't add it to the prompt
            Some(false) => modules.push(modules::custom::module(&module, &context)),
            None => match context.config.get_custom_modules() {
                Some(modules) => log::debug!(
                    "top level format contains custom module \"{}\", but no configuration was provided. Configuration for the following modules were provided: {:?}",
                    module,
                    DebugCustomModules(modules),
                    ),
                None => log::debug!(
                    "top level format contains custom module \"{}\", but no configuration was provided.",
                    module,
                    ),
            },
        }
    } else {
        log::debug!(
            "Expected top level format to contain value from {:?}. Instead received {}",
            ALL_MODULES,
            module,
        );
    }

    modules.into_iter().flatten().collect()
}

fn should_add_implicit_custom_module(
    custom_module: &str,
    config: &toml::Value,
    module_list: &BTreeSet<String>,
) -> bool {
    let explicit_module_name = format!("custom.{}", custom_module);
    let is_explicitly_specified = module_list.contains(&explicit_module_name);

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

fn better_width(s: &str) -> usize {
    s.graphemes(true).map(grapheme_width).sum()
}

pub fn format_duration(duration: &Duration) -> String {
    let milis = duration.as_millis();
    if milis == 0 {
        "<1ms".to_string()
    } else {
        format!("{:?}ms", &milis)
    }
}

// Assume that graphemes have width of the first character in the grapheme
fn grapheme_width(g: &str) -> usize {
    g.chars().next().and_then(|i| i.width()).unwrap_or(0)
}

#[test]
fn test_grapheme_aware_better_width() {
    // UnicodeWidthStr::width would return 8
    assert_eq!(2, better_width("👩‍👩‍👦‍👦"));
    assert_eq!(1, better_width("Ü"));
    assert_eq!(11, better_width("normal text"));
}
