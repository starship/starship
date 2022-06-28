use crate::config::{ModuleConfig, StarshipConfig};
use crate::configs::StarshipRootConfig;
use crate::module::Module;
use crate::utils::{create_command, exec_timeout, read_file, CommandOutput};

use crate::modules;
use crate::utils::{self, home_dir};
use clap::Parser;
use git2::{ErrorCode::UnbornBranch, Repository, RepositoryState};
use once_cell::sync::OnceCell;
#[cfg(test)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::fs;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::String;
use std::time::{Duration, Instant};
use terminal_size::terminal_size;

/// Context contains data or common methods that may be used by multiple modules.
/// The data contained within Context will be relevant to this particular rendering
/// of the prompt.
pub struct Context<'a> {
    /// The deserialized configuration map from the user's `starship.toml` file.
    pub config: StarshipConfig,

    /// The current working directory that starship is being called in.
    pub current_dir: PathBuf,

    /// A logical directory path which should represent the same directory as current_dir,
    /// though may appear different.
    /// E.g. when navigating to a PSDrive in PowerShell, or a path without symlinks resolved.
    pub logical_dir: PathBuf,

    /// A struct containing directory contents in a lookup-optimized format.
    dir_contents: OnceCell<DirContents>,

    /// Properties to provide to modules.
    pub properties: Properties,

    /// Private field to store Git information for modules who need it
    repo: OnceCell<Repo>,

    /// The shell the user is assumed to be running
    pub shell: Shell,

    /// Which prompt to print (main, right, ...)
    pub target: Target,

    /// Width of terminal, or zero if width cannot be detected.
    pub width: usize,

    /// A HashMap of environment variable mocks
    #[cfg(test)]
    pub env: HashMap<&'a str, String>,

    /// A HashMap of command mocks
    #[cfg(test)]
    pub cmd: HashMap<&'a str, Option<CommandOutput>>,

    /// a mock of the root directory
    #[cfg(test)]
    pub root_dir: tempfile::TempDir,

    #[cfg(feature = "battery")]
    pub battery_info_provider: &'a (dyn crate::modules::BatteryInfoProvider + Send + Sync),

    /// Starship root config
    pub root_config: StarshipRootConfig,

    /// Avoid issues with unused lifetimes when features are disabled
    _marker: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    /// Identify the current working directory and create an instance of Context
    /// for it. "logical-path" is used when a shell allows the "current working directory"
    /// to be something other than a file system path (like powershell provider specific paths).
    pub fn new(arguments: Properties, target: Target) -> Context<'a> {
        let shell = Context::get_shell();

        // Retrieve the "current directory".
        // If the path argument is not set fall back to the OS current directory.
        let path = arguments
            .path
            .clone()
            .or_else(|| env::current_dir().ok())
            .or_else(|| env::var("PWD").map(PathBuf::from).ok())
            .or_else(|| arguments.logical_path.clone())
            .unwrap_or_default();

        // Retrieve the "logical directory".
        // If the path argument is not set fall back to the PWD env variable set by many shells
        // or to the other path.
        let logical_path = arguments
            .logical_path
            .clone()
            .or_else(|| env::var("PWD").map(PathBuf::from).ok())
            .unwrap_or_else(|| path.clone());

        Context::new_with_shell_and_path(arguments, shell, target, path, logical_path)
    }

    /// Create a new instance of Context for the provided directory
    pub fn new_with_shell_and_path(
        mut properties: Properties,
        shell: Shell,
        target: Target,
        path: PathBuf,
        logical_path: PathBuf,
    ) -> Context<'a> {
        let config = StarshipConfig::initialize();

        // If the vector is zero-length, we should pretend that we didn't get a
        // pipestatus at all (since this is the input `--pipestatus=""`)
        if properties
            .pipestatus
            .as_deref()
            .map_or(false, |p| p.len() == 1 && p[0].is_empty())
        {
            properties.pipestatus = None;
        }
        log::trace!(
            "Received completed pipestatus of {:?}",
            properties.pipestatus
        );

        // If status-code is empty, set it to None
        if matches!(properties.status_code.as_deref(), Some("")) {
            properties.status_code = None;
        }

        // Canonicalize the current path to resolve symlinks, etc.
        // NOTE: On Windows this may convert the path to extended-path syntax.
        let current_dir = Context::expand_tilde(path);
        let current_dir = dunce::canonicalize(&current_dir).unwrap_or(current_dir);
        let logical_dir = logical_path;

        let root_config = config
            .config
            .as_ref()
            .map_or_else(StarshipRootConfig::default, StarshipRootConfig::load);

        let width = properties.terminal_width;

        Context {
            config,
            properties,
            current_dir,
            logical_dir,
            dir_contents: OnceCell::new(),
            repo: OnceCell::new(),
            shell,
            target,
            width,
            #[cfg(test)]
            root_dir: tempfile::TempDir::new().unwrap(),
            #[cfg(test)]
            env: HashMap::new(),
            #[cfg(test)]
            cmd: HashMap::new(),
            #[cfg(feature = "battery")]
            battery_info_provider: &crate::modules::BatteryInfoProviderImpl,
            root_config,
            _marker: PhantomData,
        }
    }

    // Tries to retrieve home directory from a table in testing mode or else retrieves it from the os
    pub fn get_home(&self) -> Option<PathBuf> {
        if cfg!(test) {
            return self.get_env("HOME").map(PathBuf::from).or_else(home_dir);
        }

        home_dir()
    }

    // Retrieves a environment variable from the os or from a table if in testing mode
    #[cfg(test)]
    pub fn get_env<K: AsRef<str>>(&self, key: K) -> Option<String> {
        self.env
            .get(key.as_ref())
            .map(std::string::ToString::to_string)
    }

    #[cfg(not(test))]
    #[inline]
    pub fn get_env<K: AsRef<str>>(&self, key: K) -> Option<String> {
        env::var(key.as_ref()).ok()
    }

    // Retrieves a environment variable from the os or from a table if in testing mode (os version)
    #[cfg(test)]
    pub fn get_env_os<K: AsRef<str>>(&self, key: K) -> Option<OsString> {
        self.env.get(key.as_ref()).map(OsString::from)
    }

    #[cfg(not(test))]
    #[inline]
    pub fn get_env_os<K: AsRef<str>>(&self, key: K) -> Option<OsString> {
        env::var_os(key.as_ref())
    }

    /// Convert a `~` in a path to the home directory
    pub fn expand_tilde(dir: PathBuf) -> PathBuf {
        if dir.starts_with("~") {
            let without_home = dir.strip_prefix("~").unwrap();
            return utils::home_dir().unwrap().join(without_home);
        }
        dir
    }

    /// Create a new module
    pub fn new_module(&self, name: &str) -> Module {
        let config = self.config.get_module_config(name);
        let desc = modules::description(name);

        Module::new(name, desc, config)
    }

    /// Check if `disabled` option of the module is true in configuration file.
    pub fn is_module_disabled_in_config(&self, name: &str) -> bool {
        let config = self.config.get_module_config(name);

        // If the segment has "disabled" set to "true", don't show it
        let disabled = config.and_then(|table| table.as_table()?.get("disabled")?.as_bool());

        disabled == Some(true)
    }

    /// Return whether the specified custom module has a `disabled` option set to true.
    /// If it doesn't exist, `None` is returned.
    pub fn is_custom_module_disabled_in_config(&self, name: &str) -> Option<bool> {
        let config = self.config.get_custom_module_config(name)?;
        let disabled = Some(config).and_then(|table| table.as_table()?.get("disabled")?.as_bool());

        Some(disabled == Some(true))
    }

    // returns a new ScanDir struct with reference to current dir_files of context
    // see ScanDir for methods
    pub fn try_begin_scan(&'a self) -> Option<ScanDir<'a>> {
        Some(ScanDir {
            dir_contents: self.dir_contents().ok()?,
            files: &[],
            folders: &[],
            extensions: &[],
        })
    }

    /// Will lazily get repo root and branch when a module requests it.
    pub fn get_repo(&self) -> Result<&Repo, git2::Error> {
        self.repo.get_or_try_init(|| -> Result<Repo, git2::Error> {
            let repository = if env::var("GIT_DIR").is_ok() {
                Repository::open_from_env()
            } else {
                let dirs: [PathBuf; 0] = [];
                Repository::open_ext(&self.current_dir, git2::RepositoryOpenFlags::FROM_ENV, dirs)
            }?;
            Ok(Repo {
                branch: get_current_branch(&repository),
                workdir: repository.workdir().map(Path::to_path_buf),
                path: Path::to_path_buf(repository.path()),
                state: repository.state(),
                remote: get_remote_repository_info(&repository),
            })
        })
    }

    pub fn dir_contents(&self) -> Result<&DirContents, std::io::Error> {
        self.dir_contents.get_or_try_init(|| {
            let timeout = self.root_config.scan_timeout;
            DirContents::from_path_with_timeout(&self.current_dir, Duration::from_millis(timeout))
        })
    }

    fn get_shell() -> Shell {
        let shell = env::var("STARSHIP_SHELL").unwrap_or_default();
        match shell.as_str() {
            "bash" => Shell::Bash,
            "fish" => Shell::Fish,
            "ion" => Shell::Ion,
            "powershell" | "pwsh" => Shell::PowerShell,
            "zsh" => Shell::Zsh,
            "elvish" => Shell::Elvish,
            "tcsh" => Shell::Tcsh,
            "nu" => Shell::Nu,
            "xonsh" => Shell::Xonsh,
            "cmd" => Shell::Cmd,
            _ => Shell::Unknown,
        }
    }

    // TODO: This should be used directly by clap parse
    pub fn get_cmd_duration(&self) -> Option<u128> {
        self.properties
            .cmd_duration
            .as_deref()
            .and_then(|cd| cd.parse::<u128>().ok())
    }

    /// Execute a command and return the output on stdout and stderr if successful
    #[inline]
    pub fn exec_cmd<T: AsRef<OsStr> + Debug, U: AsRef<OsStr> + Debug>(
        &self,
        cmd: T,
        args: &[U],
    ) -> Option<CommandOutput> {
        log::trace!(
            "Executing command {:?} with args {:?} from context",
            cmd,
            args
        );
        #[cfg(test)]
        {
            let command = crate::utils::display_command(&cmd, args);
            if let Some(output) = self
                .cmd
                .get(command.as_str())
                .cloned()
                .or_else(|| crate::utils::mock_cmd(&cmd, args))
            {
                return output;
            }
        }
        let mut cmd = create_command(cmd).ok()?;
        cmd.args(args).current_dir(&self.current_dir);
        exec_timeout(
            &mut cmd,
            Duration::from_millis(self.root_config.command_timeout),
        )
    }

    /// Attempt to execute several commands with `exec_cmd`, return the results of the first that works
    pub fn exec_cmds_return_first(&self, commands: Vec<Vec<&str>>) -> Option<CommandOutput> {
        commands
            .iter()
            .find_map(|attempt| self.exec_cmd(attempt[0], &attempt[1..]))
    }

    /// Returns the string contents of a file from the current working directory
    pub fn read_file_from_pwd(&self, file_name: &str) -> Option<String> {
        if !self.try_begin_scan()?.set_files(&[file_name]).is_match() {
            log::debug!(
                "Not attempting to read {file_name} because, it was not found during scan."
            );
            return None;
        }

        read_file(self.current_dir.join(file_name)).ok()
    }
}

