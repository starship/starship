use clap::{ValueEnum, builder::PossibleValue};
use indexmap::IndexSet;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Write as FmtWrite};
use std::io::{self, Write};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::thread::{self, Scope};
use std::time::Duration;
use terminal_size::terminal_size;
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};
use unicode_width::UnicodeWidthStr;

use crate::configs::PROMPT_ORDER;
use crate::context::{Context, Properties, Shell, Target};
use crate::formatter::{StringFormatter, VariableHolder};
use crate::module::ALL_MODULES;
use crate::module::Module;
use crate::modules;
use crate::painted::Painted;
use crate::segment::Segment;
use crate::shadow;

pub struct Grapheme<'a>(pub &'a str);

impl Grapheme<'_> {
    pub fn width(&self) -> usize {
        self.0.width_graphemes()
    }
}

pub trait UnicodeWidthGraphemes {
    fn width_graphemes(&self) -> usize;
}

impl<T> UnicodeWidthGraphemes for T
where
    T: AsRef<str>,
{
    /// As wide as Unicode says a terminal draws the printable text, measured
    /// whole, so that controls within a grapheme cannot split it.
    fn width_graphemes(&self) -> usize {
        let mut chunks = anstream::adapter::strip_str(self.as_ref());
        let mut printable = Cow::Borrowed(chunks.next().unwrap_or_default());
        for chunk in chunks {
            printable.to_mut().push_str(chunk);
        }
        printable.width()
    }
}

/// Source spans of the graphemes a terminal draws. Controls between a
/// grapheme's characters stay in its span, so a fill cannot cut through it.
pub fn graphemes(text: &str) -> impl Iterator<Item = &str> {
    let mut chunks = anstream::adapter::strip_str(text);
    let mut chunk = chunks.next().unwrap_or_default();
    let mut printable = Cow::Borrowed(chunk);
    for next in chunks.clone() {
        printable.to_mut().push_str(next);
    }
    let mut cursor = GraphemeCursor::new(0, printable.len(), true);
    let mut offset = 0;
    std::iter::from_fn(move || {
        let start = cursor.cur_cursor();
        let end = cursor
            .next_boundary(&printable, 0)
            .expect("complete printable text supplies every grapheme boundary")?;
        while start == offset + chunk.len() {
            offset += chunk.len();
            chunk = chunks.next()?;
        }
        let source_start = chunk.as_ptr().addr() - text.as_ptr().addr() + start - offset;
        while end > offset + chunk.len() {
            offset += chunk.len();
            chunk = chunks.next()?;
        }
        let source_end = chunk.as_ptr().addr() - text.as_ptr().addr() + end - offset;
        Some(&text[source_start..source_end])
    })
}

#[test]
fn test_grapheme_aware_width() {
    // A family of four, joined, is drawn as one.
    assert_eq!(2, "👩‍👩‍👦‍👦".width_graphemes());
    assert_eq!(1, "Ü".width_graphemes());
    assert_eq!(11, "normal text".width_graphemes());
    // Magenta string test
    assert_eq!(11, "\x1B[35;6mnormal text".width_graphemes());
    // A hyperlink, a window title and a true color draw only their text.
    assert_eq!(
        4,
        "\x1b]8;;https://x\x1b\\link\x1b]8;;\x1b\\\x1b]0;title\x07\x1b[38:2::1:2:3m"
            .width_graphemes()
    );
}

#[test]
fn a_grapheme_is_as_wide_as_a_terminal_draws_it() {
    for (grapheme, width) in [
        ("\u{2764}\u{fe0f}", 2),
        ("1\u{fe0f}\u{20e3}", 2),
        ("\u{1f1fa}\u{1f1f3}", 2),
        ("\u{1f469}\u{200d}\u{1f469}\u{200d}\u{1f466}", 2),
        ("U\u{308}", 1),
        ("\u{ac01}", 2),
    ] {
        assert_eq!(width, Grapheme(grapheme).width(), "{grapheme:?}");
        assert_eq!(
            width * 3,
            grapheme.repeat(3).width_graphemes(),
            "{grapheme:?}"
        );
    }
}

