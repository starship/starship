mod character;
mod directory;
mod line_break;
mod nodejs;

use crate::segment::Segment;
use clap::ArgMatches;
use std::path::Path;

pub fn handle(module: &str, current_dir: &Path, args: &ArgMatches) -> Option<Segment> {
    match module {
        "dir" | "directory" => directory::segment(current_dir, args),
        "char" | "character" => character::segment(current_dir, args),
        "node" | "nodejs" => nodejs::segment(current_dir, args),
        "line_break" => line_break::segment(current_dir, args),

        _ => panic!("Unknown module: {}", module),
    }
}
