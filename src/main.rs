#![warn(clippy::disallowed_methods)]

use clap::crate_authors;
use std::io;
use std::thread::available_parallelism;
use std::time::SystemTime;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell as CompletionShell};
use rand::distributions::Alphanumeric;
use rand::Rng;
use starship::context::{Properties, Target};
use starship::module::ALL_MODULES;
use starship::*;

#[derive(Parser, Debug)]
#[clap(
    author=crate_authors!(),
    version=shadow::PKG_VERSION,
    long_version=shadow::CLAP_LONG_VERSION,
    about="The cross-shell prompt for astronauts. ☄🌌️",
    subcommand_required=true,
    arg_required_else_help=true,
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a pre-populated GitHub issue with information about your configuration
    BugReport,
    /// Generate starship shell completions for your shell to stdout
    Completions {
        #[clap(value_enum)]
        shell: CompletionShell,
    },
    /// Edit the starship configuration
    Config {
        /// Configuration key to edit
        #[clap(requires = "value")]
        name: Option<String>,
        /// Value to place into that key
        value: Option<String>,
    },
    /// Explains the currently showing modules
    Explain(Properties),
    ///  Prints the shell function used to execute starship
    Init {
        shell: String,
        #[clap(long)]
        print_full_init: bool,
    },
    ///  Prints a specific prompt module
    Module {
        /// The name of the module to be printed
        #[clap(required_unless_present("list"))]
        name: Option<String>,
        /// List out all supported modules
        #[clap(short, long)]
        list: bool,
        #[clap(flatten)]
        properties: Properties,
    },
    /// Prints a preset config
    Preset {
        /// The name of preset to be printed
        #[clap(required_unless_present("list"), value_enum)]
        name: Option<print::Preset>,
        /// List out all preset names
        #[clap(short, long)]
        list: bool,
    },
    /// Prints the computed starship configuration
    PrintConfig {
        /// Print the default instead of the computed config
        #[clap(short, long)]
        default: bool,
        /// Configuration keys to print
        name: Vec<String>,
    },
    /// Prints the full starship prompt
    Prompt {
        /// Print the right prompt (instead of the standard left prompt)
        #[clap(long)]
        right: bool,
        /// Print the continuation prompt (instead of the standard left prompt)
        #[clap(long, conflicts_with = "right")]
        continuation: bool,
        #[clap(flatten)]
        properties: Properties,
    },
    /// Generate random session key
    Session,
    /// Prints time in milliseconds
    #[clap(hide = true)]
    Time,
    /// Prints timings of all active modules
    Timings(Properties),
    /// Toggle a given starship module
    Toggle {
        /// The name of the module to be toggled
        name: String,
        /// The key of the config to be toggled
        #[clap(default_value = "disabled")]
        value: String,
    },
    #[cfg(feature = "config-schema")]
    /// Generate a schema for the starship configuration as JSON-schema
    ConfigSchema,
}

fn main() {
    // Configure the current terminal on windows to support ANSI escape sequences.
    #[cfg(windows)]
    let _ = nu_ansi_term::enable_ansi_support();
    logger::init();
    init_global_threadpool();

    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(e) => {
            // if the error is not printed to stderr, this means it was not really
            // an error but rather some information is going to be listed, therefore
            // we won't print the arguments passed
            let is_info_only = !e.use_stderr();
            // print the error and void panicking in case of stdout/stderr closing unexpectedly
            let _ = e.print();
            // if there was no mistake by the user and we're only going to display information,
            // we won't put arguments or exit with non-zero code
            let exit_code = if is_info_only {
                0
            } else {
                // print the arguments
                // avoid panicking in case of stderr closing
                let mut stderr = io::stderr();
                use io::Write;
                let _ = writeln!(
                    stderr,
                    "\nNOTE:\n    passed arguments: {:?}",
                    // collect into a vec to format args as a slice
                    std::env::args().skip(1).collect::<Vec<_>>()
                );
                // clap exits with status 2 on error:
                //  https://docs.rs/clap/latest/clap/struct.Error.html#method.exit
                2
            };

            std::process::exit(exit_code);
        }
    };
    log::trace!("Parsed arguments: {:#?}", args);

    match args.command {
        Commands::Init {
            shell,
            print_full_init,
        } => {
            if print_full_init {
                init::init_main(&shell).expect("can't init_main");
            } else {
                init::init_stub(&shell).expect("can't init_stub");
            }
        }
        Commands::Prompt {
            properties,
            right,
            continuation,
        } => {
            let target = match (right, continuation) {
                (true, _) => Target::Right,
                (_, true) => Target::Continuation,
                (_, _) => Target::Main,
            };
            print::prompt(properties, target)
        }
        Commands::Module {
            name,
            list,
            properties,
        } => {
            if list {
                println!("Supported modules list");
                println!("----------------------");
                for modules in ALL_MODULES {
                    println!("{}", modules);
                }
            }
            if let Some(module_name) = name {
                print::module(&module_name, properties);
            }
        }
        Commands::Preset { name, list } => print::preset_command(name, list),
        Commands::Config { name, value } => {
            if let Some(name) = name {
                if let Some(value) = value {
                    configure::update_configuration(&name, &value)
                }
            } else if let Err(reason) = configure::edit_configuration(None) {
                eprintln!("Could not edit configuration: {}", reason);
                std::process::exit(1);
            }
        }
        Commands::PrintConfig { default, name } => configure::print_configuration(default, &name),
        Commands::Toggle { name, value } => configure::toggle_configuration(&name, &value),
        Commands::BugReport => bug_report::create(),
        Commands::Time => {
            match SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
            {
                Some(time) => println!("{}", time.as_millis()),
                None => println!("{}", -1),
            }
        }
        Commands::Explain(props) => print::explain(props),
        Commands::Timings(props) => print::timings(props),
        Commands::Completions { shell } => generate(
            shell,
            &mut Cli::command(),
            "starship",
            &mut io::stdout().lock(),
        ),
        Commands::Session => println!(
            "{}",
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect::<String>()
        ),
        #[cfg(feature = "config-schema")]
        Commands::ConfigSchema => print::print_schema(),
    }
}

/// Initialize global `rayon` thread pool
fn init_global_threadpool() {
    // Allow overriding the number of threads
    let num_threads = std::env::var("STARSHIP_NUM_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        // Default to the number of logical cores,
        // but restrict the number of threads to 8
        .unwrap_or_else(|| available_parallelism().map(usize::from).unwrap_or(1).min(8));

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to initialize worker thread pool");
}
