use crate::context::{Context, Env, Properties, Shell, Target};
use crate::logger::StarshipLogger;
use crate::{
    config::StarshipConfig,
    utils::{CommandOutput, create_command},
};
use log::{Level, LevelFilter};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;
use std::sync::Once;
use tempfile::TempDir;

static FIXTURE_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/test/fixtures/"));

static GIT_FIXTURE: LazyLock<PathBuf> = LazyLock::new(|| FIXTURE_DIR.join("git-repo.bundle"));
static GIT_SHA256_FIXTURE: LazyLock<PathBuf> =
    LazyLock::new(|| FIXTURE_DIR.join("git-repo-sha256.bundle"));
static HG_FIXTURE: LazyLock<PathBuf> = LazyLock::new(|| FIXTURE_DIR.join("hg-repo.bundle"));

static LOGGER: Once = Once::new();

static TEST_GIT_CONFIG: &[(&str, &str)] = &[
    // Dummy user
    ("user.email", "starship@example.com"),
    ("user.name", "starship"),
    // Prevent intermittent test failures and ensure that the result of git commands
    // are available during I/O-contentious tests, by having git run `fsync`.
    // This is especially important on Windows.
    // Newer, more far-reaching git setting for `fsync`, that's not yet widely supported:
    ("core.fsync", "all"),
    // Older git setting for `fsync` for compatibility with older git versions:
    ("core.fsyncObjectFiles", "true"),
    // Disable signing
    ("commit.gpgsign", "false"),
    ("tag.gpgsign", "false"),
];

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
        Properties::default(),
        Shell::Unknown,
        Target::Main,
        PathBuf::new(),
        PathBuf::new(),
        Env::default(),
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

    /// Creates a new `ModuleRenderer` with `HOME` set to a `TempDir`
    pub fn new_with_home(name: &'a str) -> io::Result<(Self, tempfile::TempDir)> {
        let module_renderer = ModuleRenderer::new(name);
        let homedir = tempfile::tempdir()?;
        let home = dunce::canonicalize(homedir.path())?;
        Ok((module_renderer.env("HOME", home.to_str().unwrap()), homedir))
    }

    pub fn path<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.context.current_dir = path.into();
        self.context
            .logical_dir
            .clone_from(&self.context.current_dir);
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
        self.context = self.context.set_config(config);
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

    pub fn width(mut self, width: usize) -> Self {
        self.context.width = width;
        self
    }

    pub fn claude_code_data(mut self, data: crate::context::ClaudeCodeData) -> Self {
        self.context.claude_code_data = Some(Box::new(data));
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
        let ret = crate::print::get_module(self.name, &self.context);
        // all tests rely on the fact that an empty module produces None as output as the
        // convention was that there would be no module but None. This is nowadays not anymore
        // the case (to get durations for all modules). So here we make it so, that an empty
        // module returns None in the tests...
        ret.filter(|s| !s.is_empty())
    }
}

impl<'a> From<ModuleRenderer<'a>> for Context<'a> {
    fn from(renderer: ModuleRenderer<'a>) -> Self {
        renderer.context
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FixtureProvider {
    Fossil,
    Git { reftable: bool, bare: bool },
    Hg,
    Pijul,
}

pub const COMMON_GIT_PROVIDERS: &[FixtureProvider] = &[
    FixtureProvider::Git {
        reftable: false,
        bare: false,
    },
    FixtureProvider::Git {
        reftable: true,
        bare: false,
    },
];

pub const BARE_GIT_PROVIDERS: &[FixtureProvider] = &[
    FixtureProvider::Git {
        reftable: false,
        bare: true,
    },
    FixtureProvider::Git {
        reftable: true,
        bare: true,
    },
];

pub fn config_cmd_for_tests(cmd: &mut Command) {
    for (key, value) in TEST_GIT_CONFIG {
        cmd.args(["-c", &format!("{key}={value}")]);
    }
}

pub fn config_git_repo_for_tests(path: &Path) -> io::Result<()> {
    for (key, value) in TEST_GIT_CONFIG {
        create_command("git")?
            .args(["config", "--local", key, value])
            .current_dir(path)
            .output()?;
    }
    Ok(())
}

pub fn fixture_repo(provider: FixtureProvider) -> io::Result<TempDir> {
    fixture_repo_with_hash(provider, rand::random())
}

pub fn fixture_repo_with_hash(provider: FixtureProvider, sha256: bool) -> io::Result<TempDir> {
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
                .truncate(false)
                .open(path.path().join(checkout_db))?
                .sync_all()?;
            Ok(path)
        }
        FixtureProvider::Git { reftable, bare } => {
            let path = tempfile::tempdir()?;

            let fixture = if sha256 {
                &GIT_SHA256_FIXTURE
            } else {
                &GIT_FIXTURE
            };

            let mut command = create_command("git")?;
            command
                .current_dir(path.path())
                .arg("clone")
                .args(reftable.then_some("--ref-format=reftable"))
                .args(["-b", "master"]);

            if bare {
                command.arg("--bare");
            }

            config_cmd_for_tests(&mut command);

            command.arg(fixture.as_os_str()).arg(path.path()).output()?;

            config_git_repo_for_tests(path.path())?;
            if !bare {
                create_command("git")?
                    .args(["reset", "--hard", "HEAD"])
                    .current_dir(path.path())
                    .output()?;
            }

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
