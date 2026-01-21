use super::{Context, Module, ModuleConfig};

use crate::configs::git_mob::GitMobConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_mob");
    let config: GitMobConfig = GitMobConfig::try_load(module.config);

    if config.disabled {
        return None;
    };

    let mob_authors = parse_git_mob_authors(
        &context
            .exec_cmd("git", &["mob-print", "--initials"])?
            .stdout,
    );

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "separator" => Some(config.separator),
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "initials" => Some(Ok(mob_authors.join(config.separator))),
                "count" => Some(Ok(mob_authors.len().to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_mob`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_git_mob_authors(git_mob_stdout: &str) -> Vec<String> {
    return git_mob_stdout.split(',').map(|s| s.to_owned()).collect();
}

#[cfg(test)]
mod tests {
    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::io;

    #[test]
    fn nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_mob")
            .path(repo_dir.path())
            .config(toml::toml! {
                [git_mob]
                    enabled = true
            })
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn nothing_on_disabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let actual = ModuleRenderer::new("git_mob")
            .path(repo_dir.path())
            .config(toml::toml! {
                [git_mob]
                    disabled = true
            })
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn nothing_on_not_existing_cmd() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_mob")
            .path(repo_dir.path())
            .config(toml::toml! {
                [git_mob]
                    disabled = false
            })
            .cmd("git mob-print --initials", None)
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn initials_and_count() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_mob")
            .path(repo_dir.path())
            .config(toml::toml! {
                [git_mob]
                    disabled = false
            })
            .cmd(
                "git mob-print --initials",
                Some(CommandOutput {
                    stdout: String::from("ab,ca,de"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("{}", Color::Green.bold().paint("👥 ab,ca,de (3)")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn initials_with_custom_separator() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_mob")
            .path(repo_dir.path())
            .config(toml::toml! {
                [git_mob]
                    disabled = false
                    separator = "|"
            })
            .cmd(
                "git mob-print --initials",
                Some(CommandOutput {
                    stdout: String::from("ab,ca,de"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("{}", Color::Green.bold().paint("👥 ab|ca|de (3)")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }
}
