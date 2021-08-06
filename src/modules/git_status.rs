use once_cell::sync::OnceCell;
use regex::Regex;

use super::{Context, Module, RootModuleConfig};

use crate::configs::git_status::GitStatusConfig;
use crate::formatter::StringFormatter;
use crate::segment::Segment;
use std::sync::Arc;

const ALL_STATUS_FORMAT: &str = "$conflicted$stashed$deleted$renamed$modified$staged$untracked";

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
/// By default, the following symbols will be used to represent the repo's status:
///   - `=` – This branch has merge conflicts
///   - `⇡` – This branch is ahead of the branch being tracked
///   - `⇣` – This branch is behind of the branch being tracked
///   - `⇕` – This branch has diverged from the branch being tracked
///   - `` – This branch is up-to-date with the branch being tracked
///   - `?` — There are untracked files in the working directory
///   - `$` — A stash exists for the local repository
///   - `!` — There are file modifications in the working directory
///   - `+` — A new file has been added to the staging area
///   - `»` — A renamed file has been added to the staging area
///   - `✘` — A file's deletion has been added to the staging area
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let info = Arc::new(GitStatusInfo::load(context));

    let mut module = context.new_module("git_status");
    let config: GitStatusConfig = GitStatusConfig::try_load(module.config);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "all_status" => Some(ALL_STATUS_FORMAT),
                _ => None,
            })
            .map_style(|variable: &str| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map_variables_to_segments(|variable: &str| {
                let info = Arc::clone(&info);
                let segments = match variable {
                    "stashed" => info.get_stashed().and_then(|count| {
                        format_count(config.stashed, "git_status.stashed", count)
                    }),
                    "ahead_behind" => {
                        info.get_ahead_behind()
                            .and_then(|(ahead, behind)| match (ahead?, behind?) {
                                (ahead, behind) => {
                                    if ahead > 0 && behind > 0 {
                                        format_text(
                                            config.diverged,
                                            "git_status.diverged",
                                            |variable| match variable {
                                                "ahead_count" => Some(ahead.to_string()),
                                                "behind_count" => Some(behind.to_string()),
                                                _ => None,
                                            },
                                        )
                                    } else if ahead > 0 && behind == 0 {
                                        format_count(config.ahead, "git_status.ahead", ahead)
                                    } else if behind > 0 && ahead == 0 {
                                        format_count(config.behind, "git_status.behind", behind)
                                    } else if ahead == 0 && behind == 0 {
                                        format_symbol(config.uptodate, "git_status.uptodate")
                                    } else {
                                        None
                                    }
                                }
                            })
                    }
                    "conflicted" => info.get_conflicted().and_then(|count| {
                        format_count(config.conflicted, "git_status.conflicted", count)
                    }),
                    "deleted" => info.get_deleted().and_then(|count| {
                        format_count(config.deleted, "git_status.deleted", count)
                    }),
                    "renamed" => info.get_renamed().and_then(|count| {
                        format_count(config.renamed, "git_status.renamed", count)
                    }),
                    "modified" => info.get_modified().and_then(|count| {
                        format_count(config.modified, "git_status.modified", count)
                    }),
                    "staged" => info
                        .get_staged()
                        .and_then(|count| format_count(config.staged, "git_status.staged", count)),
                    "untracked" => info.get_untracked().and_then(|count| {
                        format_count(config.untracked, "git_status.untracked", count)
                    }),
                    _ => None,
                };
                segments.map(Ok)
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => {
            if segments.is_empty() {
                return None;
            } else {
                segments
            }
        }
        Err(error) => {
            log::warn!("Error in module `git_status`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

struct GitStatusInfo<'a> {
    context: &'a Context<'a>,
    repo_status: OnceCell<Option<RepoStatus>>,
    stashed_count: OnceCell<Option<usize>>,
}

impl<'a> GitStatusInfo<'a> {
    pub fn load(context: &'a Context) -> Self {
        Self {
            context,
            repo_status: OnceCell::new(),
            stashed_count: OnceCell::new(),
        }
    }

    pub fn get_ahead_behind(&self) -> Option<(Option<usize>, Option<usize>)> {
        self.get_repo_status().map(|data| (data.ahead, data.behind))
    }

    pub fn get_repo_status(&self) -> &Option<RepoStatus> {
        self.repo_status
            .get_or_init(|| match get_repo_status(self.context) {
                Some(repo_status) => Some(repo_status),
                None => {
                    log::debug!("get_repo_status: git status execution failed");
                    None
                }
            })
    }

    pub fn get_stashed(&self) -> &Option<usize> {
        self.stashed_count
            .get_or_init(|| match get_stashed_count(self.context) {
                Some(stashed_count) => Some(stashed_count),
                None => {
                    log::debug!("get_stashed_count: git stash execution failed");
                    None
                }
            })
    }

    pub fn get_conflicted(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.conflicted)
    }

    pub fn get_deleted(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.deleted)
    }

    pub fn get_renamed(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.renamed)
    }

    pub fn get_modified(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.modified)
    }

    pub fn get_staged(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.staged)
    }

    pub fn get_untracked(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.untracked)
    }
}

