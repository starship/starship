use super::{Context, Module, RootModuleConfig};

use crate::configs::jobs::JobsConfig;
use crate::formatter::StringFormatter;

/// Creates a segment to show if there are any active jobs running
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jobs");
    let config: JobsConfig = JobsConfig::try_load(module.config);

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
    let mut module_number = "".to_string();
    if num_of_jobs > config.threshold {
        module_number = num_of_jobs.to_string();
    }

    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "number" => Some(module_number.to_string()),
            _ => None,
        })
    } else {
        log::warn!("Error parsing format string in `jobs.format`");
        return None;
    };

    module.set_segments(formatter.parse(None));

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}
