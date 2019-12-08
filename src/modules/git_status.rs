use ansi_term::Style;
use git2::{Repository, Status};

use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};

use crate::configs::git_status::GitStatusConfig;
use crate::segment::Segment;

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
    let branch_name = repo.branch.as_ref()?;
    let repo_root = repo.root.as_ref()?;
    let repository = Repository::open(repo_root).ok()?;

    let mut module = context.new_module("git_status");
    let config: GitStatusConfig = GitStatusConfig::try_load(module.config);

    let ahead_behind = get_ahead_behind(&repository, branch_name);
    if ahead_behind == Ok((0, 0)) {
        log::trace!("No ahead/behind found");
    } else {
        log::debug!("Repo ahead/behind: {:?}", ahead_behind);
    }

    let stash_object = repository.revparse_single("refs/stash");
    if stash_object.is_ok() {
        log::debug!("Stash object: {:?}", stash_object);
    } else {
        log::trace!("No stash object found");
    }

    let repo_status = get_repo_status(&repository);
    log::debug!("Repo status: {:?}", repo_status);

    let segments: Vec<Segment> = format_segments_nested(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "conflicted" => format_segment_with_count(
                "conflicted",
                config.conflicted_format,
                repo_status.as_ref().ok()?.conflicted,
                style,
            ),
            "ahead" => {
                let (ahead, behind) = ahead_behind.as_ref().ok()?;
                if *ahead > 0 && *behind == 0 {
                    format_segment_with_count("ahead", config.ahead_format, *ahead, style)
                } else {
                    None
                }
            }
            "behind" => {
                let (ahead, behind) = ahead_behind.as_ref().ok()?;
                if *ahead == 0 && *behind > 0 {
                    format_segment_with_count("behind", config.behind_format, *behind, style)
                } else {
                    None
                }
            }
            "diverged" => {
                let (ahead, behind) = ahead_behind.as_ref().ok()?;
                if *ahead > 0 && *behind > 0 {
                    format_segments(config.diverged_format, None, |name, query| {
                        let style = get_style_from_query(&query).or(style);
                        match name {
                            "ahead_count" => Some(Segment {
                                _name: "diverged_ahead_count".to_string(),
                                value: ahead.to_string(),
                                style,
                            }),
                            "behind_count" => Some(Segment {
                                _name: "diverged_behind_count".to_string(),
                                value: behind.to_string(),
                                style,
                            }),
                            _ => None,
                        }
                    })
                    .ok()
                } else {
                    None
                }
            }
            "stashed" => {
                if stash_object.is_ok() {
                    Some(vec![Segment {
                        _name: "stashed".to_string(),
                        value: config.stashed_format.to_string(),
                        style,
                    }])
                } else {
                    None
                }
            }
            "deleted" => format_segment_with_count(
                "deleted",
                config.deleted_format,
                repo_status.as_ref().ok()?.deleted,
                style,
            ),
            "renamed" => format_segment_with_count(
                "renamed",
                config.renamed_format,
                repo_status.as_ref().ok()?.renamed,
                style,
            ),
            "modified" => format_segment_with_count(
                "modified",
                config.modified_format,
                repo_status.as_ref().ok()?.modified,
                style,
            ),
            "staged" => format_segment_with_count(
                "staged",
                config.staged_format,
                repo_status.as_ref().ok()?.staged,
                style,
            ),
            "untracked" => format_segment_with_count(
                "untracked",
                config.untracked_format,
                repo_status.as_ref().ok()?.untracked,
                style,
            ),
            _ => None,
        }
    })
    .ok()?;

    if segments.is_empty() {
        return None;
    }

    module.set_segments(segments);

    Some(module)
}

/// Gets the number of files in various git states (staged, modified, deleted, etc...)
fn get_repo_status(repository: &Repository) -> Result<RepoStatus, git2::Error> {
    let mut status_options = git2::StatusOptions::new();

    match repository.config()?.get_entry("status.showUntrackedFiles") {
        Ok(entry) => status_options.include_untracked(entry.value() != Some("no")),
        _ => status_options.include_untracked(true),
    };
    status_options.renames_from_rewrites(true);
    status_options.renames_head_to_index(true);
    status_options.renames_index_to_workdir(true);

    let statuses: Vec<Status> = repository
        .statuses(Some(&mut status_options))?
        .iter()
        .map(|s| s.status())
        .collect();

    if statuses.is_empty() {
        return Err(git2::Error::from_str("Repo has no status"));
    }

    let repo_status: RepoStatus = RepoStatus {
        conflicted: statuses.iter().filter(|s| is_conflicted(**s)).count(),
        deleted: statuses.iter().filter(|s| is_deleted(**s)).count(),
        renamed: statuses.iter().filter(|s| is_renamed(**s)).count(),
        modified: statuses.iter().filter(|s| is_modified(**s)).count(),
        staged: statuses.iter().filter(|s| is_staged(**s)).count(),
        untracked: statuses.iter().filter(|s| is_untracked(**s)).count(),
    };

    Ok(repo_status)
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

fn format_segment_with_count(
    segment_name: &str,
    format: &str,
    count: usize,
    default_style: Option<Style>,
) -> Option<Vec<Segment>> {
    if count > 0 {
        format_segments(format, default_style, |name, query| {
            let style = get_style_from_query(&query).or(default_style);
            match name {
                "count" => Some(Segment {
                    _name: format!("{}_count", &segment_name),
                    value: count.to_string(),
                    style,
                }),
                _ => None,
            }
        })
        .ok()
    } else {
        None
    }
}
