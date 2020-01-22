use std::path::Path;
use crate::context::{Context, Shell};
use crate::config::StarshipConfig;

/// Render a specific starship module by name
pub fn render_module(module_name: &str, path: &Path) -> Option<String> {
    let mut context = Context::new_with_dir(clap::ArgMatches::default(), path);
    context.config = StarshipConfig { config: None };
    context.shell = Shell::Unknown;

    crate::print::get_module(module_name, context)
}
