//! A prompt worked out from configuration before any of its modules run.

use std::collections::{BTreeMap, BTreeSet};

use indexmap::IndexSet;

use crate::configs::PROMPT_ORDER;
use crate::context::{Context, Shell, Target};
use crate::formatter::{StringFormatter, VariableHolder};
use crate::module::ALL_MODULES;
use crate::painted::Painted;
use crate::segment::Segment;
use crate::utils::wrap_colorseq_for_shell;

/// The formats of one or more prompts, and every module they show.
///
/// Planning first lets each module run exactly once, however many times and
/// in however many prompts it is shown, and lets every module run before any
/// prompt is drawn.
pub struct Plan<'a, const PROMPTS: usize> {
    /// Every module the prompts show, each once, those of the first prompt
    /// first.
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

    /// Each prompt as its shell's prompt variable holds it, given what each of
    /// the plan's modules shows, by position.
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

/// One prompt's format, with each of its variables resolved to the modules it
/// shows.
pub struct PromptFormat<'a> {
    target: Target,
    formatter: StringFormatter<'a>,
    /// The modules each shown variable stands for, as positions in the plan's
    /// modules. A variable whose module is disabled is absent, and shows
    /// nothing.
    variables: BTreeMap<String, Vec<usize>>,
}

impl<'a> PromptFormat<'a> {
    fn new(context: &'a Context, target: Target, modules: &mut IndexSet<String>) -> Self {
        let (formatter, module_list) = load_formatter_and_modules(context, &target);
        let variables = formatter
            .get_variables()
            .into_iter()
            .filter(|variable| variable == "all" || !context.is_module_disabled_in_config(variable))
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

    /// The prompt as its shell's prompt variable holds it, given what each of
    /// the plan's modules shows, by position.
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

    // Painting collapses redundant ANSI color sequences, so apply it before modifying the ANSI
    // color sequences for this specific shell
    buf.push_str(&wrap_colorseq_for_shell(
        painted.escaped_for(context.shell),
        context.shell,
    ));

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
        let base = (variable == "env_var").then(|| variable.to_owned());

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::default_context;

    fn planned_modules(config: toml::Table) -> Vec<String> {
        let context = default_context().set_config(config);
        Plan::new(&context, [Target::Main, Target::Right])
            .modules
            .into_iter()
            .collect()
    }

    #[test]
    fn a_module_shown_many_times_runs_once() {
        let modules = planned_modules(toml::toml! {
            format = "$character$status$character"
            right_format = "$status$jobs"
        });

        assert_eq!(["character", "status", "jobs"], modules.as_slice());
    }

    #[test]
    fn a_disabled_module_does_not_run() {
        let modules = planned_modules(toml::toml! {
            format = "$character$status"
            [character]
            disabled = true
        });

        assert_eq!(["status"], modules.as_slice());
    }

    #[test]
    fn all_expands_to_every_module_neither_prompt_names() {
        let modules = planned_modules(toml::toml! {
            format = "$all"
            right_format = "$directory"
        });

        assert!(!modules[..modules.len() - 1].contains(&"directory".to_owned()));
        assert_eq!(Some("directory"), modules.last().map(String::as_str));
        assert_eq!(Some("username"), modules.first().map(String::as_str));
    }
}