#[derive(Debug)]
pub struct DirContents {
    // HashSet of all files, no folders, relative to the base directory given at construction.
    files: HashSet<PathBuf>,
    // HashSet of all file names, e.g. the last section without any folders, as strings.
    file_names: HashSet<String>,
    // HashSet of all folders, relative to the base directory given at construction.
    folders: HashSet<PathBuf>,
    // HashSet of all extensions found, without dots, e.g. "js" instead of ".js".
    extensions: HashSet<String>,
}

impl DirContents {
    #[cfg(test)]
    fn from_path(base: &Path) -> Result<Self, std::io::Error> {
        Self::from_path_with_timeout(base, Duration::from_secs(30))
    }

    fn from_path_with_timeout(base: &Path, timeout: Duration) -> Result<Self, std::io::Error> {
        let start = Instant::now();

        let mut folders: HashSet<PathBuf> = HashSet::new();
        let mut files: HashSet<PathBuf> = HashSet::new();
        let mut file_names: HashSet<String> = HashSet::new();
        let mut extensions: HashSet<String> = HashSet::new();

        fs::read_dir(base)?
            .enumerate()
            .take_while(|(n, _)| {
                cfg!(test) // ignore timeout during tests
                || n & 0xFF != 0 // only check timeout once every 2^8 entries
                || start.elapsed() < timeout
            })
            .filter_map(|(_, entry)| entry.ok())
            .for_each(|entry| {
                let path = PathBuf::from(entry.path().strip_prefix(base).unwrap());
                if entry.path().is_dir() {
                    folders.insert(path);
                } else {
                    if !path.to_string_lossy().starts_with('.') {
                        path.extension()
                            .map(|ext| extensions.insert(ext.to_string_lossy().to_string()));
                    }
                    if let Some(file_name) = path.file_name() {
                        file_names.insert(file_name.to_string_lossy().to_string());
                    }
                    files.insert(path);
                }
            });

        log::trace!(
            "Building HashSets of directory files, folders and extensions took {:?}",
            start.elapsed()
        );

        Ok(Self {
            files,
            file_names,
            folders,
            extensions,
        })
    }

