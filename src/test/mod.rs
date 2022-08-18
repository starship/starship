use crate::context::{Context, Shell, Target};
use crate::logger::StarshipLogger;
use crate::{
    config::{ModuleConfig, StarshipConfig},
    configs::StarshipRootConfig,
    utils::{create_command, CommandOutput},
};
use log::{Level, LevelFilter};
use once_cell::sync::Lazy;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::{self, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
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

pub fn default_context() -> Context<'static> {
    let mut context = Context::new_with_shell_and_path(
        Default::default(),
        Shell::Unknown,
        Target::Main,
        PathBuf::new(),
        PathBuf::new(),
    );
    context.config = StarshipConfig { config: None };
    context
}

pub fn create_repo() -> io::Result<tempfile::TempDir> {
    let repo_dir = tempfile::tempdir()?;
    let path = repo_dir.path();
    let file_in_repo = repo_dir.path().join("some_file");

    // let write_file = |text: &str| {
    //     let mut file = OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .truncate(true)
    //         .open(&file_in_repo)?;
    //     writeln!(file, "{}", text)
    // };

    // Initialize a new git repo
    run_git_cmd(
        &[
            "init",
            "--quiet",
            path.to_str().expect("Path was not UTF-8"),
        ],
        None,
        true,
    )?;

    // Set local author info
    run_git_cmd(
        &["config", "--local", "user.email", "starship@example.com"],
        Some(path),
        true,
    )?;
    run_git_cmd(
        &["config", "--local", "user.name", "starship"],
        Some(path),
        true,
    )?;
            // Prevent intermittent test failures and ensure that the result of git commands
            // are available during I/O-contentious tests, by having git run `fsync`.
            // This is especially important on Windows.
            // Newer, more far-reaching git setting for `fsync`, that's not yet widely supported:
            create_command("git")?
                .args(&["config", "--local", "core.fsync", "all"])
                .current_dir(&path.path())
                .output()?;

            // Older git setting for `fsync` for compatibility with older git versions:
            create_command("git")?
                .args(&["config", "--local", "core.fsyncObjectFiles", "true"])
                .current_dir(&path.path())
                .output()?;

    // Ensure on the expected branch.
    // If build environment has `init.defaultBranch` global set
    // it will default to an unknown branch, so need to make & change branch
    run_git_cmd(
        &["checkout", "-b", "master"],
        Some(path),
        // command expected to fail if already on the expected branch
        false,
    )?;

    // Write a file on master and commit it
    write_file(file_in_repo, "First Line\nSecond Line\nThird Line")?;
    run_git_cmd(&["add", "some_file"], Some(path), true)?;
    run_git_cmd(
        &["commit", "--message", "Commit A", "--no-gpg-sign"],
        Some(path),
        true,
    )?;

    Ok(repo_dir)
}

pub fn run_git_cmd<A, S>(args: A, dir: Option<&Path>, should_succeed: bool) -> io::Result<()>
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = create_command("git")?;
    command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    if let Some(dir) = dir {
        command.current_dir(dir);
    }

    let status = command.status()?;

    if should_succeed && !status.success() {
        Err(Error::from(ErrorKind::Other))
    } else {
        Ok(())
    }
}

pub fn write_file(file: PathBuf, text: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file)?;
    writeln!(file, "{}", text)?;
    file.sync_all()
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
        Lazy::force(&LOGGER);

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
    pub fn config(mut self, config: toml::Value) -> Self {
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
    Git,
    Hg,
}

pub fn fixture_repo(provider: FixtureProvider) -> io::Result<TempDir> {
    match provider {
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
    }
}
