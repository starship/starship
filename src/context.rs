use clap::ArgMatches;
use git2::Repository;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

pub struct Context<'a> {
    pub current_dir: PathBuf,
    pub dir_files: Vec<PathBuf>,
    pub arguments: ArgMatches<'a>,
    pub repo_root: Option<PathBuf>,
    pub branch_name: Option<String>,
}

impl<'a> Context<'a> {
    pub fn new(arguments: ArgMatches) -> Context {
        let current_dir = env::current_dir().expect("Unable to identify current directory.");
        Context::new_with_dir(arguments, current_dir)
    }

    #[allow(dead_code)]
    pub fn new_with_dir<T>(arguments: ArgMatches, dir: T) -> Context
    where
        T: Into<PathBuf>,
    {
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
            .and_then(|repo| repo.workdir().map(|repo| repo.to_path_buf()));
        let branch_name = repository
            .as_ref()
            .and_then(|repo| get_current_branch(&repo));

        Context {
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

    // returns a new ScanDir struct with reference to current dir_files of context
    // see ScanDir for methods
    pub fn new_scan_dir(&'a self) -> ScanDir<'a> {
        ScanDir {
            dir_files: self.dir_files.as_ref(),
            files: &[],
            folders: &[],
            extensions: &[],
        }
    }
}

// A struct of Criteria which will be used to verify current PathBuf is
// of X language, criteria can be set via the builder pattern
pub struct ScanDir<'a> {
    dir_files: &'a Vec<PathBuf>, // Replace with reference
    files: &'a [&'a str],
    folders: &'a [&'a str],
    extensions: &'a [&'a str],
}

impl<'a> ScanDir<'a> {
    pub fn set_files(mut self, files: &'a [&'a str]) -> Self {
        self.files = files;
        self
    }

    pub fn set_extensions(mut self, extensions: &'a [&'a str]) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn set_folders(mut self, folders: &'a [&'a str]) -> Self {
        self.folders = folders;
        self
    }

    /// based on the current Pathbuf check to see
    /// if any of this criteria match or exist and returning a boolean
    pub fn scan(&mut self) -> bool {
        self.dir_files.iter().any(|path| {
            path_has_name(&path, &self.folders)
                || path_has_name(&path, &self.files)
                || has_extension(&path, &self.extensions)
        })
    }
}

/// checks to see if the pathbuf matches a file or folder name
pub fn path_has_name<'a>(dir_entry: &PathBuf, names: &'a [&'a str]) -> bool {
    let found_file_or_folder_name = names.into_iter().find(|file_or_folder_name| {
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
    let found_ext = extensions.into_iter().find(|ext| {
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
    fn test_criteria_scan() {
        let mut failing_criteria = ScanDir {
            dir_files: &vec![PathBuf::new()],
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };

        // fails if buffer does not match any criteria
        assert_eq!(failing_criteria.scan(), false);

        let mut passing_criteria = ScanDir {
            dir_files: &vec![PathBuf::from("package.json")],
            files: &["package.json"],
            extensions: &["js"],
            folders: &["node_modules"],
        };

        assert_eq!(passing_criteria.scan(), true);
    }
}

fn get_current_branch(repository: &Repository) -> Option<String> {
    let head = repository.head().ok()?;
    let shorthand = head.shorthand();

    shorthand.map(|branch| branch.to_string())
}
