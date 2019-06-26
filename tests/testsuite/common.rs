use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use std::{io, process};

lazy_static! {
    static ref MANIFEST_DIR: &'static Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    pub static ref FIXTURES_DIR: PathBuf = MANIFEST_DIR.join("tests/fixtures");
    static ref EMPTY_CONFIG: PathBuf = MANIFEST_DIR.join("empty_config.toml");
}

pub fn render_prompt() -> process::Command {
    let mut command = process::Command::new("./target/debug/starship");
    command
        .arg("prompt")
        .env_clear()
        .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

    command
}

pub fn render_module(module_name: &str) -> process::Command {
    let mut command = process::Command::new("./target/debug/starship");
    command
        .arg("module")
        .arg(module_name)
        .env_clear()
        .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

    command
}

/// Create a temporary directory with full access permissions (rwxrwxrwt).
pub fn new_tempdir() -> io::Result<tempfile::TempDir> {
    //  Using `tempfile::TempDir` directly creates files on macOS within
    // "/var/folders", which provides us with restricted permissions (rwxr-xr-x)
    tempfile::tempdir_in("/tmp")
}
