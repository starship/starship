use clap::ArgMatches;
use std::env;
use std::io::{self, Write};

use crate::modules;

pub fn prompt(args: ArgMatches) {
    let default_prompt = vec!["directory", "nodejs", "line_break", "character"];

    // TODO: Currently gets the physical directory. Get the logical directory.
    let current_path = env::current_dir().expect("Unable to identify current directory.");

    // TODO:
    // - List files in directory
    // - Index binaries in PATH

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Write a new line before the prompt
    writeln!(handle).unwrap();

    default_prompt
        .iter()
        .map(|module| modules::handle(module, &current_path, &args)) // Compute segments
        .flatten() // Remove segments set to `None`
        .enumerate() // Turn segment into tuple with index
        .map(|(index, segment)| segment.output_index(index)) // Generate string outputs
        .for_each(|segment_string| write!(handle, "{}", segment_string).unwrap());
}
