use ansi_term::Color;
use git2::{Repository, StatusEntry};

use super::{Context, Module};

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
    const GIT_STATUS_CONFLICTED: &str = "=";
    const GIT_STATUS_AHEAD: &str = "⇡";
    const GIT_STATUS_BEHIND: &str = "⇣";
    const GIT_STATUS_DIVERGED: &str = "⇕";
    const GIT_STATUS_UNTRACKED: &str = "?";
    const GIT_STATUS_STASHED: &str = "$";
    const GIT_STATUS_MODIFIED: &str = "!";
    const GIT_STATUS_ADDED: &str = "+";
    const GIT_STATUS_RENAMED: &str = "»";
    const GIT_STATUS_DELETED: &str = "✘";
    const PREFIX: &str = "[";
    const SUFFIX: &str = "] ";

    let repo = context.get_repo().ok()?;
    let branch_name = repo.branch.as_ref()?;
    let repo_root = repo.root.as_ref()?;
    let repository = Repository::open(repo_root).ok()?;

    let mut module = context.new_module("git_status");
    let show_sync_count = module.config_value_bool("show_sync_count").unwrap_or(false);
    let show_status_count = module
        .config_value_bool("show_status_count")
        .unwrap_or(false);
    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Red.bold());
    let start_symbol = module
        .config_value_str("prefix")
        .unwrap_or(PREFIX)
        .to_owned();
    let end_symbol = module
        .config_value_str("suffix")
        .unwrap_or(SUFFIX)
        .to_owned();

    module
        .get_prefix()
        .set_value(start_symbol)
        .set_style(module_style);
    module
        .get_suffix()
        .set_value(end_symbol)
        .set_style(module_style);
    module.set_style(module_style);

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
        new_segment_with_count(
            &mut module,
            "conflicted",
            repo_status.conflicted,
            GIT_STATUS_CONFLICTED,
            show_status_count,
        );
    }

    // Add the ahead/behind segment
    if let Ok((ahead, behind)) = ahead_behind {
        let add_ahead = |m: &mut Module<'a>| {
            new_segment_with_count(m, "ahead", ahead, GIT_STATUS_AHEAD, show_sync_count);
        };

        let add_behind = |m: &mut Module<'a>| {
            new_segment_with_count(m, "behind", behind, GIT_STATUS_BEHIND, show_sync_count);
        };

        if ahead > 0 && behind > 0 {
            module.new_segment("diverged", GIT_STATUS_DIVERGED);

            if show_sync_count {
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
        new_segment_with_count(
            &mut module,
            "deleted",
            repo_status.deleted,
            GIT_STATUS_DELETED,
            show_status_count,
        );

        new_segment_with_count(
            &mut module,
            "renamed",
            repo_status.renamed,
            GIT_STATUS_RENAMED,
            show_status_count,
        );

        new_segment_with_count(
            &mut module,
            "modified",
            repo_status.modified,
            GIT_STATUS_MODIFIED,
            show_status_count,
        );

        new_segment_with_count(
            &mut module,
            "staged",
            repo_status.staged,
            GIT_STATUS_ADDED,
            show_status_count,
        );

        new_segment_with_count(
            &mut module,
            "untracked",
            repo_status.untracked,
            GIT_STATUS_UNTRACKED,
            show_status_count,
        );
    }

    if module.is_empty() {
        return None;
    }

    Some(module)
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

    let repo_status: RepoStatus = repo_file_statuses.iter().fold(RepoStatus::default(), count);

    Ok(repo_status)
}

/// Increment RepoStatus counts for a given file status
fn count(mut repo_status: RepoStatus, status_entry: StatusEntry) -> RepoStatus {
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

    repo_status
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
