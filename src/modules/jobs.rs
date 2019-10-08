use super::{Context, Module};

use crate::config::{RootModuleConfig, SegmentConfig};
use crate::configs::jobs::JobsConfig;

/// Creates a segment to show if there are any active jobs running
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jobs");
    let config: JobsConfig = JobsConfig::try_load(module.config);

    module.set_style(config.style);

    let arguments = &context.arguments;
    let num_of_jobs = arguments
        .value_of("jobs")
        .unwrap_or("0")
        .trim()
        .parse::<i64>()
        .ok()?;
    if num_of_jobs == 0 {
        return None;
    }
    module.create_segment("symbol", &config.symbol);
    if num_of_jobs > config.threshold {
        module.create_segment("number", &SegmentConfig::new(&num_of_jobs.to_string()));
    }
    module.get_prefix().set_value("");

    Some(module)
}
