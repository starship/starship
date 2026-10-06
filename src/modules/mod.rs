mod aws;
mod azure;
mod buf;
mod bun;
mod c;
mod cc;
mod character;
mod claude_context;
mod claude_cost;
mod claude_model;
mod cmake;
mod cmd_duration;
mod cobol;
mod conda;
mod container;
mod cpp;
mod crystal;
pub mod custom;
mod daml;
mod dart;
mod deno;
mod directory;
mod direnv;
mod docker_context;
mod dotnet;
mod elixir;
mod elm;
mod env_var;
mod erlang;
mod fennel;
mod fill;
mod fortran;
mod fossil_branch;
mod fossil_metrics;
mod gcloud;
mod git_branch;
mod git_commit;
mod git_metrics;
mod git_state;
pub mod git_status;
mod gleam;
mod golang;
mod gradle;
mod guix_shell;
mod haskell;
mod haxe;
mod helm;
mod hg_branch;
mod hg_state;
mod hostname;
mod java;
mod jj_bookmark;
mod jj_change;
mod jj_metrics;
mod jj_status;
mod jobs;
mod julia;
mod kotlin;
mod kubernetes;
mod line_break;
mod localip;
mod lua;
mod maven;
mod memory_usage;
mod meson;
mod mise;
mod mojo;
mod nats;
mod netns;
mod nim;
mod nix_shell;
mod nodejs;
mod ocaml;
mod odin;
mod opa;
mod openstack;
mod os;
mod package;
mod perl;
mod php;
mod pijul_channel;
mod pixi;
mod pulumi;
mod purescript;
mod python;
mod quarto;
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
mod vcs;
mod vcsh;
mod vlang;
mod xmake;
mod zig;

#[cfg(feature = "battery")]
mod battery;
mod typst;

#[cfg(all(test, feature = "battery"))]
pub use self::battery::BatteryInfo;
#[cfg(feature = "battery")]
pub use self::battery::{BatteryInfoProvider, BatteryInfoProviderImpl};

use crate::config::ModuleConfig;
#[cfg(feature = "battery")]
use crate::configs::battery::BatteryConfig;
use crate::configs::localip::LocalipConfig;
use crate::configs::memory_usage::MemoryConfig;
use crate::context::{Context, Detected, Shell};
use crate::module::Module;
use std::time::{Duration, Instant};

