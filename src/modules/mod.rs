// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
mod aws;
mod azure;
mod buf;
mod bun;
mod c;
mod character;
mod cmake;
mod cmd_duration;
mod cobol;
mod conda;
mod container;
mod crystal;
pub mod custom;
mod daml;
mod dart;
mod deno;
mod directory;
mod docker_context;
mod dotnet;
mod elixir;
mod elm;
mod env_var;
mod erlang;
mod fennel;
mod fill;
mod fossil_branch;
mod fossil_metrics;
mod gcloud;
mod git_branch;
mod git_commit;
mod git_metrics;
mod git_state;
mod git_status;
mod golang;
mod gradle;
mod guix_shell;
mod haskell;
mod haxe;
mod helm;
mod hg_branch;
mod hostname;
mod java;
mod jobs;
mod julia;
mod kotlin;
mod kubernetes;
mod line_break;
mod localip;
mod lua;
mod memory_usage;
mod meson;
mod nim;
mod nix_shell;
mod nodejs;
mod ocaml;
mod opa;
mod openstack;
mod os;
mod package;
mod perl;
mod php;
mod pijul_channel;
mod pulumi;
mod purescript;
mod python;
mod raku;
mod red;
mod rlang;
mod ruby;
mod rust;
mod scala;
mod shell;
mod shlvl;
mod singularity;
mod solidity;
mod spack;
mod status;
mod sudo;
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
mod typst;

#[cfg(feature = "battery")]
pub use self::battery::{BatteryInfoProvider, BatteryInfoProviderImpl};

use crate::config::ModuleConfig;
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
            "azure" => azure::module(context),
            #[cfg(feature = "battery")]
            "battery" => battery::module(context),
            "buf" => buf::module(context),
            "bun" => bun::module(context),
            "c" => c::module(context),
            "character" => character::module(context),
            "cmake" => cmake::module(context),
            "cmd_duration" => cmd_duration::module(context),
            "cobol" => cobol::module(context),
            "conda" => conda::module(context),
            "container" => container::module(context),
            "daml" => daml::module(context),
            "dart" => dart::module(context),
            "deno" => deno::module(context),
            "directory" => directory::module(context),
            "docker_context" => docker_context::module(context),
            "dotnet" => dotnet::module(context),
            "elixir" => elixir::module(context),
            "elm" => elm::module(context),
            "erlang" => erlang::module(context),
            "env_var" => env_var::module(None, context),
            "fennel" => fennel::module(context),
            "fill" => fill::module(context),
            "fossil_branch" => fossil_branch::module(context),
            "fossil_metrics" => fossil_metrics::module(context),
            "gcloud" => gcloud::module(context),
            "git_branch" => git_branch::module(context),
            "git_commit" => git_commit::module(context),
            "git_metrics" => git_metrics::module(context),
            "git_state" => git_state::module(context),
            "git_status" => git_status::module(context),
            "golang" => golang::module(context),
            "gradle" => gradle::module(context),
            "guix_shell" => guix_shell::module(context),
            "haskell" => haskell::module(context),
            "haxe" => haxe::module(context),
            "helm" => helm::module(context),
            "hg_branch" => hg_branch::module(context),
            "hostname" => hostname::module(context),
            "java" => java::module(context),
            "jobs" => jobs::module(context),
            "julia" => julia::module(context),
            "kotlin" => kotlin::module(context),
            "kubernetes" => kubernetes::module(context),
            "line_break" => line_break::module(context),
            "localip" => localip::module(context),
            "lua" => lua::module(context),
            "memory_usage" => memory_usage::module(context),
            "meson" => meson::module(context),
            "nim" => nim::module(context),
            "nix_shell" => nix_shell::module(context),
            "nodejs" => nodejs::module(context),
            "ocaml" => ocaml::module(context),
            "opa" => opa::module(context),
            "openstack" => openstack::module(context),
            "os" => os::module(context),
            "package" => package::module(context),
            "perl" => perl::module(context),
            "php" => php::module(context),
            "pijul_channel" => pijul_channel::module(context),
            "pulumi" => pulumi::module(context),
            "purescript" => purescript::module(context),
            "python" => python::module(context),
            "raku" => raku::module(context),
            "rlang" => rlang::module(context),
            "red" => red::module(context),
            "ruby" => ruby::module(context),
            "rust" => rust::module(context),
            "scala" => scala::module(context),
            "shell" => shell::module(context),
            "shlvl" => shlvl::module(context),
            "singularity" => singularity::module(context),
            "solidity" => solidity::module(context),
            "spack" => spack::module(context),
            "swift" => swift::module(context),
            "status" => status::module(context),
            "sudo" => sudo::module(context),
            "terraform" => terraform::module(context),
            "time" => time::module(context),
            "typst" => typst::module(context),
            "crystal" => crystal::module(context),
            "username" => username::module(context),
            "vlang" => vlang::module(context),
            "vagrant" => vagrant::module(context),
            "vcsh" => vcsh::module(context),
            "zig" => zig::module(context),
            env if env.starts_with("env_var.") => {
                env_var::module(env.strip_prefix("env_var."), context)
            }
            custom if custom.starts_with("custom.") => {
                // SAFETY: We just checked that the module starts with "custom."
                custom::module(custom.strip_prefix("custom.").unwrap(), context)
            }
            _ => {
                eprintln!("Error: Unknown module {module}. Use starship module --list to list out all supported modules.");
                None
            }
        }
    };

    let elapsed = start.elapsed();
    log::trace!("Took {:?} to compute module {:?}", elapsed, module);
    if elapsed.as_millis() >= 1 {
        // If we take less than 1ms to compute a None, then we will not return a module at all
        // if we have a module: default duration is 0 so no need to change it
        // if we took more than 1ms we want to report that and so--in case we have None currently--
        // need to create an empty module just to hold the duration for that case
        m.get_or_insert_with(|| context.new_module(module)).duration = elapsed;
    }
    m
}

