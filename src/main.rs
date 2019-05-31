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

use clap::{App, Arg, SubCommand};

fn main() {
    pretty_env_logger::init();

    let matches = App::new("Starship")
        .about("The cross-shell prompt for astronauts. ✨🚀")
        // pull the version number from Cargo.toml
        .version(crate_version!())
        // pull the authors from Cargo.toml
        .author(crate_authors!())
        .after_help("https://github.com/matchai/starship")
        .subcommand(
            SubCommand::with_name("prompt")
                .about("Prints the full starship prompt")
                .arg(
                    Arg::with_name("status_code")
                        .short("s")
                        .long("status")
                        .value_name("STATUS_CODE")
                        .help("The status code of the previously run command")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .value_name("PATH")
                        .help("The path that the prompt should render for ($PWD by default)")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("module")
                .about("Prints a specific prompt module")
                .arg(
                    Arg::with_name("module_name")
                        .help("The name of the module to be printed")
                        .required(true),
                )
                .arg(
                    Arg::with_name("status_code")
                        .short("s")
                        .long("status")
                        .value_name("STATUS_CODE")
                        .help("The status code of the previously run command")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .value_name("PATH")
                        .help("The path the prompt should render for ($PWD by default)")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("prompt", Some(sub_m)) => print::prompt(sub_m.clone()),
        ("module", Some(sub_m)) => {
            let module_name = sub_m.value_of("module_name").unwrap();
            print::module(module_name, sub_m.clone());
        }
        _ => {}
    }
}
