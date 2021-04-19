use git2::{Repository, Status};

use super::{Context, Module, RootModuleConfig};

use crate::configs::git_status::GitStatusConfig;
use crate::context::Repo;
use crate::formatter::StringFormatter;
use crate::segment::Segment;
use std::sync::{Arc, RwLock};

const ALL_STATUS_FORMAT: &str = "$conflicted$stashed$deleted$renamed$modified$staged$untracked";

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
/// By default, the following symbols will be used to represent the repo's status:
///   - `=` – This branch has merge conflicts
///   - `⇡` – This branch is ahead of the branch being tracked
///   - `⇣` – This branch is behind of the branch being tracked
///   - `⇕` – This branch has diverged from the branch being tracked
///   - `?` — There are untracked files in the working directory
///   - `$` — A stash exists for the local repository
///   - `!` — There are file modifications in the working directory
///   - `+` — A new file has been added to the staging area
///   - `»` — A renamed file has been added to the staging area
///   - `✘` — A file's deletion has been added to the staging area
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let repo = context.get_repo().ok()?;
    let info = Arc::new(GitStatusInfo::load(repo));

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
                    "ahead_behind" => info.get_ahead_behind().and_then(|(ahead, behind)| {
                        if ahead > 0 && behind > 0 {
                            format_text(config.diverged, "git_status.diverged", |variable| {
                                match variable {
                                    "ahead_count" => Some(ahead.to_string()),
                                    "behind_count" => Some(behind.to_string()),
                                    _ => None,
                                }
                            })
                        } else if ahead > 0 && behind == 0 {
                            format_count(config.ahead, "git_status.ahead", ahead)
                        } else if behind > 0 && ahead == 0 {
                            format_count(config.behind, "git_status.behind", behind)
                        } else {
                            None
                        }
                    }),
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
    repo: &'a Repo,
    ahead_behind: RwLock<Option<Result<(usize, usize), git2::Error>>>,
    repo_status: RwLock<Option<Result<RepoStatus, git2::Error>>>,
    stashed_count: RwLock<Option<Result<usize, git2::Error>>>,
}

impl<'a> GitStatusInfo<'a> {
    pub fn load(repo: &'a Repo) -> Self {
        Self {
            repo,
            ahead_behind: RwLock::new(None),
            repo_status: RwLock::new(None),
            stashed_count: RwLock::new(None),
        }
    }

    fn get_branch_name(&self) -> String {
        self.repo
            .branch
            .clone()
            .unwrap_or_else(|| String::from("master"))
    }

    fn get_repository(&self) -> Option<Repository> {
        // bare repos don't have a branch name, so `repo.branch.as_ref` would return None,
        // but git treats "master" as the default branch name
        let repo_root = self.repo.root.as_ref()?;
        Repository::open(repo_root).ok()
    }

    pub fn get_ahead_behind(&self) -> Option<(usize, usize)> {
        {
            let data = self.ahead_behind.read().unwrap();
            if let Some(result) = data.as_ref() {
                return match result.as_ref() {
                    Ok(ahead_behind) => Some(*ahead_behind),
                    Err(error) => {
                        log::debug!("get_ahead_behind: {}", error);
                        None
                    }
                };
            };
        }

        {
            let mut data = self.ahead_behind.write().unwrap();
            let repo = self.get_repository()?;
            let branch_name = self.get_branch_name();
            *data = Some(get_ahead_behind(&repo, &branch_name));
            match data.as_ref().unwrap() {
                Ok(ahead_behind) => Some(*ahead_behind),
                Err(error) => {
                    log::debug!("get_ahead_behind: {}", error);
                    None
                }
            }
        }
    }

