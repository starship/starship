use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module, RootModuleConfig};

use crate::configs::hg_branch::HgBranchConfig;

/// Creates a module with the Hg bookmark or branch in the current directory
///
/// Will display the bookmark or branch name if the current directory is an hg repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_hg_repo = context
        .try_begin_scan()?
        .set_folders(&[".hg"])
        .is_match();

    if !is_hg_repo {
        return None;
    }

    let mut module = context.new_module("hg_branch");
    let config = HgBranchConfig::try_load(module.config);
    module.set_style(config.style);

    module.get_prefix().set_value("on ");

    module.create_segment("symbol", &config.symbol);

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

    let branch_name = get_hg_current_bookmark(context)
        .unwrap_or_else(|| get_hg_branch_name(context));

    let truncated_graphemes = get_graphemes(&branch_name, len);
    // The truncation symbol should only be added if we truncated
    let truncated_and_symbol = if len < graphemes_len(&branch_name) {
        let truncation_symbol = get_graphemes(config.truncation_symbol, 1);
        truncated_graphemes + &truncation_symbol
    } else {
        truncated_graphemes
    };

    module.create_segment(
        "name",
        &config.branch_name.with_value(&truncated_and_symbol),
    );

    Some(module)
}

fn get_hg_branch_name(ctx: &Context) -> String {
    std::fs::read_to_string(ctx.current_dir.join(".hg").join("branch"))
        .map(|s| s.trim().into())
        .unwrap_or_else(|_| "(no branch)".to_string())
}

fn get_hg_current_bookmark(ctx: &Context) -> Option<String> {
    std::fs::read_to_string(ctx.current_dir.join(".hg").join("bookmarks.current"))
        .map(|s| s.trim().into())
        .ok()
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
