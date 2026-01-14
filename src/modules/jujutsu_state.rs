use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu_state::JujutsuStateConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::jujutsu::get_jujutsu_info;
use crate::modules::vcs;

/// Creates a module with the Jujutsu state indicators in the current directory
///
/// Will display state indicators (conflicted, divergent, hidden, working copy conflicts)
/// if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_state");
    let config: JujutsuStateConfig = JujutsuStateConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    }

    // Only run in jj repositories
    vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;

    let jujutsu_info = get_jujutsu_info(context)?;

    if !jujutsu_info.conflicted
        && !jujutsu_info.divergent
        && !jujutsu_info.hidden
        && !jujutsu_info.immutable
    {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                "conflicted_style" => Some(Ok(config.conflicted_style)),
                "divergent_style" => Some(Ok(config.divergent_style)),
                "hidden_style" => Some(Ok(config.hidden_style)),
                "immutable_style" => Some(Ok(config.immutable_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "conflicted" => {
                    if jujutsu_info.conflicted {
                        Some(Ok(config.conflicted))
                    } else {
                        None
                    }
                }
                "divergent" => {
                    if jujutsu_info.divergent {
                        Some(Ok(config.divergent))
                    } else {
                        None
                    }
                }
                "hidden" => {
                    if jujutsu_info.hidden {
                        Some(Ok(config.hidden))
                    } else {
                        None
                    }
                }
                "immutable" => {
                    if jujutsu_info.immutable {
                        Some(Ok(config.immutable))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jujutsu_state`:\n{}", error);
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

        let actual = ModuleRenderer::new("jujutsu_state")
            .path(repo_dir.path())
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_jujutsu_state_disabled_by_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_state")
            .path(repo_dir)
            .collect();

        let expected = None; // Should be None because disabled by default
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_state_conflicted() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_state")
            .config(toml::toml! {
                [jujutsu_state]
                disabled = false
                format = "[$conflicted]($conflicted_style)"
            })
            .path(repo_dir)
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"abcdefg","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"abcdefg","conflict":true,"divergent":false,"hidden":false,"immutable":false,"bookmark_conflict":false}"#,
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

        let expected = Some(Color::Red.paint("\u{26a0} ").to_string());
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_state_divergent() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_state")
            .config(toml::toml! {
                [jujutsu_state]
                disabled = false
                format = "[$divergent]($divergent_style)"
            })
            .path(repo_dir)
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"abcdefg","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"abcdefg","conflict":false,"divergent":true,"hidden":false,"immutable":false,"bookmark_conflict":false}"#,
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

        let expected = Some(Color::Purple.paint("󰓁 ").to_string());
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_state_hidden() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_state")
            .config(toml::toml! {
                [jujutsu_state]
                disabled = false
                format = "[$hidden]($hidden_style)"
            })
            .path(repo_dir)
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"abcdefg","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"abcdefg","conflict":false,"divergent":false,"hidden":true,"immutable":false,"bookmark_conflict":false}"#,
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

        let expected = Some("󰘓 ".to_string());
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_state_immutable() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_state")
            .config(toml::toml! {
                [jujutsu_state]
                disabled = false
                format = "[$immutable]($immutable_style)"
            })
            .path(repo_dir)
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"abcdefg","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"abcdefg","conflict":false,"divergent":false,"hidden":false,"immutable":true,"bookmark_conflict":false}"#,
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

        let expected = Some(Color::Red.paint(" ").to_string());
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_state_multiple_states() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_state")
            .config(toml::toml! {
                [jujutsu_state]
                disabled = false
                format = "[$conflicted]($conflicted_style)[$divergent]($divergent_style)[$hidden]($hidden_style)[$immutable]($immutable_style)"
            })
            .path(repo_dir)
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(r#"{"change_id":"abcdefg","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"abcdefg","conflict":true,"divergent":true,"hidden":true,"immutable":false,"bookmark_conflict":false}"#),
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

        let expected = Some("\u{1b}[31m⚠ \u{1b}[35m󰓁 \u{1b}[0m󰘓 ".to_string());
        assert_eq!(expected, actual);

        tempdir.close()
    }
}
