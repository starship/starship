use clap::ArgMatches;
use std::io::{self, Write};

use crate::context::Context;
use crate::modules;

pub fn prompt(args: ArgMatches) {
    let prompt_order = vec![
        "directory",
        "git_branch",
        "package",
        "nodejs",
        "rust",
        "python",
        "line_break",
        "character",
    ];
    let context = Context::new(args);

    // TODO:
    // - List files in directory
    // - Index binaries in PATH

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Write a new line before the prompt
    writeln!(handle).unwrap();

    prompt_order
        .iter()
        .map(|module| modules::handle(module, &context)) // Compute segments
        .flatten() // Remove segments set to `None`
        // TODO: Skip first prefix
        .for_each(|segment_string| write!(handle, "{}", segment_string).unwrap());
}
