use once_cell::sync::Lazy;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, io, process};

static MANIFEST_DIR: Lazy<&'static Path> = Lazy::new(|| Path::new(env!("CARGO_MANIFEST_DIR")));
static EMPTY_CONFIG: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("empty_config.toml"));

#[cfg(windows)]
const EXE_PATH: &str = "./target/debug/starship.exe";

#[cfg(not(windows))]
const EXE_PATH: &str = "./target/debug/starship";

/// Render the full starship prompt
pub fn render_prompt() -> process::Command {
    let mut command = process::Command::new(EXE_PATH);

    command
        .arg("prompt")
        .env_clear()
        .env("PATH", env!("PATH")) // Provide the $PATH variable so that external programs are runnable
        .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

    command
}

/// Render a specific starship module by name
pub fn render_module(module_name: &str) -> process::Command {
    let binary = fs::canonicalize(EXE_PATH).unwrap();
    let mut command = process::Command::new(binary);

    command
        .arg("module")
        .arg(module_name)
        .env_clear()
        .env("PATH", env!("PATH")) // Provide the $PATH variable so that external programs are runnable
        .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

    command
}

/// Create a repo from the fixture to be used in git module tests
pub fn create_fixture_repo() -> io::Result<PathBuf> {
    let fixture_repo_path = tempfile::tempdir()?.path().join("fixture");
    let repo_path = tempfile::tempdir()?.path().join("rocket");
    let fixture_path = env::current_dir()?.join("tests/fixtures/rocket.bundle");

    let fixture_repo_dir = path_str(&fixture_repo_path)?;
    let repo_dir = path_str(&repo_path)?;

    Command::new("git")
        .args(&["clone", "-b", "master"])
        .args(&[&fixture_path, &repo_path])
        .output()?;

    git2::Repository::clone(&fixture_repo_dir, &repo_dir).ok();

    Command::new("git")
        .args(&["config", "--local", "user.email", "starship@example.com"])
        .current_dir(&repo_path)
        .output()?;

    Command::new("git")
        .args(&["config", "--local", "user.name", "starship"])
        .current_dir(&repo_path)
        .output()?;

    Command::new("git")
        .args(&["reset", "--hard", "HEAD"])
        .current_dir(&repo_path)
        .output()?;

    Command::new("git")
        .args(&["tag", "tagOnHEAD", "-m", "Testing tags on this repo"])
        .current_dir(&repo_path)
        .output()?;

    Ok(repo_path)
}

fn path_str(repo_dir: &PathBuf) -> io::Result<String> {
    repo_dir
        .to_str()
        .ok_or_else(|| Error::from(ErrorKind::Other))
        .map(|i| i.replace("\\", "/"))
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
