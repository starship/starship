use clap::ArgMatches;
use git2::Repository;
use std::env;
use std::fs;
use std::path::PathBuf;

pub struct Context<'a> {
    pub current_dir: PathBuf,
    pub dir_files: Vec<PathBuf>,
    pub arguments: ArgMatches<'a>,
    pub repo_root: Option<PathBuf>,
    pub branch_name: Option<String>
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
        let repo_root = repository.as_ref().and_then(|repo| repo.workdir().map(|repo| repo.to_path_buf()));
        let branch_name = repository.as_ref().and_then(|repo| get_current_branch(&repo));

        Context {
            arguments,
            current_dir,
            dir_files,
            repo_root,
            branch_name
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
}

fn get_current_branch(repository: &Repository) -> Option<String> {
    let head = repository.head().ok()?;
    let shorthand = head.shorthand();
    
    shorthand.map(|branch| branch.to_string())
}
