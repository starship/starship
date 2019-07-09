use lazy_static::lazy_static;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{io, process};

lazy_static! {
    static ref MANIFEST_DIR: &'static Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    pub static ref FIXTURES_DIR: PathBuf = MANIFEST_DIR.join("tests/fixtures");
    static ref EMPTY_CONFIG: PathBuf = MANIFEST_DIR.join("empty_config.toml");
}

/// Run an instance of starship
fn run_starship() -> process::Command {
    let mut command = process::Command::new("./target/debug/starship");

    command
        .arg("prompt")
        .env_clear()
        .env("PATH", env!("PATH")) // Provide the $PATH variable so that external programs are runnable
        .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

    command
}

pub fn render_module(module_name: &str) -> process::Command {
    let mut command = process::Command::new("./target/debug/starship");

    command
        .arg("module")
        .arg(module_name)
        .env_clear()
        .env("PATH", env!("PATH")) // Provide the $PATH variable so that external programs are runnable
        .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

    command
}

/// Create a temporary directory with full access permissions (rwxrwxrwt).
pub fn new_tempdir() -> io::Result<tempfile::TempDir> {
    //  Using `tempfile::TempDir` directly creates files on macOS within
    // "/var/folders", which provides us with restricted permissions (rwxr-xr-x)
    tempfile::tempdir_in("/tmp")
}

/// Extends `std::process::Command` with methods for testing
pub trait TestCommand {
    fn use_config(&mut self, toml: toml::value::Value) -> &mut process::Command;
}

impl TestCommand for process::Command {
    /// Create a configuration file with the provided TOML and use it
    fn use_config(&mut self, toml: toml::value::Value) -> &mut process::Command {
        // Create a persistent config file in a tempdir
        let (mut config_file, config_path) =
            tempfile::NamedTempFile::new().unwrap().keep().unwrap();
        write!(config_file, "{}", toml.to_string()).unwrap();

        // Set that newly-created file as the config for the prompt instance
        self.env("STARSHIP_CONFIG", config_path)
    }
}
