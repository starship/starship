use super::{Context, Module, ModuleConfig};

use crate::configs::jj_change::JJChangeConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the JJ change and commit in the current directory
///
/// Will display the JJ change hash truncated by default if the current directory is a JJ repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_change");
    let config = JJChangeConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let current = context.get_jj_repo()?.current_change(context)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "change" => Some("[$change_prefix]($prefix_style)[$change_suffix]($suffix_style)"),
                "commit" => Some("[$commit_prefix]($prefix_style)[$commit_suffix]($suffix_style)"),
                _ => None,
            })
            .map_style(|variable| match variable {
                "prefix_style" => Some(Ok(config.prefix_style)),
                "suffix_style" => Some(Ok(config.suffix_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "change_prefix" => prefix(current.change.as_ref(), current.change_shortest).map(Ok),
                "change_suffix" => suffix(
                    current.change.as_ref(),
                    current.change_shortest,
                    config.change_hash_length,
                )
                .map(Ok),
                "commit_prefix" => prefix(current.commit.as_ref(), current.commit_shortest).map(Ok),
                "commit_suffix" => suffix(
                    current.commit.as_ref(),
                    current.commit_shortest,
                    config.commit_hash_length,
                )
                .map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_change`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn prefix(id: &str, prefix_size: u8) -> Option<&str> {
    id.get(..prefix_size.into()).filter(|s| !s.is_empty())
}

fn suffix(id: &str, prefix_size: u8, hash_length: u8) -> Option<&str> {
    let end = prefix_size.max(hash_length);
    id.get(prefix_size.into()..end.into())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::{Color, Style};
    use toml::toml;

    use crate::context::JJRepo;

    use super::{prefix, suffix};

    use crate::modules::jj_bookmark::tests::Tester;

    // == Helper tests ==

    #[test]
    fn test_prefix() {
        assert_eq!(prefix("", 42), None);
        assert_eq!(prefix("abc", 0), None);
        assert_eq!(prefix("123", 12), None);

        assert_eq!(prefix("12345678", 4), Some("1234"));
    }

    #[test]
    fn test_suffix() {
        assert_eq!(suffix("", 24, 42), None);
        assert_eq!(suffix("", 42, 24), None);
        assert_eq!(suffix("abc", 4, 3), None);
        assert_eq!(suffix("123", 2, 2), None);
        assert_eq!(suffix("12345678", 4, 10), None);

        assert_eq!(suffix("12345678", 4, 7), Some("567"));
    }

    // == Render tests ==

    fn tester(repo: &'static str) -> Tester {
        Tester::new("jj_change").repo(repo)
    }

    #[test]
    fn test_render_basics() {
        Tester::basic_tests("jj_change");
    }

    #[test]
    fn test_render_default_config() {
        tester(JJRepo::BASE)
            .expected(format!(
                "{}{} ",
                Color::Green.bold().paint("pvt"),
                Style::new().dimmed().paint("xwmv")
            ))
            .render();
    }

    #[test]
    fn test_render_style() {
        tester(JJRepo::BASE)
            .options(toml! {
                prefix_style = "italic red"
                suffix_style = "blue bold"
            })
            .expected(format!(
                "{}{} ",
                Color::Red.italic().paint("pvt"),
                Color::Blue.bold().paint("xwmv")
            ))
            .render();
    }

    #[test]
    fn test_render_format() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$change \\($commit\\)"
                prefix_style = ""
                suffix_style = ""
            })
            .expected("pvtxwmv (30363e4)")
            .render();
    }

    #[test]
    fn test_render_variables() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$change_prefix $change_suffix \\($commit_prefix $commit_suffix\\)"
            })
            .expected("pvt xwmv (3036 3e4)")
            .render();
    }

    #[test]
    fn test_render_modified_change_hash_length() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$change_prefix $change_suffix"
                change_hash_length = 10
            })
            .expected("pvt xwmvttt")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$change_prefix $change_suffix"
                change_hash_length = 2
            })
            .expected("pvt ")
            .render();
    }

    #[test]
    fn test_render_modified_commit_hash_length() {
        tester(JJRepo::BASE)
            .options(toml! {
                format = "$commit_prefix $commit_suffix"
                commit_hash_length = 12
            })
            .expected("3036 3e463b3a")
            .render();

        tester(JJRepo::BASE)
            .options(toml! {
                format = "$commit_prefix $commit_suffix"
                commit_hash_length = 2
            })
            .expected("3036 ")
            .render();
    }
}
