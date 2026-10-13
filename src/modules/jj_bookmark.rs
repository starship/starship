use crate::configs::jj_bookmark::JJBookmarkConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::truncate::truncate_text;

use super::{Context, Module, ModuleConfig};

/// Creates a module with the JJ bookmark in the current repository
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_bookmark");
    let config = JJBookmarkConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let current_change = context.get_jj_repo()?.current_change(context)?;
    let bookmarks = current_change.bookmarks.as_deref()?;

    // The overflow count filters out ignored bookmarks
    let ((name, remote, diverged), overflow_count) = {
        let mut iter = bookmarks.iter().filter_map(|b| {
            let name = b.name();
            let remote = b.remote();
            let ignored = config.ignore_names.contains(&name)
                || remote
                    .map(|r| config.ignore_remotes.contains(&r))
                    .unwrap_or_default();

            (!ignored).then_some((name, remote, b.diverged()))
        });

        (iter.next()?, iter.count())
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                "diverged" => diverged.then_some(config.diverged_symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(move |variable| match variable {
                "bookmark" => Some(Ok(format_item(&config, name))),
                "remote" => remote.map(|s| Ok(format_item(&config, s))),
                "overflow_count" => (overflow_count > 0).then(|| Ok(overflow_count.to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_bookmark`: \n{error}");
            return None;
        }
    });

    Some(module)
}

fn format_item(config: &JJBookmarkConfig, item: &str) -> String {
    truncate_text(
        item,
        config.truncation_length.into(),
        config.truncation_symbol,
    )
}

#[cfg(test)]
pub mod tests {
    use nu_ansi_term::Color;
    use toml::toml;

    use crate::context::JJRepo;
    use crate::test::JJTester;

    fn tester(repo: &'static str) -> JJTester {
        JJTester::new("jj_bookmark").repo(repo)
    }

    #[test]
    fn test_render_basics() {
        JJTester::basic_tests("jj_bookmark");
    }

    #[test]
    fn test_render_default_config() {
        tester(JJRepo::BASE)
            .expected(format!(
                "on {} ",
                Color::Purple.bold().paint("\u{e0a0} cur_local (+3 others)")
            ))
            .render();
    }

    #[test]
    fn test_render_no_current() {
        tester(JJRepo::BOOKMARK_NO_CURRENT)
            .expected(format!(
                "on {} ",
                Color::Purple.bold().paint("\u{e0a0} par_local (+3 others)")
            ))
            .render();
    }

    #[test]
    fn test_render_truncated() {
        tester(JJRepo::BASE)
            .options(toml! {
                truncation_length = 7
                truncation_symbol = "#"
            })
            .expected(format!(
                "on {} ",
                Color::Purple.bold().paint("\u{e0a0} cur_loc# (+3 others)")
            ))
            .render();
    }

    #[test]
    fn test_render_style() {
        tester(JJRepo::BASE)
            .options(toml! { style = "italic red" })
            .expected(format!(
                "on {} ",
                Color::Red.italic().paint("\u{e0a0} cur_local (+3 others)")
            ))
            .render();
    }

    #[test]
    fn test_render_format() {
        tester(JJRepo::BASE)
            .options(toml! { format = "$bookmark(@$remote)$diverged" })
            .expected("cur_local")
            .render();
    }

    #[test]
    fn test_render_ignore_names() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark(@$remote)$diverged( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked" ]
            })
            .expected("cur_modified@upstream* (+1)")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark(@$remote)$diverged( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked", "cur_untracked" ]
            })
            .expected("cur_modified@upstream*")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark(@$remote)$diverged( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked", "cur_modified", "cur_untracked" ]
            })
            .render();
    }

    #[test]
    fn test_render_diverged_symbol() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark(@$remote)$diverged( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked" ]
                diverged_symbol = "#"
            })
            .expected("cur_modified@upstream# (+1)")
            .render();
    }

    #[test]
    fn test_render_ignore_remotes() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark(@$remote)$diverged( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked" ]
                ignore_remotes = [ "upstream" ]
            })
            .expected("cur_untracked@origin")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark(@$remote)$diverged( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked" ]
                ignore_remotes = [ "origin" ]
            })
            .expected("cur_modified@upstream*")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark(@$remote)$diverged"
                ignore_names = [ "cur_local", "cur_tracked" ]
                ignore_remotes = [ "upstream", "origin" ]
            })
            .render();
    }

    #[test]
    fn test_render_remote_with_local() {
        tester(JJRepo::BASE)
            .options(toml! { format = "$remote" })
            .render();
    }

    #[test]
    fn test_render_remote_with_remote() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$remote"
                ignore_names = [ "cur_local", "cur_tracked" ]
            })
            .expected("upstream")
            .render();
    }
    #[test]
    fn test_render_remote_truncated() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$remote"
                ignore_names = [ "cur_local", "cur_tracked" ]
                truncation_length = 3
                truncation_symbol = "#"
            })
            .expected("ups#")
            .render();
    }
}
