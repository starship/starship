use ansi_term::Color;
use git2::Repository;

use super::{Context, Module};

/// Creates a segment with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn segment(context: &Context) -> Option<Module> {
    let repo_root = context.repo_root.as_ref()?;
    let repository = Repository::open(repo_root).ok()?;

    match get_current_branch(&repository) {
        Ok(branch_name) => {
            const GIT_BRANCH_CHAR: &str = " ";
            let segment_color = Color::Purple.bold();

            let mut module = Module::new("git_branch");
            module.set_style(segment_color);
            module.get_prefix().set_value("on ");

            module.new_segment("branch_char", GIT_BRANCH_CHAR);
            module.new_segment("branch_name", branch_name);

            Some(module)
        }
        Err(_e) => None,
    }
}

fn get_current_branch(repository: &Repository) -> Result<String, git2::Error> {
    let head = repository.head()?;
    let head_name = head.shorthand();
    match head_name {
        Some(name) => Ok(name.to_string()),
        None => Err(git2::Error::from_str("No branch name found")),
    }
}
