use clap::{ValueEnum, builder::PossibleValue};
use nu_ansi_term::AnsiStrings;
use rayon::prelude::*;
use regex::Regex;
use std::collections::BTreeSet;
use std::fmt::{Debug, Write as FmtWrite};
use std::io::{self, Write};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;
use terminal_size::terminal_size;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::configs::PROMPT_ORDER;
use crate::context::{Context, Properties, Shell, Target};
use crate::formatter::{StringFormatter, VariableHolder};
use crate::module::ALL_MODULES;
use crate::module::Module;
use crate::modules;
use crate::segment::Segment;
use crate::shadow;
use crate::utils::wrap_colorseq_for_shell;

pub trait UnicodeWidthGraphemes {
    fn width_graphemes(&self) -> usize;
}

static ANSI_REGEX: OnceLock<Regex> = OnceLock::new();

fn ansi_strip() -> &'static Regex {
    ANSI_REGEX.get_or_init(|| Regex::new(r"\x1B\[[0-9;]*m").unwrap())
}

impl<T> UnicodeWidthGraphemes for T
where
    T: AsRef<str>,
{
    fn width_graphemes(&self) -> usize {
        ansi_strip()
            .replace_all(self.as_ref(), "")
            .into_owned()
            .graphemes(true)
            .map(|g| g.width())
            .sum()
    }
}

#[test]
fn test_grapheme_aware_width() {
    assert_eq!(2, "👩‍👩‍👦‍👦".width_graphemes());
    assert_eq!(1, "Ü".width_graphemes());
    assert_eq!(11, "normal text".width_graphemes());
    // Magenta string test
    assert_eq!(11, "\x1B[35;6mnormal text".width_graphemes());
    // Variation selector 16 test (emoji presentation for symbols)
    assert_eq!(1, "❄".width_graphemes());
    assert_eq!(2, "❄️".width_graphemes());
}

pub fn prompt(args: Properties, target: Target) {
    let context = Context::new(args, target);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", get_prompt(&context)).unwrap();
}

const CLEAR_FROM_CURSOR_TO_END: &str = "\x1b[J";

pub fn get_prompt(context: &Context) -> String {
    let config = &context.root_config;
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
    let has_fish_clear_workaround = Shell::Fish == context.shell && context.target == Target::Main;
    if has_fish_clear_workaround {
        buf.push_str(CLEAR_FROM_CURSOR_TO_END);
    }

    let (formatter, modules) = load_formatter_and_modules(context);

    let formatter = formatter.map_variables_to_segments(|module| {
        // Make $all display all modules not explicitly referenced
        if module == "all" {
            Some(Ok(all_modules_uniq(&modules)
                .par_iter()
                .flat_map(|module| {
                    handle_module(module, context, &modules)
                        .into_iter()
                        .flat_map(|module| module.segments)
                        .collect::<Vec<Segment>>()
                })
                .collect::<Vec<_>>()))
        } else if context.is_module_disabled_in_config(module) {
            None
        } else {
            // Get segments from module
            Some(Ok(handle_module(module, context, &modules)
                .into_iter()
                .flat_map(|module| module.segments)
                .collect::<Vec<Segment>>()))
        }
    });

    // Creates a root module and prints it.
    let mut root_module = Module::new("Starship Root", "The root module", None);
    root_module.set_segments(
        formatter
            .parse(None, Some(context))
            .expect("Unexpected error returned in root format variables"),
    );

    let module_strings = root_module.ansi_strings_for_width(Some(context.width));
    if config.add_newline && context.target != Target::Continuation {
        // continuation prompts normally do not include newlines, but they can
        writeln!(buf).unwrap();
    }
    // AnsiStrings strips redundant ANSI color sequences, so apply it before modifying the ANSI
    // color sequences for this specific shell
    let shell_wrapped_output =
        wrap_colorseq_for_shell(AnsiStrings(&module_strings).to_string(), context.shell);
    write!(buf, "{shell_wrapped_output}").unwrap();

    if context.target == Target::Right {
        // right prompts generally do not allow newlines
        buf = buf.replace('\n', "");
    }

    // escape \n and ! characters for tcsh
    if context.shell == Shell::Tcsh {
        buf = buf.replace('!', "\\!");
        // space is required before newline
        buf = buf.replace('\n', " \\n");
    }

    if context.shell == Shell::Fish && context.width > 0 && context.target == Target::Main {
        let mut new_buf = String::with_capacity(buf.len());
        let mut len = 0;

        let mut push_wrapped_text = |range: Range<usize>, new_buf: &mut String| {
            for g in buf[range].graphemes(true) {
                let mut newline = g == "\n" || g == "\r\n";
                let width = if !newline { g.width() } else { 0 };

                if width > 0 && len > 0 && len + width > context.width {
                    new_buf.push('\n');
                    newline = true;
                }

                new_buf.push_str(g);
                len = if !newline { len + width } else { width };
            }
        };

        let prefix_range = if has_fish_clear_workaround {
            0..CLEAR_FROM_CURSOR_TO_END.len()
        } else {
            0..0
        };

        let ansi_ranges = std::iter::once(prefix_range.clone()).chain(
            ansi_strip()
                .find_iter(&buf)
                .map(|m| m.range())
                .filter(|r| r.start >= prefix_range.end),
        );

        let mut previous_end = 0;
        for current in ansi_ranges {
            if previous_end < current.start {
                push_wrapped_text(
                    previous_end.max(prefix_range.end)..current.start,
                    &mut new_buf,
                );
            }
            previous_end = current.end;
            new_buf.push_str(&buf[current]);
        }

        if previous_end < buf.len() {
            push_wrapped_text(previous_end..buf.len(), &mut new_buf);
        }

        buf = new_buf;
    }

    buf
}

