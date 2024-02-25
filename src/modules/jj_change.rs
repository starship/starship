use super::{Context, Module};
use crate::config::ModuleConfig;
use crate::configs::jj_change::JujutsuChangeConfig;
use crate::formatter::StringFormatter;

const MOD_NAME: &str = "jj_change";

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module(MOD_NAME);
    let config = JujutsuChangeConfig::try_load(module.config);

    // Default is `true` so it must be checked here
    if config.disabled {
        return None;
    }
    let root = context.get_jj_repo()?;

    // If the change ID is not found (not in a JJ repo), just return none and don't display the module
    let current_jj_change = {
        let template = format!("change_id.shortest({})", config.change_id_length);
        let out = context.exec_cmd(
            "jj",
            &[
                "--repository",
                root,
                "log",
                "--color",
                "never",
                "--revisions",
                "@", // Only display the current revision
                "--no-graph",
                "--template",
                &template,
                "--ignore-working-copy",
            ],
        )?;
        out.stdout.lines().next().map(ToString::to_string)?
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| (variable == "style").then_some(Ok(config.style)))
            .map(|variable| {
                (variable == "change_id")
                    .then_some(&current_jj_change)
                    .map(Ok)
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `{}`:\n{}", MOD_NAME, error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;

    use nu_ansi_term::{Color, Style};

    use super::MOD_NAME;
    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};

    enum Expect<'a> {
        ChangeId(&'a str),
        Empty,
        Style(Style),
    }

    #[test]
    fn test_show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new(MOD_NAME)
            .path(repo_dir.path())
            .collect();

        assert_eq!(actual, None);
        repo_dir.close()
    }

    #[test]
    fn test_disabled_per_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();
        expect_with_config(
            repo_dir,
            Some(toml::toml! {
                [jj_change]
                change_id_length = 7
            }),
            &[Expect::Empty],
        );
        tempdir.close()
    }

    #[test]
    fn test_configured() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();
        expect_with_config(
            repo_dir,
            Some(toml::toml! {
                [jj_change]
                change_id_length = 3
                disabled = false
                style = "underline blue"
            }),
            &[
                Expect::ChangeId("zyx"),
                Expect::Style(Color::Blue.underline()),
            ],
        );
        tempdir.close()
    }

    #[track_caller]
    fn expect_with_config(repo_dir: &Path, config: Option<toml::Table>, expectations: &[Expect]) {
        let actual = ModuleRenderer::new(MOD_NAME)
            .path(repo_dir.to_str().unwrap())
            .config(config.unwrap_or_else(|| {
                toml::toml! {
                    [jj_change]
                    disabled = false
                }
            }))
            .collect();

        let mut expect_change_id = "invalid-change-id";
        let mut expect_style = Color::Purple.bold();

        for expect in expectations {
            match expect {
                Expect::Empty => return assert_eq!(None, actual),
                Expect::ChangeId(change_id) => expect_change_id = change_id,
                Expect::Style(style) => expect_style = *style,
            }
        }

        let expected = format!("{} ", expect_style.paint(format!("({expect_change_id})")));
        assert_eq!(actual, Some(expected));
    }
}