/// Gets the number of files in various git states (staged, modified, deleted, etc...)
fn get_repo_status(context: &Context) -> Option<RepoStatus> {
    log::debug!("New repo status created");

    let mut repo_status = RepoStatus::default();
    let status_output = context.exec_cmd(
        "git",
        &[
            "-C",
            &context.current_dir.to_string_lossy(),
            "--no-optional-locks",
            "status",
            "--porcelain=2",
            "--branch",
        ],
    )?;
    let statuses = status_output.stdout.lines();

    statuses.for_each(|status| {
        if status.starts_with("# branch.ab ") {
            repo_status.set_ahead_behind(status);
        } else if !status.starts_with('#') {
            repo_status.add(status);
        }
    });

    Some(repo_status)
}

fn get_stashed_count(context: &Context) -> Option<usize> {
    let stash_output = context.exec_cmd(
        "git",
        &[
            "-C",
            &context.current_dir.to_string_lossy(),
            "--no-optional-locks",
            "stash",
            "list",
        ],
    )?;

    Some(stash_output.stdout.trim().lines().count())
}

#[derive(Default, Debug, Copy, Clone)]
struct RepoStatus {
    ahead: Option<usize>,
    behind: Option<usize>,
    conflicted: usize,
    deleted: usize,
    renamed: usize,
    modified: usize,
    staged: usize,
    untracked: usize,
}

impl RepoStatus {
    fn is_conflicted(status: &str) -> bool {
        status.starts_with("u ")
    }

    fn is_deleted(status: &str) -> bool {
        // is_wt_deleted || is_index_deleted
        status.starts_with("1 .D") || status.starts_with("1 D")
    }

    fn is_renamed(status: &str) -> bool {
        // is_wt_renamed || is_index_renamed
        // Potentially a copy and not a rename
        status.starts_with("2 ")
    }

    fn is_modified(status: &str) -> bool {
        // is_wt_modified
        status.starts_with("1 .M") || status.starts_with("1 .A")
    }

    fn is_staged(status: &str) -> bool {
        // is_index_modified || is_index_new
        status.starts_with("1 M") || status.starts_with("1 A")
    }

    fn is_untracked(status: &str) -> bool {
        // is_wt_new
        status.starts_with("? ")
    }

    fn add(&mut self, s: &str) {
        self.conflicted += Self::is_conflicted(s) as usize;
        self.deleted += Self::is_deleted(s) as usize;
        self.renamed += Self::is_renamed(s) as usize;
        self.modified += Self::is_modified(s) as usize;
        self.staged += Self::is_staged(s) as usize;
        self.untracked += Self::is_untracked(s) as usize;
    }

    fn set_ahead_behind(&mut self, s: &str) {
        let re = Regex::new(r"branch\.ab \+([0-9]+) \-([0-9]+)").unwrap();

        if let Some(caps) = re.captures(s) {
            self.ahead = caps.get(1).unwrap().as_str().parse::<usize>().ok();
            self.behind = caps.get(2).unwrap().as_str().parse::<usize>().ok();
        }
    }
}

fn format_text<F>(format_str: &str, config_path: &str, mapper: F) -> Option<Vec<Segment>>
where
    F: Fn(&str) -> Option<String> + Send + Sync,
{
    if let Ok(formatter) = StringFormatter::new(format_str) {
        formatter
            .map(|variable| mapper(variable).map(Ok))
            .parse(None)
            .ok()
    } else {
        log::warn!("Error parsing format string `{}`", &config_path);
        None
    }
}

fn format_count(format_str: &str, config_path: &str, count: usize) -> Option<Vec<Segment>> {
    if count == 0 {
        return None;
    }

    format_text(format_str, config_path, |variable| match variable {
        "count" => Some(count.to_string()),
        _ => None,
    })
}

fn format_symbol(format_str: &str, config_path: &str) -> Option<Vec<Segment>> {
    format_text(format_str, config_path, |_variable| None)
}

#[cfg(test)]
mod tests {
    use ansi_term::{ANSIStrings, Color};
    use std::fs::{self, File};
    use std::io;
    use std::path::Path;

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use crate::utils::create_command;

