use ansi_term::Color;
use git2::{Repository, Status};

use super::{Context, Module};

/// Creates a segment with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn segment(context: &Context) -> Option<Module> {
    let module_style = Color::Red.bold();

    let branch_name = context.branch_name.as_ref()?;
    let repo_root = context.repo_root.as_ref()?;
    let repository = Repository::open(repo_root).ok()?;
    let repo_status = get_repo_status(&repository)?;

    log::debug!("Repo status: {:?}", repo_status);

    let mut module = Module::new("git_status");
    module.get_prefix().set_value("[").set_style(module_style);
    module.get_suffix().set_value("] ").set_style(module_style);
    module.set_style(module_style);

    const GIT_STATUS_UNTRACKED: &str = "?";
    const GIT_STATUS_MODIFIED: &str = "!";
    const GIT_STATUS_ADDED: &str = "+";
    const GIT_STATUS_RENAMED: &str = "»";
    const GIT_STATUS_DELETED: &str = "✘";
    const GIT_STATUS_STASHED: &str = "$";
    const GIT_STATUS_UNMERGED: &str = "=";
    const GIT_STATUS_AHEAD: &str = "⇡";
    const GIT_STATUS_BEHIND: &str = "⇣";
    const GIT_STATUS_DIVERGED: &str = "⇕";

    // Check ahead/behind
    let branch_object = repository.revparse_single(&branch_name);
    let tracking_branch_name = format!("{}@{{upstream}}", branch_name);
    let tracking_object = repository.revparse_single(&tracking_branch_name);

    if branch_object.is_ok() && tracking_object.is_ok() {
        let branch_oid = branch_object.unwrap().id();
        let tracking_oid = tracking_object.unwrap().id();

        let graph_ahead_behind = repository.graph_ahead_behind(branch_oid, tracking_oid);
        let (ahead, behind) = graph_ahead_behind.unwrap();

        if ahead > 0 && behind > 0 {
            module.new_segment("diverged", GIT_STATUS_DIVERGED);
        } else if ahead > 0 {
            module.new_segment("ahead", GIT_STATUS_AHEAD);
        } else if behind > 0 {
            module.new_segment("behind", GIT_STATUS_BEHIND);
        }
    }

    let stash_object = repository.revparse_single("refs/stash");
    log::debug!("Stash object: {:?}", stash_object);

    if stash_object.is_ok() {
        module.new_segment("stashed", GIT_STATUS_STASHED);
    }

    if repo_status.is_wt_deleted() || repo_status.is_index_deleted() {
        module.new_segment("deleted", GIT_STATUS_DELETED);
    }

    if repo_status.is_wt_renamed() || repo_status.is_index_renamed() {
        module.new_segment("renamed", GIT_STATUS_RENAMED);
    }

    if repo_status.is_wt_modified() {
        module.new_segment("modified", GIT_STATUS_MODIFIED);
    }

    if repo_status.is_index_modified() || repo_status.is_index_new() {
        module.new_segment("staged", GIT_STATUS_ADDED);
    }

    if repo_status.is_wt_new() {
        module.new_segment("untracked", GIT_STATUS_UNTRACKED);
    }

    // TODO: Check for unmerged changes

    Some(module)
}

fn get_repo_status(repository: &Repository) -> Option<Status> {
    let mut status_options = git2::StatusOptions::new();
    status_options.include_untracked(true);

    let repo_file_statuses = repository.statuses(Some(&mut status_options)).ok()?;

    // Statuses are stored as bitflags, so use BitOr to join them all into a single value
    let repo_status: Status = repo_file_statuses.iter().map(|e| e.status()).collect();
    if repo_status.is_empty() {
        return None;
    }

    Some(repo_status)
}
