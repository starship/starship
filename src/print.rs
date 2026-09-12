use clap::{ValueEnum, builder::PossibleValue};
use nu_ansi_term::AnsiStrings;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::{Debug, Write as FmtWrite};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;
use terminal_size::terminal_size;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

use crate::configs::PROMPT_ORDER;
use crate::context::{Context, Properties, Shell, Target};
use crate::formatter::{StringFormatter, VariableHolder};
use crate::module::ALL_MODULES;
use crate::module::Module;
use crate::modules;
use crate::segment::Segment;
use crate::shadow;
use crate::utils::wrap_colorseq_for_shell;

pub struct Grapheme<'a>(pub &'a str);

impl Grapheme<'_> {
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
    // Magenta string test
    assert_eq!(11, "\x1B[35;6mnormal text".width_graphemes());
}

pub fn prompt(args: Properties, target: Target) {
    let context = Context::new(args, target);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write!(handle, "{}", get_prompt(&context)).unwrap();
}

pub fn prompt_with_claude_code(args: Properties, target: Target) {
    let claude_data = serde_json::from_reader(io::stdin())
        .inspect_err(|e| log::error!("Failed to read Claude Code JSON from stdin: {e}"))
        .unwrap_or_default();

    let mut context = Context::new(args, target).with_claude_code_data(claude_data);
    context.shell = Shell::Unknown;
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write!(handle, "{}", get_prompt(&context)).unwrap();
}

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
    if Shell::Fish == context.shell && context.target == Target::Main {
        buf.push_str("\x1b[J"); // An ASCII control code to clear screen
    }

    let (formatter, modules) = load_formatter_and_modules(context);
    let module_plan = create_module_plan(&formatter, context, &modules);
    let module_cache = compute_module_cache(&module_plan, context);
    let rendered_prompt = render_responsive_prompt(formatter, &module_plan, &module_cache, context);

    if config.add_newline && context.target != Target::Continuation {
        // continuation prompts normally do not include newlines, but they can
        writeln!(buf).unwrap();
    }
    // The rendered prompt has already passed through AnsiStrings, so shell wrapping happens after
    // redundant ANSI color sequences are stripped.
    let shell_wrapped_output = wrap_colorseq_for_shell(rendered_prompt, context.shell);
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

    modules.sort_by_key(|m| std::cmp::Reverse(m.duration));

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

    let (formatter, modules) = load_formatter_and_modules(context);
    let module_plan = create_module_plan(&formatter, context, &modules);

    for module in concrete_module_names(&module_plan) {
        prompt_order.extend(handle_module(&module, context));
    }

    prompt_order
}

type ModulePlan = BTreeMap<String, Vec<String>>;
type ModuleCache = HashMap<String, Vec<Segment>>;

fn create_module_plan(
    formatter: &StringFormatter<'_>,
    context: &Context,
    module_list: &BTreeSet<String>,
) -> ModulePlan {
    formatter
        .get_variables()
        .into_iter()
        .map(|variable| {
            let modules = expand_module_variable(&variable, context, module_list);
            (variable, modules)
        })
        .collect()
}

fn expand_module_variable(
    module: &str,
    context: &Context,
    module_list: &BTreeSet<String>,
) -> Vec<String> {
    if module == "all" {
        all_modules_uniq(module_list)
            .into_iter()
            .flat_map(|module| expand_module_variable(&module, context, module_list))
            .collect()
    } else if matches!(module, "custom" | "env_var") {
        grouped_module_names(module, context, module_list)
    } else {
        vec![module.to_string()]
    }
}

fn grouped_module_names(
    module: &str,
    context: &Context,
    module_list: &BTreeSet<String>,
) -> Vec<String> {
    let mut modules = Vec::new();

    if module == "env_var" {
        modules.push(module.to_string());
    }

    modules.extend(
        context
            .config
            .get_config(&[module])
            .and_then(|config| config.as_table().map(toml::map::Map::iter))
            .into_iter()
            .flatten()
            .filter_map(|(child, config)| {
                if module == "env_var" && !config.is_table() {
                    None
                } else if should_add_implicit_module(module, child, config, module_list) {
                    Some(format!("{module}.{child}"))
                } else {
                    None
                }
            }),
    );

    modules
}

