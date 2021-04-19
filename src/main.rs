use clap::{crate_authors, crate_version};
use std::io;
use std::time::SystemTime;

use clap::{App, AppSettings, Arg, Shell, SubCommand};
use rand::distributions::Alphanumeric;
use rand::Rng;
use starship::module::ALL_MODULES;
use starship::*;

fn main() {
    logger::init();

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
            "The name of the currently running shell\nCurrently supported options: bash, zsh, fish, powershell, ion",
        )
        .required(true);

    let cmd_duration_arg = Arg::with_name("cmd_duration")
        .short("d")
        .long("cmd-duration")
        .value_name("CMD_DURATION")
        .help("The execution duration of the last command, in milliseconds")
        .takes_value(true);

    let keymap_arg = Arg::with_name("keymap")
        .short("k")
        .long("keymap")
        .value_name("KEYMAP")
        // fish/zsh only
        .help("The keymap of fish/zsh")
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

    let mut app = App::new("starship")
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
        .subcommand(
            SubCommand::with_name("config")
                .alias("configure")
                .about("Edit the starship configuration")
                .arg(
                    Arg::with_name("name")
                        .help("Configuration key to edit")
                        .required(false)
                        .requires("value"),
                )
                .arg(Arg::with_name("value").help("Value to place into that key")),
        )
        .subcommand(
            SubCommand::with_name("bug-report").about(
                "Create a pre-populated GitHub issue with information about your configuration",
            ),
        )
        .subcommand(
            SubCommand::with_name("time")
                .about("Prints time in milliseconds")
                .settings(&[AppSettings::Hidden]),
        )
        .subcommand(
            SubCommand::with_name("explain").about("Explains the currently showing modules"),
        )
        .subcommand(SubCommand::with_name("timings").about("Prints timings of all active modules"))
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate starship shell completions for your shell to stdout")
                .arg(
                    Arg::with_name("shell")
                        .takes_value(true)
                        .possible_values(&Shell::variants())
                        .help("the shell to generate completions for")
                        .value_name("SHELL")
                        .required(true)
                        .env("STARSHIP_SHELL"),
                ),
        )
        .subcommand(SubCommand::with_name("session").about("Generate random session key"));

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("init", Some(sub_m)) => {
            let shell_name = sub_m.value_of("shell").expect("Shell name missing.");
            if sub_m.is_present("print_full_init") {
                init::init_main(shell_name).expect("can't init_main");
            } else {
                init::init_stub(shell_name).expect("can't init_stub");
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
        ("config", Some(sub_m)) => {
            if let Some(name) = sub_m.value_of("name") {
                if let Some(value) = sub_m.value_of("value") {
                    configure::update_configuration(name, value)
                }
            } else {
                configure::edit_configuration()
            }
        }
        ("bug-report", Some(_)) => bug_report::create(),
        ("time", _) => {
            match SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
            {
                Some(time) => println!("{}", time.as_millis()),
                None => println!("{}", -1),
            }
        }
        ("explain", Some(sub_m)) => print::explain(sub_m.clone()),
        ("timings", Some(sub_m)) => print::timings(sub_m.clone()),
        ("completions", Some(sub_m)) => {
            let shell: Shell = sub_m
                .value_of("shell")
                .expect("Shell name missing.")
                .parse()
                .expect("Invalid shell");

            app.gen_completions_to("starship", shell, &mut io::stdout().lock());
        }
        ("session", _) => println!(
            "{}",
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .collect::<String>()
        ),
        (command, _) => unreachable!("Invalid subcommand: {}", command),
    }
}