pub fn description(module: &str) -> &'static str {
    match module {
        "aws" => "The current AWS region and profile",
        "azure" => "The current Azure subscription",
        "battery" => "The current charge of the device's battery and its current charging status",
        "buf" => "The currently installed version of the Buf CLI",
        "bun" => "The currently installed version of the Bun",
        "c" => "Your C compiler type",
        "character" => {
            "A character (usually an arrow) beside where the text is entered in your terminal"
        }
        "cmake" => "The currently installed version of CMake",
        "cmd_duration" => "How long the last command took to execute",
        "cobol" => "The currently installed version of COBOL/GNUCOBOL",
        "conda" => "The current conda environment, if $CONDA_DEFAULT_ENV is set",
        "container" => "The container indicator, if inside a container.",
        "crystal" => "The currently installed version of Crystal",
        "daml" => "The Daml SDK version of your project",
        "dart" => "The currently installed version of Dart",
        "deno" => "The currently installed version of Deno",
        "directory" => "The current working directory",
        "docker_context" => "The current docker context",
        "dotnet" => "The relevant version of the .NET Core SDK for the current directory",
        "elixir" => "The currently installed versions of Elixir and OTP",
        "elm" => "The currently installed version of Elm",
        "erlang" => "Current OTP version",
        "fennel" => "The currently installed version of Fennel",
        "fill" => "Fills the remaining space on the line with a pad string",
        "fossil_branch" => "The active branch of the check-out in your current directory",
        "fossil_metrics" => "The currently added/deleted lines in your check-out",
        "gcloud" => "The current GCP client configuration",
        "git_branch" => "The active branch of the repo in your current directory",
        "git_commit" => "The active commit (and tag if any) of the repo in your current directory",
        "git_metrics" => "The currently added/deleted lines in your repo",
        "git_state" => "The current git operation, and it's progress",
        "git_status" => "Symbol representing the state of the repo",
        "golang" => "The currently installed version of Golang",
        "gradle" => "The currently installed version of Gradle",
        "guix_shell" => "The guix-shell environment",
        "haskell" => "The selected version of the Haskell toolchain",
        "haxe" => "The currently installed version of Haxe",
        "helm" => "The currently installed version of Helm",
        "hg_branch" => "The active branch and topic of the repo in your current directory",
        "hostname" => "The system hostname",
        "java" => "The currently installed version of Java",
        "jobs" => "The current number of jobs running",
        "julia" => "The currently installed version of Julia",
        "kotlin" => "The currently installed version of Kotlin",
        "kubernetes" => "The current Kubernetes context name and, if set, the namespace",
        "line_break" => "Separates the prompt into two lines",
        "localip" => "The currently assigned ipv4 address",
        "lua" => "The currently installed version of Lua",
        "memory_usage" => "Current system memory and swap usage",
        "meson" => {
            "The current Meson environment, if $MESON_DEVENV and $MESON_PROJECT_NAME are set"
        }
        "nim" => "The currently installed version of Nim",
        "nix_shell" => "The nix-shell environment",
        "nodejs" => "The currently installed version of NodeJS",
        "ocaml" => "The currently installed version of OCaml",
        "opa" => "The currently installed version of Open Platform Agent",
        "openstack" => "The current OpenStack cloud and project",
        "os" => "The current operating system",
        "package" => "The package version of the current directory's project",
        "perl" => "The currently installed version of Perl",
        "php" => "The currently installed version of PHP",
        "pijul_channel" => "The current channel of the repo in the current directory",
        "pulumi" => "The current username, stack, and installed version of Pulumi",
        "purescript" => "The currently installed version of PureScript",
        "python" => "The currently installed version of Python",
        "raku" => "The currently installed version of Raku",
        "red" => "The currently installed version of Red",
        "rlang" => "The currently installed version of R",
        "ruby" => "The currently installed version of Ruby",
        "rust" => "The currently installed version of Rust",
        "scala" => "The currently installed version of Scala",
        "shell" => "The currently used shell indicator",
        "shlvl" => "The current value of SHLVL",
        "singularity" => "The currently used Singularity image",
        "solidity" => "The current installed version of Solidity",
        "spack" => "The current spack environment, if $SPACK_ENV is set",
        "status" => "The status of the last command",
        "sudo" => "The sudo credentials are currently cached",
        "swift" => "The currently installed version of Swift",
        "terraform" => "The currently selected terraform workspace and version",
        "time" => "The current local time",
        "typst" => "The current installed version of typst",
        "username" => "The active user's username",
        "vagrant" => "The currently installed version of Vagrant",
        "vcsh" => "The currently active VCSH repository",
        "vlang" => "The currently installed version of V",
        "zig" => "The currently installed version of Zig",
        _ => "<no description>",
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::module::ALL_MODULES;

    #[test]
    fn all_modules_have_description() {
        for module in ALL_MODULES {
            println!("Checking if {module:?} has a description");
            assert_ne!(description(module), "<no description>");
        }
    }
}
