use crate::config::StarshipConfig;
use crate::context::{Context, Shell};
use std::path::Path;
use std::process::Command;

/// Render a specific starship module by name
pub fn render_module(
    module_name: &str,
    path: &Path,
    config: Option<toml::Value>,
) -> Option<String> {
    let mut context = Context::new_with_dir(clap::ArgMatches::default(), path);
    context.config = StarshipConfig { config };
    context.shell = Shell::Unknown;

    fs_sync(&path);

    crate::print::get_module(module_name, context)
}

/// Syncs directory in filesystem to disk to ensure consistent tests
#[cfg(not(windows))]
fn fs_sync(path: &Path) {
    Command::new("sync").arg(path).status().unwrap();
}

#[cfg(windows)]
fn fs_sync(path: &Path) {}
