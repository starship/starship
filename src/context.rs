use crate::config::StarshipConfig;
use crate::module::Module;
use crate::utils::{exec_cmd, CommandOutput};

use crate::modules;
use clap::ArgMatches;
use dirs_next::home_dir;
use git2::{ErrorCode::UnbornBranch, Repository, RepositoryState};
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::{Duration, Instant};

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

    /// A struct containing directory contents in a lookup-optimised format.
    dir_contents: OnceCell<DirContents>,

    /// Properties to provide to modules.
    pub properties: HashMap<&'a str, String>,

    /// Private field to store Git information for modules who need it
    repo: OnceCell<Repo>,

    /// The shell the user is assumed to be running
    pub shell: Shell,

    /// A HashMap of environment variable mocks
    #[cfg(test)]
    pub env: HashMap<&'a str, String>,

    /// A HashMap of command mocks
    #[cfg(test)]
    pub cmd: HashMap<&'a str, Option<CommandOutput>>,

    #[cfg(feature = "battery")]
    pub battery_info_provider: &'a (dyn crate::modules::BatteryInfoProvider + Send + Sync),

    /// Timeout for the execution of commands
    cmd_timeout: Duration,
}

impl<'a> Context<'a> {
    /// Identify the current working directory and create an instance of Context
    /// for it. "logical-path" is used when a shell allows the "current working directory"
    /// to be something other than a file system path (like powershell provider specific paths).
    pub fn new(arguments: ArgMatches) -> Context {
        let shell = Context::get_shell();

        // Retrieve the "current directory".
        // If the path argument is not set fall back to the OS current directory.
        let path = arguments
            .value_of("path")
            .map(PathBuf::from)
            .or_else(|| env::current_dir().ok())
            .or_else(|| env::var("PWD").map(PathBuf::from).ok())
            .or_else(|| arguments.value_of("logical_path").map(PathBuf::from))
            .unwrap_or_default();

        // Retrive the "logical directory".
        // If the path argument is not set fall back to the PWD env variable set by many shells
        // or to the other path.
        let logical_path = arguments
            .value_of("logical_path")
            .map(PathBuf::from)
            .or_else(|| env::var("PWD").map(PathBuf::from).ok())
            .unwrap_or_else(|| path.clone());

        Context::new_with_shell_and_path(arguments, shell, path, logical_path)
    }

    /// Create a new instance of Context for the provided directory
    pub fn new_with_shell_and_path(
        arguments: ArgMatches,
        shell: Shell,
        path: PathBuf,
        logical_path: PathBuf,
    ) -> Context {
        let config = StarshipConfig::initialize();

        // Unwrap the clap arguments into a simple hashtable
        // we only care about single arguments at this point, there isn't a
        // use-case for a list of arguments yet.
        let properties: HashMap<&str, std::string::String> = arguments
            .args
            .iter()
            .filter(|(_, v)| !v.vals.is_empty())
            .map(|(a, b)| (*a, b.vals.first().cloned().unwrap().into_string().unwrap()))
            .collect();

        // Canonicalize the current path to resolve symlinks, etc.
        // NOTE: On Windows this converts the path to extended-path syntax.
        let current_dir = Context::expand_tilde(path);
        let current_dir = current_dir.canonicalize().unwrap_or(current_dir);
        let logical_dir = logical_path;

        let cmd_timeout = Duration::from_millis(config.get_root_config().command_timeout);

        Context {
            config,
            properties,
            current_dir,
            logical_dir,
            dir_contents: OnceCell::new(),
            repo: OnceCell::new(),
            shell,
            #[cfg(test)]
            env: HashMap::new(),
            #[cfg(test)]
            cmd: HashMap::new(),
            #[cfg(feature = "battery")]
            battery_info_provider: &crate::modules::BatteryInfoProviderImpl,
            cmd_timeout,
        }
    }

    // Tries to retrieve home directory from a table in testing mode or else retrieves it from the os
    pub fn get_home(&self) -> Option<PathBuf> {
        if cfg!(test) {
            return self.get_env("HOME").map(PathBuf::from).or_else(home_dir);
        }

        home_dir()
    }

