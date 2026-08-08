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
            .map(|variable| match variable {
                "added" => current_change.status_added().then_some(Ok(config.added)),
                "copied" => current_change.status_copied().then_some(Ok(config.copied)),
                "deleted" => current_change
                    .status_deleted()
                    .then_some(Ok(config.deleted)),
                "modified" => current_change
                    .status_modified()
                    .then_some(Ok(config.modified)),
                "renamed" => current_change
                    .status_renamed()
                    .then_some(Ok(config.renamed)),
                "conflicted" => current_change.conflicted().then_some(Ok(config.conflicted)),
                "description" => Some(Ok(match current_change.description() {
                    true => config.description_present,
                    false => config.description_empty,
                })),
                "hidden" => current_change.hidden().then_some(Ok(config.hidden)),
                "immutable" => current_change.immutable().then_some(Ok(config.immutable)),
                _ => None,
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

    use crate::modules::jj_bookmark::tests::Tester;

    fn tester(repo: &'static str) -> Tester {
        Tester::new("jj_status").repo(repo)
    }

    #[test]
    fn test_render_basics() {
        Tester::basic_tests("jj_status");
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
    }

    #[test]
    fn test_render_format_status_copied() {
        tester(JJRepo::STATUS_COPIED)
            .options(toml! {
                format = "$copied"
            })
            .expected("=")
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
    }

    #[test]
    fn test_render_format_status_modified() {
        tester(JJRepo::STATUS_MODIFIED)
            .options(toml! {
                format = "$modified"
            })
            .expected("~")
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
    }
}
