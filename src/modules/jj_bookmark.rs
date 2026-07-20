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
    let (bookmark, overflow_count) = {
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
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(move |variable| match variable {
                "bookmark" => Some(Ok(format_bookmark(&config, bookmark))),
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

fn format_bookmark(
    config: &JJBookmarkConfig,
    (name, remote, diverged): (&str, Option<&str>, bool),
) -> String {
    let mut result = truncate_text(
        name,
        config.truncation_length.into(),
        config.truncation_symbol,
    );
    if let Some(remote) = remote {
        result.push('@');
        result.push_str(&truncate_text(
            remote,
            config.truncation_length.into(),
            config.truncation_symbol,
        ));
    }
    if diverged {
        result.push_str(config.diverged_symbol);
    }
    result
}

#[cfg(test)]
pub mod tests {
    use nu_ansi_term::Color;
    use toml::{Table, Value, toml};

    use crate::context::JJRepo;
    use crate::test::ModuleRenderer;

    /// Helper to test JJ modules
    #[must_use]
    pub struct Tester {
        /// JJ module being tested
        module: &'static str,
        /// 'Repo' (see `JJ_REPO` constants) to test in
        repo: &'static str,
        /// Options to add to the module's config, if any
        options: Option<Table>,
        /// Expected output, None by default
        expected: Option<String>,
    }

    impl Tester {
        pub fn new(module: &'static str) -> Self {
            Self {
                module,
                repo: "",
                options: None,
                expected: None,
            }
        }

        pub fn repo(mut self, repo: &'static str) -> Self {
            self.repo = repo;
            self
        }

        pub fn options(mut self, options: Table) -> Self {
            self.options = Some(options);
            self
        }

        pub fn expected(mut self, expected: impl Into<String>) -> Self {
            self.expected = Some(expected.into());
            self
        }

        /// Test rendering against `self.expected`
        #[track_caller]
        pub fn render(self) {
            assert_ne!(
                self.repo, "",
                "You forgot to set a repository for this `Tester` instance"
            );

            let rendered_output = ModuleRenderer::new(self.module)
                .jj_repo(self.repo)
                .config({
                    let mut config = Table::with_capacity(1);
                    if let Some(options) = self.options {
                        config.insert(self.module.into(), Value::Table(options));
                    }
                    config
                })
                .collect();

            assert_eq!(rendered_output, self.expected);
        }

        /// A collection of basic tests, usually run in a `fn test_render_basics()` that will ensure
        /// some sane defaults are respected, like actually disabling on `disabled = true` in the
        /// config or not rendering anything when an unknown var is used or no output is produced
        /// by the `jj` command.
        #[track_caller]
        pub fn basic_tests(module: &'static str) {
            // No JJ repo -> no command output to parse
            Self::new(module).repo(JJRepo::NONE).render();
            // JJ repo but empty output -> nothing to parse
            Self::new(module).repo(JJRepo::EMPTY_OUTPUT).render();
            // JJ repo and invalid output -> parsing fails
            Self::new(module).repo(JJRepo::INVALID_OUTPUT).render();
            // Invalid format
            Self::new(module)
                .repo(JJRepo::BASE)
                .options(toml! { format = "[" })
                .render();
            // Non existent variable
            Self::new(module)
                .repo(JJRepo::BASE)
                .options(toml! { format = "$not_a_valid_jj_variable_in_any_module" })
                .render();
            // Non existent style
            Self::new(module)
                .repo(JJRepo::BASE)
                .options(toml! { format = "[*]($not_a_valid_jj_style_variable_in_any_module)" })
                .expected("*")
                .render();
            // Disabled module
            Self::new(module)
                .repo(JJRepo::BASE)
                .options(toml! { disabled = true })
                .render();
        }
    }

    fn tester(repo: &'static str) -> Tester {
        Tester::new("jj_bookmark").repo(repo)
    }

    #[test]
    fn test_render_basics() {
        Tester::basic_tests("jj_bookmark");
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
            .options(toml! { format = "$bookmark" })
            .expected("cur_local")
            .render();
    }

    #[test]
    fn test_render_ignore_names() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked" ]
            })
            .expected("cur_modified@upstream* (+1)")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked", "cur_untracked" ]
            })
            .expected("cur_modified@upstream*")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked", "cur_modified", "cur_untracked" ]
            })
            .render();
    }

    #[test]
    fn test_render_diverged_symbol() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark( \\(+$overflow_count\\))"
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
                format = "$bookmark( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked" ]
                ignore_remotes = [ "upstream" ]
            })
            .expected("cur_untracked@origin")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark( \\(+$overflow_count\\))"
                ignore_names = [ "cur_local", "cur_tracked" ]
                ignore_remotes = [ "origin" ]
            })
            .expected("cur_modified@upstream*")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$bookmark"
                ignore_names = [ "cur_local", "cur_tracked" ]
                ignore_remotes = [ "upstream", "origin" ]
            })
            .render();
    }
}
