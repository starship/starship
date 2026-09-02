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
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{LazyLock, Once};
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
    ("user.email", "starship@example.com"),
    ("user.name", "starship"),
    ("core.fsync", "all"),             // Prevent intermittent I/O test failures
    ("core.fsyncObjectFiles", "true"), // Fallback for older git versions
    ("commit.gpgsign", "false"),
    ("tag.gpgsign", "false"),
];

fn init_logger() {
    let mut logger = StarshipLogger::default();
    let null_path = PathBuf::from(if cfg!(windows) { "nul" } else { "/dev/null" });

    log::set_max_level(LevelFilter::Trace);
    logger.set_log_level(Level::Trace);
    logger.set_log_file_path(null_path);
    log::set_boxed_logger(Box::new(logger)).unwrap();
}

/// A clean context built purely from starship's defaults, isolated from the ambient environment.
pub fn default_context() -> Context<'static> {
    Context::new_with_config(
        Properties::default(),
        Shell::Unknown,
        Target::Main,
        PathBuf::new(),
        PathBuf::new(),
        Env::default(),
        StarshipConfig { config: None },
    )
}

pub struct ModuleRenderer<'a> {
    name: &'a str,
    context: Context<'a>,
}

impl<'a> ModuleRenderer<'a> {
    pub fn new(name: &'a str) -> Self {
        LOGGER.call_once(init_logger);
        Self {
            name,
            context: default_context(),
        }
    }

    pub fn new_with_home(name: &'a str) -> io::Result<(Self, TempDir)> {
        let homedir = tempfile::tempdir()?;
        let home = dunce::canonicalize(homedir.path())?;
        let renderer = Self::new(name).env("HOME", home.to_str().unwrap());
        Ok((renderer, homedir))
    }

    pub fn path<T: Into<PathBuf>>(mut self, path: T) -> Self {
        self.context.current_dir = path.into();
        self.context
            .logical_dir
            .clone_from(&self.context.current_dir);
        self
    }

    pub fn logical_path<T: Into<PathBuf>>(mut self, path: T) -> Self {
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
    pub fn env<V: Into<String>>(mut self, key: &'a str, val: V) -> Self {
        self.context.env.insert(key, val.into());
        self
    }
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
    pub fn keymap<T: Into<String>>(mut self, keymap: T) -> Self {
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
    pub fn pipestatus(mut self, status: &[i64]) -> Self {
        self.context.properties.pipestatus = Some(status.iter().map(ToString::to_string).collect());
        self
    }

    #[cfg(feature = "battery")]
    pub fn battery_info_provider(
        mut self,
        provider: &'a (dyn crate::modules::BatteryInfoProvider + Send + Sync),
    ) -> Self {
        self.context.battery_info_provider = provider;
        self
    }

    pub fn root_path(&self) -> &Path {
        self.context.root_dir.path()
    }

    /// Renders the module, returning its output (if any).
    pub fn collect(self) -> Option<String> {
        crate::print::get_module(self.name, &self.context).filter(|s| !s.is_empty())
    }
}

impl<'a> From<ModuleRenderer<'a>> for Context<'a> {
    fn from(renderer: ModuleRenderer<'a>) -> Self {
        renderer.context
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderedModule {
    Empty,
    Styled(String),
}

impl From<Option<String>> for RenderedModule {
    fn from(collected: Option<String>) -> Self {
        collected.map_or(Self::Empty, Self::Styled)
    }
}

impl fmt::Display for RenderedModule {
    /// Renders output safely for snapshotting (escaped strings reveal ANSI codes).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("<no output>"),
            Self::Styled(text) => write!(f, "{text:?}"),
        }
    }
}

/// A throwaway project directory that owns its temporary backing path.
pub struct Project {
    directory: TempDir,
}

impl Project {
    pub fn new() -> Self {
        Self {
            directory: TempDir::new().expect("failed to create temporary project directory"),
        }
    }

    pub fn inside(parent: &Path) -> Self {
        fs::create_dir_all(parent).expect("failed to create parent directory");
        Self {
            directory: TempDir::new_in(parent)
                .expect("failed to create temporary project directory"),
        }
    }

    pub fn from_repository_fixture(provider: FixtureProvider) -> Self {
        Self {
            directory: fixture_repo(provider).expect("failed to create fixture"),
        }
    }

    pub fn path(&self) -> &Path {
        self.directory.path()
    }

    pub fn directory_name(&self) -> String {
        self.path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    }

    pub fn create_file(&self, relative_path: impl AsRef<Path>) -> &Self {
        self.write_file(relative_path, "")
    }

    pub fn write_file(&self, relative_path: impl AsRef<Path>, contents: &str) -> &Self {
        let path = self.path().join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("failed to create parent directory");
        }

        let mut file = fs::File::create(&path).expect("failed to create file");
        file.write_all(contents.as_bytes())
            .expect("failed to write file");
        file.sync_all().expect("failed to flush file");
        self
    }

