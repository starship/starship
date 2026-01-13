use super::{Context, Module, ModuleConfig, vcs};

use crate::configs::jujutsu_status::JjStatusConfig;
use crate::formatter::StringFormatter;

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

#[derive(Debug, Clone)]
pub(crate) struct JjRepoInfo {
    pub change_id: String,
    pub commit_id: String,
    pub bookmarks: Vec<BookmarkInfo>,
    pub conflicted: bool,
    pub divergent: bool,
    pub hidden: bool,
    pub immutable: bool,
    pub bookmark_conflicted: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct BookmarkInfo {
    pub name: String,
    pub remote_ahead: usize,
    pub remote_behind: usize,
    pub is_tracked: bool,
}

const TRACKED_BOOKMARKS_TEMPLATE: &str = r#"if(remote, name ++ "\x1f" ++ tracking_ahead_count.lower() ++ "\x1f" ++ tracking_behind_count.lower() ++ "\n", "")"#;

fn jujutsu_log_template(change_id_length: usize, commit_hash_length: usize) -> String {
    format!(
        r#"change_id.short({}) ++ "\n"
        ++ local_bookmarks.map(|b| b.name()).join("\x1e") ++ "\n"
        ++ remote_bookmarks.filter(|b| b.tracked()).map(|b| b.name() ++ "\x1f" ++ b.tracking_ahead_count().lower() ++ "\x1f" ++ b.tracking_behind_count().lower()).join("\x1e") ++ "\n"
        ++ commit_id.short({}) ++ "\n"
        ++ conflict ++ "\n"
        ++ divergent ++ "\n"
        ++ hidden ++ "\n"
        ++ immutable ++ "\n"
        ++ (local_bookmarks.any(|b| b.conflict()) || remote_bookmarks.any(|b| b.conflict()))"#,
        change_id_length, commit_hash_length
    )
}

fn parse_tracked_bookmarks(output: &str) -> std::collections::HashMap<String, (usize, usize)> {
    output
        .split(|c| c == '\n' || c == '\x1e')
        .filter(|entry| !entry.is_empty())
        .filter_map(|entry| {
            let mut parts = entry.split('\x1f');
            let name = parts.next()?.to_string();
            let ahead = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let behind = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            Some((name, ahead, behind))
        })
        .fold(
            std::collections::HashMap::new(),
            |mut map, (name, ahead, behind)| {
                map.entry(name)
                    .and_modify(|counts: &mut (usize, usize)| {
                        counts.0 = counts.0.max(ahead);
                        counts.1 = counts.1.max(behind);
                    })
                    .or_insert((ahead, behind));
                map
            },
        )
}

fn merge_tracked_bookmarks(
    target: &mut std::collections::HashMap<String, (usize, usize)>,
    incoming: std::collections::HashMap<String, (usize, usize)>,
) {
    for (name, (ahead, behind)) in incoming {
        target
            .entry(name)
            .and_modify(|counts| {
                counts.0 = counts.0.max(ahead);
                counts.1 = counts.1.max(behind);
            })
            .or_insert((ahead, behind));
    }
}

pub(crate) fn get_jujutsu_info(ctx: &Context) -> Option<JjRepoInfo> {
    // Only run in jj repositories
    vcs::discover_repo_root(ctx, vcs::Vcs::Jujutsu)?;

    let change_id_length = crate::configs::jujutsu_change::JujutsuChangeConfig::try_load(
        ctx.config.get_module_config("jujutsu_change"),
    )
    .change_id_length;
    let commit_hash_length = crate::configs::jujutsu_commit::JujutsuCommitConfig::try_load(
        ctx.config.get_module_config("jujutsu_commit"),
    )
    .commit_hash_length;

    // Use a single jj log command with a template to get all needed information
    // We use --ignore-working-copy to avoid automatic snapshotting
    let output = ctx
        .exec_cmd(
            "jj",
            &[
                "log",
                "--no-graph",
                "-r",
                "@",
                "--ignore-working-copy",
                "-T",
                &jujutsu_log_template(change_id_length, commit_hash_length),
            ],
        )?
        .stdout;

    let mut lines = output.lines();

    let change_id = lines.next()?.to_string();
    let bookmarks_str = lines.next().unwrap_or("");
    let tracked_bookmarks_str = lines.next().unwrap_or("");
    let mut tracked_bookmarks = parse_tracked_bookmarks(tracked_bookmarks_str);
    let tracked_bookmarks_output = ctx
        .exec_cmd(
            "jj",
            &[
                "bookmark",
                "list",
                "--tracked",
                "--ignore-working-copy",
                "-T",
                TRACKED_BOOKMARKS_TEMPLATE,
            ],
        )
        .map(|output| output.stdout)
        .unwrap_or_default();
    merge_tracked_bookmarks(
        &mut tracked_bookmarks,
        parse_tracked_bookmarks(&tracked_bookmarks_output),
    );
    let bookmarks = if bookmarks_str.is_empty() {
        Vec::new()
    } else {
        bookmarks_str
            .split('\x1e')
            .filter(|entry| !entry.is_empty())
            .map(|name| {
                let (ahead, behind, is_tracked) = tracked_bookmarks
                    .get(name)
                    .map(|(ahead, behind)| (*ahead, *behind, true))
                    .unwrap_or((0, 0, false));
                BookmarkInfo {
                    name: name.to_string(),
                    remote_ahead: ahead,
                    remote_behind: behind,
                    is_tracked,
                }
            })
            .collect()
    };
    let commit_id = lines.next().unwrap_or("").to_string();
    let conflicted = lines.next().unwrap_or("false") == "true";
    let divergent = lines.next().unwrap_or("false") == "true";
    let hidden = lines.next().unwrap_or("false") == "true";
    let immutable = lines.next().unwrap_or("false") == "true";
    let bookmark_conflicted = lines.next().unwrap_or("false") == "true";

    Some(JjRepoInfo {
        change_id,
        commit_id,
        bookmarks,
        conflicted,
        divergent,
        hidden,
        immutable,
        bookmark_conflicted,
    })
}

#[cfg(test)]
pub(crate) fn jujutsu_log_command(
    change_id_length: usize,
    commit_hash_length: usize,
) -> &'static str {
    Box::leak(
        format!(
            "jj log --no-graph -r @ --ignore-working-copy -T {}",
            jujutsu_log_template(change_id_length, commit_hash_length)
        )
        .into_boxed_str(),
    )
}

