#[macro_use]
extern crate clap;

mod init;
mod config;
mod context;
mod module;
mod modules;
mod print;
mod segment;
mod utils;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    pretty_env_logger::init();

    let status_code_arg = Arg::with_name("status_code")
        .short("s")
        .long("status")
        .value_name("STATUS_CODE")
        .help("The status code of the previously run command")
        .takes_value(true);

    let path_arg = Arg::with_name("path")
        .short("p")
        .long("path")
        .value_name("PATH")
        .help("The path that the prompt should render for")
        .takes_value(true);

    let shell_arg = Arg::with_name("shell")
        .value_name("SHELL")
        .help("The name of the currently running shell\nCurrently supported options: bash, zsh, fish")
        .required(true);

    let matches = App::new("starship")
        .about("The cross-shell prompt for astronauts. ✨🚀")
        // pull the version number from Cargo.toml
        .version(crate_version!())
        // pull the authors from Cargo.toml
        .author(crate_authors!())
        .after_help("https://github.com/matchai/starship")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Prints the shell function used to execute starship")
                .arg(&shell_arg),
        )
        .subcommand(
            SubCommand::with_name("prompt")
                .about("Prints the full starship prompt")
                .arg(&status_code_arg)
                .arg(&path_arg),
        )
        .subcommand(
            SubCommand::with_name("module")
                .about("Prints a specific prompt module")
                .arg(
                    Arg::with_name("name")
                        .help("The name of the module to be printed")
                        .required(true),
                )
                .arg(&status_code_arg)
                .arg(&path_arg),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(sub_m)) => {
            let shell_name = sub_m.value_of("shell").expect("Shell name missing.");
            init::init(shell_name)
        },
        ("prompt", Some(sub_m)) => print::prompt(sub_m.clone()),
        ("module", Some(sub_m)) => {
            let module_name = sub_m.value_of("name").expect("Module name missing.");
            print::module(module_name, sub_m.clone());
        }
        _ => {}
    }
}
