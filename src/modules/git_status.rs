use git2::{Repository, Status};

use super::{Context, Module, RootModuleConfig};

use crate::config::SegmentConfig;
use crate::configs::git_status::{CountConfig, GitStatusConfig};

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

    module
        .get_prefix()
        .set_value(config.prefix)
        .set_style(config.style);
    module
        .get_suffix()
        .set_value(config.suffix)
        .set_style(config.style);
    module.set_style(config.style);

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

    // Add the conflicted segment
    if let Ok(repo_status) = repo_status {
        create_segment_with_count(
            &mut module,
            "conflicted",
            repo_status.conflicted,
            &config.conflicted,
            config.conflicted_count,
        );
    }

    // Add the ahead/behind segment
    if let Ok((ahead, behind)) = ahead_behind {
        let add_ahead = |m: &mut Module<'a>| {
            create_segment_with_count(
                m,
                "ahead",
                ahead,
                &config.ahead,
                CountConfig {
                    enabled: config.show_sync_count,
                    style: None,
                },
            );
        };

        let add_behind = |m: &mut Module<'a>| {
            create_segment_with_count(
                m,
                "behind",
                behind,
                &config.behind,
                CountConfig {
                    enabled: config.show_sync_count,
                    style: None,
                },
            );
        };

        if ahead > 0 && behind > 0 {
            module.create_segment("diverged", &config.diverged);

            if config.show_sync_count {
                add_ahead(&mut module);
                add_behind(&mut module);
            }
        }

        if ahead > 0 && behind == 0 {
            add_ahead(&mut module);
        }

        if behind > 0 && ahead == 0 {
            add_behind(&mut module);
        }
    }

    // Add the stashed segment
    if stash_object.is_ok() {
        module.create_segment("stashed", &config.stashed);
    }

    // Add all remaining status segments
    if let Ok(repo_status) = repo_status {
        create_segment_with_count(
            &mut module,
            "deleted",
            repo_status.deleted,
            &config.deleted,
            config.deleted_count,
        );

        create_segment_with_count(
            &mut module,
            "renamed",
            repo_status.renamed,
            &config.renamed,
            config.renamed_count,
        );

        create_segment_with_count(
            &mut module,
            "modified",
            repo_status.modified,
            &config.modified,
            config.modified_count,
        );

        create_segment_with_count(
            &mut module,
            "staged",
            repo_status.staged,
            &config.staged,
            config.staged_count,
        );

        create_segment_with_count(
            &mut module,
            "untracked",
            repo_status.untracked,
            &config.untracked,
            config.untracked_count,
        );
    }

    if module.is_empty() {
        return None;
    }

    Some(module)
}

fn create_segment_with_count<'a>(
    module: &mut Module<'a>,
    name: &str,
    count: usize,
    config: &SegmentConfig<'a>,
    count_config: CountConfig,
) {
    if count > 0 {
        module.create_segment(name, &config);

        if count_config.enabled {
            module.create_segment(
                &format!("{}_count", name),
                &SegmentConfig::new(&count.to_string()).with_style(count_config.style),
            );
        }
    }
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