    pub fn files(&self) -> impl Iterator<Item = &PathBuf> {
        self.files.iter()
    }

    pub fn has_file(&self, path: &str) -> bool {
        self.files.contains(Path::new(path))
    }

    pub fn has_file_name(&self, name: &str) -> bool {
        self.file_names.contains(name)
    }

    pub fn has_any_file_name(&self, names: &[&str]) -> bool {
        names.iter().any(|name| self.has_file_name(name))
    }

    pub fn has_folder(&self, path: &str) -> bool {
        self.folders.contains(Path::new(path))
    }

    pub fn has_any_folder(&self, paths: &[&str]) -> bool {
        paths.iter().any(|path| self.has_folder(path))
    }

    pub fn has_extension(&self, ext: &str) -> bool {
        self.extensions.contains(ext)
    }

    pub fn has_any_extension(&self, exts: &[&str]) -> bool {
        exts.iter().any(|ext| self.has_extension(ext))
    }
}

pub struct Repo {
    /// If `current_dir` is a git repository or is contained within one,
    /// this is the current branch name of that repo.
    pub branch: Option<String>,

    /// If `current_dir` is a git repository or is contained within one,
    /// this is the path to the root of that repo.
    pub workdir: Option<PathBuf>,

    /// The path of the repository's `.git` directory.
    pub path: PathBuf,

