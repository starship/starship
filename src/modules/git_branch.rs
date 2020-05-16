use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module, RootModuleConfig};

use crate::configs::git_branch::GitBranchConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_branch");
    let config = GitBranchConfig::try_load(module.config);

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    let truncation_symbol = get_first_grapheme(config.truncation_symbol);

    // TODO: Once error handling is implemented, warn the user if their config
    //       truncation length is nonsensical
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

    let mut graphemes = get_graphemes(&branch_name);
    let trunc_len = len.min(graphemes.len());

    if trunc_len < graphemes.len() {
        // The truncation symbol should only be added if we truncate
        graphemes[trunc_len] = truncation_symbol;
        graphemes.truncate(trunc_len + 1)
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "branch" => Some(Ok(graphemes.concat())),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_branch`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_first_grapheme(text: &str) -> &str {
    UnicodeSegmentation::graphemes(text, true)
        .next()
        .unwrap_or("")
}

fn get_graphemes(text: &str) -> Vec<&str> {
    UnicodeSegmentation::graphemes(text, true).collect()
}
