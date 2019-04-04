mod char;

use crate::Segment;
use clap::ArgMatches;

pub fn handle(module: &str, args: &ArgMatches) -> Segment {
    match module {
        "char" => char::segment(&args),

        _ => panic!("Unknown module: {}", module),
    }
}
