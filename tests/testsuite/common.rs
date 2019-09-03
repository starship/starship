use lazy_static::lazy_static;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io, process};

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

/// Helper function to run a `git config` command
fn run_git_config_command(args: Vec<&str>) -> io::Result<process::Output> {
    Ok(Command::new("git")
        .arg("config")
        .args(args.as_slice())
        .output()?)
}

/// Helper function to see if a `git config <value>` command is set
fn git_config_exists(key: &str) -> io::Result<bool> {
    let output = run_git_config_command(vec![key])?;
    let result = String::from_utf8(output.stdout).unwrap();

    Ok(!result.is_empty())
}

/// Create a repo from the fixture to be used in git module tests
pub fn create_fixture_repo() -> io::Result<std::path::PathBuf> {
    let fixture_repo_dir = new_tempdir()?.path().join("fixture");
    let fixture = env::current_dir()?.join("tests/fixtures/rocket.bundle");

    if !git_config_exists("user.email")? {
        run_git_config_command(vec!["--global", "user.email", "starship@example.com"])?;
    }

    if !git_config_exists("user.name")? {
        run_git_config_command(vec!["--global", "user.name", "starship"])?;
    }

    Command::new("git")
        .args(&[
            "clone",
            "-b",
            "master",
            &fixture.to_str().unwrap(),
            fixture_repo_dir.to_str().unwrap(),
        ])
        .output()?;

    Ok(fixture_repo_dir)
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
