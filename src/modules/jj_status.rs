use super::utils::format;
use super::{Context, Module, ModuleConfig};

use crate::configs::jj_status::JJStatusConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the JJ status in the current working directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_status");
    let config = JJStatusConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let current_change = context.get_jj_repo()?.current_change(context)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "all" => Some("$conflicted$description$hidden$immutable$added$copied$deleted$modified$renamed"),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map_variables_to_segments(|variable| {
                let segments = match variable {
                    "added" => format::count(config.added, "jj_status.added", context, current_change.status.added),
                    "copied" => format::count(config.copied, "jj_status.copied", context, current_change.status.copied),
                    "deleted" => format::count(config.deleted, "jj_status.deleted", context, current_change.status.deleted),
                    "modified" => format::count(config.modified, "jj_status.modified", context, current_change.status.modified),
                    "renamed" => format::count(config.renamed, "jj_status.renamed", context, current_change.status.renamed),
                    "conflicted" => current_change.conflicted().then(|| format::symbol(config.conflicted, "jj_status.conflicted", context)).flatten(),
                    "description" => match current_change.description() {
                        true => format::symbol(config.description_present, "jj_status.description_present", context),
                        false => format::symbol(config.description_empty, "jj_status.description_empty", context),
                    },
                    "hidden" => current_change.hidden().then(|| format::symbol(config.hidden, "jj_status.hidden", context)).flatten(),
                    "immutable" => current_change.immutable().then(|| format::symbol(config.immutable, "jj_status.immutable", context)).flatten(),
                    _ => None,
                };

            segments.map(Ok)
        })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_status`:\n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;
    use toml::toml;

    use crate::context::JJRepo;
    use crate::test::JJTester;

    fn tester(repo: &'static str) -> JJTester {
        JJTester::new("jj_status").repo(repo)
    }

    #[test]
    fn test_render_basics() {
        JJTester::basic_tests("jj_status");
    }

    #[test]
    fn test_render_default_config() {
        tester(JJRepo::BASE)
            .expected(format!("{} ", Color::Red.bold().paint("[!◌+=✘~»]")))
            .render();
    }

    #[test]
    fn test_render_style() {
        tester(JJRepo::BASE)
            .options(toml! {
                style = "italic blue"
            })
            .expected(format!("{} ", Color::Blue.italic().paint("[!◌+=✘~»]")))
            .render();
    }

    #[test]
    fn test_render_format() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$all"
            })
            .expected("!◌+=✘~»")
            .render();
    }

    #[test]
    fn test_render_format_status_immediate_conflict() {
        tester(JJRepo::STATUS_IMMEDIATE_CONFLICT)
            .options(toml! { format = "$all" })
            .expected("!◌+=✘~»")
            .render();
        tester(JJRepo::STATUS_IMMEDIATE_CONFLICT)
            .options(toml! { format = "$conflicted" })
            .expected("!")
            .render();
    }

    #[test]
    fn test_render_format_status_no_conflict() {
        tester(JJRepo::STATUS_NO_CONFLICT)
            .options(toml! { format = "$all" })
            .expected("◌+=✘~»")
            .render();
        tester(JJRepo::STATUS_NO_CONFLICT)
            .options(toml! { format = "$conflicted" })
            .render();
    }

    #[test]
    fn test_render_format_status_description() {
        tester(JJRepo::STATUS_DESCRIPTION)
            .options(toml! {
                format = "$description"
                description_present = "d"
            })
            .expected("d")
            .render();
    }

    #[test]
    fn test_render_format_status_hidden() {
        tester(JJRepo::STATUS_HIDDEN)
            .options(toml! {
                format = "$hidden"
                hidden = "h"
            })
            .expected("h")
            .render();
    }

    #[test]
    fn test_render_format_status_immutable() {
        tester(JJRepo::STATUS_IMMUTABLE)
            .options(toml! {
                format = "$immutable"
            })
            .expected("◆")
            .render();
    }

    #[test]
    fn test_render_format_status_added() {
        tester(JJRepo::STATUS_ADDED)
            .options(toml! {
                format = "$added"
            })
            .expected("+")
            .render();

        tester(JJRepo::STATUS_NO_CHANGES)
            .options(toml! {
                format = "$added"
            })
            .render();
    }

    #[test]
    fn test_render_format_status_copied() {
        tester(JJRepo::STATUS_COPIED)
            .options(toml! {
                format = "$copied"
            })
            .expected("=")
            .render();

        tester(JJRepo::STATUS_NO_CHANGES)
            .options(toml! {
                format = "$copied"
            })
            .render();
    }

    #[test]
    fn test_render_format_status_deleted() {
        tester(JJRepo::STATUS_DELETED)
            .options(toml! {
                format = "$deleted"
            })
            .expected("✘")
            .render();

        tester(JJRepo::STATUS_NO_CHANGES)
            .options(toml! {
                format = "$deleted"
            })
            .render();
    }

    #[test]
    fn test_render_format_status_modified() {
        tester(JJRepo::STATUS_MODIFIED)
            .options(toml! {
                format = "$modified"
            })
            .expected("~")
            .render();

        tester(JJRepo::STATUS_NO_CHANGES)
            .options(toml! {
                format = "$modified"
            })
            .render();
    }

    #[test]
    fn test_render_format_status_renamed() {
        tester(JJRepo::STATUS_RENAMED)
            .options(toml! {
                format = "$renamed"
            })
            .expected("»")
            .render();

        tester(JJRepo::STATUS_NO_CHANGES)
            .options(toml! {
                format = "$renamed"
            })
            .render();
    }

    #[test]
    fn test_render_format_status_added_with_count() {
        tester(JJRepo::STATUS_ADDED)
            .options(toml! {
                format = "$added"
                added = "+$count"
            })
            .expected("+2")
            .render();
    }

    #[test]
    fn test_render_format_status_copied_with_count() {
        tester(JJRepo::STATUS_COPIED)
            .options(toml! {
                format = "$copied"
                copied = "=$count"
            })
            .expected("=3")
            .render();
    }

    #[test]
    fn test_render_format_status_deleted_with_count() {
        tester(JJRepo::STATUS_DELETED)
            .options(toml! {
                format = "$deleted"
                deleted = "✘$count"
            })
            .expected("✘4")
            .render();
    }

    #[test]
    fn test_render_format_status_modified_with_count() {
        tester(JJRepo::STATUS_MODIFIED)
            .options(toml! {
                format = "$modified"
                modified = "~$count"
            })
            .expected("~5")
            .render();
    }

    #[test]
    fn test_render_format_status_renamed_with_count() {
        tester(JJRepo::STATUS_RENAMED)
            .options(toml! {
                format = "$renamed"
                renamed = "»$count"
            })
            .expected("»6")
            .render();
    }
}
