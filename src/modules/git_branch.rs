use ansi_term::Color;
use git2::Repository;

use super::{Context, Module};

/// Creates a segment with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn segment(context: &Context) -> Option<Module> {
    if context.repository.is_none() {
        return None;
    }

    let repository = context.repository.as_ref().unwrap();
    match get_current_branch(repository) {
        Ok(branch_name) => {
            const GIT_BRANCH_CHAR: &str = " ";
            const SEGMENT_COLOR: Color = Color::Purple;

            let mut module = Module::new("git_branch");
            module.set_style(SEGMENT_COLOR.bold());

            let prefix = module.get_prefix();
            prefix.set_value("in ");

            let branch_char_segment = module.new_segment("branch_char");
            branch_char_segment.set_value(GIT_BRANCH_CHAR);

            let branch_name_segment = module.new_segment("branch_name");
            branch_name_segment.set_value(branch_name);

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
