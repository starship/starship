use super::utils::query_parser::*;
use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::jobs::JobsConfig;
use crate::segment::Segment;

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

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "number" => {
                if num_of_jobs > config.threshold {
                    Some(Segment {
                        _name: "number".to_string(),
                        value: num_of_jobs.to_string(),
                        style,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}
