use crate::context::{Context, PromptMode, Shell};
use crate::logger::StarshipLogger;
use crate::{config::StarshipConfig, utils::CommandOutput};
use log::{Level, LevelFilter};
use once_cell::sync::Lazy;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

static FIXTURE_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/test/fixtures/"));

static GIT_FIXTURE: Lazy<PathBuf> = Lazy::new(|| FIXTURE_DIR.join("git-repo.bundle"));
static HG_FIXTURE: Lazy<PathBuf> = Lazy::new(|| FIXTURE_DIR.join("hg-repo.bundle"));

static LOGGER: Lazy<()> = Lazy::new(|| {
    let mut logger = StarshipLogger::default();

    // Don't log to files during tests
    let nul = if cfg!(windows) { "nul" } else { "/dev/null" };
    let nul = PathBuf::from(nul);

    // Maxmimum log level
    log::set_max_level(LevelFilter::Trace);
    logger.set_log_level(Level::Trace);
    logger.set_log_file_path(nul);

    log::set_boxed_logger(Box::new(logger)).unwrap();
});

/// Render a specific starship module by name
pub struct ModuleRenderer<'a> {
    name: &'a str,
    context: Context<'a>,
}

impl<'a> ModuleRenderer<'a> {
    /// Creates a new ModuleRenderer
    pub fn new(name: &'a str) -> Self {
        // Start logger
        Lazy::force(&LOGGER);

        let mut context = Context::new_with_shell_and_path(
            clap::ArgMatches::default(),
            Shell::Unknown,
            PathBuf::new(),
            PathBuf::new(),
        );
        context.config = StarshipConfig { config: None };

        Self { name, context }
    }

    /// Sets the terminal width of the underlying context
    pub fn prompt_mode(mut self, prompt_mode: PromptMode) -> Self {
        self.context.prompt_mode = prompt_mode;
        self
    }

    pub fn path<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.context.current_dir = path.into();
        self.context.logical_dir = self.context.current_dir.clone();
        self
    }

    pub fn logical_path<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.context.logical_dir = path.into();
        self
    }

    /// Sets the config of the underlying context
    pub fn config(mut self, config: toml::Value) -> Self {
        self.context.config = StarshipConfig {
            config: Some(config),
        };
        self
    }

    /// Sets the terminal width of the underlying context
    pub fn width(mut self, width: usize) -> Self {
        self.context.width = width;
        self
    }

    /// Adds the variable to the env_mocks of the underlying context
    pub fn env<V: Into<String>>(mut self, key: &'a str, val: V) -> Self {
        self.context.env.insert(key, val.into());
        self
    }

    /// Adds the command to the commandv_mocks of the underlying context
    pub fn cmd(mut self, key: &'a str, val: Option<CommandOutput>) -> Self {
        self.context.cmd.insert(key, val);
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

    #[cfg(feature = "battery")]
    pub fn battery_info_provider(
        mut self,
        battery_info_provider: &'a (dyn crate::modules::BatteryInfoProvider + Send + Sync),
    ) -> Self {
        self.context.battery_info_provider = battery_info_provider;
        self
    }

    /// Renders the prompt returning its output
    pub fn prompt(self) -> String {
        crate::print::get_prompt(self.context)
    }

    /// Renders the module returning its output
    pub fn collect(self) -> Option<String> {
        let ret = crate::print::get_module(self.name, self.context);
        // all tests rely on the fact that an empty module produces None as output as the
        // convention was that there would be no module but None. This is nowadays not anymore
        // the case (to get durations for all modules). So here we make it so, that an empty
        // module returns None in the tests...
        ret.filter(|s| !s.is_empty())
    }
}

pub enum FixtureProvider {
    Git,
    Hg,
}

pub fn fixture_repo(provider: FixtureProvider) -> io::Result<TempDir> {
    match provider {
        FixtureProvider::Git => {
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
        FixtureProvider::Hg => {
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
