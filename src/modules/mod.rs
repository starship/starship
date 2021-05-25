// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
mod aws;
mod character;
mod cmake;
mod cmd_duration;
mod conda;
mod crystal;
pub(crate) mod custom;
mod dart;
mod deno;
mod directory;
mod docker_context;
mod dotnet;
mod elixir;
mod elm;
mod env_var;
mod erlang;
mod gcloud;
mod git_branch;
mod git_commit;
mod git_state;
mod git_status;
mod golang;
mod helm;
mod hg_branch;
mod hostname;
mod java;
mod jobs;
mod julia;
mod kotlin;
mod kubernetes;
mod line_break;
mod lua;
mod memory_usage;
mod nim;
mod nix_shell;
mod nodejs;
mod ocaml;
mod openstack;
mod package;
mod perl;
mod php;
mod purescript;
mod python;
mod red;
mod rlang;
mod ruby;
mod rust;
mod scala;
mod shell;
mod shlvl;
mod singularity;
mod status;
mod swift;
mod terraform;
mod time;
mod username;
mod utils;
mod vagrant;
mod vcsh;
mod vlang;
mod zig;

#[cfg(feature = "battery")]
mod battery;

use crate::config::RootModuleConfig;
use crate::context::{Context, Shell};
use crate::module::Module;
use std::time::Instant;

pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    let start: Instant = Instant::now();

    let mut m: Option<Module> = {
        match module {
            // Keep these ordered alphabetically.
            // Default ordering is handled in configs/starship_root.rs
            "aws" => aws::module(context),
            #[cfg(feature = "battery")]
            "battery" => battery::module(context),
            "character" => character::module(context),
            "cmake" => cmake::module(context),
            "cmd_duration" => cmd_duration::module(context),
            "conda" => conda::module(context),
            "dart" => dart::module(context),
            "deno" => deno::module(context),
            "directory" => directory::module(context),
            "docker_context" => docker_context::module(context),
            "dotnet" => dotnet::module(context),
            "elixir" => elixir::module(context),
            "elm" => elm::module(context),
            "erlang" => erlang::module(context),
            "env_var" => env_var::module(context),
            "gcloud" => gcloud::module(context),
            "git_branch" => git_branch::module(context),
            "git_commit" => git_commit::module(context),
            "git_state" => git_state::module(context),
            "git_status" => git_status::module(context),
            "golang" => golang::module(context),
            "helm" => helm::module(context),
            "hg_branch" => hg_branch::module(context),
            "hostname" => hostname::module(context),
            "java" => java::module(context),
            "jobs" => jobs::module(context),
            "julia" => julia::module(context),
            "kotlin" => kotlin::module(context),
            "kubernetes" => kubernetes::module(context),
            "line_break" => line_break::module(context),
            "lua" => lua::module(context),
            "memory_usage" => memory_usage::module(context),
            "nim" => nim::module(context),
            "nix_shell" => nix_shell::module(context),
            "nodejs" => nodejs::module(context),
            "ocaml" => ocaml::module(context),
            "openstack" => openstack::module(context),
            "package" => package::module(context),
            "perl" => perl::module(context),
            "php" => php::module(context),
            "purescript" => purescript::module(context),
            "python" => python::module(context),
            "rlang" => rlang::module(context),
            "red" => red::module(context),
            "ruby" => ruby::module(context),
            "rust" => rust::module(context),
            "scala" => scala::module(context),
            "shell" => shell::module(context),
            "shlvl" => shlvl::module(context),
            "singularity" => singularity::module(context),
            "swift" => swift::module(context),
            "status" => status::module(context),
            "terraform" => terraform::module(context),
            "time" => time::module(context),
            "crystal" => crystal::module(context),
            "username" => username::module(context),
            "vlang" => vlang::module(context),
            "vagrant" => vagrant::module(context),
            "vcsh" => vcsh::module(context),
            "zig" => zig::module(context),
            _ => {
                eprintln!("Error: Unknown module {}. Use starship module --list to list out all supported modules.", module);
                None
            }
        }
    };

    let elapsed = start.elapsed();
    log::trace!("Took {:?} to compute module {:?}", elapsed, module);
    if elapsed.as_millis() < 1 {
        // If we take less than 1ms to compute a None, then we will not return a module at all
        // if we have a module: default duration is 0 so no need to change it
        m
    } else {
        // if we took more than 1ms we want to report that and so--in case we have None currently--
        // need to create an empty module just to hold the duration for that case
        m.get_or_insert_with(|| context.new_module(module)).duration = elapsed;
        m
    }
}

pub fn description(module: &str) -> &'static str {
    match module {
        "aws" => "The current AWS region and profile",
        "battery" => "The current charge of the device's battery and its current charging status",
        "character" => {
            "A character (usually an arrow) beside where the text is entered in your terminal"
        }
        "cmake" => "The currently installed version of CMake",
        "cmd_duration" => "How long the last command took to execute",
        "conda" => "The current conda environment, if $CONDA_DEFAULT_ENV is set",
        "crystal" => "The currently installed version of Crystal",
        "dart" => "The currently installed version of Dart",
        "deno" => "The currently installed version of Deno",
        "directory" => "The current working directory",
        "docker_context" => "The current docker context",
        "dotnet" => "The relevant version of the .NET Core SDK for the current directory",
        "env_var" => "Displays the current value of a selected environment variable",
        "erlang" => "Current OTP version",
        "gcloud" => "The current GCP client configuration",
        "git_branch" => "The active branch of the repo in your current directory",
        "git_commit" => "The active commit (and tag if any) of the repo in your current directory",
        "git_state" => "The current git operation, and it's progress",
        "git_status" => "Symbol representing the state of the repo",
        "golang" => "The currently installed version of Golang",
        "helm" => "The currently installed version of Helm",
        "hg_branch" => "The active branch of the repo in your current directory",
        "hostname" => "The system hostname",
        "java" => "The currently installed version of Java",
        "jobs" => "The current number of jobs running",
        "julia" => "The currently installed version of Julia",
        "kotlin" => "The currently installed version of Kotlin",
        "kubernetes" => "The current Kubernetes context name and, if set, the namespace",
        "line_break" => "Separates the prompt into two lines",
        "lua" => "The currently installed version of Lua",
        "memory_usage" => "Current system memory and swap usage",
        "nim" => "The currently installed version of Nim",
        "nix_shell" => "The nix-shell environment",
        "nodejs" => "The currently installed version of NodeJS",
        "ocaml" => "The currently installed version of OCaml",
        "openstack" => "The current OpenStack cloud and project",
        "package" => "The package version of the current directory's project",
        "perl" => "The currently installed version of Perl",
        "php" => "The currently installed version of PHP",
        "purescript" => "The currently installed version of PureScript",
        "python" => "The currently installed version of Python",
        "rlang" => "The currently installed version of R",
        "red" => "The currently installed version of Red",
        "ruby" => "The currently installed version of Ruby",
        "rust" => "The currently installed version of Rust",
        "scala" => "The currently installed version of Scala",
        "swift" => "The currently installed version of Swift",
        "shell" => "The currently used shell indicator",
        "shlvl" => "The current value of SHLVL",
        "status" => "The status of the last command",
        "terraform" => "The currently selected terraform workspace and version",
        "time" => "The current local time",
        "username" => "The active user's username",
        "vlang" => "The currently installed version of V",
        "vagrant" => "The currently installed version of Vagrant",
        "vcsh" => "The currently active VCSH repository",
        "zig" => "The currently installed version of Zig",
        _ => "<no description>",
    }
}