pub fn module(module_name: &str, args: Properties) {
    let context = Context::new(args, Target::Main);
    let module = get_module(module_name, &context).unwrap_or_default();
    print!("{module}");
}

pub fn get_module(module_name: &str, context: &Context) -> Option<String> {
    modules::handle(module_name, context).map(|m| m.to_string())
}

pub fn timings(args: Properties) {
    let context = Context::new(args, Target::Main);

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
            value: nu_ansi_term::AnsiStrings(&module.ansi_strings())
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
    // Overall a line looks like this: " {module name}  -  {duration}  -  "{module value}"".
    for timing in &modules {
        println!(
            " {}{}  -  {}{}  -   \"{}\"",
            timing.name,
            " ".repeat(max_name_width - (timing.name_len)),
            " ".repeat(max_duration_width - (timing.duration_len)),
            format_duration(&timing.duration),
            timing.value
        );
    }
}

pub fn explain(args: Properties) {
    let context = Context::new(args, Target::Main);

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
                value: nu_ansi_term::AnsiStrings(&module.ansi_strings()).to_string(),
                value_len: value.width_graphemes()
                    + format_duration(&module.duration).width_graphemes(),
                desc: module.get_description().clone(),
                duration: format_duration(&module.duration),
            }
        })
        .collect::<Vec<ModuleInfo>>();

    let max_module_width = modules.iter().map(|i| i.value_len).max().unwrap_or(0);

    // In addition to the module width itself there are also 11 padding characters in each line.
    // Overall a line looks like this: " "{module value}" ({xxxms})  -  {description}".
    const PADDING_WIDTH: usize = 11;

    let desc_width = terminal_size()
        .map(|(w, _)| w.0 as usize)
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
                " \"{}\" ({}){}  -  ",
                info.value,
                info.duration,
                " ".repeat(max_module_width - (info.value_len))
            );
            for g in info.desc.graphemes(true) {
                // Handle ANSI escape sequences
                if g == "\x1B" {
                    escaping = true;
                }
                if escaping {
                    print!("{g}");
                    escaping = !(("a"..="z").contains(&g) || ("A"..="Z").contains(&g));
                    continue;
                }

                // Handle normal wrapping
                current_pos += g.width();
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
                print!("{g}");
            }
            println!();
        } else {
            println!(
                " {}{}  -  {}",
                info.value,
                " ".repeat(max_module_width - info.value_len),
                info.desc,
            );
        }
    }
}

fn compute_modules<'a>(context: &'a Context) -> Vec<Module<'a>> {
    let mut prompt_order: Vec<Module<'a>> = Vec::new();

    let (_formatter, modules) = load_formatter_and_modules(context);

    for module in &modules {
        // Manually add all modules if `$all` is encountered
        if module == "all" {
            for module in all_modules_uniq(&modules) {
                let modules = handle_module(&module, context, &modules);
                prompt_order.extend(modules);
            }
        } else {
            let modules = handle_module(module, context, &modules);
            prompt_order.extend(modules);
        }
    }

    prompt_order
}

