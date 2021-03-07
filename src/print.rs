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
use crate::context::{Context, PromptMode, Shell};
use crate::formatter::{StringFormatter, VariableHolder};
use crate::module::Module;
use crate::module::ALL_MODULES;
use crate::modules;
use crate::segment::Segment;

pub struct Grapheme<'a>(&'a str);

impl<'a> Grapheme<'a> {
    pub fn width(&self) -> usize {
        self.0
            .chars()
            .filter_map(UnicodeWidthChar::width)
            .max()
            .unwrap_or(0)
    }
}

pub trait UnicodeWidthGraphemes {
    fn width_graphemes(&self) -> usize;
}

impl<T> UnicodeWidthGraphemes for T
where
    T: AsRef<str>,
{
    fn width_graphemes(&self) -> usize {
        self.as_ref()
            .graphemes(true)
            .map(Grapheme)
            .map(|g| g.width())
            .sum()
    }
}

#[test]
fn test_grapheme_aware_width() {
    // UnicodeWidthStr::width would return 8
    assert_eq!(2, "👩‍👩‍👦‍👦".width_graphemes());
    assert_eq!(1, "Ü".width_graphemes());
    assert_eq!(11, "normal text".width_graphemes());
}

pub fn prompt(args: ArgMatches) {
    let context = Context::new(args);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", get_prompt(context)).unwrap();
}

pub fn get_prompt(context: Context) -> String {
    match context.prompt_mode {
        PromptMode::Main => get_prompt_main(context),
        PromptMode::Secondary => get_prompt_secondary(context),
    }
}

pub fn get_prompt_main(context: Context) -> String {
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

    let left_prompt = match compute_prompt(&context, &config.format) {
        Some(left_prompt) => left_prompt,
        None => {
            log::error!("Error parsing `format`");
            return buf;
        }
    };

    let right_prompt = match compute_prompt(&context, &config.format_right) {
        Some(right_prompt) => right_prompt,
        None => {
            log::error!("Error parsing `format_right`");
            return buf;
        }
    };

    if config.add_newline {
        writeln!(buf).unwrap();
    }

    let mut left_lines = left_prompt.into_iter();
    let mut right_lines = right_prompt.into_iter();
    loop {
        let left_line = left_lines.next();
        let right_line = right_lines.next();
        if left_line.is_none() && right_line.is_none() {
            break;
        }

        let add_trailing_newline = right_line.is_some() || left_lines.len() > 0;

        match (left_line, right_line) {
            (
                Some((left_line_string, left_line_width)),
                Some((right_line_string, right_line_width)),
            ) => {
                if left_line_width + config.split_padding + right_line_width <= context.width {
                    // To end cursor in correct position, we print right, carriage return, print left.
                    let padding = context.width - (left_line_width + right_line_width);
                    write!(
                        buf,
                        "{}{pad_char:padding$}{}",
                        left_line_string,
                        right_line_string,
                        pad_char = ' ',
                        padding = padding,
                    )
                    .unwrap();
                } else {
                    write!(buf, "{}", left_line_string).unwrap();
                }
            }
            (Some((left_line_string, _)), None) => {
                write!(buf, "{}", left_line_string).unwrap();
            }
            (None, Some((right_line_string, right_line_width))) => {
                if right_line_width <= context.width {
                    // To end cursor in correct position, we print right, carriage return.
                    let padding = context.width - right_line_width;
                    write!(
                        buf,
                        "{pad_char:padding$}{}",
                        right_line_string,
                        pad_char = ' ',
                        padding = padding,
                    )
                    .unwrap();
                }
            }
            _ => (),
        }

        if add_trailing_newline {
            writeln!(buf).unwrap();
        }
    }

    // escape \n and ! characters for tcsh
    if let Shell::Tcsh = context.shell {
        buf = buf.replace('!', "\\!");
        // space is required before newline
        buf = buf.replace('\n', " \\n");
    }

    buf
}

pub fn get_prompt_secondary(context: Context) -> String {
    let config = context.config.get_root_config();
    let mut buf = String::new();

    match std::env::var_os("TERM") {
        Some(term) if term == "dumb" => {
            log::error!("Under a 'dumb' terminal (TERM=dumb).");
            buf.push_str("Starship disabled due to TERM=dumb");
            return buf;
        }
        _ => {}
    }

    if config.format_secondary.contains('\n') {
        log::error!("Secondary prompt contains linebreak.");
        buf.push_str("Secondary prompt contains linebreak");
        return buf;
    }

    let secondary_prompt = match compute_prompt(&context, &config.format_secondary) {
        Some(secondary_prompt) => secondary_prompt,
        None => {
            log::error!("Error parsing `format_secondary`");
            return buf;
        }
    };

    if let Some((secondary_line, _)) = secondary_prompt.first() {
        write!(buf, "{}", secondary_line).unwrap();
    }

    // escape \n and ! characters for tcsh
    if let Shell::Tcsh = context.shell {
        buf = buf.replace('!', "\\!");
        // space is required before newline
        buf = buf.replace('\n', " \\n");
    }

    buf
}

