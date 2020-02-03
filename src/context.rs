use crate::config::StarshipConfig;
use crate::module::Module;

use crate::modules;
use clap::ArgMatches;
use git2::{Repository, RepositoryState};
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::{Duration, SystemTime};

/// Context contains data or common methods that may be used by multiple modules.
/// The data contained within Context will be relevant to this particular rendering
/// of the prompt.
pub struct Context<'a> {
    /// The deserialized configuration map from the user's `starship.toml` file.
    pub config: StarshipConfig,

    /// The current working directory that starship is being called in.
    pub current_dir: PathBuf,

    /// A struct containing directory contents in a lookup-optimised format.
    dir_contents: OnceCell<DirContents>,

    /// Properties to provide to modules.
    pub properties: HashMap<&'a str, String>,

    /// Private field to store Git information for modules who need it
    repo: OnceCell<Repo>,

    /// The shell the user is assumed to be running
    pub shell: Shell,
}

impl<'a> Context<'a> {
    /// Identify the current working directory and create an instance of Context
    /// for it.
    pub fn new(arguments: ArgMatches) -> Context {
        // Retrieve the "path" flag. If unavailable, use the current directory instead.
        let path = arguments
            .value_of("path")
            .map(From::from)
            .unwrap_or_else(|| {
                env::var("PWD").map(PathBuf::from).unwrap_or_else(|err| {
                    log::debug!("Unable to get path from $PWD: {}", err);
                    env::current_dir().expect("Unable to identify current directory. Error")
                })
            });

        Context::new_with_dir(arguments, path)
    }

    /// Create a new instance of Context for the provided directory
    pub fn new_with_dir<T>(arguments: ArgMatches, dir: T) -> Context
    where
        T: Into<PathBuf>,
    {
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

        // TODO: Currently gets the physical directory. Get the logical directory.
        let current_dir = Context::expand_tilde(dir.into());

        let shell = Context::get_shell();

        Context {
            config,
            properties,
            current_dir,
            dir_contents: OnceCell::new(),
            repo: OnceCell::new(),
            shell,
        }
    }

    /// Convert a `~` in a path to the home directory
    fn expand_tilde(dir: PathBuf) -> PathBuf {
        if dir.starts_with("~") {
            let without_home = dir.strip_prefix("~").unwrap();
            return dirs::home_dir().unwrap().join(without_home);
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
                let repository = Repository::discover(&self.current_dir).ok();
                let branch = repository
                    .as_ref()
                    .and_then(|repo| get_current_branch(repo));
                let root = repository
                    .as_ref()
                    .and_then(|repo| repo.workdir().map(Path::to_path_buf));
                let state = repository.as_ref().map(|repo| repo.state());

                Ok(Repo {
                    branch,
                    root,
                    state,
                })
            })
    }

    pub fn dir_contents(&self) -> Result<&DirContents, std::io::Error> {
        self.dir_contents.get_or_try_init(|| {
            let timeout = Duration::from_millis(self.config.get_root_config().scan_timeout);
            DirContents::from_path_with_timeout(&self.current_dir, timeout)
        })
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
    fn from_path(base: &PathBuf) -> Result<Self, std::io::Error> {
        Self::from_path_with_timeout(base, Duration::from_secs(30))
    }

    fn from_path_with_timeout(base: &PathBuf, timeout: Duration) -> Result<Self, std::io::Error> {
        let start = SystemTime::now();

        let mut folders: HashSet<PathBuf> = HashSet::new();
        let mut files: HashSet<PathBuf> = HashSet::new();
        let mut file_names: HashSet<String> = HashSet::new();
        let mut extensions: HashSet<String> = HashSet::new();

        fs::read_dir(base)?
            .take_while(|_| SystemTime::now().duration_since(start).unwrap() < timeout)
            .filter_map(Result::ok)
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
            SystemTime::now().duration_since(start).unwrap()
        );

        Ok(DirContents {
            folders,
            files,
            file_names,
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

    fn get_shell() -> Shell {
        let shell = std::env::var("STARSHIP_SHELL").unwrap_or_default();
        match shell.as_str() {
            "bash" => Shell::Bash,
            "fish" => Shell::Fish,
            "ion" => Shell::Ion,
            "powershell" => Shell::PowerShell,
            "zsh" => Shell::Zsh,
            _ => Shell::Unknown,
        }
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

    /// based on the current Pathbuf check to see
    /// if any of this criteria match or exist and returning a boolean
    pub fn is_match(&self) -> bool {
        self.dir_contents.has_any_extension(self.extensions)
            || self.dir_contents.has_any_folder(self.folders)
            || self.dir_contents.has_any_file_name(self.files)
    }
}

fn get_current_branch(repository: &Repository) -> Option<String> {
    let head = repository.head().ok()?;
    let shorthand = head.shorthand();

    shorthand.map(std::string::ToString::to_string)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shell {
    Bash,
    Fish,
    Ion,
    PowerShell,
    Zsh,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let empty_dc = DirContents::from_path(&PathBuf::from(empty.path()))?;

        assert_eq!(
            ScanDir {
                dir_contents: &empty_dc,
                files: &["package.json"],
                extensions: &["js"],
                folders: &["node_modules"],
            }
            .is_match(),
            false
        );

        let rust = testdir(&["README.md", "Cargo.toml", "src/main.rs"])?;
        let rust_dc = DirContents::from_path(&PathBuf::from(rust.path()))?;
        assert_eq!(
            ScanDir {
                dir_contents: &rust_dc,
                files: &["package.json"],
                extensions: &["js"],
                folders: &["node_modules"],
            }
            .is_match(),
            false
        );

        let java = testdir(&["README.md", "src/com/test/Main.java", "pom.xml"])?;
        let java_dc = DirContents::from_path(&PathBuf::from(java.path()))?;
        assert_eq!(
            ScanDir {
                dir_contents: &java_dc,
                files: &["package.json"],
                extensions: &["js"],
                folders: &["node_modules"],
            }
            .is_match(),
            false
        );

        let node = testdir(&["README.md", "node_modules/lodash/main.js", "package.json"])?;
        let node_dc = DirContents::from_path(&PathBuf::from(node.path()))?;
        assert_eq!(
            ScanDir {
                dir_contents: &node_dc,
                files: &["package.json"],
                extensions: &["js"],
                folders: &["node_modules"],
            }
            .is_match(),
            true
        );

        Ok(())
    }
}