fn handle_module<'a>(
    module: &str,
    context: &'a Context,
    module_list: &BTreeSet<String>,
) -> Vec<Module<'a>> {
    let mut modules: Vec<Module> = Vec::new();

    if ALL_MODULES.contains(&module) {
        // Write out a module if it isn't disabled
        if !context.is_module_disabled_in_config(module) {
            modules.extend(modules::handle(module, context));
        }
    } else if module.starts_with("custom.") || module.starts_with("env_var.") {
        // custom.<name> and env_var.<name> are special cases and handle disabled modules themselves
        modules.extend(modules::handle(module, context));
    } else if matches!(module, "custom" | "env_var") {
        // env var is a spacial case and may contain a top-level module definition
        if module == "env_var" {
            modules.extend(modules::handle(module, context));
        }

        // Write out all custom modules, except for those that are explicitly set
        modules.extend(
            context
                .config
                .get_config(&[module])
                .and_then(|config| config.as_table().map(toml::map::Map::iter))
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .par_iter()
                .filter_map(|(child, config)| {
                    // Some env var keys may be part of a top-level module definition
                    if module == "env_var" && !config.is_table() {
                        None
                    } else if should_add_implicit_module(module, child, config, module_list) {
                        Some(modules::handle(&format!("{module}.{child}"), context))
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<Module>>(),
        );
    } else {
        log::debug!(
            "Expected top level format to contain value from {ALL_MODULES:?}. Instead received {module}",
        );
    }

    modules
}

fn should_add_implicit_module(
    parent_module: &str,
    child_module: &str,
    config: &toml::Value,
    module_list: &BTreeSet<String>,
) -> bool {
    let explicit_module_name = format!("{parent_module}.{child_module}");
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

/// Return the modules from $all that are not already in the list
fn all_modules_uniq(module_list: &BTreeSet<String>) -> Vec<String> {
    let mut prompt_order: Vec<String> = Vec::new();
    for module in PROMPT_ORDER {
        if !module_list.contains(*module) {
            prompt_order.push(String::from(*module));
        }
    }

    prompt_order
}

/// Load the correct formatter for the context (ie left prompt or right prompt)
/// and the list of all modules used in a format string
fn load_formatter_and_modules<'a>(context: &'a Context) -> (StringFormatter<'a>, BTreeSet<String>) {
    let config = &context.root_config;

    if context.target == Target::Continuation {
        let cf = &config.continuation_prompt;
        let formatter = StringFormatter::new(cf);
        return match formatter {
            Ok(f) => {
                let modules = f.get_variables().into_iter().collect();
                (f, modules)
            }
            Err(e) => {
                log::error!("Error parsing continuation prompt: {e}");
                (StringFormatter::raw(">"), BTreeSet::new())
            }
        };
    }

    let (left_format_str, right_format_str): (&str, &str) = match context.target {
        Target::Main | Target::Right => (&config.format, &config.right_format),
        Target::Profile(ref name) => {
            if let Some(lf) = config.profiles.get(name) {
                (lf, "")
            } else {
                log::error!("Profile {name:?} not found");
                return (StringFormatter::raw(">"), BTreeSet::new());
            }
        }
        Target::Continuation => unreachable!("Continuation prompt should have been handled above"),
    };

    let lf = StringFormatter::new(left_format_str);
    let rf = StringFormatter::new(right_format_str);

    if let Err(ref e) = lf {
        let name = if let Target::Profile(ref profile_name) = context.target {
            format!("profile.{profile_name}")
        } else {
            "format".to_string()
        };
        log::error!("Error parsing {name:?}: {e}");
    }

    if let Err(ref e) = rf {
        log::error!("Error parsing right_format: {e}");
    }

    let modules = [&lf, &rf]
        .into_iter()
        .flatten()
        .flat_map(VariableHolder::get_variables)
        .collect();

    let main_formatter = match context.target {
        Target::Main | Target::Profile(_) => lf,
        Target::Right => rf,
        Target::Continuation => unreachable!("Continuation prompt should have been handled above"),
    };

    match main_formatter {
        Ok(f) => (f, modules),
        _ => (StringFormatter::raw(">"), BTreeSet::new()),
    }
}

#[cfg(feature = "config-schema")]
pub fn print_schema() {
    let schema = schemars::schema_for!(crate::configs::FullConfig);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

#[derive(Clone, Debug)]
pub struct Preset(pub &'static str);

impl ValueEnum for Preset {
    fn value_variants<'a>() -> &'a [Self] {
        shadow::get_preset_list()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.0))
    }
}

pub fn preset_command(name: Option<Preset>, output: Option<PathBuf>, list: bool) {
    if list {
        println!("{}", preset_list());
        return;
    }
    let variant = name.expect("name argument must be specified");
    let content = shadow::get_preset_content(variant.0);
    if let Some(output) = output {
        if let Err(err) = std::fs::write(output, content) {
            eprintln!("Error writing preset to file: {err}");
            std::process::exit(1);
        }
    } else if let Err(err) = std::io::stdout().write_all(content) {
        eprintln!("Error writing preset to stdout: {err}");
        std::process::exit(1);
    }
}

fn preset_list() -> String {
    Preset::value_variants()
        .iter()
        .fold(String::new(), |mut output, b| {
            let _ = writeln!(output, "{}", b.0);
            output
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::default_context;
    use crate::utils;

    const NULL_DEVICE: &str = if cfg!(windows) { "NUL" } else { "/dev/null" };

    #[test]
    fn main_prompt() {
        let mut context = default_context().set_config(toml::toml! {
                add_newline=false
                format="$character"
                [character]
                format=">\n>"
        });
        context.target = Target::Main;

        let expected = String::from(">\n>");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn right_prompt() {
        let mut context = default_context().set_config(toml::toml! {
                right_format="$character"
                [character]
                format=">\n>"
        });
        context.target = Target::Right;

        let expected = String::from(">>"); // should strip new lines
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn prompt_with_all() -> io::Result<()> {
        let mut context = default_context().set_config(toml::toml! {
                add_newline = false
                right_format= "$directory$line_break"
                format="$all"
                [character]
                format=">"
        });
        context.env.insert("HOME", NULL_DEVICE.to_string());
        let dir = tempfile::tempdir().unwrap();
        context.current_dir = dir.path().to_path_buf();

        let expected = String::from(">");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn rprompt_with_all() -> io::Result<()> {
        let mut context = default_context().set_config(toml::toml! {
            format= "$directory$line_break"
            right_format="$all"
            [character]
            format=">"
        });
        context.env.insert("HOME", NULL_DEVICE.to_string());
        let dir = tempfile::tempdir().unwrap();
        context.current_dir = dir.path().to_path_buf();

        context.target = Target::Right;

        let expected = String::from(">");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn custom_prompt() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            [profiles]
            test="0_0$character"
            [character]
            format=">>"
        });
        context.target = Target::Profile("test".to_string());

        let expected = String::from("0_0>>");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn custom_prompt_fallback() {
        let mut context = default_context().set_config(toml::toml! {
                add_newline=false
                [profiles]
                test="0_0$character"
                [character]
                format=">>"
        });
        context.target = Target::Profile("wrong_prompt".to_string());

        let expected = String::from(">");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn continuation_prompt() {
        let mut context = default_context().set_config(toml::toml! {
                continuation_prompt="><>"
        });
        context.target = Target::Continuation;

        let expected = String::from("><>");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn preset_list_returns_one_or_more_items() {
        assert!(preset_list().lines().count() > 0);
    }

    #[test]
    fn preset_command_does_not_panic_on_correct_inputs() {
        preset_command(None, None, true);
        for v in Preset::value_variants() {
            preset_command(Some(v.clone()), None, false);
        }
    }

    #[test]
    fn preset_command_output_to_file() -> std::io::Result<()> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("preset.toml");
        preset_command(Some(Preset("nerd-font-symbols")), Some(path.clone()), false);

        let actual = utils::read_file(&path)?;
        let expected = include_str!("../docs/public/presets/toml/nerd-font-symbols.toml");
        assert_eq!(actual, expected);

        dir.close()
    }

    #[test]
    #[cfg(feature = "config-schema")]
    fn print_schema_does_not_panic() {
        print_schema();
    }

    #[test]
    fn custom_expands() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut context = default_context().set_config(toml::toml! {
                format="$custom"
                [custom.a]
                when=true
                format="a"
                [custom.b]
                when=true
                format="b"
        });
        context.current_dir = dir.path().to_path_buf();

        let expected = String::from("\nab");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn env_expands() {
        let mut context = default_context().set_config(toml::toml! {
                format="$env_var"
                [env_var]
                format="$env_value"
                variable = "a"
                [env_var.b]
                format="$env_value"
                [env_var.c]
                format="$env_value"
        });
        context.env.insert("a", "a".to_string());
        context.env.insert("b", "b".to_string());
        context.env.insert("c", "c".to_string());

        let expected = String::from("\nabc");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn custom_mixed() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut context = default_context().set_config(toml::toml! {
                format="${custom.c}$custom${custom.b}"
                [custom.a]
                when=true
                format="a"
                [custom.b]
                when=true
                format="b"
                [custom.c]
                when=true
                format="c"
        });
        context.current_dir = dir.path().to_path_buf();

        let expected = String::from("\ncab");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn env_mixed() {
        let mut context = default_context().set_config(toml::toml! {
                format="${env_var.c}$env_var${env_var.b}"
                [env_var]
                format="$env_value"
                variable = "d"
                [env_var.a]
                format="$env_value"
                [env_var.b]
                format="$env_value"
                [env_var.c]
                format="$env_value"
        });
        context.env.insert("a", "a".to_string());
        context.env.insert("b", "b".to_string());
        context.env.insert("c", "c".to_string());
        context.env.insert("d", "d".to_string());

        let expected = String::from("\ncdab");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn custom_subset() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut context = default_context().set_config(toml::toml! {
                format="${custom.b}"
                [custom.a]
                when=true
                format="a"
                [custom.b]
                when=true
                format="b"
        });
        context.current_dir = dir.path().to_path_buf();

        let expected = String::from("\nb");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn custom_missing() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut context = default_context().set_config(toml::toml! {
                format="${custom.b}"
                [custom.a]
                when=true
                format="a"
        });
        context.current_dir = dir.path().to_path_buf();

        let expected = String::from("\n");
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn wraps_prompt_on_fish_shell_with_colors_at_start_and_middle() {
        let mut context = default_context().set_config(toml::toml! {
                add_newline=false
                format="$character"
                [character]
                format="[ab](fg:purple)cdef\n🫨🙏😺[test](fg:cyan)\n🙈🙉🙊❄✈☢❄✈☢❄️✈️☢️>\n>"
        });
        context.target = Target::Main;
        context.shell = Shell::Fish;
        context.width = 6;

        let expected = CLEAR_FROM_CURSOR_TO_END.to_owned()
            + "\u{1b}[35mab\u{1b}[0mcdef\n🫨🙏😺\u{1b}[36m\ntest\u{1b}[0m\n🙈🙉🙊\n❄✈☢❄✈☢\n❄️✈️☢️\n>\n>";
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn wraps_prompt_on_fish_shell_with_colors_in_middle_only() {
        let mut context = default_context().set_config(toml::toml! {
                add_newline=false
                format="$character"
                [character]
                format="abcdef\n🫨🙏😺[test](fg:cyan)\n🙈🙉🙊❄✈☢❄✈☢❄️✈️☢️>\n>"
        });
        context.target = Target::Main;
        context.shell = Shell::Fish;
        context.width = 6;

        let expected = CLEAR_FROM_CURSOR_TO_END.to_owned()
            + "abcdef\n🫨🙏😺\u{1b}[36m\ntest\u{1b}[0m\n🙈🙉🙊\n❄✈☢❄✈☢\n❄️✈️☢️\n>\n>";
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }

    #[test]
    fn wraps_prompt_on_fish_shell_with_colors_at_end_only() {
        let mut context = default_context().set_config(toml::toml! {
                add_newline=false
                format="$character"
                [character]
                format="abcdef\n🫨🙏😺test\n🙈🙉🙊❄✈☢❄✈☢❄️✈️☢️>\n[>](fg:purple)"
        });
        context.target = Target::Main;
        context.shell = Shell::Fish;
        context.width = 6;

        let expected = CLEAR_FROM_CURSOR_TO_END.to_owned()
            + "abcdef\n🫨🙏😺\ntest\n🙈🙉🙊\n❄✈☢❄✈☢\n❄️✈️☢️\n>\n\u{1b}[35m>\u{1b}[0m";
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }
}
