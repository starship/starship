use git2::{Repository, Status};

use super::{Context, Module, RootModuleConfig};

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
    let repo = context.get_repo().ok()?;
    let branch_name = repo.branch.as_ref()?;
    let repo_root = repo.root.as_ref()?;
    let repository = Repository::open(repo_root).ok()?;

    let mut module = context.new_module("git_status");
    let config = GitStatusConfig::try_load(module.config);

    module
        .get_prefix()
        .set_value(config.prefix.value)
        .set_style(config.style);
    module
        .get_suffix()
        .set_value(config.suffix.value)
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
        if repo_status.is_conflicted() {
            module.create_segment("conflicted", &config.conflicted);
        }
    }

    // Add the ahead/behind segment
    if let Ok((ahead, behind)) = ahead_behind {
        let add_ahead = |m: &mut Module<'a>| {
            m.create_segment("ahead", &config.ahead);

            if config.show_sync_count {
                m.create_segment("ahead_count", &config.ahead.with_value(&ahead.to_string()));
            }
        };

        let add_behind = |m: &mut Module<'a>| {
            m.create_segment("behind", &config.behind);

            if config.show_sync_count {
                m.create_segment(
                    "behind_count",
                    &config.behind.with_value(&behind.to_string()),
                );
            }
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
        if repo_status.is_wt_deleted() || repo_status.is_index_deleted() {
            module.create_segment("deleted", &config.deleted);
        }

        if repo_status.is_wt_renamed() || repo_status.is_index_renamed() {
            module.create_segment("renamed", &config.renamed);
        }

        if repo_status.is_wt_modified() {
            module.create_segment("modified", &config.modified);
        }

        if repo_status.is_index_modified() || repo_status.is_index_new() {
            module.create_segment("staged", &config.staged);
        }

        if repo_status.is_wt_new() {
            module.create_segment("untracked", &config.untracked);
        }
    }

    if module.is_empty() {
        return None;
    }

    Some(module)
}

/// Gets the bitflags associated with the repo's git status
fn get_repo_status(repository: &Repository) -> Result<Status, git2::Error> {
    let mut status_options = git2::StatusOptions::new();

    match repository.config()?.get_entry("status.showUntrackedFiles") {
        Ok(entry) => status_options.include_untracked(entry.value() != Some("no")),
        _ => status_options.include_untracked(true),
    };
    status_options.renames_from_rewrites(true);
    status_options.renames_head_to_index(true);
    status_options.renames_index_to_workdir(true);

    let repo_file_statuses = repository.statuses(Some(&mut status_options))?;

    // Statuses are stored as bitflags, so use BitOr to join them all into a single value
    let repo_status: Status = repo_file_statuses.iter().map(|e| e.status()).collect();
    if repo_status.is_empty() {
        return Err(git2::Error::from_str("Repo has no status"));
    }

    Ok(repo_status)
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
