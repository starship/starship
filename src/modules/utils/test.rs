use crate::config::StarshipConfig;
use crate::context::{Context, Shell};
use std::path::Path;

/// Render a specific starship module by name
pub async fn render_module(module_name: &str, path: &Path) -> Option<String> {
  let mut context = Context::new_with_dir(clap::ArgMatches::default(), path);
  context.config = StarshipConfig { config: None };
  context.shell = Shell::Unknown;

  crate::print::get_module(module_name, context).await
}
