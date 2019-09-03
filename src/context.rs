use crate::config::Config;
use crate::module::Module;

use clap::ArgMatches;
use git2::Repository;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

/// Context contains data or common methods that may be used by multiple modules.
/// The data contained within Context will be relevant to this particular rendering
/// of the prompt.
pub struct Context<'a> {
    /// The deserialized configuration map from the user's `starship.toml` file.
    pub config: toml::value::Table,

    /// The current working directory that starship is being called in.
    pub current_dir: PathBuf,

    /// A vector containing the full paths of all the files in `current_dir`.
    pub dir_files: Vec<PathBuf>,

    /// The map of arguments that were passed when starship was called.
    pub arguments: ArgMatches<'a>,

    /// If `current_dir` is a git repository or is contained within one,
    /// this is the path to the root of that repo.
    pub repo_root: Option<PathBuf>,

    /// If `current_dir` is a git repository or is contained within one,
    /// this is the current branch name of that repo.
    pub branch_name: Option<String>,
}

impl<'a> Context<'a> {
    /// Identify the current working directory and create an instance of Context
    /// for it.
    pub fn new(arguments: ArgMatches) -> Context {
        // Retrieve the "path" flag. If unavailable, use the current directory instead.
        let path = arguments
            .value_of("path")
            .map(From::from)
            .unwrap_or_else(|| env::current_dir().expect("Unable to identify current directory."));

        Context::new_with_dir(arguments, path)
    }

    /// Create a new instance of Context for the provided directory
    pub fn new_with_dir<T>(arguments: ArgMatches, dir: T) -> Context
    where
        T: Into<PathBuf>,
    {
        let config = toml::value::Table::initialize();

        // TODO: Currently gets the physical directory. Get the logical directory.
        let current_dir = Context::expand_tilde(dir.into());

        let dir_files = fs::read_dir(&current_dir)
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to read current directory: {}",
                    current_dir.to_string_lossy()
                )
            })
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect::<Vec<PathBuf>>();

        let repository = Repository::discover(&current_dir).ok();
        let repo_root = repository
            .as_ref()
            .and_then(|repo| repo.workdir().map(std::path::Path::to_path_buf));
        let branch_name = repository
            .as_ref()
            .and_then(|repo| get_current_branch(repo));

        Context {
            config,
            arguments,
            current_dir,
            dir_files,
            repo_root,
            branch_name,
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
    ///
    /// Will return `None` if the module is disabled by configuration, by setting
    /// the `disabled` key to `true` in the configuration for that module.
    pub fn new_module(&self, name: &str) -> Option<Module> {
        let config = self.config.get_module_config(name);

        // If the segment has "disabled" set to "true", don't show it
        let disabled = config.and_then(|table| table.get_as_bool("disabled"));

        if disabled == Some(true) {
            return None;
        }

        Some(Module::new(name, config))
    }

    // returns a new ScanDir struct with reference to current dir_files of context
    // see ScanDir for methods
    pub fn new_scan_dir(&'a self) -> ScanDir<'a> {
        ScanDir {
            dir_files: self.dir_files.as_ref(),
            extensions: &[],
            files: &[],
            folders: &[],
            ignores: &[],
        }
    }
}

// A struct of Criteria which will be used to verify current PathBuf is
// of X language, criteria can be set via the builder pattern
pub struct ScanDir<'a> {
    dir_files: &'a Vec<PathBuf>, // Replace with reference
    extensions: &'a [&'a str],
    files: &'a [&'a str],
    folders: &'a [&'a str],
    ignores: &'a [&'a str],
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

    pub const fn set_ignores(mut self, ignores: &'a [&'a str]) -> Self {
        self.ignores = ignores;
        self
    }

    /// based on the current Pathbuf check to see
    /// if any of this criteria match or exist and returning a boolean
    pub fn scan(&mut self) -> bool {
        let ignores: Vec<_> = self
            .ignores
            .iter()
            .map(|path| Context::expand_tilde(PathBuf::from(path)))
            .collect();

        self.dir_files
            .iter()
            .filter(|path| !ignores.contains(path))
            .any(|path| {
                if path.is_dir() {
                    path_has_name(path, self.folders)
                } else {
                    path_has_name(path, self.files) || has_extension(path, self.extensions)
                }
            })
    }
}

/// checks to see if the pathbuf matches a file or folder name
pub fn path_has_name<'a>(dir_entry: &PathBuf, names: &'a [&'a str]) -> bool {
    let found_file_or_folder_name = names.iter().find(|file_or_folder_name| {
        dir_entry
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            == **file_or_folder_name
    });

    match found_file_or_folder_name {
        Some(name) => !name.is_empty(),
        None => false,
    }
}

/// checks if pathbuf matches the extension provided
pub fn has_extension<'a>(dir_entry: &PathBuf, extensions: &'a [&'a str]) -> bool {
    let found_ext = extensions.iter().find(|ext| {
        dir_entry
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            == **ext
    });

    match found_ext {
        Some(extension) => !extension.is_empty(),
        None => false,
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

    #[test]
    fn test_path_has_name() {
        let mut buf = PathBuf::from("/");
        let files = vec!["package.json"];

        assert_eq!(path_has_name(&buf, &files), false);

        buf.set_file_name("some-file.js");
        assert_eq!(path_has_name(&buf, &files), false);

        buf.set_file_name("package.json");
        assert_eq!(path_has_name(&buf, &files), true);
    }

    #[test]
    fn test_has_extension() {
        let mut buf = PathBuf::from("/");
        let extensions = vec!["js"];

        assert_eq!(has_extension(&buf, &extensions), false);

        buf.set_file_name("some-file.rs");
        assert_eq!(has_extension(&buf, &extensions), false);

        buf.set_file_name("some-file.js");
        assert_eq!(has_extension(&buf, &extensions), true)
    }

    #[test]
    fn test_criteria_scan_fails() {
        let mut failing_criteria = ScanDir {
            dir_files: &vec![PathBuf::new()],
            extensions: &["js"],
            files: &["package.json"],
            folders: &["node_modules"],
            ignores: &[],
        };

        // fails if buffer does not match any criteria
        assert_eq!(failing_criteria.scan(), false);

        let mut failing_dir_criteria = ScanDir {
            dir_files: &vec![PathBuf::from("/package.js/dog.go")],
            extensions: &["js"],
            files: &["package.json"],
            folders: &["node_modules"],
            ignores: &[],
        };

        // fails when passed a pathbuf dir matches extension path
        assert_eq!(failing_dir_criteria.scan(), false);
    }

    #[test]
    fn test_criteria_scan_passes() {
        let mut passing_criteria = ScanDir {
            dir_files: &vec![PathBuf::from("package.json")],
            extensions: &["js"],
            files: &["package.json"],
            folders: &["node_modules"],
            ignores: &[],
        };

        assert_eq!(passing_criteria.scan(), true);
    }
}
