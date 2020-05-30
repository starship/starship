use super::{Context, Module, RootModuleConfig};

use crate::configs::jobs::JobsConfig;
use crate::formatter::StringFormatter;

/// Creates a segment to show if there are any active jobs running
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jobs");
    let config = JobsConfig::try_load(module.config);

    let props = &context.properties;
    let num_of_jobs = props
        .get("jobs")
        .unwrap_or(&"0".into())
        .trim()
        .parse::<i64>()
        .ok()?;
    if num_of_jobs == 0 {
        return None;
    }

    let module_number = if num_of_jobs > config.threshold {
        num_of_jobs.to_string()
    } else {
        "".to_string()
    };

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
                "number" => Some(Ok(module_number.clone())),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jobs`:\n{}", error);
            return None;
        }
    });

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}
