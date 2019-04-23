use ansi_term::Color;
use git2::Repository;

use super::Segment;
use crate::context::Context;

/// Creates a segment with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn segment(context: &Context) -> Option<Segment> {
    if context.repository.is_none() {
        return None;
    }

    let repository = context.repository.as_ref().unwrap();
    match get_current_branch(repository) {
        Ok(branch_name) => {
            const GIT_BRANCH_CHAR: &str = "";
            const SEGMENT_COLOR: Color = Color::Purple;

            let mut segment_prefix = Segment::new("git_branch_prefix");
            segment_prefix
                .set_value(GIT_BRANCH_CHAR)
                .set_style(SEGMENT_COLOR.bold());

            let mut segment = Segment::new("git_branch");
            segment
                .set_prefix(Some(Box::new(segment_prefix)))
                .set_style(SEGMENT_COLOR.bold())
                .set_value(branch_name);

            Some(segment)
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
