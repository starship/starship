use regex::Regex;
use std::sync::OnceLock;

use super::{Context, Module, ModuleConfig};

use crate::configs::git_status::GitStatusConfig;
use crate::context;
use crate::formatter::StringFormatter;
use crate::segment::Segment;
use std::sync::Arc;

const ALL_STATUS_FORMAT: &str =
    "$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked";

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
    let mut module = context.new_module("git_status");
    let config: GitStatusConfig = GitStatusConfig::try_load(module.config);

    // Return None if not in git repository
    let repo = context.get_repo().ok()?;

    if repo.kind.is_bare() {
        log::debug!("This is a bare repository, git_status is not applicable");
        return None;
    }

    if let Some(git_status) = git_status_wsl(context, &config) {
        if git_status.is_empty() {
            return None;
        }
        module.set_segments(Segment::from_text(None, git_status));
        return Some(module);
    }

    let info = Arc::new(GitStatusInfo::load(context, repo, config.clone()));

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
                        format_count(config.stashed, "git_status.stashed", context, count)
                    }),
                    "ahead_behind" => info.get_ahead_behind().and_then(|(ahead, behind)| {
                        let (ahead, behind) = (ahead?, behind?);
                        if ahead > 0 && behind > 0 {
                            format_text(
                                config.diverged,
                                "git_status.diverged",
                                context,
                                |variable| match variable {
                                    "ahead_count" => Some(ahead.to_string()),
                                    "behind_count" => Some(behind.to_string()),
                                    _ => None,
                                },
                            )
                        } else if ahead > 0 && behind == 0 {
                            format_count(config.ahead, "git_status.ahead", context, ahead)
                        } else if behind > 0 && ahead == 0 {
                            format_count(config.behind, "git_status.behind", context, behind)
                        } else {
                            format_symbol(config.up_to_date, "git_status.up_to_date", context)
                        }
                    }),
                    "conflicted" => info.get_conflicted().and_then(|count| {
                        format_count(config.conflicted, "git_status.conflicted", context, count)
                    }),
                    "deleted" => info.get_deleted().and_then(|count| {
                        format_count(config.deleted, "git_status.deleted", context, count)
                    }),
                    "renamed" => info.get_renamed().and_then(|count| {
                        format_count(config.renamed, "git_status.renamed", context, count)
                    }),
                    "modified" => info.get_modified().and_then(|count| {
                        format_count(config.modified, "git_status.modified", context, count)
                    }),
                    "staged" => info.get_staged().and_then(|count| {
                        format_count(config.staged, "git_status.staged", context, count)
                    }),
                    "untracked" => info.get_untracked().and_then(|count| {
                        format_count(config.untracked, "git_status.untracked", context, count)
                    }),
                    "typechanged" => info.get_typechanged().and_then(|count| {
                        format_count(config.typechanged, "git_status.typechanged", context, count)
                    }),
                    "worktree_added" => info.get_worktree_added().and_then(|count| {
                        format_count(
                            config.worktree_added,
                            "git_status.worktree_added",
                            context,
                            count,
                        )
                    }),
                    "worktree_deleted" => info.get_worktree_deleted().and_then(|count| {
                        format_count(
                            config.worktree_deleted,
                            "git_status.worktree_deleted",
                            context,
                            count,
                        )
                    }),
                    "worktree_modified" => info.get_worktree_modified().and_then(|count| {
                        format_count(
                            config.worktree_modified,
                            "git_status.worktree_modified",
                            context,
                            count,
                        )
                    }),
                    "worktree_typechanged" => info.get_worktree_typechanged().and_then(|count| {
                        format_count(
                            config.worktree_typechanged,
                            "git_status.worktree_typechanged",
                            context,
                            count,
                        )
                    }),
                    "index_added" => info.get_index_added().and_then(|count| {
                        format_count(config.index_added, "git_status.index_added", context, count)
                    }),
                    "index_deleted" => info.get_index_deleted().and_then(|count| {
                        format_count(
                            config.index_deleted,
                            "git_status.index_deleted",
                            context,
                            count,
                        )
                    }),
                    "index_modified" => info.get_index_modified().and_then(|count| {
                        format_count(
                            config.index_modified,
                            "git_status.index_modified",
                            context,
                            count,
                        )
                    }),
                    "index_typechanged" => info.get_index_typechanged().and_then(|count| {
                        format_count(
                            config.index_typechanged,
                            "git_status.index_typechanged",
                            context,
                            count,
                        )
                    }),
                    _ => None,
                };
                segments.map(Ok)
            })
            .parse(None, Some(context))
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
    repo: &'a context::Repo,
    config: GitStatusConfig<'a>,
    repo_status: OnceLock<Option<RepoStatus>>,
    stashed_count: OnceLock<Option<usize>>,
}

