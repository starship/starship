#[macro_use]
extern crate clap;

mod config;
mod context;
mod init;
mod module;
mod modules;
mod print;
mod segment;
mod utils;

use crate::module::ALL_MODULES;
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
        .help(
            "The name of the currently running shell\nCurrently supported options: bash, zsh, fish",
        )
        .required(true);

    let cmd_duration_arg = Arg::with_name("cmd_duration")
        .short("d")
        .long("cmd-duration")
        .value_name("CMD_DURATION")
        .help("The execution duration of the last command, in seconds")
        .takes_value(true);

    let keymap_arg = Arg::with_name("keymap")
        .short("k")
        .long("keymap")
        .value_name("KEYMAP")
        // zsh only
        .help("The keymap of zsh")
        .takes_value(true);

    let jobs_arg = Arg::with_name("jobs")
        .short("j")
        .long("jobs")
        .value_name("JOBS")
        .help("The number of currently running jobs")
        .takes_value(true);

    let init_scripts_arg = Arg::with_name("print_full_init")
        .long("print-full-init")
        .help("Print the main initialization script (as opposed to the init stub)");

    let matches = App::new("starship")
        .about("The cross-shell prompt for astronauts. ☄🌌️")
        // pull the version number from Cargo.toml
        .version(crate_version!())
        // pull the authors from Cargo.toml
        .author(crate_authors!())
        .after_help("https://github.com/starship/starship")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Prints the shell function used to execute starship")
                .arg(&shell_arg)
                .arg(&init_scripts_arg),
        )
        .subcommand(
            SubCommand::with_name("prompt")
                .about("Prints the full starship prompt")
                .arg(&status_code_arg)
                .arg(&path_arg)
                .arg(&cmd_duration_arg)
                .arg(&keymap_arg)
                .arg(&jobs_arg),
        )
        .subcommand(
            SubCommand::with_name("module")
                .about("Prints a specific prompt module")
                .arg(
                    Arg::with_name("name")
                        .help("The name of the module to be printed")
                        .required(true)
                        .required_unless("list"),
                )
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("List out all supported modules"),
                )
                .arg(&status_code_arg)
                .arg(&path_arg)
                .arg(&cmd_duration_arg)
                .arg(&keymap_arg)
                .arg(&jobs_arg),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(sub_m)) => {
            let shell_name = sub_m.value_of("shell").expect("Shell name missing.");
            if sub_m.is_present("print_full_init") {
                init::init_main(shell_name);
            } else {
                init::init_stub(shell_name);
            }
        }
        ("prompt", Some(sub_m)) => print::prompt(sub_m.clone()),
        ("module", Some(sub_m)) => {
            if sub_m.is_present("list") {
                println!("Supported modules list");
                println!("----------------------");
                for modules in ALL_MODULES {
                    println!("{}", modules);
                }
            }
            if let Some(module_name) = sub_m.value_of("name") {
                print::module(module_name, sub_m.clone());
            }
        }
        _ => {}
    }
}
