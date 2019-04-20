use clap::ArgMatches;
use std::env;
use std::fs::{self, ReadDir};
use std::path::PathBuf;

pub struct Context<'a> {
    pub current_dir: PathBuf,
    pub dir_files: ReadDir,
    pub arguments: ArgMatches<'a>,
}

impl<'a> Context<'a> {
    pub fn new(arguments: ArgMatches) -> Context {
        // TODO: Currently gets the physical directory. Get the logical directory.
        let current_dir = env::current_dir().expect("Unable to identify current directory.");
        let dir_files = fs::read_dir(&current_dir).unwrap_or_else(|_| {
            panic!(
                "Unable to read current directory: {}",
                current_dir.to_string_lossy()
            )
        });

        Context {
            current_dir,
            dir_files,
            arguments,
        }
    }

    pub fn new_with_dir<T>(arguments: ArgMatches, dir: T) -> Context
    where
        T: Into<PathBuf>,
    {
        Context {
            current_dir: dir.into(),
            arguments,
        }
    }
}
