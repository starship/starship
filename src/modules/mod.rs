mod character;
mod directory;
mod line_break;
mod nodejs;

use crate::segment::Segment;
use clap::ArgMatches;

// pub static current_dir: PathBuf = env::current_dir().expect("Unable to identify current directory");
// TODO: Currently gets the physical directory. Get the logical directory.

pub fn handle(module: &str, args: &ArgMatches) -> Segment {
    match module {
        "dir" | "directory" => directory::segment(&args),
        "char" | "character" => character::segment(&args),
        "node" | "nodejs" => nodejs::segment(&args),
        "line_break" => line_break::segment(&args),

        _ => panic!("Unknown module: {}", module),
    }
}
