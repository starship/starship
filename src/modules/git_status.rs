use ansi_term::{Color, Style};
use git2::{Repository, StatusEntry};

use super::{Context, Module, RootModuleConfig};

use crate::config::SegmentConfig;
use crate::configs::git_status::GitStatusConfig;

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
    // This is the order that the sections will appear in
    const GIT_STATUS_STASHED: &str = "$";

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
            !config.conflicted_count_disabled
        );
    }

    // Add the ahead/behind segment
    if let Ok((ahead, behind)) = ahead_behind {
        let add_ahead = |m: &mut Module<'a>| {
            create_segment_with_count(m, "ahead", ahead, &config.ahead, config.show_sync_count);
        };

        let add_behind = |m: &mut Module<'a>| {
            create_segment_with_count(m, "behind", behind, &config.behind, config.show_sync_count);
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
        module.new_segment("stashed", GIT_STATUS_STASHED);
    }

    // Add all remaining status segments
    if let Ok(repo_status) = repo_status {
        create_segment_with_count(
            &mut module,
            "deleted",
            repo_status.deleted,
            &config.deleted,
            !config.deleted_count_disabled
        );

        create_segment_with_count(
            &mut module,
            "renamed",
            repo_status.renamed,
            &config.renamed,
            !config.renamed_count_disabled
        );

        create_segment_with_count(
            &mut module,
            "modified",
            repo_status.modified,
            &config.modified,
            !config.modified_count_disabled
        );

        create_segment_with_count(
            &mut module,
            "staged",
            repo_status.staged,
            &config.staged,
            !config.staged_count_disabled
        );

        create_segment_with_count(
            &mut module,
            "untracked",
            repo_status.untracked,
            &config.untracked,
            !config.untracked_count_disabled
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
    show_count: bool,
) {
    if count > 0 {
        module.create_segment(name, &config);

        if show_count {
            module.create_segment(&format!("{}_count", name), &config.with_value(&count.to_string()));
        }
    }

}

/// Adds a segment with an optional count segment (e.g. "+2")
fn new_segment_with_count<'a>(
    module: &mut Module<'a>,
    name: &str,
    count: usize,
    symbol: &str,
    show_count: bool,
) {
    if count > 0 {
        module.new_segment(name, symbol);

        if show_count {
            module.new_segment(&format!("{}_count", name), &count.to_string());
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

    let repo_file_statuses = repository.statuses(Some(&mut status_options))?;

    if repo_file_statuses.is_empty() {
        return Err(git2::Error::from_str("Repo has no status"));
    }

    let repo_status: RepoStatus =
        repo_file_statuses
            .iter()
            .fold(RepoStatus::default(), |mut repo_status, entry| {
                count(&mut repo_status, entry);
                repo_status
            });

    Ok(repo_status)
}

/// Increment RepoStatus counts for a given file status
fn count(repo_status: &mut RepoStatus, status_entry: StatusEntry) {
    let status = status_entry.status();

    if status.is_conflicted() {
        repo_status.conflicted += 1;
    }

    if status.is_wt_deleted() || status.is_index_deleted() {
        repo_status.deleted += 1;
    }

    if status.is_wt_renamed() || status.is_index_renamed() {
        repo_status.renamed += 1;
    }

    if status.is_wt_modified() {
        repo_status.modified += 1;
    }

    if status.is_index_modified() || status.is_index_new() {
        repo_status.staged += 1;
    }

    if status.is_wt_new() {
        repo_status.untracked += 1;
    }
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
