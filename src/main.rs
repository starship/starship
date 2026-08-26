#![warn(clippy::disallowed_methods)]

use clap::crate_authors;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::generate;
use rand::RngExt;
use starship::context::{Context, Properties, Target};
use starship::module::ALL_MODULES;
use starship::stream::LatencyEstimates;
use starship::{
    ProcessId, StreamingTransport, bug_report, configure, init, logger, print, shadow, stream,
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum CompletionShell {
    Bash,
    Elvish,
    Fish,
    Nushell,
    #[clap(name = "powershell", alias = "pwsh", alias = "power-shell")]
    PowerShell,
    Zsh,
}

fn generate_shell(shell: impl clap_complete::Generator) {
    generate(
        shell,
        &mut Cli::command(),
        "starship",
        &mut io::stdout().lock(),
    );
}

fn generate_completions(shell: CompletionShell) {
    match shell {
        CompletionShell::Bash => generate_shell(clap_complete::Shell::Bash),
        CompletionShell::Elvish => generate_shell(clap_complete::Shell::Elvish),
        CompletionShell::Fish => generate_shell(clap_complete::Shell::Fish),
        CompletionShell::PowerShell => generate_shell(clap_complete::Shell::PowerShell),
        CompletionShell::Zsh => generate_shell(clap_complete::Shell::Zsh),
        CompletionShell::Nushell => generate_shell(clap_complete_nushell::Nushell),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Statuslines {
    #[clap(alias = "claude")]
    ClaudeCode,
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
        /// Output the preset to a file instead of stdout
        #[clap(short, long, conflicts_with = "list")]
        output: Option<PathBuf>,
        /// Forcibly overwrite the output file if it already exists
        #[clap(short, long, requires = "output")]
        force: bool,
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
        /// Print the prompt with the specified profile name (instead of the standard left prompt)
        #[clap(long, conflicts_with = "right")]
        profile: Option<String>,
        /// Print the continuation prompt (instead of the standard left prompt)
        #[clap(long, conflicts_with = "right", conflicts_with = "profile")]
        continuation: bool,
        #[clap(flatten)]
        properties: Properties,
    },
    /// Prints the prompt as a stream of frames: an immediate first paint,
    /// then incremental repaints as slow modules resolve
    Stream {
        /// Stream the right prompt (instead of the standard left prompt)
        #[clap(long, conflicts_with = "both")]
        right: bool,
        /// Stream both prompts from this one renderer: the right side's paints
        /// arrive as RIGHT frames, or as the second field of the snapshot record
        /// under --publish-state. A shell that draws both sides spends one
        /// process a prompt this way instead of two — which otherwise costs a
        /// whole process to render nothing whenever no right prompt is set.
        #[clap(long)]
        both: bool,
        /// What earlier prompts of this shell session measured each module to
        /// cost, as the payload of the last TIMING frame. Chooses how repaints
        /// are grouped and nothing else; a shell that keeps none of it, or
        /// hands back something malformed, gets a prompt that draws to a fixed
        /// window instead.
        #[clap(long, default_value = "", value_parser = LatencyEstimates::parse_argument)]
        timings: LatencyEstimates,
        /// Shell transport for refinements. `ble` enables the ble.sh hook in bash.
        #[clap(long, value_enum, default_value_t)]
        transport: StreamingTransport,
        /// fish integration: instead of streaming frames to stdout, atomically
        /// rewrite this file as `left\0right\0timings` after each paint. fish
        /// holds no pipe open across a prompt, so every paint after the first
        /// reaches it out of this file rather than off a stream.
        #[clap(long)]
        publish_state: Option<PathBuf>,
        /// Companion to --publish-state: write this prompt's completion timings
        /// here, for the next prompt's `--timings` handoff.
        #[clap(long, requires = "publish_state")]
        timings_out: Option<PathBuf>,
        /// Companion to --publish-state: signal this process `SIGUSR1` after
        /// each published paint, so fish re-reads the state file.
        #[clap(long, requires = "publish_state")]
        signal_pid: Option<u32>,
        /// Companion to --publish-state: fork into the background before doing
        /// any work, and announce the first paint on standard output as
        /// `left\0right\0pid\0`. The shell reads that straight out of the pipeline it
        /// launched this in, so it waits exactly as long as the first paint
        /// takes, and gets the renderer's pid to stop it with later.
        #[clap(long, requires = "publish_state")]
        detach: bool,
        #[clap(flatten)]
        properties: Properties,
    },
    /// Generate random session key
    Session,
    /// Prints the statusline with a specific profile
    Statusline {
        /// The statusline provider to use
        provider: Statuslines,
        #[clap(long)]
        profile: Option<String>,
        #[clap(flatten)]
        properties: Properties,
    },
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

/// Puts a `--detach`ed stream into the background before anything else runs.
///
/// fish has no way to wait for a background renderer's first paint: it cannot
/// hold a pipe open across a prompt, and its `read` has no timeout, so it used
/// to poll a file and pay a `sleep` fork plus a whole tick of latency. Detaching
/// inverts that. The renderer is run in the *foreground* of a pipeline, and this
/// fork hands the work to a child that keeps stdout: the parent exits at once so
/// the shell's job completes, and the shell's `read` blocks on the pipe until
/// the child announces its first paint — exactly as long as the paint takes, and
/// not one syscall longer.
///
/// This has to happen before anything spawns a thread — the log cleanup below,
/// and every module thread after it. `fork` carries only the calling thread into
/// the child, so any thread made first would be one the child believes in and
/// does not have. It reads the raw argument list for the same reason: clap has
/// not run yet.
#[cfg(unix)]
fn detach_if_requested() {
    if !std::env::args_os().any(|argument| argument == "--detach") {
        return;
    }
    // SAFETY: no thread has been spawned yet, so the child inherits a process
    // whose every lock and thread is accounted for.
    match unsafe { nix::unistd::fork() } {
        Ok(nix::unistd::ForkResult::Parent { .. }) => std::process::exit(0),
        // A session of its own, so a signal aimed at the shell's foreground job
        // cannot reach a renderer that has outlived the command line it drew.
        Ok(nix::unistd::ForkResult::Child) => {
            let _ = nix::unistd::setsid();
        }
        // Nothing has been written yet, so staying in the foreground is a slow
        // prompt rather than a broken one.
        Err(_) => {}
    }
}

fn main() {
    // Configure the current terminal on windows to support ANSI escape sequences.
    #[cfg(windows)]
    let _ = nu_ansi_term::enable_ansi_support();
    #[cfg(unix)]
    detach_if_requested();
    logger::init();

    // Delete old log files. Detached: nothing later waits on it, and the
    // process exits when the prompt is written whether or not it has finished.
    std::thread::spawn(|| {
        let log_dir = logger::get_log_dir();
        logger::cleanup_log_files(log_dir);
    });

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
                use io::Write;

                // print the arguments
                // avoid panicking in case of stderr closing
                let mut stderr = io::stderr();
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
    log::trace!("Parsed arguments: {args:#?}");

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
            profile,
            continuation,
        } => {
            let target = match (right, profile, continuation) {
                (true, _, _) => Target::Right,
                (_, Some(profile_name), _) => Target::Profile(profile_name),
                (_, _, true) => Target::Continuation,
                (_, _, _) => Target::Main,
            };
            print::prompt(properties, target);
        }
        Commands::Stream {
            right,
            both,
            timings,
            transport,
            publish_state,
            timings_out,
            signal_pid,
            detach,
            properties,
        } => {
            let target = if right { Target::Right } else { Target::Main };
            let delivery = match publish_state {
                Some(state) => stream::Delivery::Snapshot {
                    state,
                    timings_out,
                    signal_pid: signal_pid.map(ProcessId::from_raw),
                    announce_first_paint: detach,
                },
                None => stream::Delivery::Frames,
            };
            // A closed pipe just means the shell stopped wanting this prompt.
            if let Err(error) =
                stream::stream(properties, target, &timings, transport, delivery, both)
                && !error.is_broken_pipe()
            {
                eprintln!("Error streaming the prompt: {error}");
                std::process::exit(1);
            }
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
                    println!("{modules}");
                }
            }
            if let Some(module_name) = name {
                print::module(&module_name, properties);
            }
        }
        Commands::Preset {
            name,
            list,
            output,
            force,
        } => print::preset_command(name, output, force, list),
        Commands::Config { name, value } => {
            let context = Context::default();
            if let Some(name) = name {
                if let Some(value) = value {
                    configure::update_configuration(&context, &name, &value);
                }
            } else if let Err(reason) = configure::edit_configuration(&context, None) {
                eprintln!("Could not edit configuration: {reason}");
                std::process::exit(1);
            }
        }
        Commands::PrintConfig { default, name } => {
            configure::print_configuration(&Context::default(), default, &name);
        }
        Commands::Toggle { name, value } => {
            configure::toggle_configuration(&Context::default(), &name, &value);
        }
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
        Commands::Completions { shell } => generate_completions(shell),
        Commands::Session => println!(
            "{}",
            rand::rng()
                .sample_iter(rand::distr::Alphanumeric)
                .take(16)
                .map(char::from)
                .collect::<String>()
        ),
        Commands::Statusline {
            provider,
            profile,
            properties,
        } => {
            let profile = profile.unwrap_or_else(|| match provider {
                Statuslines::ClaudeCode => "claude-code".to_string(),
            });

            let target = Target::Profile(profile);
            print::prompt_with_claude_code(properties, target);
        }
        #[cfg(feature = "config-schema")]
        Commands::ConfigSchema => print::print_schema(),
    }
}
