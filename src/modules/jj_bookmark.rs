use crate::configs::jj_bookmark::JJBookmarkConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::truncate::truncate_text;

use super::{Context, Module, ModuleConfig};

/// Creates a module with the JJ bookmark in the current repository
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_bookmark");
    let config = JJBookmarkConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let current_change = context.get_jj_repo()?.current_change(context)?;
    let bookmarks = current_change.bookmarks.as_deref()?;

    // The overflow count filters out ignored bookmarks
    let ((name, remote, diverged), overflow_count) = {
        let mut iter = bookmarks.iter().filter_map(|b| {
            let name = b.name();
            let remote = b.remote();
            let ignored = config.ignore_names.contains(&name)
                || remote
                    .map(|r| config.ignore_remotes.contains(&r))
                    .unwrap_or_default();

            (!ignored).then_some((name, remote, b.diverged()))
        });

        (iter.next()?, iter.count())
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                "diverged" => diverged.then_some(config.diverged_symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(move |variable| match variable {
                "bookmark" => Some(Ok(format_item(&config, name))),
                "remote" => remote.map(|s| Ok(format_item(&config, s))),
                "overflow_count" => (overflow_count > 0).then(|| Ok(overflow_count.to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_bookmark`: \n{error}");
            return None;
        }
    });

    Some(module)
}

fn format_item(config: &JJBookmarkConfig, item: &str) -> String {
    truncate_text(
        item,
        config.truncation_length.into(),
        config.truncation_symbol,
    )
}
