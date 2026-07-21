use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu_bookmark::JujutsuBookmarkConfig;
use crate::formatter::StringFormatter;
use crate::formatter::string_formatter::StringFormatterError;
use crate::modules::utils::jujutsu::{JujutsuBookmarkInfo, get_jujutsu_bookmarks};
use crate::modules::vcs;
use crate::segment::Segment;

/// Creates a module with the Jujutsu bookmarks in the current directory
///
/// Will display bookmarks pointing to the current change if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_bookmark");
    let config: JujutsuBookmarkConfig = JujutsuBookmarkConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    }

    // Only run in jj repositories
    vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;

    let jujutsu_bookmarks = get_jujutsu_bookmarks(context, config.max_depth)?;

    let bookmarks = if jujutsu_bookmarks.is_empty() {
        return None;
    } else {
        jujutsu_bookmarks
            .iter()
            .take(config.max_shown)
            .map(|bookmark| format_bookmark(&config, bookmark, context))
            .collect::<Result<Vec<_>, _>>()
            .map(|nested_vec| {
                nested_vec
                    .into_iter()
                    .enumerate()
                    .flat_map(|(i, segments)| {
                        if i == 0 {
                            segments
                        } else {
                            Segment::from_text(None, config.joiner)
                                .into_iter()
                                .chain(segments)
                                .collect::<Vec<_>>()
                        }
                    })
                    .collect::<Vec<_>>()
            })
    };

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
            .map_variables_to_segments(|variable| match variable {
                "bookmarks" => Some(bookmarks.clone()),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jujutsu_bookmark`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn format_bookmark(
    config: &JujutsuBookmarkConfig,
    bookmark: &JujutsuBookmarkInfo,
    context: &Context,
) -> Result<Vec<Segment>, StringFormatterError> {
    StringFormatter::new(config.bookmark_format).and_then(|formatter| {
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
                "bookmark_name" => {
                    let chars = bookmark.name.chars().collect::<Vec<_>>();
                    if chars.len() > config.max_length {
                        Some(Ok(chars
                            .into_iter()
                            .take(config.max_length.saturating_sub(1))
                            .chain(['…'])
                            .collect::<String>()))
                    } else {
                        Some(Ok(bookmark.name.clone()))
                    }
                }
                "distance" if bookmark.distance > 0 => Some(Ok(bookmark.distance.to_string())),
                _ => None,
            })
            .map_variables_to_segments(|variable| match variable {
                "ahead_behind" => {
                    if bookmark.is_tracked {
                        let local_ahead = bookmark.remote_behind;
                        let local_behind = bookmark.remote_ahead;
                        if local_ahead > 0 && local_behind > 0 {
                            Some(StringFormatter::new(config.diverged).and_then(|formatter| {
                                formatter
                                    .map(|variable| match variable {
                                        "ahead_count" => Some(Ok(local_ahead.to_string())),
                                        "behind_count" => Some(Ok(local_behind.to_string())),
                                        _ => None,
                                    })
                                    .parse(None, Some(context))
                            }))
                        } else if local_ahead > 0 || local_behind > 0 {
                            let (template, count) = if local_ahead > 0 {
                                (config.ahead, local_ahead)
                            } else {
                                (config.behind, local_behind)
                            };
                            Some(StringFormatter::new(template).and_then(|formatter| {
                                formatter
                                    .map(|variable| match variable {
                                        "count" => Some(Ok(count.to_string())),
                                        _ => None,
                                    })
                                    .parse(None, Some(context))
                            }))
                        } else {
                            Some(
                                StringFormatter::new(config.up_to_date)
                                    .and_then(|formatter| formatter.parse(None, Some(context))),
                            )
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .parse(None, Some(context))
    })
}