    // Retrives a environment variable from the os or from a table if in testing mode
    #[cfg(test)]
    pub fn get_env<K: AsRef<str>>(&self, key: K) -> Option<String> {
        self.env.get(key.as_ref()).map(|val| val.to_string())
    }

    #[cfg(not(test))]
    #[inline]
    pub fn get_env<K: AsRef<str>>(&self, key: K) -> Option<String> {
        env::var(key.as_ref()).ok()
    }

    // Retrives a environment variable from the os or from a table if in testing mode (os version)
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
            return dirs_next::home_dir().unwrap().join(without_home);
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
    pub fn get_repo(&self) -> Result<&Repo, std::io::Error> {
        self.repo
            .get_or_try_init(|| -> Result<Repo, std::io::Error> {
                let repository = if env::var("GIT_DIR").is_ok() {
                    Repository::open_from_env().ok()
                } else {
                    Repository::discover(&self.current_dir).ok()
                };
                let branch = repository
                    .as_ref()
                    .and_then(|repo| get_current_branch(repo));
                let root = repository
                    .as_ref()
                    .and_then(|repo| repo.workdir().map(Path::to_path_buf));
                let state = repository.as_ref().map(|repo| repo.state());
                let remote = repository
                    .as_ref()
                    .and_then(|repo| get_remote_repository_info(repo));
                Ok(Repo {
                    branch,
                    root,
                    state,
                    remote,
                })
            })
    }

    pub fn dir_contents(&self) -> Result<&DirContents, std::io::Error> {
        self.dir_contents.get_or_try_init(|| {
            let timeout = Duration::from_millis(self.config.get_root_config().scan_timeout);
            DirContents::from_path_with_timeout(&self.current_dir, timeout)
        })
    }

    fn get_shell() -> Shell {
        let shell = env::var("STARSHIP_SHELL").unwrap_or_default();
        match shell.as_str() {
            "bash" => Shell::Bash,
            "fish" => Shell::Fish,
            "ion" => Shell::Ion,
            "powershell" => Shell::PowerShell,
            "zsh" => Shell::Zsh,
            "elvish" => Shell::Elvish,
            "tcsh" => Shell::Tcsh,
            _ => Shell::Unknown,
        }
    }

    pub fn get_cmd_duration(&self) -> Option<u128> {
        self.properties.get("cmd_duration")?.parse::<u128>().ok()
    }

    /// Execute a command and return the output on stdout and stderr if successful
    #[inline]
    pub fn exec_cmd(&self, cmd: &str, args: &[&str]) -> Option<CommandOutput> {
        #[cfg(test)]
        {
            let command = match args.len() {
                0 => cmd.to_owned(),
                _ => format!("{} {}", cmd, args.join(" ")),
            };
            if let Some(output) = self.cmd.get(command.as_str()) {
                return output.clone();
            }
        }
        exec_cmd(cmd, args, self.cmd_timeout)
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
                n & 0xFF != 0 // only check timeout once every 2^8 entries
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

        Ok(DirContents {
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
    pub root: Option<PathBuf>,

    /// State
    pub state: Option<RepositoryState>,

    /// Remote repository
    pub remote: Option<Remote>,
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
    pub const fn set_files(mut self, files: &'a [&'a str]) -> Self {
        self.files = files;
        self
    }

    pub const fn set_extensions(mut self, extensions: &'a [&'a str]) -> Self {
        self.extensions = extensions;
        self
    }

    pub const fn set_folders(mut self, folders: &'a [&'a str]) -> Self {
        self.folders = folders;
        self
    }

    /// based on the current PathBuf check to see
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
                    .map(|r| r.to_owned())
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
    Unknown,
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
            ArgMatches::default(),
            Shell::Unknown,
            test_path.clone(),
            test_path.clone(),
        );

        assert_ne!(context.current_dir, context.logical_dir);

        let expected_current_dir = path_actual
            .join("yyy")
            .canonicalize()
            .expect("canonicalize");
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
            ArgMatches::default(),
            Shell::Unknown,
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
        use dirs_next::home_dir;

        // Mock navigation to a directory which does not exist on disk
        let test_path = Path::new("~/path_which_does_not_exist").to_path_buf();
        let context = Context::new_with_shell_and_path(
            ArgMatches::default(),
            Shell::Unknown,
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
}