#[test]
fn controls_inside_a_grapheme_preserve_its_width_and_source_span() {
    for plain in ["👩‍👩‍👦‍👦", "Ü", "🇺🇳", "각"] {
        for control in [
            "\x1b[31m",
            "\x1b[38:2::1:2:3m",
            "\x1b]8;;https://x\x1b\\",
            "\x1b]0;title\x07",
            "\x07",
        ] {
            for boundary in plain.char_indices().map(|(index, _)| index) {
                let controlled = format!("{}{control}{}", &plain[..boundary], &plain[boundary..]);
                assert_eq!(
                    plain.width_graphemes(),
                    controlled.width_graphemes(),
                    "{controlled:?}"
                );
                let spans: Vec<_> = graphemes(&controlled).collect();
                assert_eq!(1, spans.len(), "{controlled:?}");
                assert_eq!(plain.width_graphemes(), Grapheme(spans[0]).width());
                assert!(controlled.ends_with(spans[0]));
            }
            let controlled = plain
                .chars()
                .map(|ch| format!("{ch}{control}"))
                .collect::<String>();
            assert_eq!(plain.width_graphemes(), controlled.width_graphemes());
            assert_eq!(1, graphemes(&controlled).count(), "{controlled:?}");
        }
    }
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

/// What starship shows in place of a prompt under a terminal that cannot draw one.
pub(crate) const DUMB_TERMINAL_PROMPT: &str = "Starship disabled due to TERM=dumb > ";

pub(crate) fn is_dumb_terminal(context: &Context) -> bool {
    context.get_env("TERM").as_deref() == Some("dumb")
}

pub fn get_prompt(context: &Context) -> String {
    if is_dumb_terminal(context) {
        log::error!("Under a 'dumb' terminal (TERM=dumb).");
        return DUMB_TERMINAL_PROMPT.to_owned();
    }

    let plan = Plan::new(context, [context.target.clone()]);
    let rendered = plan.render(context);
    let [prompt] = plan.prompts(&|position| &rendered[position], context);
    prompt
}

/// Prompt formats sharing one ordered set of modules to render.
pub struct Plan<'a, const PROMPTS: usize> {
    /// Deduplicated modules: those of each prompt in turn, alphabetically.
    pub modules: IndexSet<String>,
    /// The format of each requested prompt, in the order requested.
    pub formats: [PromptFormat<'a>; PROMPTS],
}

impl<'a, const PROMPTS: usize> Plan<'a, PROMPTS> {
    pub fn new(context: &'a Context, targets: [Target; PROMPTS]) -> Self {
        let mut modules = IndexSet::new();
        let formats = targets.map(|target| PromptFormat::new(context, target, &mut modules));
        Self { modules, formats }
    }

    /// Renders every module of the plan at once, each into a cell of its own.
    pub fn render(&self, context: &Context) -> Vec<Vec<Segment>> {
        let shown: Vec<OnceLock<Vec<Segment>>> =
            self.modules.iter().map(|_| OnceLock::new()).collect();
        let render = |position: usize| {
            let _ = shown[position].set(render(&self.modules[position], context));
        };
        thread::scope(|scope| fan_out(scope, self.modules.len(), render));
        shown
            .into_iter()
            .map(|shown| shown.into_inner().unwrap_or_default())
            .collect()
    }

    /// Compose shell-ready prompts from plan-indexed segments.
    pub fn prompts<'s>(
        &self,
        shown: &impl Fn(usize) -> &'s [Segment],
        context: &Context,
    ) -> [String; PROMPTS] {
        self.formats
            .each_ref()
            .map(|format| format.prompt(shown, context))
    }
}

/// What `module` shows: nothing if it panics while rendering, which takes no
/// other module down with it.
pub fn render(module: &str, context: &Context) -> Vec<Segment> {
    match catch_unwind(AssertUnwindSafe(|| modules::handle(module, context))) {
        Ok(rendered) => rendered.map_or_else(Vec::new, |rendered| rendered.segments),
        Err(_) => {
            log::error!("The {module} module panicked while rendering");
            Vec::new()
        }
    }
}

/// Runs `job` for every index below `count` at once, each on a thread of its
/// own. The threads start one another, each the next two of a binary tree, so
/// that starting a hundred takes about as long as starting seven in turn. A
/// job whose thread cannot start runs on the thread that tried to start it.
pub fn fan_out<'scope, F>(scope: &'scope Scope<'scope, '_>, count: usize, job: F)
where
    F: Fn(usize) + Send + Copy + 'scope,
{
    fn branch<'scope, F>(scope: &'scope Scope<'scope, '_>, index: usize, count: usize, job: F)
    where
        F: Fn(usize) + Send + Copy + 'scope,
    {
        if index >= count {
            return;
        }
        let subtree = move || {
            branch(scope, 2 * index + 1, count, job);
            branch(scope, 2 * index + 2, count, job);
            job(index);
        };
        if let Err(error) = thread::Builder::new().spawn_scoped(scope, subtree) {
            log::debug!("Unable to start a thread to render on: {error}");
            subtree();
        }
    }
    branch(scope, 0, count, job);
}

