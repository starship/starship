use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};

use crate::configs::ruby::RubyConfig;
use crate::segment::Segment;
use crate::utils;

/// Creates a module with the current Ruby version
///
/// Will display the Ruby version if any of the following criteria are met:
///     - Current directory contains a `.rb` file
///     - Current directory contains a `Gemfile` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_rb_project = context
        .try_begin_scan()?
        .set_files(&["Gemfile"])
        .set_extensions(&["rb"])
        .is_match();

    if !is_rb_project {
        return None;
    }

    let ruby_version = utils::exec_cmd("ruby", &["-v"])?.stdout;
    let formatted_version = format_ruby_version(&ruby_version)?;

    let mut module = context.new_module("ruby");
    let config: RubyConfig = RubyConfig::try_load(module.config);

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "version" => Some(Segment {
                _name: "version".to_string(),
                value: formatted_version.clone(),
                style,
            }),
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}

fn format_ruby_version(ruby_version: &str) -> Option<String> {
    let version = ruby_version
        // split into ["ruby", "2.6.0p0", "linux/amd64"]
        .split_whitespace()
        // return "2.6.0p0"
        .nth(1)?
        .get(0..5)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}