fn concrete_module_names(module_plan: &ModulePlan) -> Vec<String> {
    let mut seen = BTreeSet::new();

    module_plan
        .values()
        .flatten()
        .filter(|module| seen.insert((*module).clone()))
        .cloned()
        .collect()
}

fn compute_module_cache(module_plan: &ModulePlan, context: &Context) -> ModuleCache {
    concrete_module_names(module_plan)
        .par_iter()
        .map(|module| {
            let segments = handle_module(module, context)
                .into_iter()
                .flat_map(|module| module.segments)
                .collect();
            (module.clone(), segments)
        })
        .collect()
}

fn map_cached_modules<'a>(
    formatter: StringFormatter<'a>,
    module_plan: &ModulePlan,
    module_cache: &ModuleCache,
    hidden_modules: &BTreeSet<String>,
) -> StringFormatter<'a> {
    formatter.map_variables_to_segments(|variable| {
        let modules = module_plan.get(variable)?;

        if modules.iter().all(|module| hidden_modules.contains(module)) {
            return None;
        }

        Some(Ok(modules
            .iter()
            .filter(|module| !hidden_modules.contains(*module))
            .filter_map(|module| module_cache.get(module))
            .flatten()
            .cloned()
            .collect()))
    })
}

fn render_responsive_prompt<'a>(
    formatter: StringFormatter<'a>,
    module_plan: &ModulePlan,
    module_cache: &ModuleCache,
    context: &'a Context,
) -> String {
    let mut hidden_modules = BTreeSet::new();
    let mut rendered = render_prompt(
        formatter,
        module_plan,
        module_cache,
        &hidden_modules,
        context,
    );

    if context.width == 0
        || context.target == Target::Continuation
        || context.root_config.responsive.drop_order.is_empty()
        || prompt_fits(&rendered, context)
    {
        return rendered;
    }

    let Some(format) = selected_format(context) else {
        return rendered;
    };

    for module in &context.root_config.responsive.drop_order {
        if matches!(module.as_str(), "character" | "line_break" | "fill")
            || !module_cache.contains_key(module)
            || !hidden_modules.insert(module.clone())
        {
            continue;
        }

        let formatter = StringFormatter::new(format)
            .expect("responsive format was successfully parsed before rendering");
        rendered = render_prompt(
            formatter,
            module_plan,
            module_cache,
            &hidden_modules,
            context,
        );

        if prompt_fits(&rendered, context) {
            break;
        }
    }

    rendered
}

fn render_prompt<'a>(
    formatter: StringFormatter<'a>,
    module_plan: &ModulePlan,
    module_cache: &ModuleCache,
    hidden_modules: &BTreeSet<String>,
    context: &'a Context,
) -> String {
    let formatter = map_cached_modules(formatter, module_plan, module_cache, hidden_modules);
    let mut root_module = Module::new("Starship Root", "The root module", None);
    root_module.set_segments(
        formatter
            .parse(None, Some(context))
            .expect("Unexpected error returned in root format variables"),
    );

    let module_strings = root_module.ansi_strings_for_width(Some(context.width));
    AnsiStrings(&module_strings).to_string()
}

fn prompt_fits(rendered: &str, context: &Context) -> bool {
    // Measure before shell wrappers are added. Module values have already passed through
    // shell_prompt_escape, so escaped Zsh percent signs and Bash metacharacters may over-measure.
    if context.target == Target::Right {
        rendered.replace('\n', "").width_graphemes() <= context.width
    } else {
        rendered
            .lines()
            .all(|line| line.width_graphemes() <= context.width)
    }
}

