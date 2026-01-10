use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu_closest_bookmarks::JujutsuClosestBookmarksConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::jujutsu::get_closest_jujutsu_bookmarks_info;
use crate::modules::vcs;

/// Creates a module with the Jujutsu bookmarks in the current directory
///
/// Will display bookmarks pointing to the current change if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_closest_bookmarks");
    let config: JujutsuClosestBookmarksConfig =
        JujutsuClosestBookmarksConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    }

    // Only run in jj repositories
    vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;

    let jujutsu_info = get_closest_jujutsu_bookmarks_info(context)?;

    let bookmarks = if jujutsu_info.bookmarks.is_empty() {
        None
    } else {
        Some(
            jujutsu_info
                .bookmarks
                .iter()
                .map(|bookmark| {
                    let mut name = bookmark.name.clone();
                    let local_ahead = bookmark.remote_behind;
                    let local_behind = bookmark.remote_ahead;
                    if bookmark.is_tracked {
                        if local_ahead > 0 && local_behind > 0 {
                            name.push_str(&format!(" ⇡{}⇣{}", local_ahead, local_behind));
                        } else if local_ahead > 0 {
                            name.push_str(&format!(" ⇡{}", local_ahead));
                        } else if local_behind > 0 {
                            name.push_str(&format!(" ⇣{}", local_behind));
                        }
                    }
                    name
                })
                .collect::<Vec<_>>(),
        )
    };

    bookmarks.as_ref()?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "bookmarks" => Some(Ok(bookmarks
                    .as_ref()?
                    .iter()
                    .map(|bookmark| bookmark.to_string())
                    .collect::<Vec<_>>()
                    .join(" "))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jujutsu_closest_bookmarks`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
