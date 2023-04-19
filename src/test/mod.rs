use crate::context::{Context, Shell, Target};
use crate::logger::StarshipLogger;
use crate::{
    config::{ModuleConfig, StarshipConfig},
    configs::StarshipRootConfig,
    utils::{create_command, CommandOutput},
};
use log::{Level, LevelFilter};
use once_cell::sync::Lazy;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Once;
use tempfile::TempDir;

static FIXTURE_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/test/fixtures/"));

static GIT_FIXTURE: Lazy<PathBuf> = Lazy::new(|| FIXTURE_DIR.join("git-repo.bundle"));
static HG_FIXTURE: Lazy<PathBuf> = Lazy::new(|| FIXTURE_DIR.join("hg-repo.bundle"));

static LOGGER: Once = Once::new();

fn init_logger() {
    let mut logger = StarshipLogger::default();

    // Don't log to files during tests
    let nul = if cfg!(windows) { "nul" } else { "/dev/null" };
    let nul = PathBuf::from(nul);

    // Maximum log level
    log::set_max_level(LevelFilter::Trace);
    logger.set_log_level(Level::Trace);
    logger.set_log_file_path(nul);

    log::set_boxed_logger(Box::new(logger)).unwrap();
}

pub fn default_context() -> Context<'static> {
    let mut context = Context::new_with_shell_and_path(
        Default::default(),
        Shell::Unknown,
        Target::Main,
        PathBuf::new(),
        PathBuf::new(),
        Default::default(),
    );
    context.config = StarshipConfig { config: None };
    context
}

/// Render a specific starship module by name
pub struct ModuleRenderer<'a> {
    name: &'a str,
    context: Context<'a>,
}

impl<'a> ModuleRenderer<'a> {
    /// Creates a new `ModuleRenderer`
    pub fn new(name: &'a str) -> Self {
        // Start logger
        LOGGER.call_once(init_logger);

        let context = default_context();

        Self { name, context }
    }

    pub fn path<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.context.current_dir = path.into();
        self.context.logical_dir = self.context.current_dir.clone();
        self
    }

    pub fn root_path(&self) -> &Path {
        self.context.root_dir.path()
    }

    pub fn logical_path<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.context.logical_dir = path.into();
        self
    }

    /// Sets the config of the underlying context
    pub fn config(mut self, config: toml::Table) -> Self {
        self.context.root_config = StarshipRootConfig::load(&config);
        self.context.config = StarshipConfig {
            config: Some(config),
        };
        self
    }

    /// Adds the variable to the `env_mocks` of the underlying context
    pub fn env<V: Into<String>>(mut self, key: &'a str, val: V) -> Self {
        self.context.env.insert(key, val.into());
        self
    }

    /// Adds the command to the `command_mocks` of the underlying context
    pub fn cmd(mut self, key: &'a str, val: Option<CommandOutput>) -> Self {
        self.context.cmd.insert(key, val);
        self
    }

    pub fn shell(mut self, shell: Shell) -> Self {
        self.context.shell = shell;
        self
    }

    pub fn jobs(mut self, jobs: i64) -> Self {
        self.context.properties.jobs = jobs;
        self
    }

    pub fn cmd_duration(mut self, duration: u64) -> Self {
        self.context.properties.cmd_duration = Some(duration.to_string());
        self
    }

    pub fn keymap<T>(mut self, keymap: T) -> Self
    where
        T: Into<String>,
    {
        self.context.properties.keymap = keymap.into();
        self
    }

    pub fn status(mut self, status: i64) -> Self {
        self.context.properties.status_code = Some(status.to_string());
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

    pub fn pipestatus(mut self, status: &[i64]) -> Self {
        self.context.properties.pipestatus = Some(
            status
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        );
        self
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

#[derive(Clone, Copy)]
pub enum FixtureProvider {
    Fossil,
    Git,
    Hg,
    Pijul,
}

pub fn fixture_repo(provider: FixtureProvider) -> io::Result<TempDir> {
    match provider {
        FixtureProvider::Fossil => {
            let checkout_db = if cfg!(windows) {
                "_FOSSIL_"
            } else {
                ".fslckout"
            };
            let path = tempfile::tempdir()?;
            fs::create_dir(path.path().join("subdir"))?;
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(path.path().join(checkout_db))?
                .sync_all()?;
            Ok(path)
        }
        FixtureProvider::Git => {
            let path = tempfile::tempdir()?;

            create_command("git")?
                .current_dir(path.path())
                .args(["clone", "-b", "master"])
                .arg(GIT_FIXTURE.as_os_str())
                .arg(path.path())
                .output()?;

            create_command("git")?
                .args(["config", "--local", "user.email", "starship@example.com"])
                .current_dir(path.path())
                .output()?;

            create_command("git")?
                .args(["config", "--local", "user.name", "starship"])
                .current_dir(path.path())
                .output()?;

            // Prevent intermittent test failures and ensure that the result of git commands
            // are available during I/O-contentious tests, by having git run `fsync`.
            // This is especially important on Windows.
            // Newer, more far-reaching git setting for `fsync`, that's not yet widely supported:
            create_command("git")?
                .args(["config", "--local", "core.fsync", "all"])
                .current_dir(path.path())
                .output()?;

            // Older git setting for `fsync` for compatibility with older git versions:
            create_command("git")?
                .args(["config", "--local", "core.fsyncObjectFiles", "true"])
                .current_dir(path.path())
                .output()?;

            create_command("git")?
                .args(["reset", "--hard", "HEAD"])
                .current_dir(path.path())
                .output()?;

            Ok(path)
        }
        FixtureProvider::Hg => {
            let path = tempfile::tempdir()?;

            create_command("hg")?
                .current_dir(path.path())
                .arg("clone")
                .arg(HG_FIXTURE.as_os_str())
                .arg(path.path())
                .output()?;

            Ok(path)
        }
        FixtureProvider::Pijul => {
            let path = tempfile::tempdir()?;
            fs::create_dir(path.path().join(".pijul"))?;
            Ok(path)
        }
    }
}