fn selected_format<'a>(context: &'a Context<'_>) -> Option<&'a str> {
    match &context.target {
        Target::Main => Some(&context.root_config.format),
        Target::Right => Some(&context.root_config.right_format),
        Target::Profile(name) => context
            .root_config
            .user_profiles
            .get(name)
            .or_else(|| context.root_config.internal_profiles.get(name))
            .map(String::as_str),
        Target::Continuation => Some(&context.root_config.continuation_prompt),
    }
}

fn handle_module<'a>(module: &str, context: &'a Context) -> Vec<Module<'a>> {
    let mut modules: Vec<Module> = Vec::new();

    if ALL_MODULES.contains(&module) || module == "env_var" {
        // Write out a module if it isn't disabled
        if !context.is_module_disabled_in_config(module) {
            modules.extend(modules::handle(module, context));
        }
    } else if module.starts_with("custom.") || module.starts_with("env_var.") {
        // custom.<name> and env_var.<name> are special cases and handle disabled modules themselves
        modules.extend(modules::handle(module, context));
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
        format!("{milis:?}ms")
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
            if let Some(lf) = config
                .user_profiles
                .get(name)
                .or_else(|| config.internal_profiles.get(name))
            {
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

pub fn preset_command(name: Option<Preset>, output: Option<PathBuf>, force: bool, list: bool) {
    if list {
        println!("{}", preset_list());
        return;
    }
    let variant = name.expect("name argument must be specified");
    let content = shadow::get_preset_content(variant.0);
    if let Some(output) = output {
        if let Err(e) = crate::utils::write_file_atomic(&output, content, force) {
            eprintln!("Error writing preset to {output:?}: {e}");
            std::process::exit(1);
        }
    } else if let Err(err) = std::io::stdout().write_all(content.as_bytes()) {
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
    fn responsive_prompt_drops_modules_in_configured_order() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${env_var.first}${env_var.second}${env_var.essential}"
            [responsive]
            drop_order = ["env_var.first", "env_var.second"]
            [env_var.first]
            variable = "first"
            format = "$env_value"
            [env_var.second]
            variable = "second"
            format = "$env_value"
            [env_var.essential]
            variable = "essential"
            format = "$env_value"
        });
        context.env.insert("first", "123".to_string());
        context.env.insert("second", "45".to_string());
        context.env.insert("essential", "ok".to_string());
        context.width = 4;

        assert_eq!(get_prompt(&context), "45ok");
    }

    #[test]
    fn responsive_measurement_uses_visible_line_width() {
        let mut context = default_context();
        context.width = 2;

        assert!(prompt_fits("\x1b[31mab\x1b[0m\n👩‍👩‍👦‍👦", &context));
        assert!(!prompt_fits("abc", &context));
    }

    #[test]
    fn responsive_absent_and_empty_config_preserve_output() {
        let mut absent = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${env_var.value}"
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        absent.env.insert("value", "long".to_string());
        absent.width = 1;

        let mut empty = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${env_var.value}"
            [responsive]
            drop_order = []
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        empty.env.insert("value", "long".to_string());
        empty.width = 1;

        assert_eq!(get_prompt(&absent), "long");
        assert_eq!(get_prompt(&empty), "long");
    }

    #[test]
    fn responsive_exact_width_keeps_all_modules() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${env_var.value}"
            [responsive]
            drop_order = ["env_var.value"]
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        context.env.insert("value", "fits".to_string());
        context.width = 4;

        assert_eq!(get_prompt(&context), "fits");
    }

    #[test]
    fn responsive_exhaustion_keeps_remaining_content() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "literal${env_var.value}"
            [responsive]
            drop_order = ["env_var.value"]
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        context.env.insert("value", "drop".to_string());
        context.width = 3;

        assert_eq!(get_prompt(&context), "literal");
    }

    #[test]
    fn responsive_hides_conditional_separators_but_not_literals() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "L( | ${env_var.value})R"
            [responsive]
            drop_order = ["env_var.value"]
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        context.env.insert("value", "long".to_string());
        context.width = 2;

        assert_eq!(get_prompt(&context), "LR");
    }

    #[test]
    fn responsive_skips_missing_empty_disabled_and_repeated_entries() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${env_var.disabled}${env_var.empty}${env_var.first}${env_var.second}"
            [responsive]
            drop_order = [
                "missing",
                "env_var.disabled",
                "env_var.empty",
                "env_var.first",
                "env_var.first",
                "env_var.second",
            ]
            [env_var.disabled]
            disabled = true
            variable = "disabled"
            format = "$env_value"
            [env_var.empty]
            variable = "empty"
            format = "$env_value"
            [env_var.first]
            variable = "first"
            format = "$env_value"
            [env_var.second]
            variable = "second"
            format = "$env_value"
        });
        context.env.insert("disabled", "ignored".to_string());
        context.env.insert("first", "123".to_string());
        context.env.insert("second", "45".to_string());
        context.width = 2;

        assert_eq!(get_prompt(&context), "45");
    }

    #[test]
    fn responsive_measures_every_main_prompt_line() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${env_var.short}\n${env_var.long}"
            [responsive]
            drop_order = ["env_var.long"]
            [env_var.short]
            variable = "short"
            format = "$env_value"
            [env_var.long]
            variable = "long"
            format = "$env_value"
        });
        context.env.insert("short", "ok".to_string());
        context.env.insert("long", "overflow".to_string());
        context.width = 2;

        assert_eq!(get_prompt(&context), "ok\n");
    }

    #[test]
    fn responsive_all_preserves_concrete_module_identity() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$all"
            right_format = "$directory"
            [responsive]
            drop_order = ["custom.extra"]
            [custom.extra]
            when = true
            format = "xx"
            [line_break]
            disabled = true
            [character]
            format = ">"
        });
        context.env.insert("HOME", NULL_DEVICE.to_string());
        context.current_dir = dir.path().to_path_buf();
        context.width = 1;

        assert_eq!(get_prompt(&context), ">");
        dir.close()
    }

    #[test]
    fn responsive_implicit_groups_preserve_child_identity() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$custom$env_var$character"
            [responsive]
            drop_order = ["custom.extra", "env_var.extra"]
            [custom.extra]
            when = true
            format = "aa"
            [env_var.extra]
            variable = "extra"
            format = "$env_value"
            [character]
            format = ">"
        });
        context.env.insert("extra", "bb".to_string());
        context.width = 3;

        assert_eq!(get_prompt(&context), "bb>");
    }

    #[test]
    fn responsive_right_prompt_measures_joined_lines() {
        let mut context = default_context().set_config(toml::toml! {
            right_format = "${env_var.first}\n${env_var.second}"
            [responsive]
            drop_order = ["env_var.first"]
            [env_var.first]
            variable = "first"
            format = "$env_value"
            [env_var.second]
            variable = "second"
            format = "$env_value"
        });
        context.env.insert("first", "ab".to_string());
        context.env.insert("second", "cd".to_string());
        context.target = Target::Right;
        context.width = 3;

        assert_eq!(get_prompt(&context), "cd");
    }

    #[test]
    fn responsive_profile_prompt_drops_modules() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            [responsive]
            drop_order = ["env_var.extra"]
            [profiles]
            test = "${env_var.extra}$character"
            [env_var.extra]
            variable = "extra"
            format = "$env_value"
            [character]
            format = ">"
        });
        context.env.insert("extra", "long".to_string());
        context.target = Target::Profile("test".to_string());
        context.width = 1;

        assert_eq!(get_prompt(&context), ">");
    }

    #[test]
    fn responsive_bypasses_continuation_and_unknown_width() {
        let mut continuation = default_context().set_config(toml::toml! {
            continuation_prompt = "${env_var.value}"
            [responsive]
            drop_order = ["env_var.value"]
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        continuation.env.insert("value", "long".to_string());
        continuation.target = Target::Continuation;
        continuation.width = 1;

        let mut unknown_width = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${env_var.value}"
            [responsive]
            drop_order = ["env_var.value"]
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        unknown_width.env.insert("value", "long".to_string());
        unknown_width.width = 0;

        assert_eq!(get_prompt(&continuation), "long");
        assert_eq!(get_prompt(&unknown_width), "long");
    }

    #[test]
    fn responsive_protects_structural_modules_and_reflows_fill() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$fill$line_break$character${env_var.value}"
            [responsive]
            drop_order = ["fill", "line_break", "character", "env_var.value"]
            [fill]
            symbol = "."
            style = ""
            [character]
            format = ">"
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        context.env.insert("value", "long".to_string());
        context.width = 1;

        assert_eq!(get_prompt(&context), ".\n>");
    }

    #[test]
    fn responsive_treats_vcs_as_an_atomic_module() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        std::fs::create_dir(dir.path().join(".hg"))?;
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "$vcs$character"
            [responsive]
            drop_order = ["custom.inner"]
            [vcs]
            order = ["hg"]
            hg_modules = "${custom.inner}"
            [custom.inner]
            when = true
            format = "inner"
            [character]
            format = ">"
        });
        context.current_dir = dir.path().to_path_buf();
        context.width = 1;

        assert_eq!(get_prompt(&context), "inner>");

        context.root_config.responsive.drop_order = vec!["vcs".to_string()];
        assert_eq!(get_prompt(&context), ">");
        dir.close()
    }

    #[test]
    fn responsive_zsh_escape_can_overmeasure_module_output() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "${env_var.value}"
            [responsive]
            drop_order = ["env_var.value"]
            [env_var.value]
            variable = "value"
            format = "$env_value"
        });
        context.env.insert("value", "%".to_string());
        context.shell = Shell::Zsh;
        context.width = 1;

        // Zsh consumes the doubled percent as one visible character, but responsive measurement
        // sees the already escaped value and may conservatively drop it.
        assert_eq!(get_prompt(&context), "");
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
        preset_command(None, None, false, true);
        for v in Preset::value_variants() {
            preset_command(Some(v.clone()), None, false, false);
        }
    }

    #[test]
    fn preset_command_output_to_file() -> std::io::Result<()> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("preset.toml");
        preset_command(
            Some(Preset("nerd-font-symbols")),
            Some(path.clone()),
            false,
            false,
        );

        let actual = utils::read_file(&path)?;
        let expected = include_str!("../docs/public/presets/toml/nerd-font-symbols.toml");
        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn preset_command_output_existing_file_force() -> io::Result<()> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("preset.toml");
        utils::write_file(&path, "existing content")?;

        preset_command(
            Some(Preset("nerd-font-symbols")),
            Some(path.clone()),
            true,
            false,
        );

        let actual = utils::read_file(&path).unwrap();
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
    fn all_plan_preserves_implicit_custom_identity() {
        let context = default_context().set_config(toml::toml! {
                format="$all${custom.b}"
                [custom.a]
                when=true
                format="a"
                [custom.b]
                when=true
                format="b"
        });
        let (formatter, modules) = load_formatter_and_modules(&context);
        let module_plan = create_module_plan(&formatter, &context, &modules);

        assert!(module_plan["all"].contains(&"custom.a".to_string()));
        assert!(!module_plan["all"].contains(&"custom.b".to_string()));
        assert_eq!(module_plan["custom.b"], ["custom.b"]);
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
    fn env_plan_preserves_top_level_and_implicit_identity() {
        let context = default_context().set_config(toml::toml! {
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
        let (formatter, modules) = load_formatter_and_modules(&context);
        let module_plan = create_module_plan(&formatter, &context, &modules);

        assert_eq!(module_plan["env_var"], ["env_var", "env_var.a"]);
        assert_eq!(module_plan["env_var.b"], ["env_var.b"]);
        assert_eq!(module_plan["env_var.c"], ["env_var.c"]);
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
    fn test_prefer_user_profile() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            [profiles]
            claude-code = "user profile"
        });
        context.target = Target::Profile("claude-code".to_string());

        let expected = "user profile";
        let actual = get_prompt(&context);
        assert_eq!(expected, actual);
    }
}