#[cfg(test)]
pub(crate) fn jujutsu_tracked_bookmarks_command() -> &'static str {
    Box::leak(
        format!(
            "jj bookmark list --tracked --ignore-working-copy -T {}",
            TRACKED_BOOKMARKS_TEMPLATE
        )
        .into_boxed_str(),
    )
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
                super::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "kxmynpv\nmain\n\n1234567\nfalse\nfalse\nfalse\nfalse\nfalse",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                super::jujutsu_tracked_bookmarks_command(),
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
                super::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from("kxmynpv\n\n\n1234567\ntrue\nfalse\nfalse\nfalse\nfalse"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                super::jujutsu_tracked_bookmarks_command(),
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
                super::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from("kxmynpv\n\n\n1234567\nfalse\nfalse\nfalse\ntrue\nfalse"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                super::jujutsu_tracked_bookmarks_command(),
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
                super::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "kxmynpv\nmain\n\n1234567\nfalse\nfalse\nfalse\nfalse\ntrue",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                super::jujutsu_tracked_bookmarks_command(),
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
                super::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from("kxmynpv\n\n\n1234567\nfalse\ntrue\nfalse\nfalse\nfalse"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                super::jujutsu_tracked_bookmarks_command(),
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
                super::jujutsu_log_command(7, 7),
                Some(CommandOutput {
                    stdout: String::from(
                        "kxmynpv\nmain\n\n1234567\nfalse\nfalse\nfalse\nfalse\nfalse",
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                super::jujutsu_tracked_bookmarks_command(),
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
