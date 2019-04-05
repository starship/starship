mod character;
mod directory;
mod line_break;

use ansi_term::Style;
use clap::ArgMatches;

pub struct Segment {
    pub style: Style,
    pub value: String,
    pub prefix: Option<Box<Segment>>,
    pub suffix: Option<Box<Segment>>,
}

impl Default for Segment {
    fn default() -> Segment {
        let default_suffix = Some(Box::new(Segment {
            style: Style::default(),
            value: String::from(" "),
            prefix: None,
            suffix: None,
        }));

        Segment {
            style: Style::default(),
            value: String::from(""),
            prefix: None,
            suffix: default_suffix,
        }
    }
}

pub fn handle(module: &str, args: &ArgMatches) -> Segment {
    match module {
        "char" | "character" => character::segment(&args),
        "dir" | "directory" => directory::segment(&args),
        "line_break" => line_break::segment(&args),

        _ => panic!("Unknown module: {}", module),
    }
}
