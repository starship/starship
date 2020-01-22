use std::path::Path;

/// Render a specific starship module by name
pub fn render_module(module_name: &str, path: &Path) -> Option<String> {
    let mut context = crate::context::Context::new_with_dir(clap::ArgMatches::default(), path);
    context.config = crate::config::StarshipConfig { config: None };

    crate::print::get_module(module_name, context)
}
