use super::{Context, Module, ModuleConfig};
use crate::configs::jj_status::JjStatusConfig;
use crate::formatter::StringFormatter;
use crate::segment::Segment;

const ALL_STATUS_FORMAT: &str = "$conflicted$modified$staged$renamed$deleted$untracked";

/// Creates a module with the Jujutsu repository status
///
/// Will display the status of the repository if the current directory is a Jujutsu repo
/// By default, the following symbols will be used to represent the repo's status:
///   - `=` – This repo has conflicted files
///   - `?` – There are untracked files in the working directory
///   - `!` – There are file modifications in the working directory
///   - `+` – A new file has been added (staged)
///   - `»` – A renamed file has been added
///   - `✘` – A file's deletion has been added
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_status");
    let config = JjStatusConfig::try_load(module.config);

    // Early return if disabled
    if config.disabled {
        return None;
    }

    // Detect if we're in a jj repository by looking for .jj directory
    context.begin_ancestor_scan().set_folders(&[".jj"]).scan()?;

    // Load status information
    let info = JjStatusInfo::load(context, &config);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "all_status" => Some(ALL_STATUS_FORMAT),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map_variables_to_segments(|variable| {
                let segments = match variable {
                    "conflicted" => info.get_conflicted().and_then(|count| {
                        format_count(config.conflicted, "jj_status.conflicted", context, count)
                    }),
                    "modified" => info.get_modified().and_then(|count| {
                        format_count(config.modified, "jj_status.modified", context, count)
                    }),
                    "staged" => info.get_staged().and_then(|count| {
                        format_count(config.staged, "jj_status.staged", context, count)
                    }),
                    "renamed" => info.get_renamed().and_then(|count| {
                        format_count(config.renamed, "jj_status.renamed", context, count)
                    }),
                    "deleted" => info.get_deleted().and_then(|count| {
                        format_count(config.deleted, "jj_status.deleted", context, count)
                    }),
                    "untracked" => info.get_untracked().and_then(|count| {
                        format_count(config.untracked, "jj_status.untracked", context, count)
                    }),
                    "ahead_behind" => info.get_ahead_behind().and_then(|(ahead, behind)| {
                        format_ahead_behind(&config, context, ahead, behind)
                    }),
                    _ => None,
                };
                segments.map(Ok)
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_status`: {error}");
            return None;
        }
    });

    Some(module)
}

/// Container for Jujutsu status information
struct JjStatusInfo {
    status: Option<JjStatus>,
    ahead_behind: Option<(usize, usize)>,
}

impl JjStatusInfo {
    pub fn load(context: &Context, config: &JjStatusConfig) -> Self {
        let status = get_jj_status(context);
        let ahead_behind = if config.ahead_behind {
            get_ahead_behind(context)
        } else {
            None
        };
        Self {
            status,
            ahead_behind,
        }
    }

    pub fn get_conflicted(&self) -> Option<usize> {
        self.status.as_ref().map(|s| s.conflicted)
    }

    pub fn get_modified(&self) -> Option<usize> {
        self.status.as_ref().map(|s| s.modified)
    }

    pub fn get_staged(&self) -> Option<usize> {
        self.status.as_ref().map(|s| s.staged)
    }

    pub fn get_renamed(&self) -> Option<usize> {
        self.status.as_ref().map(|s| s.renamed)
    }

    pub fn get_deleted(&self) -> Option<usize> {
        self.status.as_ref().map(|s| s.deleted)
    }

    pub fn get_untracked(&self) -> Option<usize> {
        self.status.as_ref().map(|s| s.untracked)
    }

    pub fn get_ahead_behind(&self) -> Option<(usize, usize)> {
        self.ahead_behind
    }
}

/// Parsed status information from jj status output
#[derive(Default, Debug)]
struct JjStatus {
    conflicted: usize,
    modified: usize,
    staged: usize,
    renamed: usize,
    deleted: usize,
    untracked: usize,
}

/// Get the status from jj
fn get_jj_status(context: &Context) -> Option<JjStatus> {
    let output = context
        .exec_cmd("jj", &["status", "--ignore-working-copy", "--color=never"])?
        .stdout;

    Some(parse_jj_status(&output))
}

/// Parse the output of `jj status` command
fn parse_jj_status(output: &str) -> JjStatus {
    let mut status = JjStatus::default();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Status lines start with a letter followed by a space
        // Example: "M modified_file.txt"
        if let Some(first_char) = line.chars().next() {
            match first_char {
                'M' => status.modified += 1,
                'A' => status.staged += 1,
                'D' => status.deleted += 1,
                'C' => status.conflicted += 1,
                'R' => status.renamed += 1,
                '?' => status.untracked += 1,
                _ => {}
            }
        }
    }

    status
}

/// Get ahead/behind counts by comparing local bookmark with remote
fn get_ahead_behind(context: &Context) -> Option<(usize, usize)> {
    // Get the current change's bookmarks and check if any have remotes
    // Using jj log to get bookmark information
    let output = context
        .exec_cmd(
            "jj",
            &[
                "log",
                "-r",
                "@",
                "--no-graph",
                "-T",
                r#"bookmarks.map(|b| b ++ "\n").join("")"#,
            ],
        )?
        .stdout;

    // If there are no bookmarks on current change, return None
    let bookmarks: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    if bookmarks.is_empty() {
        return None;
    }

    // For each bookmark, check if it has a remote tracking bookmark
    // This is a simplified implementation - in reality, jj's bookmark tracking
    // is more complex. For now, we'll use a heuristic approach.

    // Try to get ahead/behind using jj's revset language
    // Count commits between @ and the remote bookmark
    for bookmark in bookmarks {
        let bookmark = bookmark.trim();
        if bookmark.is_empty() {
            continue;
        }

        // Check if there's a remote version of this bookmark
        let remote_bookmark = format!("{}@origin", bookmark);

        // Count ahead (commits in @ but not in remote)
        let ahead_output = context.exec_cmd(
            "jj",
            &[
                "log",
                "-r",
                &format!("{}..@", remote_bookmark),
                "--no-graph",
                "-T",
                "commit_id",
            ],
        )?;

        let ahead = ahead_output
            .stdout
            .lines()
            .filter(|l| !l.trim().is_empty())
            .count();

        // Count behind (commits in remote but not in @)
        let behind_output = context.exec_cmd(
            "jj",
            &[
                "log",
                "-r",
                &format!("@..{}", remote_bookmark),
                "--no-graph",
                "-T",
                "commit_id",
            ],
        )?;

        let behind = behind_output
            .stdout
            .lines()
            .filter(|l| !l.trim().is_empty())
            .count();

        // If we found tracking information, return it
        if ahead > 0 || behind > 0 {
            return Some((ahead, behind));
        }
    }

    None
}

/// Format ahead/behind status
fn format_ahead_behind(
    config: &JjStatusConfig,
    context: &Context,
    ahead: usize,
    behind: usize,
) -> Option<Vec<Segment>> {
    if ahead > 0 && behind > 0 {
        format_text(
            config.diverged,
            "jj_status.diverged",
            context,
            |variable| match variable {
                "ahead_count" => Some(ahead.to_string()),
                "behind_count" => Some(behind.to_string()),
                _ => None,
            },
        )
    } else if ahead > 0 {
        format_count(config.ahead, "jj_status.ahead", context, ahead)
    } else if behind > 0 {
        format_count(config.behind, "jj_status.behind", context, behind)
    } else {
        None
    }
}

/// Format a text with variable substitution
fn format_text<F>(
    format_str: &str,
    config_path: &str,
    context: &Context,
    mapper: F,
) -> Option<Vec<Segment>>
where
    F: Fn(&str) -> Option<String> + Send + Sync,
{
    if let Ok(formatter) = StringFormatter::new(format_str) {
        formatter
            .map(|variable| mapper(variable).map(Ok))
            .parse(None, Some(context))
            .ok()
    } else {
        log::warn!("Error parsing format string `{}`", &config_path);
        None
    }
}

/// Format a count with the given format string
fn format_count(
    format_str: &str,
    config_path: &str,
    context: &Context,
    count: usize,
) -> Option<Vec<Segment>> {
    if count == 0 {
        return None;
    }

    format_text(
        format_str,
        config_path,
        context,
        |variable| match variable {
            "count" => Some(count.to_string()),
            _ => None,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::io;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn show_nothing_when_disabled() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = true
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_parse_jj_status() {
        let output = "Working copy changes:\nM modified.txt\nA added.txt\nD deleted.txt\n";
        let status = parse_jj_status(output);

        assert_eq!(status.modified, 1);
        assert_eq!(status.staged, 1);
        assert_eq!(status.deleted, 1);
        assert_eq!(status.conflicted, 0);
        assert_eq!(status.renamed, 0);
        assert_eq!(status.untracked, 0);
    }

    #[test]
    fn test_parse_jj_status_with_conflicts() {
        let output = "Working copy changes:\nC conflicted.txt\nM modified.txt\n";
        let status = parse_jj_status(output);

        assert_eq!(status.conflicted, 1);
        assert_eq!(status.modified, 1);
    }

    #[test]
    fn test_parse_multiple_files() {
        let output =
            "Working copy changes:\nM file1.txt\nM file2.txt\nA file3.txt\n? untracked.txt\n";
        let status = parse_jj_status(output);

        assert_eq!(status.modified, 2);
        assert_eq!(status.staged, 1);
        assert_eq!(status.untracked, 1);
    }

    fn format_output(symbols: &str) -> Option<String> {
        Some(format!(
            "{} ",
            Color::Red.bold().paint(format!("[{symbols}]"))
        ))
    }

    #[test]
    fn shows_modified() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nM modified.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("!");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_modified_with_count() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                modified = "!$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from(
                        "Working copy changes:\nM file1.txt\nM file2.txt\nM file3.txt\n",
                    ),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("!3");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_conflicted() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nC conflict.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("=");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_conflicted_with_count() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                conflicted = "=$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nC file1.txt\nC file2.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("=2");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_untracked_file() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\n? untracked.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("?");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_untracked_with_count() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                untracked = "?$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\n? file1.txt\n? file2.txt\n? file3.txt\n? file4.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("?4");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_file() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nA added.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("+");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_with_count() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                staged = "+$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nA file1.txt\nA file2.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("+2");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_renamed_file() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nR old.txt => new.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("»");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_renamed_with_count() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                renamed = "»$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nR old1.txt => new1.txt\nR old2.txt => new2.txt\nR old3.txt => new3.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("»3");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_file() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nD deleted.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("✘");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_with_count() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                deleted = "✘$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nD file1.txt\nD file2.txt\nD file3.txt\nD file4.txt\nD file5.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("✘5");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_mixed_status() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nC conflict.txt\nM modified.txt\nA added.txt\nD deleted.txt\n? untracked.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("=!+✘?");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_nothing_with_all_zero() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn custom_format() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                format = "(jj:$conflicted$modified )"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nC conflict.txt\nM modified.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(String::from("jj:=! "));
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn custom_symbols() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                conflicted = "X"
                modified = "M"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nC conflict.txt\nM modified.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("XM");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn custom_style() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                style = "bold green"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\nM modified.txt\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("[!]")));
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_conflicted_without_count() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
                conflicted = "="
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from(
                        "Working copy changes:\nC file1.txt\nC file2.txt\nC file3.txt\n",
                    ),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("=");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_ahead() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead = "⇡$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r @ --no-graph -T bookmarks.map(|b| b ++ \"\\n\").join(\"\")",
                Some(CommandOutput {
                    stdout: String::from("main\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r main@origin..@ --no-graph -T commit_id",
                Some(CommandOutput {
                    stdout: String::from("abc123\ndef456\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r @..main@origin --no-graph -T commit_id",
                Some(CommandOutput {
                    stdout: String::from(""),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("⇡2");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_behind() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                behind = "⇣$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r @ --no-graph -T bookmarks.map(|b| b ++ \"\\n\").join(\"\")",
                Some(CommandOutput {
                    stdout: String::from("main\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r main@origin..@ --no-graph -T commit_id",
                Some(CommandOutput {
                    stdout: String::from(""),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r @..main@origin --no-graph -T commit_id",
                Some(CommandOutput {
                    stdout: String::from("xyz789\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("⇣1");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_diverged() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                diverged = "⇕⇡$ahead_count⇣$behind_count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r @ --no-graph -T bookmarks.map(|b| b ++ \"\\n\").join(\"\")",
                Some(CommandOutput {
                    stdout: String::from("main\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r main@origin..@ --no-graph -T commit_id",
                Some(CommandOutput {
                    stdout: String::from("abc123\ndef456\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r @..main@origin --no-graph -T commit_id",
                Some(CommandOutput {
                    stdout: String::from("xyz789\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("⇕⇡2⇣1");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_ahead_behind_with_custom_symbols() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead = "↑$count"
                behind = "↓$count"
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r @ --no-graph -T bookmarks.map(|b| b ++ \"\\n\").join(\"\")",
                Some(CommandOutput {
                    stdout: String::from("main\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r main@origin..@ --no-graph -T commit_id",
                Some(CommandOutput {
                    stdout: String::from("abc123\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "jj log -r @..main@origin --no-graph -T commit_id",
                Some(CommandOutput {
                    stdout: String::from(""),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = format_output("↑1");
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn disabled_ahead_behind() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        std::fs::create_dir(repo_dir.path().join(".jj"))?;

        let actual = ModuleRenderer::new("jj_status")
            .path(repo_dir.path())
            .config(toml::toml! {
                [jj_status]
                disabled = false
                ahead_behind = false
            })
            .cmd(
                "jj status --ignore-working-copy --color=never",
                Some(CommandOutput {
                    stdout: String::from("Working copy changes:\n"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }
}
