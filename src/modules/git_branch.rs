use ansi_term::Color;

use super::{Context, Module};

/// Creates a segment with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn segment<'a>(context: &'a Context) -> Option<Module<'a>> {
    let branch_name = context.branch_name.as_ref()?;

    const GIT_BRANCH_CHAR: &str = " ";
    let segment_color = Color::Purple.bold();

    let mut module = context.new_module("git_branch");
    module.set_style(segment_color);
    module.get_prefix().set_value("on ");

    module.new_segment("branch_char", GIT_BRANCH_CHAR);
    module.new_segment("branch_name", branch_name.to_string());

    Some(module)
}