    /// State
    pub state: RepositoryState,

    /// Remote repository
    pub remote: Option<Remote>,
}

impl Repo {
    /// Opens the associated git repository.
    pub fn open(&self) -> Result<Repository, git2::Error> {
        Repository::open(&self.path)
    }
}

/// Remote repository
pub struct Remote {
    pub branch: Option<String>,
    pub name: Option<String>,
}

// A struct of Criteria which will be used to verify current PathBuf is
// of X language, criteria can be set via the builder pattern
pub struct ScanDir<'a> {
    dir_contents: &'a DirContents,
    files: &'a [&'a str],
    folders: &'a [&'a str],
    extensions: &'a [&'a str],
}

impl<'a> ScanDir<'a> {
    #[must_use]
    pub const fn set_files(mut self, files: &'a [&'a str]) -> Self {
        self.files = files;
        self
    }

    #[must_use]
    pub const fn set_extensions(mut self, extensions: &'a [&'a str]) -> Self {
        self.extensions = extensions;
        self
    }

    #[must_use]
    pub const fn set_folders(mut self, folders: &'a [&'a str]) -> Self {
        self.folders = folders;
        self
    }

    /// based on the current `PathBuf` check to see
    /// if any of this criteria match or exist and returning a boolean
    pub fn is_match(&self) -> bool {
        self.dir_contents.has_any_extension(self.extensions)
            || self.dir_contents.has_any_folder(self.folders)
            || self.dir_contents.has_any_file_name(self.files)
    }
}

