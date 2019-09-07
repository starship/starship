use lazy_static::lazy_static;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, io, process};

lazy_static! {
    static ref MANIFEST_DIR: &'static Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    pub static ref FIXTURES_DIR: PathBuf = MANIFEST_DIR.join("tests/fixtures");
    static ref EMPTY_CONFIG: PathBuf = MANIFEST_DIR.join("empty_config.toml");
}

/// Render the full starship prompt
pub fn render_prompt() -> process::Command {
    let mut command = process::Command::new("./target/debug/starship");

    command
        .arg("prompt")
        .env_clear()
        .env("PATH", env!("PATH")) // Provide the $PATH variable so that external programs are runnable
        .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

    command
}

/// Render a specific starship module by name
pub fn render_module(module_name: &str) -> process::Command {
    let binary = fs::canonicalize("./target/debug/starship").unwrap();
    let mut command = process::Command::new(binary);

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

/// Create a repo from the fixture to be used in git module tests
pub fn create_fixture_repo() -> io::Result<std::path::PathBuf> {
    let fixture_repo_dir = new_tempdir()?.path().join("fixture");
    let repo_dir = new_tempdir()?.path().join("rocket");
    let fixture = env::current_dir()?.join("tests/fixtures/rocket.bundle");

    Command::new("git")
        .args(&[
            "clone",
            "-b",
            "master",
            &fixture.to_str().unwrap(),
            fixture_repo_dir.to_str().unwrap(),
        ])
        .output()?;

    git2::Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    Command::new("git")
        .args(&["config", "--local", "user.email", "starship@example.com"])
        .current_dir(repo_dir.as_path())
        .output()?;

    Command::new("git")
        .args(&["config", "--local", "user.name", "starship"])
        .current_dir(repo_dir.as_path())
        .output()?;

    Ok(repo_dir)
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