impl<'a> GitStatusInfo<'a> {
    pub fn load(
        context: &'a Context,
        repo: &'a context::Repo,
        config: GitStatusConfig<'a>,
    ) -> Self {
        Self {
            context,
            repo,
            config,
            repo_status: OnceLock::new(),
            stashed_count: OnceLock::new(),
        }
    }

    pub fn get_ahead_behind(&self) -> Option<(Option<usize>, Option<usize>)> {
        self.get_repo_status().map(|data| (data.ahead, data.behind))
    }

    pub fn get_repo_status(&self) -> &Option<RepoStatus> {
        self.repo_status.get_or_init(|| {
            match get_repo_status(self.context, self.repo, &self.config) {
                Some(repo_status) => Some(repo_status),
                None => {
                    log::debug!("get_repo_status: git status execution failed");
                    None
                }
            }
        })
    }

    pub fn get_stashed(&self) -> &Option<usize> {
        self.stashed_count
            .get_or_init(|| match get_stashed_count(self.repo) {
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

    pub fn get_typechanged(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.typechanged)
    }

    pub fn get_worktree_added(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.worktree_added)
    }

    pub fn get_worktree_deleted(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.worktree_deleted)
    }

    pub fn get_worktree_modified(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.worktree_modified)
    }

    pub fn get_worktree_typechanged(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.worktree_typechanged)
    }

    pub fn get_index_added(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.index_added)
    }

    pub fn get_index_deleted(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.index_deleted)
    }

    pub fn get_index_modified(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.index_modified)
    }

    pub fn get_index_typechanged(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.index_typechanged)
    }
}

