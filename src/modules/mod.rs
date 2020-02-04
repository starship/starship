// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
mod aws;
mod character;
mod cmd_duration;
mod conda;
mod crystal;
mod directory;
mod dotnet;
mod env_var;
mod git_branch;
mod git_commit;
mod git_state;
mod git_status;
mod golang;
mod haskell;
mod hg_branch;
mod hostname;
mod java;
mod jobs;
mod kubernetes;
mod line_break;
mod memory_usage;
mod nix_shell;
mod nodejs;
mod package;
mod php;
mod python;
mod ruby;
mod rust;
mod terraform;
mod time;
mod username;
mod utils;

#[cfg(feature = "battery")]
mod battery;

use crate::config::{RootModuleConfig, SegmentConfig};
use crate::context::{Context, Shell};
use crate::module::Module;

pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    match module {
        // Keep these ordered alphabetically.
        // Default ordering is handled in configs/mod.rs
        "aws" => aws::module(context),
        #[cfg(feature = "battery")]
        "battery" => battery::module(context),
        "character" => character::module(context),
        "cmd_duration" => cmd_duration::module(context),
        "conda" => conda::module(context),
        "directory" => directory::module(context),
        "dotnet" => dotnet::module(context),
        "env_var" => env_var::module(context),
        "git_branch" => git_branch::module(context),
        "git_commit" => git_commit::module(context),
        "git_state" => git_state::module(context),
        "git_status" => git_status::module(context),
        "golang" => golang::module(context),
        "haskell" => haskell::module(context),
        "hg_branch" => hg_branch::module(context),
        "hostname" => hostname::module(context),
        "java" => java::module(context),
        "jobs" => jobs::module(context),
        "kubernetes" => kubernetes::module(context),
        "line_break" => line_break::module(context),
        "memory_usage" => memory_usage::module(context),
        "nix_shell" => nix_shell::module(context),
        "nodejs" => nodejs::module(context),
        "package" => package::module(context),
        "php" => php::module(context),
        "python" => python::module(context),
        "ruby" => ruby::module(context),
        "rust" => rust::module(context),
        "terraform" => terraform::module(context),
        "time" => time::module(context),
        "crystal" => crystal::module(context),
        "username" => username::module(context),
        _ => {
            eprintln!("Error: Unknown module {}. Use starship module --list to list out all supported modules.", module);
            None
        }
    }
}

pub fn description(module: &str) -> &'static str {
    match module {
        "aws" => "The current AWS region and profile",
        "battery" => "The current charge of the device's battery and its current charging status",
        "character" => {
            "A character (usually an arrow) beside where the text is entered in your terminal"
        }
        "cmd_duration" => "How long the last command took to execute",
        "conda" => "The current conda environment, if $CONDA_DEFAULT_ENV is set",
        "directory" => "The current working directory",
        "dotnet" => "The relevant version of the .NET Core SDK for the current directory",
        "env_var" => "Displays the current value of a selected environment variable",
        "git_branch" => "The active branch of the repo in your current directory",
        "git_commit" => "The active commit of the repo in your current directory",
        "git_state" => "The current git operation, and it's progress",
        "git_status" => "Symbol representing the state of the repo",
        "golang" => "The currently installed version of Golang",
        "hg_branch" => "The active branch of the repo in your current directory",
        "hostname" => "The system hostname",
        "java" => "The currently installed version of Java",
        "jobs" => "The current number of jobs running",
        "kubernetes" => "The current Kubernetes context name and, if set, the namespace",
        "line_break" => "Separates the prompt into two lines",
        "memory_usage" => "Current system memory and swap usage",
        "nix_shell" => "The nix-shell environment",
        "nodejs" => "The currently installed version of NodeJS",
        "package" => "The package version of the current directory's project",
        "php" => "The currently installed version of PHP",
        "python" => "The currently installed version of Python",
        "ruby" => "The currently installed version of Ruby",
        "rust" => "The currently installed version of Rust",
        "terraform" => "The currently selected terraform workspace and version",
        "time" => "The current local time",
        "username" => "The active user's username",
        _ => "<no description>",
    }
}
