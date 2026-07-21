use super::{Context, Module, ModuleConfig};

use crate::configs::jj_metrics::JJMetricsConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the JJ metrics in the current repository
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_metrics");
    let config = JJMetricsConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let current_change = context.get_jj_repo()?.current_change(context)?;

    let render_value = |value: u32| {
        if config.only_nonzero_diffs && value == 0 {
            return None;
        }

        Some(Ok(value.to_string()))
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "added_style" => Some(Ok(config.added_style)),
                "deleted_style" => Some(Ok(config.deleted_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "added" => render_value(current_change.lines_added),
                "deleted" => render_value(current_change.lines_removed),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_metrics`:\n{error}");
            return None;
        }
    });

    Some(module)
}
