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

    /// A vector containing the full paths of all the files in `current_dir`.
    folders_files_extensions: OnceCell<(HashSet<PathBuf>, HashSet<PathBuf>, HashSet<String>)>,

    /// Properties to provide to modules.
    pub properties: HashMap<&'a str, String>,

    /// Private field to store Git information for modules who need it
    repo: OnceCell<Repo>,
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

        Context {
            config,
            properties,
            current_dir,
            folders_files_extensions: OnceCell::new(),
            repo: OnceCell::new(),
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
            folders_files_extensions: self.get_folders_files_extensions().ok()?,
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

    pub fn files(&self) -> Result<impl Iterator<Item = &PathBuf>, std::io::Error> {
        let (_, files, _) = self.get_folders_files_extensions()?;
        Ok(files.iter())
    }

    pub fn has_file(&self, name: &str) -> Result<bool, std::io::Error> {
        let (_, files, _) = self.get_folders_files_extensions()?;
        Ok(files.contains(Path::new(name)))
    }

    pub fn get_folders_files_extensions(
        &self,
    ) -> Result<&(HashSet<PathBuf>, HashSet<PathBuf>, HashSet<String>), std::io::Error> {
        let start_time = SystemTime::now();
        let scan_timeout = Duration::from_millis(self.config.get_root_config().scan_timeout);

        self.folders_files_extensions.get_or_try_init(
            || -> Result<(HashSet<PathBuf>, HashSet<PathBuf>, HashSet<String>), std::io::Error> {
                let mut folders: HashSet<PathBuf> = HashSet::new();
                let mut files: HashSet<PathBuf> = HashSet::new();
                let mut extensions: HashSet<String> = HashSet::new();

                fs::read_dir(&self.current_dir)?
                    .take_while(|_item| {
                        SystemTime::now().duration_since(start_time).unwrap() < scan_timeout
                    })
                    .filter_map(Result::ok)
                    .for_each(|entry| {
                        let path = entry.path();
                        if path.is_dir() {
                            folders.insert(path.clone());
                        } else {
                            files.insert(path.clone());
                            if !path.to_string_lossy().starts_with('.') {
                                path.extension().map(|ext| {
                                    extensions.insert(ext.to_string_lossy().to_string())
                                });
                            }
                        }
                    });

                log::trace!(
                    "Building HashSets of directory files, folders and extensions took {:?}",
                    SystemTime::now().duration_since(start_time).unwrap()
                );

                Ok((folders, files, extensions))
            },
        )
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
    folders_files_extensions: &'a (HashSet<PathBuf>, HashSet<PathBuf>, HashSet<String>),
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
        let (folders, files, extensions) = self.folders_files_extensions;

        if self
            .folders
            .iter()
            .any(|folder| folders.contains(Path::new(folder)))
        {
            return true;
        }

        if self
            .files
            .iter()
            .any(|file| files.contains(Path::new(file)))
        {
            return true;
        }

        if self
            .extensions
            .iter()
            .any(|ext| extensions.contains(ext.to_owned()))
        {
            return true;
        }

        false
    }
}

fn get_current_branch(repository: &Repository) -> Option<String> {
    let head = repository.head().ok()?;
    let shorthand = head.shorthand();

    shorthand.map(std::string::ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;
    use std::str::FromStr;

    fn folders_files_extensions(
        folders: &[&str],
        files: &[&str],
        extensions: &[&str],
    ) -> (HashSet<PathBuf>, HashSet<PathBuf>, HashSet<String>) {
        (
            HashSet::from_iter(folders.iter().map(|f| PathBuf::from_str(f).unwrap())),
            HashSet::from_iter(files.iter().map(|f| PathBuf::from_str(f).unwrap())),
            HashSet::from_iter(extensions.iter().map(|s| s.to_string())),
        )
    }

    #[test]
    fn test_criteria_scan_fails() {
        let empty = ScanDir {
            folders_files_extensions: &folders_files_extensions(&[], &[], &[]),
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };
        assert_eq!(empty.is_match(), false);

        let folder_doesnt_match = ScanDir {
            folders_files_extensions: &folders_files_extensions(&[], &["foo"], &[]),
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };
        assert_eq!(folder_doesnt_match.is_match(), false);

        let file_doesnt_match = ScanDir {
            folders_files_extensions: &folders_files_extensions(&["foo"], &[], &[]),
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };
        assert_eq!(file_doesnt_match.is_match(), false);

        let extension_doesnt_match = ScanDir {
            folders_files_extensions: &folders_files_extensions(&[], &[], &["foo"]),
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };
        assert_eq!(extension_doesnt_match.is_match(), false);
    }

    #[test]
    fn test_criteria_scan_passes() {
        let file_matches = ScanDir {
            folders_files_extensions: &folders_files_extensions(&[], &["package.json"], &[]),
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };
        assert_eq!(file_matches.is_match(), true);

        let folder_matches = ScanDir {
            folders_files_extensions: &folders_files_extensions(&["node_modules"], &[], &[]),
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };
        assert_eq!(folder_matches.is_match(), true);

        let extension_matches = ScanDir {
            folders_files_extensions: &folders_files_extensions(&[], &[], &["js"]),
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };
        assert_eq!(extension_matches.is_match(), true);
    }
}
