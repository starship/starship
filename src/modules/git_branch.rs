use ansi_term::Color;
use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module};

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const GIT_BRANCH_CHAR: &str = " ";

    let mut module = context.new_module("git_branch")?;

    let segment_color = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Purple.bold());
    module.set_style(segment_color);
    module.get_prefix().set_value("on ");

    let unsafe_truncation_length = module
        .config_value_i64("truncation_length")
        .unwrap_or(std::i64::MAX);
    let truncation_symbol = get_graphemes(
        module.config_value_str("truncation_symbol").unwrap_or("…"),
        1,
    );

    module.new_segment("symbol", GIT_BRANCH_CHAR);

    // TODO: Once error handling is implemented, warn the user if their config
    // truncation length is nonsensical
    let len = if unsafe_truncation_length <= 0 {
        log::debug!(
            "[WARN]: \"truncation_length\" should be a positive value, found {}",
            unsafe_truncation_length
        );
        std::usize::MAX
    } else {
        unsafe_truncation_length as usize
    };
    let branch_name = context.branch_name.as_ref()?;
    let truncated_graphemes = get_graphemes(&branch_name, len);
    // The truncation symbol should only be added if we truncated
    let truncated_and_symbol = if len < graphemes_len(&branch_name) {
        truncated_graphemes + &truncation_symbol
    } else {
        truncated_graphemes
    };

    module.new_segment("name", &truncated_and_symbol);

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
