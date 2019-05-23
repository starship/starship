#[macro_use]
extern crate clap;

extern crate ansi_term;
extern crate battery;
extern crate dirs;
extern crate git2;
extern crate pretty_env_logger;

mod context;
mod module;
mod modules;
mod print;
mod segment;

use clap::{App, Arg};

fn main() {
    pretty_env_logger::init();

    let args = App::new("Starship")
        .about("The cross-shell prompt for astronauts. ☄🌌️")
        // pull the version number from Cargo.toml
        .version(crate_version!())
        // pull the authors from Cargo.toml
        .author(crate_authors!())
        .after_help("https://github.com/matchai/starship")
        .arg(
            Arg::with_name("status_code")
                .help("The status code of the previously run command")
                .required(true),
        )
        .get_matches();

    print::prompt(args);
}
