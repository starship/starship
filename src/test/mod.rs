use crate::context::{Context, Env, JJRepo, Properties, Shell, Target};
use crate::logger::StarshipLogger;
use crate::{
    config::StarshipConfig,
    utils::{CommandOutput, create_command},
};
use log::{Level, LevelFilter};
use rstest::fixture;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;
use std::sync::Once;
use tempfile::TempDir;

mod jj_tester;
pub(crate) use jj_tester::JJTester;

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

    /// Init at `JJRepo` with a mocked path, allowing to test JJ modules in several situations:
    /// valid repo, invalid repo, no repo at all.
    pub fn jj_repo<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.context.set_jj_repo(JJRepo::with_root(path.into()));
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

/// What a starship module produced when rendered.
///
/// This exists so a rendered module can be compared and snapshotted as a value
/// with an explicit "the module rendered nothing" case, rather than as an
/// `Option<String>` whose `None` and `Some("")` cases both mean "nothing" in
/// different places.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderedModule {
    /// The module declined to render, or rendered nothing at all.
    Empty,
    /// The module rendered the contained text, ANSI styling escape sequences
    /// included. The styling is part of what module tests assert on, so it is
    /// never stripped.
    Styled(String),
}

impl From<Option<String>> for RenderedModule {
    fn from(collected: Option<String>) -> Self {
        match collected {
            None => Self::Empty,
            Some(text) => Self::Styled(text),
        }
    }
}

impl fmt::Display for RenderedModule {
    /// Renders the module output in a form that is safe and reviewable inside a
    /// snapshot.
    ///
    /// Rendered output is written as a quoted, escaped Rust string literal, for
    /// two reasons. Control characters — above all the ANSI escape sequences
    /// that carry the styling contract — become visible `\u{1b}` text instead
    /// of raw bytes a reviewer cannot see in a diff. And the surrounding quotes
    /// pin down the exact extent of the output: snapshot files are stored with
    /// trailing whitespace trimmed, so an unquoted rendering would silently
    /// discard the trailing space that most module formats end with, and a
    /// regression that dropped it would go unnoticed.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("<no output>"),
            Self::Styled(text) => write!(formatter, "{text:?}"),
        }
    }
}

/// A throwaway project directory that owns the temporary directory backing it
/// and renders starship modules against itself.
///
/// This replaces the `tempfile::tempdir()?` / `File::create(..)?.sync_all()?` /
/// `directory.close()` preamble that otherwise forces every module test to
/// return `io::Result<()>` purely as plumbing. The temporary directory is
/// removed when the `Project` is dropped, and a set-up failure panics
/// immediately, naming the path that failed, instead of being threaded through
/// a `Result` that says nothing about which step broke.
pub struct Project {
    directory: TempDir,
}

impl Project {
    /// Creates a project in a fresh temporary directory under the platform's
    /// temporary-file root.
    pub fn new() -> Self {
        Self {
            directory: TempDir::new().expect("failed to create a temporary project directory"),
        }
    }

    /// Creates a project in a fresh temporary directory nested inside
    /// `parent_directory`, for tests whose expectations depend on where the
    /// project lives (under the user's home directory, under a known root, ...).
    pub fn inside(parent_directory: &Path) -> Self {
        fs::create_dir_all(parent_directory).unwrap_or_else(|error| {
            panic!(
                "failed to create parent directory {}: {error}",
                parent_directory.display()
            )
        });
        Self {
            directory: TempDir::new_in(parent_directory).unwrap_or_else(|error| {
                panic!(
                    "failed to create a temporary project directory inside {}: {error}",
                    parent_directory.display()
                )
            }),
        }
    }

    /// The absolute path of this project's directory.
    pub fn path(&self) -> &Path {
        self.directory.path()
    }

    /// Creates an empty file at `relative_path`, together with any missing
    /// parent directories, and flushes it to disk before returning.
    ///
    /// The flush matters: several modules detect a project by reading the
    /// directory, and an unflushed file has been observed to be invisible to
    /// the freshly spawned processes some modules use.
    pub fn create_file(&self, relative_path: impl AsRef<Path>) -> &Self {
        self.write_file(relative_path, "")
    }

    /// Creates a file at `relative_path` holding `contents`, together with any
    /// missing parent directories, and flushes it to disk before returning.
    pub fn write_file(&self, relative_path: impl AsRef<Path>, contents: &str) -> &Self {
        use std::io::Write;

        let path = self.path().join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|error| {
                panic!("failed to create directory {}: {error}", parent.display())
            });
        }
        let mut file = fs::File::create(&path)
            .unwrap_or_else(|error| panic!("failed to create file {}: {error}", path.display()));
        file.write_all(contents.as_bytes())
            .unwrap_or_else(|error| panic!("failed to write file {}: {error}", path.display()));
        file.sync_all()
            .unwrap_or_else(|error| panic!("failed to flush file {}: {error}", path.display()));
        self
    }

    /// Creates a directory at `relative_path`, together with any missing parent
    /// directories.
    pub fn create_directory(&self, relative_path: impl AsRef<Path>) -> &Self {
        let path = self.path().join(relative_path);
        fs::create_dir_all(&path).unwrap_or_else(|error| {
            panic!("failed to create directory {}: {error}", path.display())
        });
        self
    }

    /// A [`ModuleRenderer`] already pointed at this project's directory, for
    /// tests that need to configure the render further before collecting it.
    pub fn renderer<'a>(&self, module_name: &'a str) -> ModuleRenderer<'a> {
        ModuleRenderer::new(module_name).path(self.path())
    }

    /// Renders `module_name` with this project's directory as the current
    /// directory.
    pub fn render(&self, module_name: &str) -> RenderedModule {
        self.renderer(module_name).collect().into()
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

/// An `rstest` fixture supplying a fresh, empty [`Project`].
#[fixture]
pub fn project() -> Project {
    Project::new()
}

/// An `rstest` fixture supplying a fresh [`Project`] nested inside the current
/// user's home directory, for the modules whose output depends on the rendered
/// path lying under `~`.
#[fixture]
pub fn home_project() -> Project {
    Project::inside(
        crate::utils::home_dir()
            .expect("tests require a resolvable home directory")
            .as_path(),
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FixtureProvider {
    Fossil,
    Git { reftable: bool, bare: bool },
    Hg,
    Pijul,
}

impl FixtureProvider {
    /// A working git checkout using the classic loose/packed reference files.
    pub const GIT: Self = Self::Git {
        reftable: false,
        bare: false,
    };
    /// A working git checkout using the newer `reftable` reference backend.
    pub const GIT_REFTABLE: Self = Self::Git {
        reftable: true,
        bare: false,
    };
    /// A bare git repository using the classic loose/packed reference files.
    pub const BARE_GIT: Self = Self::Git {
        reftable: false,
        bare: true,
    };
    /// A bare git repository using the newer `reftable` reference backend.
    pub const BARE_GIT_REFTABLE: Self = Self::Git {
        reftable: true,
        bare: true,
    };
}

pub const COMMON_GIT_PROVIDERS: &[FixtureProvider] =
    &[FixtureProvider::GIT, FixtureProvider::GIT_REFTABLE];

pub const BARE_GIT_PROVIDERS: &[FixtureProvider] = &[
    FixtureProvider::BARE_GIT,
    FixtureProvider::BARE_GIT_REFTABLE,
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
