use std::env;

use super::utils::query_parser::*;
use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::conda::CondaConfig;
use crate::segment::Segment;

/// Creates a module with the current Conda environment
///
/// Will display the Conda environment iff `$CONDA_DEFAULT_ENV` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // Reference implementation: https://github.com/denysdovhan/spaceship-prompt/blob/master/sections/conda.zsh
    let conda_env = env::var("CONDA_DEFAULT_ENV").unwrap_or_else(|_| "".into());
    if conda_env.trim().is_empty() {
        return None;
    }

    let mut module = context.new_module("conda");
    let config = CondaConfig::try_load(module.config);

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "env" => Some(Segment {
                _name: "env".to_string(),
                value: conda_env.clone(),
                style,
            }),
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}
