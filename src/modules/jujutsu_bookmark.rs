use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu_bookmark::JujutsuBookmarkConfig;
use crate::formatter::StringFormatter;
use crate::modules::vcs;

/// Creates a module with the Jujutsu bookmarks in the current directory
///
/// Will display bookmarks pointing to the current change if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_bookmark");
    let config: JujutsuBookmarkConfig = JujutsuBookmarkConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    }

    // Only run in jj repositories
    vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;

    let jujutsu_info = crate::modules::jujutsu_status::get_jujutsu_info(context)?;

    let bookmarks = if jujutsu_info.bookmarks.is_empty() {
        None
    } else {
        Some(
            jujutsu_info
                .bookmarks
                .iter()
                .map(|bookmark| {
                    let mut name = bookmark.name.clone();
                    let local_ahead = bookmark.remote_behind;
                    let local_behind = bookmark.remote_ahead;
                    let tracked_diverged =
                        bookmark.is_tracked && (local_ahead > 0 || local_behind > 0);
                    if tracked_diverged {
                        name.push('*');
                    }
                    if bookmark.is_tracked {
                        if local_ahead > 0 && local_behind > 0 {
                            name.push_str(&format!(" ⇕⇡{}⇣{}", local_ahead, local_behind));
                        } else if local_ahead > 0 {
                            name.push_str(&format!(" ⇡{}", local_ahead));
                        } else if local_behind > 0 {
                            name.push_str(&format!(" ⇣{}", local_behind));
                        }
                    }
                    name
                })
                .collect::<Vec<_>>(),
        )
    };

    bookmarks.as_ref()?;

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
                "bookmarks" => Some(Ok(bookmarks
                    .as_ref()?
                    .iter()
                    .map(|bookmark| {
                        if jujutsu_info.bookmark_conflicted {
                            format!("{}{}", bookmark, config.bookmark_conflicted)
                        } else {
                            bookmark.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" "))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jujutsu_bookmark`:\n{}", error);
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

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .path(repo_dir.path())
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_jujutsu_bookmark_disabled_by_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .path(repo_dir)
            .collect();

        let expected = None; // Should be None because disabled by default
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_bookmark_single_bookmark() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .config(toml::toml! {
                [jujutsu_bookmark]
                disabled = false
                format = "[$symbol$bookmarks]($style) "
            })
            .path(repo_dir)
            .cmd(
                crate::modules::jujutsu_status::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "abcdefg\nmain\n\nabcdefg\nfalse\nfalse\nfalse\nfalse\nfalse",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::jujutsu_status::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("{} ", Color::Purple.paint("\u{f045f} main")));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_bookmark_multiple_bookmarks() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .config(toml::toml! {
                [jujutsu_bookmark]
                disabled = false
                format = "[$symbol$bookmarks]($style) "
            })
            .path(repo_dir)
            .cmd(
                crate::modules::jujutsu_status::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "abcdefg\nmain\x1efeature\x1ebugfix\n\nabcdefg\nfalse\nfalse\nfalse\nfalse\nfalse",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::jujutsu_status::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Purple.paint("\u{f045f} main feature bugfix")
        ));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_bookmark_with_truncation() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .config(toml::toml! {
                [jujutsu_bookmark]
                disabled = false
                format = "[$symbol$bookmarks]($style) "
            })
            .path(repo_dir)
            .cmd(
                crate::modules::jujutsu_status::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "abcdefg\nmain\x1every-long-bookmark-name\x1efeature\n\nabcdefg\nfalse\nfalse\nfalse\nfalse\nfalse",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::jujutsu_status::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Purple.paint("\u{f045f} main very-long-bookmark-name feature")
        ));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_bookmark_no_bookmarks() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .config(toml::toml! {
                [jujutsu_bookmark]
                disabled = false
                format = "[$symbol$bookmarks]($style) "
            })
            .path(repo_dir)
            .cmd(
                crate::modules::jujutsu_status::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from("abcdefg\n\n\nabcdefg\nfalse\nfalse\nfalse\nfalse\nfalse"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::jujutsu_status::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = None; // Should be None when no bookmarks
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_bookmark_ahead_behind() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .config(toml::toml! {
                [jujutsu_bookmark]
                disabled = false
                format = "[$symbol$bookmarks]($style) "
            })
            .path(repo_dir)
            .cmd(
                crate::modules::jujutsu_status::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "abcdefg\nmain\n\nabcdefg\nfalse\nfalse\nfalse\nfalse\nfalse",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::jujutsu_status::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::from("main\x1f5\x1f2\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("{} ", Color::Purple.paint("\u{f045f} main* ⇕⇡2⇣5")));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_bookmark_conflicted() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .config(toml::toml! {
                [jujutsu_bookmark]
                disabled = false
                format = "[$symbol$bookmarks]($style) "
            })
            .path(repo_dir)
            .cmd(
                crate::modules::jujutsu_status::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "abcdefg\nmain\n\nabcdefg\nfalse\nfalse\nfalse\nfalse\ntrue\nfalse",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::jujutsu_status::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("{} ", Color::Purple.paint("\u{f045f} main??")));
        assert_eq!(expected, actual);

        tempdir.close()
    }

    #[test]
    fn test_jujutsu_bookmark_tracked_in_sync() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Jujutsu)?;
        let repo_dir = tempdir.path();

        let actual = ModuleRenderer::new("jujutsu_bookmark")
            .config(toml::toml! {
                [jujutsu_bookmark]
                disabled = false
                format = "[$symbol$bookmarks]($style) "
            })
            .path(repo_dir)
            .cmd(
                crate::modules::jujutsu_status::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "abcdefg\nmain\n\nabcdefg\nfalse\nfalse\nfalse\nfalse\nfalse",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                crate::modules::jujutsu_status::jujutsu_tracked_bookmarks_command(),
                Some(CommandOutput {
                    stdout: String::from("main\x1f0\x1f0\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("{} ", Color::Purple.paint("\u{f045f} main")));
        assert_eq!(expected, actual);

        tempdir.close()
    }
}