/// One format with variables resolved to module positions.
pub struct PromptFormat<'a> {
    target: Target,
    formatter: StringFormatter<'a>,
    /// Plan positions for each variable; disabled variables are absent.
    variables: BTreeMap<String, Vec<usize>>,
}

impl<'a> PromptFormat<'a> {
    fn new(context: &'a Context, target: Target, modules: &mut IndexSet<String>) -> Self {
        let (formatter, module_list) = load_formatter_and_modules(context, &target);
        let variables = formatter
            .get_variables()
            .into_iter()
            .filter(|variable| {
                matches!(variable.as_str(), "all" | "env_var")
                    || !context.is_module_disabled_in_config(variable)
            })
            .map(|variable| {
                let positions = modules_named(&variable, context, &module_list)
                    .into_iter()
                    .map(|module| modules.insert_full(module).0)
                    .collect();
                (variable, positions)
            })
            .collect();
        Self {
            target,
            formatter,
            variables,
        }
    }

    fn prompt<'s>(&self, shown: &impl Fn(usize) -> &'s [Segment], context: &Context) -> String {
        let segments = self
            .formatter
            .clone()
            .map_variables_to_segments(|variable| {
                let positions = self.variables.get(variable)?;
                Some(Ok(positions
                    .iter()
                    .flat_map(|&position| shown(position).iter().cloned())
                    .collect()))
            })
            .parse(None, Some(context))
            .expect("Unexpected error returned in root format variables");
        let painted = Painted::new(&segments, Some(context.width));
        // continuation prompts normally do not include newlines, but they can
        if context.root_config.add_newline && self.target != Target::Continuation {
            shell_prompt(&painted.below_a_line_break(), context, &self.target)
        } else {
            shell_prompt(&painted, context, &self.target)
        }
    }
}