fn get_current_branch(repository: &Repository) -> Option<String> {
    let head = match repository.head() {
        Ok(reference) => reference,
        Err(e) => {
            return if e.code() == UnbornBranch {
                // HEAD should only be an unborn branch if the repository is fresh,
                // in that case read directly from `.git/HEAD`
                let mut head_path = repository.path().to_path_buf();
                head_path.push("HEAD");

                // get first line, then last path segment
                fs::read_to_string(&head_path)
                    .ok()?
                    .lines()
                    .next()?
                    .trim()
                    .split('/')
                    .last()
                    .map(std::borrow::ToOwned::to_owned)
            } else {
                None
            };
        }
    };

    let shorthand = head.shorthand();

    shorthand.map(std::string::ToString::to_string)
}

fn get_remote_repository_info(repository: &Repository) -> Option<Remote> {
    if let Ok(head) = repository.head() {
        if let Some(local_branch_ref) = head.name() {
            let remote_ref = match repository.branch_upstream_name(local_branch_ref) {
                Ok(remote_ref) => remote_ref.as_str()?.to_owned(),
                Err(_) => return None,
            };

            let mut v = remote_ref.splitn(4, '/');
            let remote_name = v.nth(2)?.to_owned();
            let remote_branch = v.last()?.to_owned();

            return Some(Remote {
                branch: Some(remote_branch),
                name: Some(remote_name),
            });
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shell {
    Bash,
    Fish,
    Ion,
    PowerShell,
    Zsh,
    Elvish,
    Tcsh,
    Nu,
    Xonsh,
    Cmd,
    Unknown,
}

/// Which kind of prompt target to print (main prompt, rprompt, ...)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    Main,
    Right,
    Continuation,
}

/// Properties as passed on from the shell as arguments
#[derive(Parser, Debug)]
pub struct Properties {
    /// The status code of the previously run command as an unsigned or signed 32bit integer
    #[clap(short = 's', long = "status")]
    pub status_code: Option<String>,
    /// Bash, Fish and Zsh support returning codes for each process in a pipeline.
    #[clap(long, value_delimiter = ' ')]
    pub pipestatus: Option<Vec<String>>,
    /// The width of the current interactive terminal.
    #[clap(short = 'w', long, default_value_t=default_width(), value_parser=parse_width)]
    terminal_width: usize,
    /// The path that the prompt should render for.
    #[clap(short, long)]
    path: Option<PathBuf>,
    /// The logical path that the prompt should render for.
    /// This path should be a virtual/logical representation of the PATH argument.
    #[clap(short = 'P', long)]
    logical_path: Option<PathBuf>,
    /// The execution duration of the last command, in milliseconds
    #[clap(short = 'd', long)]
    pub cmd_duration: Option<String>,
    /// The keymap of fish/zsh/cmd
    #[clap(short = 'k', long, default_value = "viins")]
    pub keymap: String,
    /// The number of currently running jobs
    #[clap(short, long, default_value_t, value_parser=parse_jobs)]
    pub jobs: i64,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            status_code: None,
            pipestatus: None,
            terminal_width: default_width(),
            path: None,
            logical_path: None,
            cmd_duration: None,
            keymap: "viins".to_string(),
            jobs: 0,
        }
    }
}

/// Parse String, but treat empty strings as `None`
fn parse_trim<F: FromStr>(value: &str) -> Option<Result<F, F::Err>> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    Some(F::from_str(value))
}

fn parse_jobs(jobs: &str) -> Result<i64, ParseIntError> {
    parse_trim(jobs).unwrap_or(Ok(0))
}

fn default_width() -> usize {
    terminal_size().map_or(80, |(w, _)| w.0 as usize)
}

