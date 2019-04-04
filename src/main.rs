#[macro_use]
extern crate clap;
extern crate ansi_term;

mod modules;
mod print;

use ansi_term::Style;
use clap::App;

pub struct Segment {
    style: Style,
    value: String,
    prefix: Option<Box<Segment>>,
    suffix: Option<Box<Segment>>,
}

fn main() {
    App::new("Starship")
        .about("The cross-platform prompt for astronauts. ✨🚀")
        // pull the version number from Cargo.toml
        .version(crate_version!())
        // pull the authors from Cargo.toml
        .author(crate_authors!())
        .after_help("https://github.com/matchai/starship")
        .get_matches();

    print::prompt();
}