    pub fn create_directory(&self, relative_path: impl AsRef<Path>) -> &Self {
        fs::create_dir_all(self.path().join(relative_path)).expect("failed to create directory");
        self
    }

    pub fn renderer<'a>(&self, module_name: &'a str) -> ModuleRenderer<'a> {
        ModuleRenderer::new(module_name).path(self.path())
    }

    pub fn render(&self, module_name: &str) -> RenderedModule {
        self.renderer(module_name).collect().into()
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

#[fixture]
pub fn project() -> Project {
    Project::new()
}

#[fixture]
pub fn home_project() -> Project {
    Project::inside(&crate::utils::home_dir().expect("tests require a resolvable home directory"))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FixtureProvider {
    Fossil,
    Git { reftable: bool, bare: bool },
    Hg,
    Pijul,
}

impl FixtureProvider {
    pub const GIT: Self = Self::Git {
        reftable: false,
        bare: false,
    };
    pub const GIT_REFTABLE: Self = Self::Git {
        reftable: true,
        bare: false,
    };
    pub const BARE_GIT: Self = Self::Git {
        reftable: false,
        bare: true,
    };
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
    let dir = tempfile::tempdir()?;
    let path = dir.path();

    match provider {
        FixtureProvider::Fossil => {
            let db = if cfg!(windows) {
                "_FOSSIL_"
            } else {
                ".fslckout"
            };
            fs::create_dir(path.join("subdir"))?;
            fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path.join(db))?
                .sync_all()?;
        }
        FixtureProvider::Git { reftable, bare } => {
            let fixture = if sha256 {
                &*GIT_SHA256_FIXTURE
            } else {
                &*GIT_FIXTURE
            };
            let mut cmd = create_command("git")?;

            cmd.current_dir(path)
                .arg("clone")
                .args(reftable.then_some("--ref-format=reftable"))
                .args(["-b", "master"]);

            if bare {
                cmd.arg("--bare");
            }
            config_cmd_for_tests(&mut cmd);
            cmd.arg(fixture.as_os_str()).arg(path).output()?;
            config_git_repo_for_tests(path)?;

            if !bare {
                create_command("git")?
                    .args(["reset", "--hard", "HEAD"])
                    .current_dir(path)
                    .output()?;
            }
        }
        FixtureProvider::Hg => {
            create_command("hg")?
                .current_dir(path)
                .args(["clone", HG_FIXTURE.to_str().unwrap()])
                .arg(path)
                .output()?;
        }
        FixtureProvider::Pijul => {
            fs::create_dir(path.join(".pijul"))?;
        }
    }
    Ok(dir)
}

/// The regression net for test-suite hermeticity.
#[cfg(test)]
mod hermeticity {
    use super::{ModuleRenderer, default_context};
    use crate::configs::StarshipRootConfig;
    use crate::context::{Context, Env, Properties, Shell, Target};
    use std::ffi::OsString;
    use std::path::PathBuf;

    const LOUD_CONFIG: &str = r##"
add_newline = false
palette = "planted"

[palettes.planted]
green = "#ff0000"
red = "#00ff00"

[nodejs]
format = "PLANTED"
"##;

    #[test]
    fn test_build_ignores_ambient_config() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let config_path = dir.path().join("starship.toml");
        std::fs::write(&config_path, LOUD_CONFIG).expect("failed to plant config");

        let mut env = Env::default();
        env.insert("STARSHIP_CONFIG", config_path.display().to_string());

        let context = Context::new_with_shell_and_path(
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            PathBuf::new(),
            PathBuf::new(),
            env,
        );

        assert_eq!(
            context.get_config_path_os(),
            Some(OsString::from(config_path.display().to_string()))
        );
        assert!(
            context.config.config.is_none(),
            "Test leaked hermeticity and read the planted config!"
        );
        assert_eq!(
            root_config_text(&context.root_config),
            root_config_text(&StarshipRootConfig::default())
        );
    }

    #[test]
    fn test_harness_carries_no_config() {
        let context = default_context();
        assert!(
            context.config.config.is_none(),
            "default_context() carries a configuration"
        );
        assert_eq!(
            root_config_text(&context.root_config),
            root_config_text(&StarshipRootConfig::default())
        );
    }

    #[test]
    fn test_rendered_modules_use_defaults() {
        let rendered = ModuleRenderer::new("character")
            .config(toml::toml! {
                [character]
                success_symbol = "[>](green)"
            })
            .collect()
            .expect("the character module always renders");

        assert!(
            rendered.contains("\u{1b}[32m"),
            "Expected ANSI green, found overridden palette color: {rendered:?}"
        );
    }

    fn root_config_text(config: &StarshipRootConfig) -> String {
        toml::to_string(config).expect("root configuration must serialize")
    }
}
