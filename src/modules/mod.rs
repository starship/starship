mod char;

use clap::ArgMatches;
use ansi_term::Style;

pub struct Segment {
    pub style: Style,
    pub value: String,
    pub prefix: Option<Box<Segment>>,
    pub suffix: Option<Box<Segment>>,
}

impl Default for Segment {
    fn default() -> Segment {
        Segment {
            style: Style::default(),
            value: String::from(""),
            prefix: None,
            suffix: None
        }
    }
}

pub fn handle(module: &str, args: &ArgMatches) -> Segment {
    match module {
        "char" => char::segment(&args),

        _ => panic!("Unknown module: {}", module),
    }
}
