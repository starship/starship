mod character;
mod directory;
mod line_break;
mod nodejs;

use ansi_term::Style;
use clap::ArgMatches;

// pub static current_dir: PathBuf = env::current_dir().expect("Unable to identify current directory");
// TODO: Currently gets the physical directory. Get the logical directory.

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
        "dir" | "directory" => directory::segment(&args),
        "char" | "character" => character::segment(&args),
        "node" | "nodejs" => nodejs::segment(&args),
        "line_break" => line_break::segment(&args),

        _ => panic!("Unknown module: {}", module),
    }
}
