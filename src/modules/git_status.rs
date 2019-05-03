use ansi_term::Color;
use git2::{Status};

use super::{Context, Module};

/// Creates a segment with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn segment(context: &Context) -> Option<Module> {
    let repository = context.repository.as_ref()?;

    let mut status_options = git2::StatusOptions::new();
    status_options.include_untracked(true);

    let repo_file_statuses = repository.statuses(Some(&mut status_options)).ok()?;
    if repo_file_statuses.is_empty() {
        return None;
    }

    // Statuses are stored as bitflags, so use BitOr to join them all into a single value
    let repo_status = repo_file_statuses.iter().fold(Status::empty(), |acc, x| acc | x.status());
    info!("repo status: {:?}", repo_status);

    let module_style = Color::Red.bold();

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

    if repo_status.is_wt_new() {
        module.new_segment("untracked", GIT_STATUS_UNTRACKED);
    }

    if repo_status.is_index_modified() {
        module.new_segment("staged", GIT_STATUS_ADDED);
    }

    if repo_status.is_wt_modified() {
        module.new_segment("modified", GIT_STATUS_MODIFIED);
    }

    if repo_status.is_wt_renamed() || repo_status.is_index_renamed() {
        module.new_segment("renamed", GIT_STATUS_RENAMED);
    }

    if repo_status.is_wt_deleted() || repo_status.is_index_deleted() {
        module.new_segment("deleted", GIT_STATUS_DELETED);
    }

    // TODO: Check for stashed files
    // TODO: Check for unmerged changes
    // TODO: Check whether branch is ahead
    // TODO: Check whether branch is behind
    // TODO: Check whether branch is diverged
    
    Some(module)
}
