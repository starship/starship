use crate::config::StarshipConfig;
use crate::context::{Context, Shell};
use once_cell::sync::Lazy;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

static FIXTURE_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/test/fixtures/"));

static GIT_FIXTURE: Lazy<PathBuf> = Lazy::new(|| FIXTURE_DIR.join("git-repo.bundle"));
static HG_FIXTURE: Lazy<PathBuf> = Lazy::new(|| FIXTURE_DIR.join("hg-repo.bundle"));

/// Render a specific starship module by name
pub struct ModuleRenderer<'a> {
    name: &'a str,
    context: Context<'a>,
}

impl<'a> ModuleRenderer<'a> {
    /// Creates a new ModuleRenderer
    pub fn new(name: &'a str) -> Self {
        let mut context = Context::new_with_dir(clap::ArgMatches::default(), PathBuf::new());
        context.shell = Shell::Unknown;
        context.config = StarshipConfig { config: None };

        Self { name, context }
    }

    pub fn path<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.context.current_dir = path.into();
        self
    }

    /// Sets the config of the underlying context
    pub fn config(mut self, config: toml::Value) -> Self {
        self.context.config = StarshipConfig {
            config: Some(config),
        };
        self
    }

    /// Adds the variable to the env_mocks of the underlying context
    pub fn env<V: Into<String>>(mut self, key: &'a str, val: V) -> Self {
        self.context.env.insert(key, val.into());
        self
    }

    pub fn shell(mut self, shell: Shell) -> Self {
        self.context.shell = shell;
        self
    }

    pub fn jobs(mut self, jobs: u64) -> Self {
        self.context.properties.insert("jobs", jobs.to_string());
        self
    }

    pub fn cmd_duration(mut self, duration: u64) -> Self {
        self.context
            .properties
            .insert("cmd_duration", duration.to_string());
        self
    }

    pub fn keymap<T>(mut self, keymap: T) -> Self
    where
        T: Into<String>,
    {
        self.context.properties.insert("keymap", keymap.into());
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.context
            .properties
            .insert("status_code", status.to_string());
        self
    }

    /// Renders the module returning its output
    pub fn collect(self) -> Option<String> {
        let ret = crate::print::get_module(self.name, self.context);
        // all tests rely on the fact that an empty module produces None as output as the
        // convention was that there would be no module but None. This is nowadays not anymore
        // the case (to get durations for all modules). So here we make it so, that an empty
        // module returns None in the tests...
        ret.filter(|s| s != "")
    }
}

pub enum FixtureProvider {
    GIT,
    HG,
}

pub fn fixture_repo(provider: FixtureProvider) -> io::Result<TempDir> {
    match provider {
        FixtureProvider::GIT => {
            let path = tempfile::tempdir()?;

            Command::new("git")
                .current_dir(path.path())
                .args(&["clone", "-b", "master"])
                .arg(GIT_FIXTURE.as_os_str())
                .arg(&path.path())
                .output()?;

            Command::new("git")
                .args(&["config", "--local", "user.email", "starship@example.com"])
                .current_dir(&path.path())
                .output()?;

            Command::new("git")
                .args(&["config", "--local", "user.name", "starship"])
                .current_dir(&path.path())
                .output()?;

            Command::new("git")
                .args(&["reset", "--hard", "HEAD"])
                .current_dir(&path.path())
                .output()?;

            Ok(path)
        }
        FixtureProvider::HG => {
            let path = tempfile::tempdir()?;

            Command::new("hg")
                .current_dir(path.path())
                .arg("clone")
                .arg(HG_FIXTURE.as_os_str())
                .arg(&path.path())
                .output()?;

            Ok(path)
        }
    }
}
