use super::{Context, Module, ModuleConfig, vcs};

use crate::configs::jujutsu_status::JjStatusConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::jujutsu::get_jujutsu_info;

/// Creates a module with the Jujutsu status in the current directory
///
/// Will display the current change ID, bookmarks, and status if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_status");
    let config: JjStatusConfig = JjStatusConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    }

    vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;

    let jujutsu_info = get_jujutsu_info(context)?;

    let bookmarks = if jujutsu_info.bookmarks.is_empty() {
        None
    } else {
        Some(
            jujutsu_info
                .bookmarks
                .iter()
                .map(|b| {
                    let mut s = b.name.clone();
                    let local_ahead = b.remote_behind;
                    let local_behind = b.remote_ahead;
                    let tracked_diverged = b.is_tracked && (local_ahead > 0 || local_behind > 0);
                    if tracked_diverged {
                        s.push('*');
                    }
                    if b.is_tracked {
                        if local_ahead > 0 && local_behind > 0 {
                            s.push_str(&format!(" ⇕⇡{}⇣{}", local_ahead, local_behind));
                        } else if local_ahead > 0 {
                            s.push_str(&format!(" ⇡{}", local_ahead));
                        } else if local_behind > 0 {
                            s.push_str(&format!(" ⇣{}", local_behind));
                        }
                    }
                    s
                })
                .collect::<Vec<String>>()
                .join(" "),
        )
    };

    let conflicted = if jujutsu_info.conflicted {
        Some(config.conflicted)
    } else {
        None
    };

    let divergent = if jujutsu_info.divergent {
        Some(config.divergent)
    } else {
        None
    };

    let hidden = if jujutsu_info.hidden {
        Some(config.hidden)
    } else {
        None
    };

    let immutable = if jujutsu_info.immutable {
        Some(config.immutable)
    } else {
        None
    };

    let bookmark_conflicted = if jujutsu_info.bookmark_conflicted {
        Some(config.bookmark_conflicted)
    } else {
        None
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                "conflicted_style" => Some(Ok(config.conflicted_style)),
                "divergent_style" => Some(Ok(config.divergent_style)),
                "hidden_style" => Some(Ok(config.hidden_style)),
                "immutable_style" => Some(Ok(config.immutable_style)),
                "bookmark_conflicted_style" => Some(Ok(config.bookmark_conflicted_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "change_id" => Some(Ok(jujutsu_info.change_id.as_str())),
                "bookmarks" => bookmarks.as_deref().map(Ok),
                "conflicted" => conflicted.map(Ok),
                "divergent" => divergent.map(Ok),
                "hidden" => hidden.map(Ok),
                "immutable" => immutable.map(Ok),
                "bookmark_conflicted" => bookmark_conflicted.map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jujutsu_status`:\n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;
    use std::io;
    use std::path::Path;

    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use crate::utils::CommandOutput;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("jujutsu_status")
            .path(repo_dir.path())
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_jujutsu_autodisabled() -> io::Result<()> {
        let tempdir = tempfile::tempdir()?;
        expect_jujutsu_disabled(tempdir.path(), None);
        tempdir.close()
    }

    #[test]
    fn test_jujutsu_status_simple() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_status")
            .path(repo_dir)
            .config(toml::toml! {
                [jujutsu_status]
                disabled = false
            })
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"kxmynpv","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"1234567","conflict":false,"divergent":false,"hidden":false,"immutable":false,"bookmark_conflict":false}"#,
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

        let expected = Some(format!("{} ", Color::Purple.paint("kxmynpv main")));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_status_conflicted() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_status")
            .path(repo_dir)
            .config(toml::toml! {
                [jujutsu_status]
                disabled = false
            })
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(r#"{"change_id":"kxmynpv","local_bookmarks":[],"tracked_remote_bookmarks":[],"commit_id":"1234567","conflict":true,"divergent":false,"hidden":false,"immutable":false,"bookmark_conflict":false}"#),
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

        let expected = Some("\u{1b}[35mkxmynpv\u{1b}[31m⚠ \u{1b}[0m ".to_string());
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_status_immutable() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_status")
            .path(repo_dir)
            .config(toml::toml! {
                [jujutsu_status]
                disabled = false
            })
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(r#"{"change_id":"kxmynpv","local_bookmarks":[],"tracked_remote_bookmarks":[],"commit_id":"1234567","conflict":false,"divergent":false,"hidden":false,"immutable":true,"bookmark_conflict":false}"#),
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

        let expected = Some("\u{1b}[35mkxmynpv\u{1b}[31m \u{1b}[0m ".to_string());
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_status_bookmark_conflicted() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_status")
            .path(repo_dir)
            .config(toml::toml! {
                [jujutsu_status]
                disabled = false
            })
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"kxmynpv","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"1234567","conflict":false,"divergent":false,"hidden":false,"immutable":false,"bookmark_conflict":true}"#,
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

        let expected = Some("\u{1b}[35mkxmynpv main\u{1b}[31m??\u{1b}[0m ".to_string());
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_status_divergent() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_status")
            .path(repo_dir)
            .config(toml::toml! {
                [jujutsu_status]
                disabled = false
            })
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(r#"{"change_id":"kxmynpv","local_bookmarks":[],"tracked_remote_bookmarks":[],"commit_id":"1234567","conflict":false,"divergent":true,"hidden":false,"immutable":false,"bookmark_conflict":false}"#),
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

        let expected = Some(format!("{} ", Color::Purple.paint("kxmynpv󰓁 ")));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_configured() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_status")
            .path(repo_dir)
            .config(toml::toml! {
                [jujutsu_status]
                style = "underline blue"
                symbol = "J "
                disabled = false
            })
            .cmd(
                crate::modules::utils::jujutsu::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{"change_id":"kxmynpv","local_bookmarks":["main"],"tracked_remote_bookmarks":[],"commit_id":"1234567","conflict":false,"divergent":false,"hidden":false,"immutable":false,"bookmark_conflict":false}"#,
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
            "{} ",
            Color::Blue.underline().paint("J kxmynpv main")
        ));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    fn expect_jujutsu_disabled(repo_dir: &Path, config: Option<toml::Table>) {
        let mut renderer = ModuleRenderer::new("jujutsu_status").path(repo_dir.to_str().unwrap());

        if let Some(config) = config {
            renderer = renderer.config(config);
        }

        let actual = renderer.collect();

        assert_eq!(None, actual);
    }
}
