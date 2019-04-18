use clap::ArgMatches;
use std::env;
use std::path::PathBuf;

pub struct Context<'a> {
    pub current_dir: PathBuf,
    pub arguments: ArgMatches<'a>,
}

impl<'a> Context<'a> {
    pub fn new(arguments: ArgMatches) -> Context {
        // TODO: Currently gets the physical directory. Get the logical directory.
        let current_dir = env::current_dir().expect("Unable to identify current directory.");

        Context {
            current_dir,
            arguments,
        }
    }

    pub fn new_with_dir<T>(arguments: ArgMatches, dir: T) -> Context where T: Into<PathBuf> {
        Context {
            current_dir: dir.into(),
            arguments,
        }
    }
}