/// Declares each built-in module once: the name a format string uses, which is
/// also the name of the module that renders it, and its description.
macro_rules! builtin_modules {
    ($($(#[$attribute:meta])* $name:ident: $description:literal,)*) => {
        /// Every built-in module, in alphabetical order. Default ordering is
        /// handled in configs/starship_root.rs.
        pub const ALL_MODULES: &[&str] = &[$($(#[$attribute])* stringify!($name),)*];

        fn builtin(name: &str) -> Option<Builtin> {
            match name {
                $($(#[$attribute])* stringify!($name) => Some(Builtin {
                    render: $name::module,
                    description: $description,
                }),)*
                _ => None,
            }
        }
    };
}

struct Builtin {
    render: for<'a> fn(&'a Context) -> Option<Module<'a>>,
    description: &'static str,
}

// Keep these ordered alphabetically.
builtin_modules! {
    aws: "The current AWS region and profile",
    azure: "The current Azure subscription",
    #[cfg(feature = "battery")]
    battery: "The current charge of the device's battery and its current charging status",
    buf: "The currently installed version of the Buf CLI",
    bun: "The currently installed version of the Bun",
    c: "Your C compiler type",
    character: "A character (usually an arrow) beside where the text is entered in your terminal",
    claude_context: "Context window usage for Claude Code session",
    claude_cost: "Cost info for Claude Code session",
    claude_model: "AI model name for Claude Code session",
    cmake: "The currently installed version of CMake",
    cmd_duration: "How long the last command took to execute",
    cobol: "The currently installed version of COBOL/GNUCOBOL",
    conda: "The current conda environment, if $CONDA_DEFAULT_ENV is set",
    container: "The container indicator, if inside a container.",
    cpp: "your cpp compiler type",
    crystal: "The currently installed version of Crystal",
    daml: "The Daml SDK version of your project",
    dart: "The currently installed version of Dart",
    deno: "The currently installed version of Deno",
    directory: "The current working directory",
    direnv: "The currently applied direnv file",
    docker_context: "The current docker context",
    dotnet: "The relevant version of the .NET Core SDK for the current directory",
    elixir: "The currently installed versions of Elixir and OTP",
    elm: "The currently installed version of Elm",
    erlang: "Current OTP version",
    fennel: "The currently installed version of Fennel",
    fill: "Fills the remaining space on the line with a pad string",
    fortran: "The currently used version of Fortran",
    fossil_branch: "The active branch of the check-out in your current directory",
    fossil_metrics: "The currently added/deleted lines in your check-out",
    gcloud: "The current GCP client configuration",
    git_branch: "The active branch of the current Git repo",
    git_commit: "The active commit (and tag if any) of the current Git repo",
    git_metrics: "The currently added/deleted lines in your Git repo",
    git_state: "The current Git operation, and its progress",
    git_status: "Symbols representing the state of the current Git repo, filtered to your current directory",
    gleam: "The currently installed version of Gleam",
    golang: "The currently installed version of Golang",
    gradle: "The currently installed version of Gradle",
    guix_shell: "The guix-shell environment",
    haskell: "The selected version of the Haskell toolchain",
    haxe: "The currently installed version of Haxe",
    helm: "The currently installed version of Helm",
    hg_branch: "The active branch and topic of the repo in your current directory",
    hg_state: "The current hg operation",
    hostname: "The system hostname",
    java: "The currently installed version of Java",
    jj_bookmark: "The closest ancestor bookmark in Jujutsu",
    jj_change: "The current change in Jujutsu",
    jj_metrics: "The number of added and deleted lines in Jujutsu",
    jj_status: "Current status in Jujutsu represented via symbols",
    jobs: "The current number of jobs running",
    julia: "The currently installed version of Julia",
    kotlin: "The currently installed version of Kotlin",
    kubernetes: "The current Kubernetes context name and, if set, the namespace",
    line_break: "Separates the prompt into two lines",
    localip: "The currently assigned ipv4 address",
    lua: "The currently installed version of Lua",
    maven: "The Maven Wrapper version of the current project",
    memory_usage: "Current system memory and swap usage",
    meson: "The current Meson environment, if $MESON_DEVENV and $MESON_PROJECT_NAME are set",
    mise: "The current mise status",
    mojo: "The currently installed version of Mojo",
    nats: "The current NATS context",
    netns: "The current network namespace",
    nim: "The currently installed version of Nim",
    nix_shell: "The nix-shell environment",
    nodejs: "The currently installed version of NodeJS",
    ocaml: "The currently installed version of OCaml",
    odin: "The currently installed version of Odin",
    opa: "The currently installed version of Open Platform Agent",
    openstack: "The current OpenStack cloud and project",
    os: "The current operating system",
    package: "The package version of the current directory's project",
    perl: "The currently installed version of Perl",
    php: "The currently installed version of PHP",
    pijul_channel: "The current channel of the repo in the current directory",
    pixi: "The currently installed version of Pixi, and the active environment if $PIXI_ENVIRONMENT_NAME is set",
    pulumi: "The current username, stack, and installed version of Pulumi",
    purescript: "The currently installed version of PureScript",
    python: "The currently installed version of Python",
    quarto: "The current installed version of quarto",
    raku: "The currently installed version of Raku",
    red: "The currently installed version of Red",
    rlang: "The currently installed version of R",
    ruby: "The currently installed version of Ruby",
    rust: "The currently installed version of Rust",
    scala: "The currently installed version of Scala",
    shell: "The currently used shell indicator",
    shlvl: "The current value of SHLVL",
    singularity: "The currently used Singularity image",
    solidity: "The current installed version of Solidity",
    spack: "The current spack environment, if $SPACK_ENV is set",
    status: "The status of the last command",
    sudo: "The sudo credentials are currently cached",
    swift: "The currently installed version of Swift",
    terraform: "The currently selected terraform workspace and version",
    time: "The current local time",
    typst: "The current installed version of typst",
    username: "The active user's username",
    vagrant: "The currently installed version of Vagrant",
    vcs: "The currently active VCS repository (first one matching)",
    vcsh: "The currently active VCSH repository",
    vlang: "The currently installed version of V",
    xmake: "The currently installed version of XMake",
    zig: "The currently installed version of Zig",
}

pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    let start: Instant = Instant::now();
    let mut m: Option<Module> = match module {
        "env_var" => env_var::module(None, context),
        env if env.starts_with("env_var.") => {
            env_var::module(env.strip_prefix("env_var."), context)
        }
        custom if custom.starts_with("custom.") => {
            // SAFETY: We just checked that the module starts with "custom."
            custom::module(custom.strip_prefix("custom.").unwrap(), context)
        }
        _ => match builtin(module) {
            Some(builtin) => (builtin.render)(context),
            None => {
                eprintln!(
                    "Error: Unknown module {module}. Use starship module --list to list out all supported modules."
                );
                None
            }
        },
    };

    let elapsed = start.elapsed();
    log::trace!("Took {elapsed:?} to compute module {module:?}");
    if elapsed.as_millis() >= 1 {
        // If we take less than 1ms to compute a None, then we will not return a module at all
        // if we have a module: default duration is 0 so no need to change it
        // if we took more than 1ms we want to report that and so--in case we have None currently--
        // need to create an empty module just to hold the duration for that case
        m.get_or_insert_with(|| context.new_module(module)).duration = elapsed;
    }
    m
}

/// How often `module` renders again while a prompt is shown, if it is enabled
/// and its `refresh` option says to: a clock as often as it turns over, a
/// battery, memory and an address every so often, and a custom module as often
/// as it is configured to.
pub fn period(module: &str, context: &Context) -> Option<Duration> {
    let configuration = || context.config.get_module_config(module);
    let every = |seconds, refresh: bool, disabled: bool| {
        (refresh && !disabled).then(|| Duration::from_secs(seconds))
    };
    match module {
        "time" => time::period(context),
        #[cfg(feature = "battery")]
        "battery" => {
            let battery = BatteryConfig::try_load(configuration());
            every(30, battery.refresh, battery.disabled)
        }
        "memory_usage" => {
            let memory = MemoryConfig::try_load(configuration());
            every(5, memory.refresh, memory.disabled)
        }
        "localip" => {
            let localip = LocalipConfig::try_load(configuration());
            every(30, localip.refresh, localip.disabled)
        }
        custom => custom::period(custom.strip_prefix("custom.")?, context),
    }
}

pub fn description(module: &str) -> &'static str {
    builtin(module).map_or("<no description>", |builtin| builtin.description)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn only_enabled_modules_that_go_stale_render_again() {
        let context = crate::test::default_context().set_config(toml::toml! {
            memory_usage.disabled = false
            localip.disabled = true
            [custom.weather]
            command = "forecast"
            refresh = 600_000
            [custom.once]
            command = "true"
        });

        assert_eq!(
            Some(Duration::from_secs(5)),
            period("memory_usage", &context)
        );
        assert_eq!(None, period("localip", &context));
        assert_eq!(None, period("time", &context), "disabled unless configured");
        assert_eq!(None, period("git_status", &context));
        assert_eq!(
            Some(Duration::from_secs(600)),
            period("custom.weather", &context)
        );
        assert_eq!(None, period("custom.once", &context));
        assert_eq!(None, period("custom.missing", &context));
    }

    #[test]
    fn a_module_set_not_to_refresh_renders_once() {
        let context = crate::test::default_context().set_config(toml::toml! {
            memory_usage.disabled = false
            memory_usage.refresh = false
            time.disabled = false
            time.refresh = false
            [custom.weather]
            command = "forecast"
            refresh = 600_000
            disabled = true
        });

        for module in ["memory_usage", "time", "custom.weather"] {
            assert_eq!(None, period(module, &context), "{module}");
        }
    }

    #[test]
    fn all_modules_have_description() {
        for module in ALL_MODULES {
            println!("Checking if {module:?} has a description");
            assert_ne!(description(module), "<no description>");
        }
    }
}
