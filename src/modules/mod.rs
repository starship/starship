mod char;
mod dir;
mod line_sep;

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
        let default_suffix = Some(Box::new(Segment {
            style: Style::default(),
            value: String::from(" "),
            prefix: None,
            suffix: None
        }));

        Segment {
            style: Style::default(),
            value: String::from(""),
            prefix: None,
            suffix: default_suffix
        }
    }
}

pub fn handle(module: &str, args: &ArgMatches) -> Segment {
    match module {
        "char" => char::segment(&args),
        "dir" => dir::segment(&args),
        "line_sep" => line_sep::segment(&args),

        _ => panic!("Unknown module: {}", module),
    }
}