    pub fn get_repo_status(&self) -> Option<RepoStatus> {
        {
            let data = self.repo_status.read().unwrap();
            if let Some(result) = data.as_ref() {
                return match result.as_ref() {
                    Ok(repo_status) => Some(*repo_status),
                    Err(error) => {
                        log::debug!("get_repo_status: {}", error);
                        None
                    }
                };
            };
        }

        {
            let mut data = self.repo_status.write().unwrap();
            let mut repo = self.get_repository()?;
            *data = Some(get_repo_status(&mut repo));
            match data.as_ref().unwrap() {
                Ok(repo_status) => Some(*repo_status),
                Err(error) => {
                    log::debug!(" get_repo_status: {}", error);
                    None
                }
            }
        }
    }

    pub fn get_stashed(&self) -> Option<usize> {
        {
            let data = self.stashed_count.read().unwrap();
            if let Some(result) = data.as_ref() {
                return match result.as_ref() {
                    Ok(stashed_count) => Some(*stashed_count),
                    Err(error) => {
                        log::debug!("get_stashed_count: {}", error);
                        None
                    }
                };
            };
        }

        {
            let mut data = self.stashed_count.write().unwrap();
            let mut repo = self.get_repository()?;
            *data = Some(get_stashed_count(&mut repo));
            match data.as_ref().unwrap() {
                Ok(stashed_count) => Some(*stashed_count),
                Err(error) => {
                    log::debug!("get_stashed_count: {}", error);
                    None
                }
            }
        }
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
fn get_repo_status(repository: &mut Repository) -> Result<RepoStatus, git2::Error> {
    log::debug!("New repo status created");
    let mut status_options = git2::StatusOptions::new();

    let mut repo_status = RepoStatus::default();

    match repository.config()?.get_entry("status.showUntrackedFiles") {
        Ok(entry) => status_options.include_untracked(entry.value() != Some("no")),
        _ => status_options.include_untracked(true),
    };
    status_options
        .renames_from_rewrites(true)
        .renames_head_to_index(true)
        .include_unmodified(true);

    let statuses = repository.statuses(Some(&mut status_options))?;

    if statuses.is_empty() {
        return Err(git2::Error::from_str("Repo has no status"));
    }

    statuses
        .iter()
        .map(|s| s.status())
        .for_each(|status| repo_status.add(status));

    Ok(repo_status)
}

fn get_stashed_count(repository: &mut Repository) -> Result<usize, git2::Error> {
    let mut count = 0;
    repository.stash_foreach(|_, _, _| {
        count += 1;
        true
    })?;
    Result::Ok(count)
}

/// Compares the current branch with the branch it is tracking to determine how
/// far ahead or behind it is in relation
fn get_ahead_behind(
    repository: &Repository,
    branch_name: &str,
) -> Result<(usize, usize), git2::Error> {
    let branch_object = repository.revparse_single(branch_name)?;
    let tracking_branch_name = format!("{}@{{upstream}}", branch_name);
    let tracking_object = repository.revparse_single(&tracking_branch_name)?;

    let branch_oid = branch_object.id();
    let tracking_oid = tracking_object.id();

    repository.graph_ahead_behind(branch_oid, tracking_oid)
}

#[derive(Default, Debug, Copy, Clone)]
struct RepoStatus {
    conflicted: usize,
    deleted: usize,
    renamed: usize,
    modified: usize,
    staged: usize,
    untracked: usize,
}

impl RepoStatus {
    fn is_conflicted(status: Status) -> bool {
        status.is_conflicted()
    }

    fn is_deleted(status: Status) -> bool {
        status.is_wt_deleted() || status.is_index_deleted()
    }

    fn is_renamed(status: Status) -> bool {
        status.is_wt_renamed() || status.is_index_renamed()
    }

    fn is_modified(status: Status) -> bool {
        status.is_wt_modified()
    }

    fn is_staged(status: Status) -> bool {
        status.is_index_modified() || status.is_index_new()
    }

    fn is_untracked(status: Status) -> bool {
        status.is_wt_new()
    }

    fn add(&mut self, s: Status) {
        self.conflicted += RepoStatus::is_conflicted(s) as usize;
        self.deleted += RepoStatus::is_deleted(s) as usize;
        self.renamed += RepoStatus::is_renamed(s) as usize;
        self.modified += RepoStatus::is_modified(s) as usize;
        self.staged += RepoStatus::is_staged(s) as usize;
        self.untracked += RepoStatus::is_untracked(s) as usize;
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

#[cfg(test)]
mod tests {
    use ansi_term::{ANSIStrings, Color};
    use std::fs::{self, File};
    use std::io;
    use std::path::Path;
    use std::process::Command;

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};

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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        behind(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();
        let expected = format_output("⇣");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_behind_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        behind(&repo_dir.path())?;

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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        File::create(repo_dir.path().join("readme.md"))?.sync_all()?;
        ahead(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("⇡");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_ahead_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        File::create(repo_dir.path().join("readme.md"))?.sync_all()?;
        ahead(&repo_dir.path())?;

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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        diverge(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("⇕");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_diverged_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        diverge(&repo_dir.path())?;

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
    fn shows_conflicted() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_conflict(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("=");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_conflicted_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_conflict(&repo_dir.path())?;

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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_untracked(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("?");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_untracked_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_untracked(&repo_dir.path())?;

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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_untracked(&repo_dir.path())?;

        Command::new("git")
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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;
        barrier();

        create_stash(&repo_dir.path())?;

        Command::new("git")
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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;
        barrier();

        create_stash(&repo_dir.path())?;
        barrier();

        Command::new("git")
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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_modified(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("!");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_modified_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_modified(&repo_dir.path())?;

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
    fn shows_staged_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_staged(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("+");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_staged(&repo_dir.path())?;

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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_renamed(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("»");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_renamed_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_renamed(&repo_dir.path())?;

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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_deleted(&repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(&repo_dir.path())
            .collect();
        let expected = format_output("✘");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        create_deleted(&repo_dir.path())?;

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

    // Whenever a file is manually renamed, git itself ('git status') does not treat such file as renamed,
    // but as untracked instead. The following test checks if manually deleted and manually renamed
    // files are tracked by git_status module in the same way 'git status' does.
    #[test]
    #[ignore]
    fn ignore_manually_renamed() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;
        File::create(repo_dir.path().join("a"))?.sync_all()?;
        File::create(repo_dir.path().join("b"))?.sync_all()?;
        Command::new("git")
            .args(&["add", "--all"])
            .current_dir(&repo_dir.path())
            .output()?;
        Command::new("git")
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

        Command::new("git")
            .args(&["commit", "-am", "Update readme", "--no-gpg-sign"])
            .current_dir(&repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn behind(repo_dir: &Path) -> io::Result<()> {
        Command::new("git")
            .args(&["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn diverge(repo_dir: &Path) -> io::Result<()> {
        Command::new("git")
            .args(&["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        fs::write(repo_dir.join("Cargo.toml"), " ")?;

        Command::new("git")
            .args(&["commit", "-am", "Update readme", "--no-gpg-sign"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_conflict(repo_dir: &Path) -> io::Result<()> {
        Command::new("git")
            .args(&["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        fs::write(repo_dir.join("readme.md"), "# goodbye")?;

        Command::new("git")
            .args(&["add", "."])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Command::new("git")
            .args(&["commit", "-m", "Change readme", "--no-gpg-sign"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Command::new("git")
            .args(&["pull", "--rebase"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_stash(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("readme.md"))?.sync_all()?;
        barrier();

        Command::new("git")
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

    fn create_modified(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("readme.md"))?.sync_all()?;

        Ok(())
    }

    fn create_staged(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("license"))?.sync_all()?;

        Command::new("git")
            .args(&["add", "."])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Ok(())
    }

    fn create_renamed(repo_dir: &Path) -> io::Result<()> {
        Command::new("git")
            .args(&["mv", "readme.md", "readme.md.bak"])
            .current_dir(repo_dir)
            .output()?;
        barrier();

        Command::new("git")
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
