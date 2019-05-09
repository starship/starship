use ansi_term::Color;
use git2::{Repository, Status};

use super::{Context, Module};

/// Creates a segment with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn segment(context: &Context) -> Option<Module> {
    let module_style = Color::Red.bold();

    let repo_root = context.repo_root.as_ref()?;
    let repository = Repository::open(repo_root).ok()?;
    let repo_status = get_repo_status(&repository)?;

    debug!("Repo status: {:?}", repo_status);

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

    if repository.revparse_single("refs/stash").is_ok() {
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
    // TODO: Check whether branch is ahead
    // TODO: Check whether branch is behind
    // TODO: Check whether branch is diverged
    
    Some(module)
}

fn get_repo_status(repository: &Repository) -> Option<Status> {
    let mut status_options = git2::StatusOptions::new();
    status_options.include_untracked(true);

    let repo_file_statuses = repository.statuses(Some(&mut status_options)).ok()?;

    // Statuses are stored as bitflags, so use BitOr to join them all into a single value
    let repo_status = repo_file_statuses.iter().fold(Status::empty(), |acc, x| acc | x.status());
    if repo_status.is_empty() {
        return None;
    }

    Some(repo_status)
}