    /// Right after the calls to git the filesystem state may not have finished
    /// updating yet causing some of the tests to fail. These barriers are placed
    /// after each call to git.
    /// This barrier is windows-specific though other operating systems may need it
    /// in the future.
    #[cfg(not(windows))]
    fn barrier() {}
    #[cfg(windows)]
    fn barrier() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    #[allow(clippy::unnecessary_wraps)]
    fn format_output(symbols: &str) -> Option<String> {
        Some(format!(
            "{} ",
            Color::Red.bold().paint(format!("[{}]", symbols))
        ))
    }

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_behind() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        behind(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();
        let expected = format_output("⇣");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_behind_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        behind(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                behind = "⇣$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = format_output("⇣1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_ahead() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        File::create(repo_dir.path().join("readme.md"))?.sync_all()?;
        ahead(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("⇡");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_ahead_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        File::create(repo_dir.path().join("readme.md"))?.sync_all()?;
        ahead(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                ahead="⇡$count"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("⇡1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_diverged() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        diverge(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("⇕");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_diverged_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        diverge(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                diverged=r"⇕⇡$ahead_count⇣$behind_count"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("⇕⇡1⇣1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_up_to_date_with_upstream() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                uptodate="✓"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("✓");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_conflicted() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_conflict(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("=");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_conflicted_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_conflict(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                conflicted = "=$count"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("=1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_untracked_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_untracked(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("?");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_untracked_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_untracked(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                untracked = "?$count"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("?1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn doesnt_show_untracked_file_if_disabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_untracked(repo_dir.path())?;

        create_command("git")?
            .args(&["config", "status.showUntrackedFiles", "no"])
            .current_dir(repo_dir.path())
            .output()?;
        barrier();

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_stashed() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;
        barrier();

        create_stash(repo_dir.path())?;

        create_command("git")?
            .args(&["reset", "--hard", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?;
        barrier();

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("$");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_stashed_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;
        barrier();

        create_stash(repo_dir.path())?;
        barrier();

        create_command("git")?
            .args(&["reset", "--hard", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?;
        barrier();

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                stashed = r"\$$count"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("$1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_modified() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_modified(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("!");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_modified_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_modified(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                modified = "!$count"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("!1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_added() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_added(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("!");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_staged(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("+");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_staged(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                staged = "+[$count](green)"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = Some(format!(
            "{} ",
            ANSIStrings(&[
                Color::Red.bold().paint("[+"),
                Color::Green.paint("1"),
                Color::Red.bold().paint("]"),
            ])
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_renamed_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_renamed(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("»");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_renamed_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_renamed(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                renamed = "»$count"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("»1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_deleted(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("✘");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_deleted(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                deleted = "✘$count"
            })
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("✘1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn worktree_in_different_dir() -> io::Result<()> {
        let worktree_dir = tempfile::tempdir()?;
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(&[
                "config",
                "core.worktree",
                &worktree_dir.path().to_string_lossy(),
            ])
            .current_dir(repo_dir.path())
            .output()?;

        File::create(worktree_dir.path().join("test_file"))?.sync_all()?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("✘?");

        assert_eq!(expected, actual);
        worktree_dir.close()?;
        repo_dir.close()
    }

    // Whenever a file is manually renamed, git itself ('git status') does not treat such file as renamed,
    // but as untracked instead. The following test checks if manually deleted and manually renamed
    // files are tracked by git_status module in the same way 'git status' does.
    #[test]
    #[ignore]
    fn ignore_manually_renamed() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;
        File::create(repo_dir.path().join("a"))?.sync_all()?;
        File::create(repo_dir.path().join("b"))?.sync_all()?;
        create_command("git")?
            .args(&["add", "--all"])
            .current_dir(&repo_dir.path())
            .output()?;
        create_command("git")?
            .args(&["commit", "-m", "add new files", "--no-gpg-sign"])
            .current_dir(&repo_dir.path())
            .output()?;

        fs::remove_file(repo_dir.path().join("a"))?;
        fs::rename(repo_dir.path().join("b"), repo_dir.path().join("c"))?;
        barrier();

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .config(toml::toml! {
                [git_status]
                ahead = "A"
                deleted = "D"
                untracked = "U"
                renamed = "R"
            })
            .collect();
        let expected = format_output("DUA");

        assert_eq!(actual, expected);

        repo_dir.close()
    }

    fn ahead(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("readme.md"))?.sync_all()?;

        create_command("git")?
            .args(&["commit", "-am", "Update readme", "--no-gpg-sign"])
            .current_dir(&repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn behind(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(&["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn diverge(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(&["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        fs::write(repo_dir.join("Cargo.toml"), " ")?;

        create_command("git")?
            .args(&["commit", "-am", "Update readme", "--no-gpg-sign"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_conflict(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(&["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        fs::write(repo_dir.join("readme.md"), "# goodbye")?;

        create_command("git")?
            .args(&["add", "."])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        create_command("git")?
            .args(&["commit", "-m", "Change readme", "--no-gpg-sign"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        create_command("git")?
            .args(&["pull", "--rebase"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_stash(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("readme.md"))?.sync_all()?;
        barrier();

        create_command("git")?
            .args(&["stash", "--all"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_untracked(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("license"))?.sync_all()?;

        Ok(())
    }

    fn create_added(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("license"))?.sync_all()?;

        create_command("git")?
            .args(&["add", "-A", "-N"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_modified(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("readme.md"))?.sync_all()?;

        Ok(())
    }

    fn create_staged(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("license"))?.sync_all()?;

        create_command("git")?
            .args(&["add", "."])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_renamed(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(&["mv", "readme.md", "readme.md.bak"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        create_command("git")?
            .args(&["add", "-A"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_deleted(repo_dir: &Path) -> io::Result<()> {
        fs::remove_file(repo_dir.join("readme.md"))?;

        Ok(())
    }
}