fn parse_width(width: &str) -> Result<usize, ParseIntError> {
    parse_trim(width).unwrap_or_else(|| Ok(default_width()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    fn testdir(paths: &[&str]) -> Result<tempfile::TempDir, std::io::Error> {
        let dir = tempfile::tempdir()?;
        for path in paths {
            let p = dir.path().join(Path::new(path));
            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::File::create(p)?.sync_all()?;
        }
        Ok(dir)
    }

    #[test]
    fn test_scan_dir() -> Result<(), Box<dyn std::error::Error>> {
        let empty = testdir(&[])?;
        let empty_dc = DirContents::from_path(empty.path())?;

        assert!(!ScanDir {
            dir_contents: &empty_dc,
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        }
        .is_match());
        empty.close()?;

        let rust = testdir(&["README.md", "Cargo.toml", "src/main.rs"])?;
        let rust_dc = DirContents::from_path(rust.path())?;
        assert!(!ScanDir {
            dir_contents: &rust_dc,
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        }
        .is_match());
        rust.close()?;

        let java = testdir(&["README.md", "src/com/test/Main.java", "pom.xml"])?;
        let java_dc = DirContents::from_path(java.path())?;
        assert!(!ScanDir {
            dir_contents: &java_dc,
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        }
        .is_match());
        java.close()?;

        let node = testdir(&["README.md", "node_modules/lodash/main.js", "package.json"])?;
        let node_dc = DirContents::from_path(node.path())?;
        assert!(ScanDir {
            dir_contents: &node_dc,
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        }
        .is_match());
        node.close()?;

        Ok(())
    }

    #[test]
    fn context_constructor_should_canonicalize_current_dir() -> io::Result<()> {
        #[cfg(not(windows))]
        use std::os::unix::fs::symlink as symlink_dir;
        #[cfg(windows)]
        use std::os::windows::fs::symlink_dir;

        let tmp_dir = tempfile::TempDir::new()?;
        let path = tmp_dir.path().join("a/xxx/yyy");
        fs::create_dir_all(&path)?;

        // Set up a mock symlink
        let path_actual = tmp_dir.path().join("a/xxx");
        let path_symlink = tmp_dir.path().join("a/symlink");
        symlink_dir(&path_actual, &path_symlink).expect("create symlink");

        // Mock navigation into the symlink path
        let test_path = path_symlink.join("yyy");
        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            test_path.clone(),
            test_path.clone(),
        );

        assert_ne!(context.current_dir, context.logical_dir);

        let expected_current_dir =
            dunce::canonicalize(path_actual.join("yyy")).expect("canonicalize");
        assert_eq!(expected_current_dir, context.current_dir);

        let expected_logical_dir = test_path;
        assert_eq!(expected_logical_dir, context.logical_dir);

        tmp_dir.close()
    }

    #[test]
    fn context_constructor_should_fail_gracefully_when_canonicalization_fails() {
        // Mock navigation to a directory which does not exist on disk
        let test_path = Path::new("/path_which_does_not_exist").to_path_buf();
        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            test_path.clone(),
            test_path.clone(),
        );

        let expected_current_dir = &test_path;
        assert_eq!(expected_current_dir, &context.current_dir);

        let expected_logical_dir = &test_path;
        assert_eq!(expected_logical_dir, &context.logical_dir);
    }

    #[test]
    fn context_constructor_should_fall_back_to_tilde_replacement_when_canonicalization_fails() {
        use utils::home_dir;

        // Mock navigation to a directory which does not exist on disk
        let test_path = Path::new("~/path_which_does_not_exist").to_path_buf();
        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            test_path.clone(),
            test_path.clone(),
        );

        let expected_current_dir = home_dir()
            .expect("home_dir")
            .join("path_which_does_not_exist");
        assert_eq!(expected_current_dir, context.current_dir);

        let expected_logical_dir = test_path;
        assert_eq!(expected_logical_dir, context.logical_dir);
    }

    #[cfg(windows)]
    #[test]
    fn strip_extended_path_prefix() {
        let test_path = Path::new(r"\\?\C:\").to_path_buf();
        let context = Context::new_with_shell_and_path(
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            test_path.clone(),
            test_path,
        );

        let expected_path = Path::new(r"C:\");

        assert_eq!(&context.current_dir, expected_path);
    }
}
