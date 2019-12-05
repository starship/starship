use unicode_segmentation::UnicodeSegmentation;

use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};

use crate::configs::git_branch::GitBranchConfig;
use crate::segment::Segment;

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_branch");
    let config = GitBranchConfig::try_load(module.config);

    let truncation_symbol = get_graphemes(config.truncation_symbol, 1);

    // TODO: Once error handling is implemented, warn the user if their config
    // truncation length is nonsensical
    let len = if config.truncation_length <= 0 {
        log::warn!(
            "\"truncation_length\" should be a positive value, found {}",
            config.truncation_length
        );
        std::usize::MAX
    } else {
        config.truncation_length as usize
    };

    let repo = context.get_repo().ok()?;
    let branch_name = repo.branch.as_ref()?;
    let truncated_graphemes = get_graphemes(&branch_name, len);
    // The truncation symbol should only be added if we truncated
    let truncated_and_symbol = if len < graphemes_len(&branch_name) {
        truncated_graphemes + &truncation_symbol
    } else {
        truncated_graphemes
    };

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "name" => Some(Segment {
                _name: "name".to_string(),
                value: truncated_and_symbol.clone(),
                style,
            }),
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}

fn get_graphemes(text: &str, length: usize) -> String {
    UnicodeSegmentation::graphemes(text, true)
        .take(length)
        .collect::<Vec<&str>>()
        .concat()
}

fn graphemes_len(text: &str) -> usize {
    UnicodeSegmentation::graphemes(&text[..], true).count()
}
