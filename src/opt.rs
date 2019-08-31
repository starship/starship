use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct CommonOpts {
    /// The execution duration of the last command, in seconds
    #[structopt(short = "d", long = "cmd-duration")]
    pub cmd_duration: Option<u64>,

    /// The number of currently running jobs
    #[structopt(short = "j", long = "jobs", default_value = "0")]
    pub jobs: i64,

    /// The keymap of fish/zsh
    #[structopt(short = "k", long = "keymap", default_value = "viins")]
    pub keymap: String,

    /// The path that the prompt should render for
    #[structopt(short = "p", long = "path")]
    pub path: Option<PathBuf>,

    /// The status code of the previously run command
    #[structopt(
        name = "STATUS_CODE",
        short = "s",
        long = "status",
        default_value = "0"
    )]
    pub status: String,
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "git", about = "the stupid content tracker")]
pub enum SubCommand {
    /// Prints a specific prompt module
    #[structopt(name = "module")]
    Module {
        /// List out all supported modules
        #[structopt(short = "l", long = "list")]
        list: bool,

        #[structopt(flatten)]
        common_opts: CommonOpts,

        /// The name of the currently running shell
        /// Currently supported options: bash, zsh, fish
        #[structopt(name = "SHELL")]
        shell: String,
    },
    /// Prints the shell function used to execute starship
    #[structopt(name = "init")]
    Init {
        /// The name of the currently running shell
        /// Currently supported options: bash, zsh, fish
        #[structopt(name = "SHELL")]
        shell: String,

        /// Print the main initialization script (as opposed to the init stub)
        #[structopt(long = "print-full-init")]
        print_full_init: bool,
    },
    /// Prints the full starship prompt
    #[structopt(name = "prompt")]
    Prompt {
        #[structopt(flatten)]
        common_opts: CommonOpts,
    },
}

/// The cross-shell prompt for astronauts. ☄🌌️
#[derive(Clone, Debug, StructOpt)]
#[structopt(author, after_help = "https://github.com/starship/starship")]
pub struct Opt {
    #[structopt(subcommand)]
    pub sub_command: SubCommand,
}
