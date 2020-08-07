use crate::config::StarshipConfig;
use crate::context::{Context, Shell};
use std::path::Path;
use std::{fs, io};

/// Render a specific starship module by name
pub fn render_module(
    module_name: &str,
    path: &Path,
    config: Option<toml::Value>,
) -> Option<String> {
    let mut context = Context::new_with_dir(clap::ArgMatches::default(), path);
    context.config = StarshipConfig { config };
    context.shell = Shell::Unknown;

    sync_dir(&path).unwrap();

    crate::print::get_module(module_name, context)
}

/// Syncs directory in filesystem to disk to ensure consistent tests
/// Duplicate defintion in tests/testsuite/common.rs. Always change both!
fn sync_dir(path: &Path) -> io::Result<()> {
    let mut opener = fs::OpenOptions::new();

    // Fix opening directories
    #[cfg(not(windows))]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opener.read(true);
        opener.custom_flags(nix::fcntl::OFlag::O_DIRECTORY.bits());
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        opener.write(true);
        opener.custom_flags(winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS);
    }

    opener.open(path)?.sync_all()
}
