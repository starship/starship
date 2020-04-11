use git2::{Repository, Status};

use super::{Context, Module, RootModuleConfig};

use crate::configs::git_status::GitStatusConfig;
use crate::context::Repo;
use crate::formatter::StringFormatter;
use crate::segment::Segment;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

const ALL_STATUS_VARIABLES: [&str; 7] = [
    "conflicted",
    "stashed",
    "deleted",
    "renamed",
    "modified",
    "staged",
    "untracked",
];

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

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    let variable_mapper = |variable: &str| -> Option<Vec<Segment>> {
        let info = Arc::clone(&info);
        match variable {
            "stashed" => info.get_stashed().and_then(|count| {
                format_count(config.stashed_format, "git_status.stashed_format", count)
            }),
            "ahead_behind" => info.get_ahead_behind().and_then(|(ahead, behind)| {
                if ahead > 0 && behind > 0 {
                    format_text(
                        config.diverged_format,
                        "git_status.diverged_format",
                        |variable| match variable {
                            "ahead_count" => Some(ahead.to_string()),
                            "behind_count" => Some(behind.to_string()),
                            _ => None,
                        },
                    )
                } else if ahead > 0 && behind == 0 {
                    format_count(config.ahead_format, "git_status.ahead_format", ahead)
                } else if behind > 0 && ahead == 0 {
                    format_count(config.behind_format, "git_status.behind_format", behind)
                } else {
                    None
                }
            }),
            "conflicted" => info.get_conflicted().and_then(|count| {
                format_count(
                    config.conflicted_format,
                    "git_status.conflicted_format",
                    count,
                )
            }),
            "deleted" => info.get_deleted().and_then(|count| {
                format_count(config.deleted_format, "git_status.deleted_format", count)
            }),
            "renamed" => info.get_renamed().and_then(|count| {
                format_count(config.renamed_format, "git_status.renamed_format", count)
            }),
            "modified" => info.get_modified().and_then(|count| {
                format_count(config.modified_format, "git_status.modified_format", count)
            }),
            "staged" => info.get_staged().and_then(|count| {
                format_count(config.staged_format, "git_status.staged_format", count)
            }),
            "untracked" => info.get_untracked().and_then(|count| {
                format_count(
                    config.untracked_format,
                    "git_status.untracked_format",
                    count,
                )
            }),
            _ => None,
        }
    };

    let segments = format_segments(
        config.format,
        "git_status.format",
        |variable| match variable {
            "all_status" => {
                let segments = ALL_STATUS_VARIABLES
                    .iter()
                    .flat_map(|variable| variable_mapper(variable))
                    .flatten()
                    .collect::<Vec<Segment>>();

                if segments.is_empty() {
                    None
                } else {
                    Some(segments)
                }
            }
            _ => variable_mapper(variable),
        },
    )?;

    module.set_segments(segments);

    Some(module)
}

struct GitStatusInfo<'a> {
    repo: &'a Repo,
    ahead_behind: RwLock<Option<Result<(usize, usize), git2::Error>>>,
    repo_status: RwLock<Option<Result<RepoStatus, git2::Error>>>,
}

impl<'a> GitStatusInfo<'a> {
    pub fn load(repo: &'a Repo) -> Self {
        Self {
            repo,
            ahead_behind: RwLock::new(None),
            repo_status: RwLock::new(None),
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
                        log::warn!("Warn: get_ahead_behind: {}", error);
                        None
                    }
                };
            };
        }

        {
            let repo = self.get_repository()?;
            let branch_name = self.get_branch_name();
            let mut data = self.ahead_behind.write().unwrap();
            *data = Some(get_ahead_behind(&repo, &branch_name));
            match data.as_ref().unwrap() {
                Ok(ahead_behind) => Some(*ahead_behind),
                Err(error) => {
                    log::warn!("Warn: get_ahead_behind: {}", error);
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
                        log::warn!("Warn: get_repo_status: {}", error);
                        None
                    }
                };
            };
        }

        {
            let mut repo = self.get_repository()?;
            let mut data = self.repo_status.write().unwrap();
            *data = Some(get_repo_status(&mut repo));
            match data.as_ref().unwrap() {
                Ok(repo_status) => Some(*repo_status),
                Err(error) => {
                    log::warn!("Warn: get_repo_status: {}", error);
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

    pub fn get_stashed(&self) -> Option<usize> {
        self.get_repo_status().map(|data| data.stashed)
    }
}

/// Gets the number of files in various git states (staged, modified, deleted, etc...)
fn get_repo_status(repository: &mut Repository) -> Result<RepoStatus, git2::Error> {
    let mut status_options = git2::StatusOptions::new();

    match repository.config()?.get_entry("status.showUntrackedFiles") {
        Ok(entry) => status_options.include_untracked(entry.value() != Some("no")),
        _ => status_options.include_untracked(true),
    };
    status_options
        .renames_from_rewrites(true)
        .renames_head_to_index(true)
        .renames_index_to_workdir(true)
        .include_unmodified(true);

    let statuses: Vec<Status> = repository
        .statuses(Some(&mut status_options))?
        .iter()
        .map(|s| s.status())
        .collect();

    if statuses.is_empty() {
        return Err(git2::Error::from_str("Repo has no status"));
    }

    let statuses_count = count_statuses(statuses);

    let repo_status: RepoStatus = RepoStatus {
        conflicted: *statuses_count.get("conflicted").unwrap_or(&0),
        deleted: *statuses_count.get("deleted").unwrap_or(&0),
        renamed: *statuses_count.get("renamed").unwrap_or(&0),
        modified: *statuses_count.get("modified").unwrap_or(&0),
        staged: *statuses_count.get("staged").unwrap_or(&0),
        untracked: *statuses_count.get("untracked").unwrap_or(&0),
        stashed: stashed_count(repository)?,
    };

    Ok(repo_status)
}

fn count_statuses(statuses: Vec<Status>) -> HashMap<&'static str, usize> {
    let mut predicates: HashMap<&'static str, fn(git2::Status) -> bool> = HashMap::new();
    predicates.insert("conflicted", is_conflicted);
    predicates.insert("deleted", is_deleted);
    predicates.insert("renamed", is_renamed);
    predicates.insert("modified", is_modified);
    predicates.insert("staged", is_staged);
    predicates.insert("untracked", is_untracked);

    statuses.iter().fold(HashMap::new(), |mut map, status| {
        for (key, predicate) in predicates.iter() {
            if predicate(*status) {
                let entry = map.entry(key).or_insert(0);
                *entry += 1;
            }
        }
        map
    })
}

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

fn stashed_count(repository: &mut Repository) -> Result<usize, git2::Error> {
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
    stashed: usize,
}

fn format_segments<F>(format_str: &str, config_path: &str, mapper: F) -> Option<Vec<Segment>>
where
    F: Fn(&str) -> Option<Vec<Segment>> + Send + Sync,
{
    if let Ok(formatter) = StringFormatter::new(format_str) {
        Some(formatter.map_variables_to_segments(mapper).parse(None))
    } else {
        log::error!("Error parsing format string `{}`", &config_path);
        None
    }
}

fn format_text<F>(format_str: &str, config_path: &str, mapper: F) -> Option<Vec<Segment>>
where
    F: Fn(&str) -> Option<String> + Send + Sync,
{
    if let Ok(formatter) = StringFormatter::new(format_str) {
        Some(formatter.map(mapper).parse(None))
    } else {
        log::error!("Error parsing format string `{}`", &config_path);
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