pub fn compute_prompt(context: &Context, format: &str) -> Option<Vec<(String, usize)>> {
    let string_formatters = match format
        .lines()
        .map(|line| StringFormatter::new(line))
        .collect::<Result<Vec<StringFormatter>, _>>()
    {
        Ok(string_formatters) => string_formatters,
        Err(_) => return None,
    };

    let result = string_formatters
        .into_iter()
        .map(|string_formatter| {
            let module = compute_format(&context, string_formatter);
            let string = format!(
                "{}",
                ANSIStrings(&module.ansi_strings_for_shell(context.shell))
            );
            let width = module.get_segments_width();

            (string, width)
        })
        .collect();

    Some(result)
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
            name_len: module.get_name().width_graphemes(),
            value: ansi_term::ANSIStrings(&module.ansi_strings())
                .to_string()
                .replace('\n', "\\n"),
            duration: module.duration,
            duration_len: format_duration(&module.duration).width_graphemes(),
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

    static DONT_PRINT: &[&str] = &["line_break"];

    let modules = compute_modules(&context)
        .into_iter()
        .filter(|module| !DONT_PRINT.contains(&module.get_name().as_str()))
        // this contains empty modules which should not print
        .filter(|module| !module.is_empty())
        .map(|module| {
            let value = module.get_segments().join("");
            ModuleInfo {
                value: ansi_term::ANSIStrings(&module.ansi_strings()).to_string(),
                value_len: value.width_graphemes()
                    + format_duration(&module.duration).width_graphemes(),
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
                current_pos += Grapheme(g).width();
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

fn compute_format<'a>(context: &'a Context, formatter: StringFormatter) -> Module<'a> {
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
    root_module
}

fn compute_modules<'a>(context: &'a Context) -> Vec<Module<'a>> {
    let mut prompt_order: Vec<Module<'a>> = Vec::new();

    let config = context.config.get_root_config();

    let left_formatter = if let Ok(left_formatter) = StringFormatter::new(config.format) {
        left_formatter
    } else {
        log::error!("Error parsing `format`");
        return Vec::new();
    };
    let right_formatter = if let Ok(right_formatter) = StringFormatter::new(config.format_right) {
        right_formatter
    } else {
        log::error!("Error parsing `right_format`");
        return Vec::new();
    };

    let modules = {
        let mut result = BTreeSet::new();
        result.extend(left_formatter.get_variables());
        result.extend(right_formatter.get_variables());
        result
    };

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

    let mut modules: Vec<Module> = Vec::new();

    if ALL_MODULES.contains(&module) {
        // Write out a module if it isn't disabled
        if !context.is_module_disabled_in_config(module) {
            modules.extend(modules::handle(module, &context));
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

pub fn format_duration(duration: &Duration) -> String {
    let milis = duration.as_millis();
    if milis == 0 {
        "<1ms".to_string()
    } else {
        format!("{:?}ms", &milis)
    }
}

#[cfg(test)]
mod tests {
    use crate::context::PromptMode;
    use crate::test::ModuleRenderer;

    #[test]
    fn test_prompt_left() {
        let actual = ModuleRenderer::new("")
            .config(toml::toml! {
                format = "HOST:user  ~\n> "
                add_newline = false
            })
            .width(40)
            .prompt();

        let expected = "HOST:user  ~\n> ";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_prompt_right() {
        // Prompt can't end on a right line, so it must always end of a trailing
        // line break if right prompt has more lines than left prompt.
        let actual = ModuleRenderer::new("")
            .config(toml::toml! {
                format = ""
                format_right = "5:17 PM\nSaturday, May 5"
                add_newline = false
                split_padding = 10
            })
            .width(40)
            .prompt();

        let expected =
            "                                 5:17 PM\n                         Saturday, May 5\n";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_prompt_both() {
        let actual = ModuleRenderer::new("")
            .config(toml::toml! {
                format = "HOST:user  ~\n> "
                format_right = "5:17 PM"
                add_newline = false
                split_padding = 10
            })
            .width(40)
            .prompt();

        let expected = "HOST:user  ~                     5:17 PM\n> ";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_prompt_both_too_narrow() {
        let actual = ModuleRenderer::new("")
            .config(toml::toml! {
                format = "HOST:user  ~\n> "
                format_right = "5:17 PM"
                add_newline = false
                split_padding = 30
            })
            .width(40)
            .prompt();

        let expected = "HOST:user  ~\n> ";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_prompt_secondary() {
        let actual = ModuleRenderer::new("")
            .config(toml::toml! {
                format_secondary = "<<"
            })
            .prompt_mode(PromptMode::Secondary)
            .prompt();

        let expected = "<<";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_prompt_secondary_multiline() {
        let actual = ModuleRenderer::new("")
            .config(toml::toml! {
                format_secondary = "<<\n<<"
            })
            .prompt_mode(PromptMode::Secondary)
            .prompt();

        let expected = "Secondary prompt contains linebreak";
        assert_eq!(expected, actual);
    }
}