/// The text a shell's prompt variable holds for the painted `target` prompt.
fn shell_prompt(painted: &Painted, context: &Context, target: &Target) -> String {
    let mut buf = String::new();

    // A workaround for a fish bug (see #739,#279). Applying it to all shells
    // breaks things (see #808,#824,#834). Should only be printed in fish.
    if Shell::Fish == context.shell && *target == Target::Main {
        buf.push_str("\x1b[J"); // An ASCII control code to clear screen
    }

    buf.push_str(&painted.escaped_for(context.shell));

    if *target == Target::Right {
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
    modules::handle(module_name, context).map(|module| module.to_string())
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
            value: module.to_string().replace('\n', "\\n"),
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
                value: module.to_string(),
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
    Plan::new(context, [Target::Main, Target::Right])
        .modules
        .iter()
        .filter_map(|module| modules::handle(module, context))
        .collect()
}

/// The modules a format variable shows, in order: none, one, or for `$all`,
/// `$custom` and `$env_var`, every module they expand to.
fn modules_named(variable: &str, context: &Context, module_list: &BTreeSet<String>) -> Vec<String> {
    if variable == "all" {
        // Make $all display all modules not explicitly referenced
        all_modules_uniq(module_list)
            .iter()
            .flat_map(|module| modules_named(module, context, module_list))
            .collect()
    } else if ALL_MODULES.contains(&variable) {
        // Write out a module if it isn't disabled
        if context.is_module_disabled_in_config(variable) {
            Vec::new()
        } else {
            vec![variable.to_owned()]
        }
    } else if variable.starts_with("custom.") || variable.starts_with("env_var.") {
        // custom.<name> and env_var.<name> are special cases and handle disabled modules themselves
        vec![variable.to_owned()]
    } else if matches!(variable, "custom" | "env_var") {
        // env var is a special case and may contain a top-level module definition
        let base = (variable == "env_var" && !context.is_module_disabled_in_config(variable))
            .then(|| variable.to_owned());

        // Write out all custom modules, except for those that are explicitly set
        let children = context
            .config
            .get_config(&[variable])
            .and_then(toml::Value::as_table)
            .into_iter()
            .flatten()
            .filter(|(child, config)| {
                // Some env var keys may be part of a top-level module definition
                !(variable == "env_var" && !config.is_table())
                    && should_add_implicit_module(variable, child, config, module_list)
            })
            .map(|(child, _)| format!("{variable}.{child}"));

        base.into_iter().chain(children).collect()
    } else {
        log::debug!(
            "Expected top level format to contain value from {ALL_MODULES:?}. Instead received {variable}",
        );
        Vec::new()
    }
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

/// Load the correct formatter for the target (ie left prompt or right prompt)
/// and the list of all modules used in a format string
fn load_formatter_and_modules<'a>(
    context: &'a Context,
    target: &Target,
) -> (StringFormatter<'a>, BTreeSet<String>) {
    let config = &context.root_config;

    if *target == Target::Continuation {
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

    let (left_format_str, right_format_str): (&str, &str) = match target {
        Target::Main | Target::Right => (&config.format, &config.right_format),
        Target::Profile(name) => {
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
        let name = if let Target::Profile(profile_name) = target {
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

    let main_formatter = match target {
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
    fn a_fill_measures_text_as_displayed_rather_than_as_escaped() {
        let mut context = default_context().set_config(toml::toml! {
            add_newline = false
            format = "100%$fill|"
            [fill]
            style = ""
        });
        context.shell = Shell::Zsh;
        context.width = 10;

        assert_eq!("100%%.....|", get_prompt(&context));
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
    fn standalone_modules_keep_literal_text_for_every_shell() {
        let literal = "$HOME `hostname` \\n 50%";
        for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell] {
            let actual = crate::test::ModuleRenderer::new("env_var")
                .config(toml::toml! {
                    [env_var]
                    variable = "VALUE"
                    format = "$env_value"
                })
                .env("VALUE", literal)
                .shell(shell)
                .collect();
            assert_eq!(Some(literal.to_owned()), actual);
        }
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

    fn planned_modules(config: toml::Table) -> Vec<String> {
        let context = default_context().set_config(config);
        Plan::new(&context, [Target::Main, Target::Right])
            .modules
            .into_iter()
            .collect()
    }

    #[test]
    fn a_fan_out_runs_every_job_once_however_its_tree_is_shaped() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        for count in [0, 1, 2, 3, 6, 7, 8, 63, 64, 65, 200] {
            let runs: Vec<AtomicUsize> = (0..count).map(|_| AtomicUsize::new(0)).collect();
            let job = |index: usize| {
                runs[index].fetch_add(1, Ordering::Relaxed);
            };
            thread::scope(|scope| fan_out(scope, count, job));
            let runs: Vec<usize> = runs.into_iter().map(AtomicUsize::into_inner).collect();
            assert_eq!(vec![1; count], runs, "{count} jobs");
        }
    }

    #[test]
    fn shared_modules_have_one_plan_position() {
        let modules = planned_modules(toml::toml! {
            format = "$character$status$character"
            right_format = "$status$jobs"
        });

        assert_eq!(["character", "status", "jobs"], modules.as_slice());
    }

    #[test]
    fn disabled_modules_are_absent_from_the_plan() {
        let modules = planned_modules(toml::toml! {
            format = "$character$status"
            [character]
            disabled = true
        });

        assert_eq!(["status"], modules.as_slice());
    }

    #[test]
    fn disabled_env_var_base_preserves_enabled_children() {
        for format in ["$env_var", "$all"] {
            let mut config = toml::toml! {
                format = format
                right_format = format
                add_newline = false
                [env_var]
                disabled = true
                default = "BASE"
                [env_var.ENABLED]
                disabled = false
                default = "CHILD"
                format = "$env_value"
                [env_var.DISABLED]
                disabled = true
                default = "HIDDEN"
            };
            // Isolate $all from filesystem, command, and platform modules.
            for module in ALL_MODULES.iter().filter(|module| **module != "env_var") {
                config.insert((*module).to_owned(), toml::toml! { disabled = true }.into());
            }

            let context = default_context().set_config(config);
            let plan = Plan::new(&context, [Target::Main, Target::Right]);
            assert_eq!(
                ["env_var.ENABLED"],
                plan.modules
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .as_slice()
            );

            let rendered = plan.render(&context);
            assert_eq!(
                ["CHILD", "CHILD"],
                plan.prompts(&|position| &rendered[position], &context),
            );
        }
    }

    #[test]
    fn all_expands_to_every_module_neither_prompt_names() {
        let modules = planned_modules(toml::toml! {
            format = "$all"
            right_format = "$directory"
        });

        let expected: Vec<_> = PROMPT_ORDER
            .iter()
            .copied()
            .filter(|module| !matches!(*module, "directory" | "custom"))
            .chain(["directory"])
            .collect();
        assert_eq!(expected, modules);
    }
}
