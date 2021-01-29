use crate::utils::parallel_map;
use ansi_term::ANSIStrings;
use clap::ArgMatches;
use futures::stream;
use std::collections::BTreeSet;
use std::fmt::{self, Debug, Write as FmtWrite};

use std::io::{self, Write};
use std::sync::Arc;
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

use async_std::task::block_on;
use futures::StreamExt;

pub fn prompt(args: ArgMatches<'static>) {
    let context = Context::new(args);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", block_on(get_prompt(context))).unwrap();
}

pub async fn get_prompt(context: Context<'static>) -> String {
    let context = Arc::new(context);
    let config = context.config.get_root_config();
    let mut buf = String::new();

    match std::env::var_os("TERM") {
        Some(term) if term == "dumb" => {
            log::error!("Under a 'dumb' terminal (TERM=dumb).");
            buf.push_str("Starship disabled due to TERM=dumb > ");
            return buf;
        }
        _ => {}
    }

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
    let modules = Arc::new(formatter.get_variables());
    let formatter = formatter
        .map_variables_to_segments(|module| {
            let context = Arc::clone(&context);
            let modules = Arc::clone(&modules);
            let module = module.to_string();
            async move {
                // Make $all display all modules
                if module == "all" {
                    let segments: Vec<Segment> = parallel_map(PROMPT_ORDER, |module| {
                        let context = Arc::clone(&context);
                        let modules = Arc::clone(&modules);
                        async move {
                            handle_module(&module, &context, &modules, true)
                                .await
                                .into_iter()
                                .flat_map(|m| m.segments.into_iter())
                                .collect::<Vec<Segment>>()
                        }
                    })
                    .flat_map(stream::iter)
                    .collect()
                    .await;
                    Some(Ok(segments))
                } else if context.is_module_disabled_in_config(&module) {
                    None
                } else {
                    // Get segments from module
                    Some(Ok(handle_module(&module, &context, &modules, true)
                        .await
                        .into_iter()
                        .flat_map(|module| module.segments)
                        .collect::<Vec<Segment>>()))
                }
            }
        })
        .await;

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

    // escape \n and ! characters for tcsh
    if let Shell::Tcsh = context.shell {
        buf = buf.replace('!', "\\!");
        // space is required before newline
        buf = buf.replace('\n', " \\n");
    }

    buf
}

pub fn module(module_name: &str, args: ArgMatches) {
    let context = Context::new(args);
    let module = get_module(module_name, context).unwrap_or_default();
    print!("{}", module);
}

pub fn get_module(module_name: &str, context: Context) -> Option<String> {
    block_on(modules::handle(module_name, &context, false)).map(|m| m.to_string())
}

pub fn timings(args: ArgMatches) {
    let context = Context::new(args);

    struct ModuleTiming {
        name: String,
        name_len: usize,
        value: String,
        duration: Duration,
        duration_len: usize,
        timeout_overrun: String,
    }

    let mut modules = block_on(compute_modules(&context))
        .iter()
        .filter(|module| !module.is_empty() || module.duration.as_millis() > 0)
        .map(|module| {
            let timeout = context
                .module_timeout(module.get_name())
                .unwrap_or(context.prompt_timeout);

            let timeout_overrun = if module.duration > timeout {
                format!(" (>{})", format_duration(&timeout))
            } else {
                "".into()
            };

            ModuleTiming {
                name: String::from(module.get_name().as_str()),
                name_len: better_width(module.get_name().as_str()),
                value: ansi_term::ANSIStrings(&module.ansi_strings())
                    .to_string()
                    .replace('\n', "\\n"),
                duration: module.duration,
                duration_len: better_width(format_duration(&module.duration).as_str()),
                timeout_overrun,
            }
        })
        .collect::<Vec<ModuleTiming>>();

    modules.sort_by(|a, b| b.duration.cmp(&a.duration));

    let max_name_width = modules.iter().map(|i| i.name_len).max().unwrap_or(0);
    let max_duration_width = modules.iter().map(|i| i.duration_len).max().unwrap_or(0);
    let max_timeout_overrun = modules
        .iter()
        .map(|i| i.timeout_overrun.len())
        .max()
        .unwrap_or(0);

    println!("\n Here are the timings of modules in your prompt (>=1ms or output):");

    // for now we do not expect a wrap around at the end... famous last words
    // Overall a line looks like this: " {module name}  -  {duration} {overrun}  -  {module value}".
    for timing in &modules {
        println!(
            " {}{}  -  {}{}{}{}  -   {}",
            timing.name,
            " ".repeat(max_name_width - (timing.name_len)),
            " ".repeat(max_duration_width - (timing.duration_len)),
            format_duration(&timing.duration),
            " ".repeat(max_timeout_overrun - (timing.timeout_overrun.len())),
            ansi_term::Color::Red.paint(&timing.timeout_overrun),
            timing.value,
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

    static DONT_PRINT: &[&str] = &["line_break"];

    let modules = block_on(compute_modules(&context))
        .into_iter()
        .filter(|module| !DONT_PRINT.contains(&module.get_name().as_str()))
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
                    escaping = !(("a"..="z").contains(&g) || ("A"..="Z").contains(&g));
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

async fn compute_modules<'a>(context: &'a Context<'a>) -> Vec<Module<'a>> {
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
                let modules = handle_module(module, &context, &modules, false).await;
                prompt_order.extend(modules.into_iter());
            }
        } else {
            let modules = handle_module(module, &context, &modules, false).await;
            prompt_order.extend(modules.into_iter());
        }
    }

    prompt_order
}

async fn handle_module<'a>(
    module: &str,
    context: &'a Context<'a>,
    module_list: &BTreeSet<String>,
    enforce_timeout: bool,
) -> Vec<Module<'a>> {
    struct DebugCustomModules<'tmp>(&'tmp toml::value::Table);

    impl Debug for DebugCustomModules<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.0.keys()).finish()
        }
    }

    let mut modules: Vec<Module> = Vec::new();

    if ALL_MODULES.contains(&module) {
        // Write out a module if it isn't disabled
        if !context.is_module_disabled_in_config(module) {
            modules.extend(modules::handle(module, &context, enforce_timeout).await);
        }
    } else if module == "custom" {
        // Write out all custom modules, except for those that are explicitly set
        if let Some(custom_modules) = context.config.get_custom_modules() {
            let custom_modules = custom_modules.iter().filter_map(|(custom_module, config)| {
                if should_add_implicit_custom_module(custom_module, config, &module_list) {
                    modules::custom::module(custom_module, &context)
                } else {
                    None
                }
            });
            modules.extend(custom_modules);
        }
    } else if let Some(module) = module.strip_prefix("custom.") {
        // Write out a custom module if it isn't disabled (and it exists...)
        match context.is_custom_module_disabled_in_config(&module) {
            Some(true) => (), // Module is disabled, we don't add it to the prompt
            Some(false) => modules.extend(modules::custom::module(&module, &context)),
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

    modules
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