/// Gets the number of files in various git states (staged, modified, deleted, etc...)
fn get_repo_status(
    context: &Context,
    repo: &context::Repo,
    config: &GitStatusConfig,
) -> Option<RepoStatus> {
    log::debug!("New repo status created");

    let mut repo_status = RepoStatus::default();
    let mut args = vec!["status", "--porcelain=2"];

    // for performance reasons, only pass flags if necessary...
    let has_ahead_behind = !config.ahead.is_empty() || !config.behind.is_empty();
    let has_up_to_date_diverged = !config.up_to_date.is_empty() || !config.diverged.is_empty();
    if has_ahead_behind || has_up_to_date_diverged {
        args.push("--branch");
    }

    // ... and add flags that omit information the user doesn't want
    let has_untracked = !config.untracked.is_empty();
    if !has_untracked {
        args.push("--untracked-files=no");
    }
    if config.ignore_submodules {
        args.push("--ignore-submodules=dirty");
    } else if !has_untracked {
        args.push("--ignore-submodules=untracked");
    }

    let status_output = repo.exec_git(context, &args)?;
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

fn get_stashed_count(repo: &context::Repo) -> Option<usize> {
    let repo = repo.open();
    let reference = match repo.try_find_reference("refs/stash") {
        // Only proceed if the found reference has the expected name (not tags/refs/stash etc.)
        Ok(Some(reference)) if reference.name().as_bstr() == b"refs/stash".as_slice() => reference,
        // No stash reference found
        Ok(_) => return Some(0),
        Err(err) => {
            log::debug!("Error finding stash reference: {err}");
            return None;
        }
    };

    match reference.log_iter().all() {
        Ok(Some(log)) => Some(log.count()),
        Ok(None) => {
            log::debug!("No reflog found for stash");
            Some(0)
        }
        Err(err) => {
            log::debug!("Error getting stash log: {err}");
            None
        }
    }
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
    typechanged: usize,
    untracked: usize,
    worktree_added: usize,
    worktree_deleted: usize,
    worktree_modified: usize,
    worktree_typechanged: usize,
    index_added: usize,
    index_deleted: usize,
    index_modified: usize,
    index_typechanged: usize,
}

impl RepoStatus {
    fn is_index_typechanged(short_status: &str) -> bool {
        short_status.starts_with('T')
    }

    fn is_worktree_typechanged(short_status: &str) -> bool {
        short_status.ends_with('T')
    }

    fn is_index_deleted(short_status: &str) -> bool {
        short_status.starts_with('D')
    }

    fn is_worktree_deleted(short_status: &str) -> bool {
        short_status.ends_with('D')
    }

    fn is_index_added(short_status: &str) -> bool {
        short_status.starts_with('A')
    }

    fn is_worktree_added(short_status: &str) -> bool {
        short_status.ends_with('A')
    }

    fn is_index_modified(short_status: &str) -> bool {
        short_status.starts_with('M')
    }

    fn is_worktree_modified(short_status: &str) -> bool {
        short_status.ends_with('M')
    }

    fn parse_normal_status(&mut self, short_status: &str) {
        if Self::is_worktree_added(short_status) {
            self.worktree_added += 1;
            self.modified += 1; // is_wt_modified || is_wt_added
        }

        if Self::is_worktree_deleted(short_status) {
            self.worktree_deleted += 1;
            self.deleted += 1; // is_wt_deleted || is_index_deleted
        }

        if Self::is_worktree_modified(short_status) {
            self.worktree_modified += 1;
            self.modified += 1; // is_wt_modified || is_wt_added
        }

        if Self::is_worktree_typechanged(short_status) {
            self.worktree_typechanged += 1;
            self.typechanged += 1;
        }

        if Self::is_index_added(short_status) {
            self.index_added += 1;
            self.staged += 1; // is_index_modified || is_index_added || is_index_typechanged
        }

        if Self::is_index_deleted(short_status) {
            self.index_deleted += 1;
            self.deleted += 1; // is_wt_deleted || is_index_deleted
        }

        if Self::is_index_modified(short_status) {
            self.index_modified += 1;
            self.staged += 1; // is_index_modified || is_index_added || is_index_typechanged
        }

        if Self::is_index_typechanged(short_status) {
            self.index_typechanged += 1;
            self.staged += 1; // is_index_modified || is_index_added || is_index_typechanged
        }
    }

    fn add(&mut self, s: &str) {
        match s.chars().next() {
            Some('1') => self.parse_normal_status(&s[2..4]),
            Some('2') => {
                self.renamed += 1;
                self.parse_normal_status(&s[2..4])
            }
            Some('u') => self.conflicted += 1,
            Some('?') => self.untracked += 1,
            Some('!') => (),
            Some(_) => log::error!("Unknown line type in git status output"),
            None => log::error!("Missing line type in git status output"),
        }
    }

    fn set_ahead_behind(&mut self, s: &str) {
        let re = Regex::new(r"branch\.ab \+([0-9]+) \-([0-9]+)").unwrap();

        if let Some(caps) = re.captures(s) {
            self.ahead = caps.get(1).unwrap().as_str().parse::<usize>().ok();
            self.behind = caps.get(2).unwrap().as_str().parse::<usize>().ok();
        }
    }
}

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

fn format_symbol(format_str: &str, config_path: &str, context: &Context) -> Option<Vec<Segment>> {
    format_text(format_str, config_path, context, |_variable| None)
}

#[cfg(target_os = "linux")]
fn git_status_wsl(context: &Context, conf: &GitStatusConfig) -> Option<String> {
    use crate::utils::create_command;
    use nix::sys::utsname::uname;
    use std::env;
    use std::ffi::OsString;
    use std::io::ErrorKind;

    let starship_exe = conf.windows_starship?;

    // Ensure this is WSL
    // This is lowercase in WSL1 and uppercase in WSL2, just skip the first letter
    if !uname()
        .ok()?
        .release()
        .to_string_lossy()
        .contains("icrosoft")
    {
        return None;
    }

    log::trace!("Using WSL mode");

    // Get Windows path
    let wslpath = create_command("wslpath")
        .map(|mut c| {
            c.arg("-w").arg(&context.current_dir);
            c
        })
        .and_then(|mut c| c.output());
    let winpath = match wslpath {
        Ok(r) => r,
        Err(e) => {
            // Not found might means this might not be WSL after all
            let level = if e.kind() == ErrorKind::NotFound {
                log::Level::Debug
            } else {
                log::Level::Error
            };

            log::log!(level, "Failed to get Windows path:\n{:?}", e);

            return None;
        }
    };

    let winpath = match std::str::from_utf8(&winpath.stdout) {
        Ok(r) => r.trim_end(),
        Err(e) => {
            log::error!("Failed to parse Windows path:\n{:?}", e);

            return None;
        }
    };

    log::trace!("Windows path: {}", winpath);

    // In Windows or Linux dir?
    if winpath.starts_with(r"\\wsl") {
        log::trace!("Not a Windows path");
        return None;
    }

    // Get foreign starship to use WSL config
    // https://devblogs.microsoft.com/commandline/share-environment-vars-between-wsl-and-windows/
    let wslenv = env::var("WSLENV").map_or_else(
        |_| "STARSHIP_CONFIG/wp".to_string(),
        |e| e + ":STARSHIP_CONFIG/wp",
    );

    let exe = create_command(starship_exe)
        .map(|mut c| {
            c.env(
                "STARSHIP_CONFIG",
                context
                    .get_config_path_os()
                    .unwrap_or_else(|| OsString::from("/dev/null")),
            )
            .env("WSLENV", wslenv)
            .args(["module", "git_status", "--path", winpath]);
            c
        })
        .and_then(|mut c| c.output());

    let out = match exe {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to run Git Status module on Windows:\n{}", e);

            return None;
        }
    };

    match String::from_utf8(out.stdout) {
        Ok(r) => Some(r),
        Err(e) => {
            log::error!(
                "Failed to parse Windows Git Status module status output:\n{}",
                e
            );

            None
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn git_status_wsl(_context: &Context, _conf: &GitStatusConfig) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::{AnsiStrings, Color};
    use std::ffi::OsStr;
    use std::fs::{self, File};
    use std::io::{self, prelude::*};
    use std::path::Path;

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use crate::utils::create_command;

    #[allow(clippy::unnecessary_wraps)]
    fn format_output(symbols: &str) -> Option<String> {
        Some(format!(
            "{} ",
            Color::Red.bold().paint(format!("[{symbols}]"))
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
            .path(repo_dir.path())
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
            .path(repo_dir.path())
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
            .path(repo_dir.path())
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
            .path(repo_dir.path())
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
                up_to_date="✓"
            })
            .path(repo_dir.path())
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
            .path(repo_dir.path())
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
            .path(repo_dir.path())
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
            .path(repo_dir.path())
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
            .path(repo_dir.path())
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
            .args(["config", "status.showUntrackedFiles", "no"])
            .current_dir(repo_dir.path())
            .output()?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    #[cfg(unix)]
    fn doesnt_run_fsmonitor() -> io::Result<()> {
        use std::os::unix::fs::PermissionsExt;
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut f = File::create(repo_dir.path().join("do_not_execute"))?;
        write!(f, "#!/bin/sh\necho executed > executed\nsync executed")?;
        let metadata = f.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o700);
        f.set_permissions(permissions)?;
        f.sync_all()?;

        create_command("git")?
            .args(["config", "core.fsmonitor"])
            .arg(repo_dir.path().join("do_not_execute"))
            .current_dir(repo_dir.path())
            .output()?;

        ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();

        let created_file = repo_dir.path().join("executed").exists();

        assert!(!created_file);

        repo_dir.close()
    }

    #[test]
    fn shows_stashed() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_stash(repo_dir.path())?;

        create_command("git")?
            .args(["reset", "--hard", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$stashed"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("$"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_no_stashed_after_undo() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_stash(repo_dir.path())?;
        undo_stash(repo_dir.path())?;

        create_command("git")?
            .args(["reset", "--hard", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$stashed"
            })
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_stashed_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_stash(repo_dir.path())?;
        undo_stash(repo_dir.path())?;
        create_stash(repo_dir.path())?;
        create_stash(repo_dir.path())?;

        create_command("git")?
            .args(["reset", "--hard", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                stashed = r"\$$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = format_output("$2");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_typechanged() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_typechanged(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                typechanged = "⇢"
            })
            .path(repo_dir.path())
            .collect();
        let expected = format_output("⇢");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_worktree_typechanged_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_typechanged(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$worktree_typechanged"
                worktree_typechanged = "$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("1"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_modified() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_modified(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
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
            .path(repo_dir.path())
            .collect();
        let expected = format_output("!1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_worktree_modified_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_modified(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$worktree_modified"
                worktree_modified = "$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("1"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_index_modified_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_indexed_modified(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$index_modified"
                index_modified = "$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("1"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_added() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_added(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();
        let expected = format_output("!");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_worktree_added_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_added(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$worktree_added"
                worktree_added = "$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("1"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_staged(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
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
            .path(repo_dir.path())
            .collect();
        let expected = Some(format!(
            "{} ",
            AnsiStrings(&[
                Color::Red.bold().paint("[+"),
                Color::Green.paint("1"),
                Color::Red.bold().paint("]"),
            ])
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_index_added_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_staged(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$index_added"
                index_added = "$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("1"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_typechange_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_staged_typechange(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                staged = "+[$count](green)"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(format!(
            "{} ",
            AnsiStrings(&[
                Color::Red.bold().paint("[+"),
                Color::Green.paint("1"),
                Color::Red.bold().paint("]"),
            ])
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_index_typechanged_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_staged_typechange(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$index_typechanged"
                index_typechanged = "$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("1"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_and_modified_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_staged_and_modified(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();
        let expected = format_output("!+");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_renamed_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_renamed(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
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
            .path(repo_dir.path())
            .collect();
        let expected = format_output("»1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_renamed_and_modified_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_renamed_and_modified(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();
        let expected = format_output("»!");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_deleted(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
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
            .path(repo_dir.path())
            .collect();
        let expected = format_output("✘1");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_worktree_deleted_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_deleted(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$worktree_deleted"
                worktree_deleted = "$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("1"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_index_deleted_file_with_count() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_indexed_deleted(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .config(toml::toml! {
                [git_status]
                format = "$index_deleted"
                index_deleted = "$count"
            })
            .path(repo_dir.path())
            .collect();
        let expected = Some(String::from("1"));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn doesnt_show_ignored_file() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_staged_and_ignored(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();
        let expected = format_output("+");

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn worktree_in_different_dir() -> io::Result<()> {
        let worktree_dir = tempfile::tempdir()?;
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args([
                OsStr::new("config"),
                OsStr::new("core.worktree"),
                worktree_dir.path().as_os_str(),
            ])
            .current_dir(repo_dir.path())
            .output()?;

        File::create(worktree_dir.path().join("test_file"))?.sync_all()?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
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
            .args(["add", "--all"])
            .current_dir(repo_dir.path())
            .output()?;
        create_command("git")?
            .args(["commit", "-m", "add new files", "--no-gpg-sign"])
            .current_dir(repo_dir.path())
            .output()?;

        fs::remove_file(repo_dir.path().join("a"))?;
        fs::rename(repo_dir.path().join("b"), repo_dir.path().join("c"))?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
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

    #[test]
    fn doesnt_generate_git_status_for_bare_repo() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GitBare)?;

        create_added(repo_dir.path())?;

        let actual = ModuleRenderer::new("git_status")
            .path(repo_dir.path())
            .collect();

        assert_eq!(None, actual);

        repo_dir.close()
    }

    fn ahead(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("readme.md"))?.sync_all()?;

        create_command("git")?
            .args(["commit", "-am", "Update readme", "--no-gpg-sign"])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn behind(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn diverge(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;

        fs::write(repo_dir.join("Cargo.toml"), " ")?;

        create_command("git")?
            .args(["commit", "-am", "Update readme", "--no-gpg-sign"])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_typechanged(repo_dir: &Path) -> io::Result<()> {
        fs::remove_file(repo_dir.join("readme.md"))?;

        #[cfg(not(target_os = "windows"))]
        std::os::unix::fs::symlink(repo_dir.join("Cargo.toml"), repo_dir.join("readme.md"))?;

        #[cfg(target_os = "windows")]
        std::os::windows::fs::symlink_file(
            repo_dir.join("Cargo.toml"),
            repo_dir.join("readme.md"),
        )?;

        Ok(())
    }

    fn create_staged_typechange(repo_dir: &Path) -> io::Result<()> {
        create_typechanged(repo_dir)?;

        create_command("git")?
            .args(["add", "."])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_conflict(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(["reset", "--hard", "HEAD^"])
            .current_dir(repo_dir)
            .output()?;

        fs::write(repo_dir.join("readme.md"), "# goodbye")?;

        create_command("git")?
            .args(["add", "."])
            .current_dir(repo_dir)
            .output()?;

        create_command("git")?
            .args(["commit", "-m", "Change readme", "--no-gpg-sign"])
            .current_dir(repo_dir)
            .output()?;

        create_command("git")?
            .args(["pull", "--rebase"])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_stash(repo_dir: &Path) -> io::Result<()> {
        let (file, _path) = tempfile::NamedTempFile::new_in(repo_dir)?.keep()?;
        file.sync_all()?;

        create_command("git")?
            .args(["stash", "--all"])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn undo_stash(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(["stash", "pop"])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_untracked(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("license"))?.sync_all()?;

        Ok(())
    }

    fn create_added(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("license"))?.sync_all()?;

        create_command("git")?
            .args(["add", "-A", "-N"])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_modified(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("readme.md"))?.sync_all()?;

        Ok(())
    }

    fn create_indexed_modified(repo_dir: &Path) -> io::Result<()> {
        create_modified(repo_dir)?;

        create_command("git")?
            .args(["add", "."])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_staged(repo_dir: &Path) -> io::Result<()> {
        File::create(repo_dir.join("license"))?.sync_all()?;

        create_command("git")?
            .args(["add", "."])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_staged_and_modified(repo_dir: &Path) -> io::Result<()> {
        let mut file = File::create(repo_dir.join("readme.md"))?;
        file.sync_all()?;

        create_command("git")?
            .args(["add", "."])
            .current_dir(repo_dir)
            .output()?;

        writeln!(&mut file, "modified")?;
        file.sync_all()?;

        Ok(())
    }

    fn create_renamed(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(["mv", "readme.md", "readme.md.bak"])
            .current_dir(repo_dir)
            .output()?;

        create_command("git")?
            .args(["add", "-A"])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_renamed_and_modified(repo_dir: &Path) -> io::Result<()> {
        create_command("git")?
            .args(["mv", "readme.md", "readme.md.bak"])
            .current_dir(repo_dir)
            .output()?;

        create_command("git")?
            .args(["add", "-A"])
            .current_dir(repo_dir)
            .output()?;

        let mut file = File::create(repo_dir.join("readme.md.bak"))?;
        writeln!(&mut file, "modified")?;
        file.sync_all()?;

        Ok(())
    }

    fn create_deleted(repo_dir: &Path) -> io::Result<()> {
        fs::remove_file(repo_dir.join("readme.md"))?;

        Ok(())
    }

    fn create_indexed_deleted(repo_dir: &Path) -> io::Result<()> {
        create_deleted(repo_dir)?;

        create_command("git")?
            .args(["add", "."])
            .current_dir(repo_dir)
            .output()?;

        Ok(())
    }

    fn create_staged_and_ignored(repo_dir: &Path) -> io::Result<()> {
        let mut file = File::create(repo_dir.join(".gitignore"))?;
        writeln!(&mut file, "ignored.txt")?;
        file.sync_all()?;

        create_command("git")?
            .args(["add", ".gitignore"])
            .current_dir(repo_dir)
            .output()?;

        let mut file = File::create(repo_dir.join("ignored.txt"))?;
        writeln!(&mut file, "modified")?;
        file.sync_all()?;

        Ok(())
    }
}
