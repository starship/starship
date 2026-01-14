use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu_commit::JujutsuCommitConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::jujutsu::get_jujutsu_info;
use crate::modules::vcs;

/// Creates a module with the Jujutsu commit hash in the current directory
///
/// Will display the commit hash if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_commit");
    let config: JujutsuCommitConfig = JujutsuCommitConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    }

    // Only run in jj repositories
    vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;

    let jujutsu_info = get_jujutsu_info(context)?;

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
            .map(|variable| match variable {
                "commit" => Some(Ok(jujutsu_info.commit_id.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jujutsu_commit`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;
    use std::io;

    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use crate::utils::CommandOutput;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("jujutsu_commit")
            .path(repo_dir.path())
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_jujutsu_commit_disabled_by_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_commit")
            .path(repo_dir)
            .collect();

        let expected = None; // Should be None because disabled by default
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_commit_simple() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_commit")
            .config(toml::toml! {
                [jujutsu_commit]
                disabled = false
                format = "on [$symbol$commit]($style) "
                style = "bold green"
            })
            .path(repo_dir)
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"abcdefg","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"abcdefg","conflict":false,"divergent":false,"hidden":false,"immutable":false,"bookmark_conflict":false}"#,
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("on {} ", Color::Green.bold().paint("abcdefg")));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_commit_with_custom_symbol_and_style() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_commit")
            .config(toml::toml! {
                [jujutsu_commit]
                disabled = false
                format = "commit: [$symbol$commit]($style)"
                symbol = "🔧 "
                style = "cyan bold"
            })
            .path(repo_dir)
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"abcdefg","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"abcdefg","conflict":false,"divergent":false,"hidden":false,"immutable":false,"bookmark_conflict":false}"#,
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!(
            "commit: {}",
            Color::Cyan.bold().paint("🔧 abcdefg")
        ));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_commit_with_custom_hash_length() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_commit")
            .config(toml::toml! {
                [jujutsu_commit]
                disabled = false
                format = "on [$symbol$commit]($style) "
                commit_hash_length = 4
                style = "bold green"
            })
            .path(repo_dir)
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 4),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"abcdefg","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"abcd","conflict":false,"divergent":false,"hidden":false,"immutable":false,"bookmark_conflict":false}"#,
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("on {} ", Color::Green.bold().paint("abcd")));
        assert_eq!(expected, actual);

        tempdir.close()
    }
}
