use crate::config::StarshipConfig;
use crate::context::{Context, Shell};
use std::fs::OpenOptions;
use std::path::Path;

#[cfg(not(windows))]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

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
fn fs_sync(path: &Path) {
    let mut opener = OpenOptions::new();

    // Fix opening directories
    #[cfg(not(windows))]
    {
        opener.read(true);
        opener.custom_flags(nix::fcntl::OFlag::O_DIRECTORY.bits());
    }

    #[cfg(windows)]
    {
        opener.write(true);
        opener.custom_flags(winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS);
    }

    opener.open(path).unwrap().sync_all().unwrap();
}
