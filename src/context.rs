use clap::ArgMatches;
use std::env;
use std::fs;
use std::path::PathBuf;

pub struct Context<'a> {
    pub current_dir: PathBuf,
    pub dir_files: Vec<PathBuf>,
    pub arguments: ArgMatches<'a>,
}

impl<'a> Context<'a> {
    pub fn new(arguments: ArgMatches) -> Context {
        let current_dir = env::current_dir().expect("Unable to identify current directory.");
        Context::new_with_dir(arguments, current_dir)
    }

    pub fn new_with_dir<T>(arguments: ArgMatches, dir: T) -> Context
    where
        T: Into<PathBuf>,
    {
        let current_dir = dir.into();

        // TODO: Currently gets the physical directory. Get the logical directory.
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

        Context {
            current_dir,
            dir_files,
            arguments,
        }
    }
}
