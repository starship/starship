#[macro_use]
extern crate clap;
use clap::App;
use std::io;

mod char;

fn main() {
    App::new("Starship")
        .about("The cross-platform prompt for astronauts. ✨🚀")
        // pull the version number from Cargo.toml
        .version(crate_version!())
        // pull the authors from Cargo.toml
        .author(crate_authors!())
        .get_matches();

    prompt::char();
    // let stdout = io::stdout();
    // let mut handle = io::BufWriter::new(stdout);
}
