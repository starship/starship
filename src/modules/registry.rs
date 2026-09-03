//! Module registry.

use std::sync::LazyLock;

use crate::context::Context;
use crate::module::Module;

/// Declares [`BuiltinModule`] and everything that can be generated from
/// enumerating it alone: its name in each direction, and the array of every
/// variant. See the module-level doc comment for why this is a macro.
macro_rules! builtin_modules {
    ( $( $(#[$meta:meta])* $variant:ident = $name:literal ),+ $(,)? ) => {
        /// The closed set of modules starship ships with, dispatched by exact
        /// name — as opposed to `custom.*` and `env_var.*`, which are open-ended
        /// families dispatched by prefix. See [`ModuleId`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum BuiltinModule {
            $( $(#[$meta])* $variant, )+
        }

        impl BuiltinModule {
            /// Every variant, exactly once, in the same order declared below.
            ///
            /// This is the one place a new module has to be added by hand with
            /// nothing enforcing completeness at compile time — Rust has no way to
            /// enumerate an enum's own variants. Everything downstream of this
            /// array *is* enforced: [`entry_for`] and [`dispatch_builtin`] are
            /// exhaustive matches over `BuiltinModule`, so a variant added to the
            /// enum without a corresponding row in either fails to compile.
            pub const ALL: &'static [BuiltinModule] = &[
                $( $(#[$meta])* BuiltinModule::$variant, )+
            ];

            /// The name this module is configured and dispatched under.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $( $(#[$meta])* Self::$variant => $name, )+
                }
            }

            /// The builtin named `name`, or `None` — including when `name` names a
            /// member of one of the prefixed families, which are not part of this
            /// closed set at all.
            #[must_use]
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $( $(#[$meta])* $name => Some(Self::$variant), )+
                    _ => None,
                }
            }
        }
    };
}

builtin_modules! {
    Aws = "aws",
    Azure = "azure",
    #[cfg(feature = "battery")]
    Battery = "battery",
    Buf = "buf",
    Bun = "bun",
    C = "c",
    Character = "character",
    ClaudeContext = "claude_context",
    ClaudeCost = "claude_cost",
    ClaudeModel = "claude_model",
    Cmake = "cmake",
    CmdDuration = "cmd_duration",
    Cobol = "cobol",
    Conda = "conda",
    Container = "container",
    Cpp = "cpp",
    Crystal = "crystal",
    Daml = "daml",
    Dart = "dart",
    Deno = "deno",
    Directory = "directory",
    Direnv = "direnv",
    DockerContext = "docker_context",
    Dotnet = "dotnet",
    Elixir = "elixir",
    Elm = "elm",
    Erlang = "erlang",
    Fennel = "fennel",
    Fill = "fill",
    Fortran = "fortran",
    FossilBranch = "fossil_branch",
    FossilMetrics = "fossil_metrics",
    Gcloud = "gcloud",
    GitBranch = "git_branch",
    GitCommit = "git_commit",
    GitMetrics = "git_metrics",
    GitState = "git_state",
    GitStatus = "git_status",
    Gleam = "gleam",
    Golang = "golang",
    Gradle = "gradle",
    GuixShell = "guix_shell",
    Haskell = "haskell",
    Haxe = "haxe",
    Helm = "helm",
    HgBranch = "hg_branch",
    HgState = "hg_state",
    Hostname = "hostname",
    Java = "java",
    JjBookmark = "jj_bookmark",
    Jobs = "jobs",
    Julia = "julia",
    Kotlin = "kotlin",
    Kubernetes = "kubernetes",
    LineBreak = "line_break",
    Localip = "localip",
    Lua = "lua",
    Maven = "maven",
    MemoryUsage = "memory_usage",
    Meson = "meson",
    Mise = "mise",
    Mojo = "mojo",
    Nats = "nats",
    Netns = "netns",
    Nim = "nim",
    NixShell = "nix_shell",
    Nodejs = "nodejs",
    Ocaml = "ocaml",
    Odin = "odin",
    Opa = "opa",
    Openstack = "openstack",
    Os = "os",
    Package = "package",
    Perl = "perl",
    Php = "php",
    PijulChannel = "pijul_channel",
    Pixi = "pixi",
    Pulumi = "pulumi",
    Purescript = "purescript",
    Python = "python",
    Quarto = "quarto",
    Raku = "raku",
    Red = "red",
    Rlang = "rlang",
    Ruby = "ruby",
    Rust = "rust",
    Scala = "scala",
    Shell = "shell",
    Shlvl = "shlvl",
    Singularity = "singularity",
    Solidity = "solidity",
    Spack = "spack",
    Status = "status",
    Sudo = "sudo",
    Swift = "swift",
    Terraform = "terraform",
    Time = "time",
    Typst = "typst",
    Username = "username",
    Vagrant = "vagrant",
    Vcs = "vcs",
    Vcsh = "vcsh",
    Vlang = "vlang",
    Xmake = "xmake",
    Zig = "zig",
}

/// What the table says about a module, beyond what [`BuiltinModule`] already
/// carries (its name).
pub struct ModuleEntry {
    pub description: &'static str,
    /// This module's place in [`PROMPT_ORDER`], or `None` for a module that is
    /// not part of the default ordering — a profile-only module such as
    /// `claude_model`, or `fill` and `vcs`, which a format string must name
    /// explicitly to use.
    pub prompt_position: Option<u16>,
}

/// What the table says about `module`.
///
/// Exhaustive over [`BuiltinModule`]: adding a variant to the `builtin_modules!`
/// table above without describing it here fails to compile, rather than leaving
/// the new module without a cadence the way a missing `match` arm keyed by name
/// used to. Grouped by *why* each module is classified as it is, the same
/// grouping the hand-written `cadence` match used before this table replaced it.
#[must_use]
pub const fn entry_for(module: BuiltinModule) -> ModuleEntry {
    use BuiltinModule as M;
    match module {
        // Instant — a cheap system call, or reading shell-supplied or environment state. No file system traversal and no subprocess.
        M::Character => ModuleEntry {
            description: "A character (usually an arrow) beside where the text is entered in your terminal",
            prompt_position: Some(100),
        },
        M::ClaudeContext => ModuleEntry {
            description: "Context window usage for Claude Code session",
            prompt_position: None,
        },
        M::ClaudeCost => ModuleEntry {
            description: "Cost info for Claude Code session",
            prompt_position: None,
        },
        M::ClaudeModel => ModuleEntry {
            description: "AI model name for Claude Code session",
            prompt_position: None,
        },
        M::CmdDuration => ModuleEntry {
            description: "How long the last command took to execute",
            prompt_position: Some(90),
        },
        M::Conda => ModuleEntry {
            description: "The current conda environment, if $CONDA_DEFAULT_ENV is set",
            prompt_position: Some(75),
        },
        M::Fill => ModuleEntry {
            description: "Fills the remaining space on the line with a pad string",
            prompt_position: None,
        },
        M::GuixShell => ModuleEntry {
            description: "The guix-shell environment",
            prompt_position: Some(73),
        },
        M::Hostname => ModuleEntry {
            description: "The system hostname",
            prompt_position: Some(1),
        },
        M::Jobs => ModuleEntry {
            description: "The current number of jobs running",
            prompt_position: Some(92),
        },
        M::LineBreak => ModuleEntry {
            description: "Separates the prompt into two lines",
            prompt_position: Some(91),
        },
        M::Meson => ModuleEntry {
            description: "The current Meson environment, if $MESON_DEVENV and $MESON_PROJECT_NAME are set",
            prompt_position: Some(77),
        },
        M::NixShell => ModuleEntry {
            description: "The nix-shell environment",
            prompt_position: Some(74),
        },
        M::Shell => ModuleEntry {
            description: "The currently used shell indicator",
            prompt_position: Some(99),
        },
        M::Shlvl => ModuleEntry {
            description: "The current value of SHLVL",
            prompt_position: Some(3),
        },
        M::Singularity => ModuleEntry {
            description: "The currently used Singularity image",
            prompt_position: Some(4),
        },
        M::Spack => ModuleEntry {
            description: "The current spack environment, if $SPACK_ENV is set",
            prompt_position: Some(78),
        },
        M::Status => ModuleEntry {
            description: "The status of the last command",
            prompt_position: Some(95),
        },
        M::Username => ModuleEntry {
            description: "The active user's username",
            prompt_position: Some(0),
        },
        M::Vcsh => ModuleEntry {
            description: "The currently active VCSH repository",
            prompt_position: Some(8),
        },

        // Instant — a container marker under `/run`, a virtual filesystem backed by memory rather than a disk or a network mount: the `stat` or `open` against it cannot stall the way one against a real filesystem can.
        M::Container => ModuleEntry {
            description: "The container indicator, if inside a container.",
            prompt_position: Some(96),
        },

        // Dynamic — intrinsically time-varying values, re-polled on their own period.
        #[cfg(feature = "battery")]
        M::Battery => ModuleEntry {
            description: "The current charge of the device's battery and its current charging status",
            prompt_position: Some(93),
        },
        M::Localip => ModuleEntry {
            description: "The currently assigned ipv4 address",
            prompt_position: Some(2),
        },
        M::MemoryUsage => ModuleEntry {
            description: "Current system memory and swap usage",
            prompt_position: Some(79),
        },
        M::Time => ModuleEntry {
            description: "The current local time",
            prompt_position: Some(94),
        },

        // Deferred — read one or more credentials or profile files under the user's home directory: `~/.aws/{config,credentials}`, `~/.azure/azureProfile.json`, `~/.config/gcloud/...`, `~/.config/openstack/clouds.yaml`. Each path is known up front, but "known up front" does not mean bounded — the directory it lives under may be a slow or network mount, and these are full file reads (parsed as INI or JSON), not a single cheap `stat`.
        M::Aws => ModuleEntry {
            description: "The current AWS region and profile",
            prompt_position: Some(80),
        },
        M::Azure => ModuleEntry {
            description: "The current Azure subscription",
            prompt_position: Some(83),
        },
        M::Gcloud => ModuleEntry {
            description: "The current GCP client configuration",
            prompt_position: Some(81),
        },
        M::Openstack => ModuleEntry {
            description: "The current OpenStack cloud and project",
            prompt_position: Some(82),
        },

        // Deferred — discover a repository, either by opening it through `Context::get_repo` or by walking the ancestors of the working directory looking for a control directory.
        M::Directory => ModuleEntry {
            description: "The current working directory",
            prompt_position: Some(7),
        },
        M::GitCommit => ModuleEntry {
            description: "The active commit (and tag if any) of the current Git repo",
            prompt_position: Some(12),
        },
        M::GitState => ModuleEntry {
            description: "The current Git operation and its progress",
            prompt_position: Some(13),
        },
        M::HgBranch => ModuleEntry {
            description: "The active branch and topic of the repo in your current directory",
            prompt_position: Some(16),
        },
        M::HgState => ModuleEntry {
            description: "The current hg operation",
            prompt_position: Some(17),
        },
        M::Vcs => ModuleEntry {
            description: "The currently active VCS repository (first one matching)",
            prompt_position: None,
        },

        // Deferred — repository discovery followed by a `git` subprocess.
        M::GitBranch => ModuleEntry {
            description: "The active branch of the current Git repo",
            prompt_position: Some(11),
        },
        M::GitMetrics => ModuleEntry {
            description: "The currently added/deleted lines in your Git repo",
            prompt_position: Some(14),
        },
        M::GitStatus => ModuleEntry {
            description: "Symbols representing the state of the current Git repo, filtered to your current directory",
            prompt_position: Some(15),
        },

        // Deferred — scan the working directory for marker files. No subprocess, but the scan itself is unbounded: its cost is set by the directory it lands in, not by the module.
        M::Daml => ModuleEntry {
            description: "The Daml SDK version of your project",
            prompt_position: Some(26),
        },
        M::DockerContext => ModuleEntry {
            description: "The current docker context",
            prompt_position: Some(19),
        },
        M::Gradle => ModuleEntry {
            description: "The currently installed version of Gradle",
            prompt_position: Some(37),
        },
        M::Kubernetes => ModuleEntry {
            description: "The current Kubernetes context name and, if set, the namespace",
            prompt_position: Some(5),
        },
        M::Maven => ModuleEntry {
            description: "The Maven Wrapper version of the current project",
            prompt_position: Some(45),
        },

        // Deferred — `os_info` inspects the running system, which on several platforms means spawning a helper such as `sw_vers` or `lsb_release`.
        M::Os => ModuleEntry {
            description: "The current operating system",
            prompt_position: Some(98),
        },

        // Deferred — spawn a subprocess to ask a tool for its version or its state. Most of these gate the subprocess behind a directory scan first, which makes them cheap in an unrelated directory but does not make them bounded: the scan is itself unbounded input/output.
        M::Buf => ModuleEntry {
            description: "The currently installed version of the Buf CLI",
            prompt_position: Some(72),
        },
        M::Bun => ModuleEntry {
            description: "The currently installed version of the Bun",
            prompt_position: Some(21),
        },
        M::C => ModuleEntry {
            description: "Your C compiler type",
            prompt_position: Some(22),
        },
        M::Cmake => ModuleEntry {
            description: "The currently installed version of CMake",
            prompt_position: Some(23),
        },
        M::Cobol => ModuleEntry {
            description: "The currently installed version of COBOL/GNUCOBOL",
            prompt_position: Some(24),
        },
        M::Cpp => ModuleEntry {
            description: "Your C++ compiler type",
            prompt_position: Some(25),
        },
        M::Crystal => ModuleEntry {
            description: "The currently installed version of Crystal",
            prompt_position: Some(87),
        },
        M::Dart => ModuleEntry {
            description: "The currently installed version of Dart",
            prompt_position: Some(27),
        },
        M::Deno => ModuleEntry {
            description: "The currently installed version of Deno",
            prompt_position: Some(28),
        },
        M::Direnv => ModuleEntry {
            description: "The currently applied direnv file",
            prompt_position: Some(84),
        },
        M::Dotnet => ModuleEntry {
            description: "The relevant version of the .NET Core SDK for the current directory",
            prompt_position: Some(29),
        },
        M::Elixir => ModuleEntry {
            description: "The currently installed versions of Elixir and OTP",
            prompt_position: Some(30),
        },
        M::Elm => ModuleEntry {
            description: "The currently installed version of Elm",
            prompt_position: Some(31),
        },
        M::Erlang => ModuleEntry {
            description: "Current OTP version",
            prompt_position: Some(32),
        },
        M::Fennel => ModuleEntry {
            description: "The currently installed version of Fennel",
            prompt_position: Some(33),
        },
        M::Fortran => ModuleEntry {
            description: "The currently used version of Fortran",
            prompt_position: Some(34),
        },
        M::FossilBranch => ModuleEntry {
            description: "The active branch of the check-out in your current directory",
            prompt_position: Some(9),
        },
        M::FossilMetrics => ModuleEntry {
            description: "The currently added/deleted lines in your check-out",
            prompt_position: Some(10),
        },
        M::Gleam => ModuleEntry {
            description: "The currently installed version of Gleam",
            prompt_position: Some(35),
        },
        M::Golang => ModuleEntry {
            description: "The currently installed version of Golang",
            prompt_position: Some(36),
        },
        M::Haskell => ModuleEntry {
            description: "The selected version of the Haskell toolchain",
            prompt_position: Some(38),
        },
        M::Haxe => ModuleEntry {
            description: "The currently installed version of Haxe",
            prompt_position: Some(39),
        },
        M::Helm => ModuleEntry {
            description: "The currently installed version of Helm",
            prompt_position: Some(40),
        },
        M::Java => ModuleEntry {
            description: "The currently installed version of Java",
            prompt_position: Some(41),
        },
        M::JjBookmark => ModuleEntry {
            description: "The closest ancestor bookmark in Jujutsu",
            prompt_position: None,
        },
        M::Julia => ModuleEntry {
            description: "The currently installed version of Julia",
            prompt_position: Some(42),
        },
        M::Kotlin => ModuleEntry {
            description: "The currently installed version of Kotlin",
            prompt_position: Some(43),
        },
        M::Lua => ModuleEntry {
            description: "The currently installed version of Lua",
            prompt_position: Some(44),
        },
        M::Mise => ModuleEntry {
            description: "The current mise status",
            prompt_position: Some(86),
        },
        M::Mojo => ModuleEntry {
            description: "The currently installed version of Mojo",
            prompt_position: Some(46),
        },
        M::Nats => ModuleEntry {
            description: "The current NATS context",
            prompt_position: Some(6),
        },
        M::Netns => ModuleEntry {
            description: "The current network namespace",
            prompt_position: Some(97),
        },
        M::Nim => ModuleEntry {
            description: "The currently installed version of Nim",
            prompt_position: Some(47),
        },
        M::Nodejs => ModuleEntry {
            description: "The currently installed version of NodeJS",
            prompt_position: Some(48),
        },
        M::Ocaml => ModuleEntry {
            description: "The currently installed version of OCaml",
            prompt_position: Some(49),
        },
        M::Odin => ModuleEntry {
            description: "The currently installed version of Odin",
            prompt_position: Some(50),
        },
        M::Opa => ModuleEntry {
            description: "The currently installed version of Open Platform Agent",
            prompt_position: Some(51),
        },
        M::Package => ModuleEntry {
            description: "The package version of the current directory's project",
            prompt_position: Some(20),
        },
        M::Perl => ModuleEntry {
            description: "The currently installed version of Perl",
            prompt_position: Some(52),
        },
        M::Php => ModuleEntry {
            description: "The currently installed version of PHP",
            prompt_position: Some(53),
        },
        M::PijulChannel => ModuleEntry {
            description: "The current channel of the repo in the current directory",
            prompt_position: Some(18),
        },
        M::Pixi => ModuleEntry {
            description: "The currently installed version of Pixi, and the active environment if $PIXI_ENVIRONMENT_NAME is set",
            prompt_position: Some(76),
        },
        M::Pulumi => ModuleEntry {
            description: "The current username, stack, and installed version of Pulumi",
            prompt_position: Some(54),
        },
        M::Purescript => ModuleEntry {
            description: "The currently installed version of PureScript",
            prompt_position: Some(55),
        },
        M::Python => ModuleEntry {
            description: "The currently installed version of Python",
            prompt_position: Some(56),
        },
        M::Quarto => ModuleEntry {
            description: "The currently installed version of Quarto",
            prompt_position: Some(57),
        },
        M::Raku => ModuleEntry {
            description: "The currently installed version of Raku",
            prompt_position: Some(58),
        },
        M::Red => ModuleEntry {
            description: "The currently installed version of Red",
            prompt_position: Some(60),
        },
        M::Rlang => ModuleEntry {
            description: "The currently installed version of R",
            prompt_position: Some(59),
        },
        M::Ruby => ModuleEntry {
            description: "The currently installed version of Ruby",
            prompt_position: Some(61),
        },
        M::Rust => ModuleEntry {
            description: "The currently installed version of Rust",
            prompt_position: Some(62),
        },
        M::Scala => ModuleEntry {
            description: "The currently installed version of Scala",
            prompt_position: Some(63),
        },
        M::Solidity => ModuleEntry {
            description: "The current installed version of Solidity",
            prompt_position: Some(64),
        },
        M::Sudo => ModuleEntry {
            description: "The sudo credentials are currently cached",
            prompt_position: Some(89),
        },
        M::Swift => ModuleEntry {
            description: "The currently installed version of Swift",
            prompt_position: Some(65),
        },
        M::Terraform => ModuleEntry {
            description: "The currently selected terraform workspace and version",
            prompt_position: Some(66),
        },
        M::Typst => ModuleEntry {
            description: "The currently installed version of Typst",
            prompt_position: Some(67),
        },
        M::Vagrant => ModuleEntry {
            description: "The currently installed version of Vagrant",
            prompt_position: Some(69),
        },
        M::Vlang => ModuleEntry {
            description: "The currently installed version of V",
            prompt_position: Some(68),
        },
        M::Xmake => ModuleEntry {
            description: "The currently installed version of XMake",
            prompt_position: Some(70),
        },
        M::Zig => ModuleEntry {
            description: "The currently installed version of Zig",
            prompt_position: Some(71),
        },
    }
}

/// Renders a builtin module by calling its own `module` function.
///
/// Exhaustive over [`BuiltinModule`] for the same reason [`entry_for`] is: the
/// dispatch table [`crate::modules::handle`] and [`crate::modules::instant`]
/// delegate to, for anything in the closed set, cannot silently omit a variant.
pub fn dispatch_builtin<'a>(module: BuiltinModule, context: &'a Context) -> Option<Module<'a>> {
    use BuiltinModule as M;
    match module {
        M::Aws => super::aws::module(context),
        M::Azure => super::azure::module(context),
        #[cfg(feature = "battery")]
        M::Battery => super::battery::module(context),
        M::Buf => super::buf::module(context),
        M::Bun => super::bun::module(context),
        M::C => super::c::module(context),
        M::Character => super::character::module(context),
        M::ClaudeContext => super::claude_context::module(context),
        M::ClaudeCost => super::claude_cost::module(context),
        M::ClaudeModel => super::claude_model::module(context),
        M::Cmake => super::cmake::module(context),
        M::CmdDuration => super::cmd_duration::module(context),
        M::Cobol => super::cobol::module(context),
        M::Conda => super::conda::module(context),
        M::Container => super::container::module(context),
        M::Cpp => super::cpp::module(context),
        M::Crystal => super::crystal::module(context),
        M::Daml => super::daml::module(context),
        M::Dart => super::dart::module(context),
        M::Deno => super::deno::module(context),
        M::Directory => super::directory::module(context),
        M::Direnv => super::direnv::module(context),
        M::DockerContext => super::docker_context::module(context),
        M::Dotnet => super::dotnet::module(context),
        M::Elixir => super::elixir::module(context),
        M::Elm => super::elm::module(context),
        M::Erlang => super::erlang::module(context),
        M::Fennel => super::fennel::module(context),
        M::Fill => super::fill::module(context),
        M::Fortran => super::fortran::module(context),
        M::FossilBranch => super::fossil_branch::module(context),
        M::FossilMetrics => super::fossil_metrics::module(context),
        M::Gcloud => super::gcloud::module(context),
        M::GitBranch => super::git_branch::module(context),
        M::GitCommit => super::git_commit::module(context),
        M::GitMetrics => super::git_metrics::module(context),
        M::GitState => super::git_state::module(context),
        M::GitStatus => super::git_status::module(context),
        M::Gleam => super::gleam::module(context),
        M::Golang => super::golang::module(context),
        M::Gradle => super::gradle::module(context),
        M::GuixShell => super::guix_shell::module(context),
        M::Haskell => super::haskell::module(context),
        M::Haxe => super::haxe::module(context),
        M::Helm => super::helm::module(context),
        M::HgBranch => super::hg_branch::module(context),
        M::HgState => super::hg_state::module(context),
        M::Hostname => super::hostname::module(context),
        M::Java => super::java::module(context),
        M::JjBookmark => super::jj_bookmark::module(context),
        M::Jobs => super::jobs::module(context),
        M::Julia => super::julia::module(context),
        M::Kotlin => super::kotlin::module(context),
        M::Kubernetes => super::kubernetes::module(context),
        M::LineBreak => super::line_break::module(context),
        M::Localip => super::localip::module(context),
        M::Lua => super::lua::module(context),
        M::Maven => super::maven::module(context),
        M::MemoryUsage => super::memory_usage::module(context),
        M::Meson => super::meson::module(context),
        M::Mise => super::mise::module(context),
        M::Mojo => super::mojo::module(context),
        M::Nats => super::nats::module(context),
        M::Netns => super::netns::module(context),
        M::Nim => super::nim::module(context),
        M::NixShell => super::nix_shell::module(context),
        M::Nodejs => super::nodejs::module(context),
        M::Ocaml => super::ocaml::module(context),
        M::Odin => super::odin::module(context),
        M::Opa => super::opa::module(context),
        M::Openstack => super::openstack::module(context),
        M::Os => super::os::module(context),
        M::Package => super::package::module(context),
        M::Perl => super::perl::module(context),
        M::Php => super::php::module(context),
        M::PijulChannel => super::pijul_channel::module(context),
        M::Pixi => super::pixi::module(context),
        M::Pulumi => super::pulumi::module(context),
        M::Purescript => super::purescript::module(context),
        M::Python => super::python::module(context),
        M::Quarto => super::quarto::module(context),
        M::Raku => super::raku::module(context),
        M::Red => super::red::module(context),
        M::Rlang => super::rlang::module(context),
        M::Ruby => super::ruby::module(context),
        M::Rust => super::rust::module(context),
        M::Scala => super::scala::module(context),
        M::Shell => super::shell::module(context),
        M::Shlvl => super::shlvl::module(context),
        M::Singularity => super::singularity::module(context),
        M::Solidity => super::solidity::module(context),
        M::Spack => super::spack::module(context),
        M::Status => super::status::module(context),
        M::Sudo => super::sudo::module(context),
        M::Swift => super::swift::module(context),
        M::Terraform => super::terraform::module(context),
        M::Time => super::time::module(context),
        M::Typst => super::typst::module(context),
        M::Username => super::username::module(context),
        M::Vagrant => super::vagrant::module(context),
        M::Vcs => super::vcs::module(context),
        M::Vcsh => super::vcsh::module(context),
        M::Vlang => super::vlang::module(context),
        M::Xmake => super::xmake::module(context),
        M::Zig => super::zig::module(context),
    }
}

/// Identifies a module by what dispatches it.
///
/// `custom.<name>` and `env_var.<name>` are user-defined — a user may write
/// `custom.anything` in their own configuration — so no closed enum could ever
/// list them. Keeping them as a variant that carries the suffix, rather than
/// accepting any `&str` as a module name everywhere `cadence`, `description` and
/// dispatch are used, is what stops an arbitrary string from masquerading as a
/// builtin: the only way to produce a `ModuleId::Builtin` is
/// [`BuiltinModule::from_name`] recognising it exactly, and every other spelling —
/// including one that merely *starts with* a builtin's name — falls through to
/// one of the other two variants, or to `None` from [`ModuleId::parse`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleId<'a> {
    /// One of the closed set of modules starship ships with.
    Builtin(BuiltinModule),
    /// `env_var` (`None`) or `env_var.<name>` (`Some`) — the bare form names no
    /// particular variable, and is what an unqualified `$env_var` means.
    EnvVar(Option<&'a str>),
    /// `custom.<name>`. Bare `custom` is not a module — it never dispatched to
    /// anything before this type existed either — so it has no representation
    /// here at all.
    Custom(&'a str),
}

impl<'a> ModuleId<'a> {
    /// Classifies `name` the way [`crate::modules::handle`] would.
    #[must_use]
    pub fn parse(name: &'a str) -> Option<Self> {
        if let Some(builtin) = BuiltinModule::from_name(name) {
            return Some(Self::Builtin(builtin));
        }
        if name == "env_var" {
            return Some(Self::EnvVar(None));
        }
        if let Some(suffix) = name.strip_prefix("env_var.") {
            return Some(Self::EnvVar(Some(suffix)));
        }
        if let Some(suffix) = name.strip_prefix("custom.") {
            return Some(Self::Custom(suffix));
        }
        None
    }
}

/// `module`'s description, or a placeholder for one that has none — which is
/// every `custom.*` and `env_var.*` module, exactly as before this table existed.
#[must_use]
pub fn description(module: &str) -> &'static str {
    match ModuleId::parse(module) {
        Some(ModuleId::Builtin(builtin)) => entry_for(builtin).description,
        _ => "<no description>",
    }
}

/// Renders `module` by dispatching on what kind of [`ModuleId`] it parses as, or
/// `None` if `name` is not a module starship recognises at all.
#[must_use]
pub fn render<'a>(module: &str, context: &'a Context) -> Option<Option<Module<'a>>> {
    Some(match ModuleId::parse(module)? {
        ModuleId::Builtin(builtin) => dispatch_builtin(builtin, context),
        ModuleId::EnvVar(name) => super::env_var::module(name, context),
        ModuleId::Custom(name) => super::custom::module(name, context),
    })
}

const ALL_MODULE_NAME_COUNT: usize = BuiltinModule::ALL.len();

const fn all_module_names() -> [&'static str; ALL_MODULE_NAME_COUNT] {
    let mut names = [""; ALL_MODULE_NAME_COUNT];
    let mut i = 0;
    while i < ALL_MODULE_NAME_COUNT {
        names[i] = BuiltinModule::ALL[i].name();
        i += 1;
    }
    names
}

const ALL_MODULE_NAMES: [&str; ALL_MODULE_NAME_COUNT] = all_module_names();

/// Every module starship ships with, alphabetically.
///
/// Alphabetical because the `builtin_modules!` invocation above is declared that
/// way — see `test_all_modules_is_in_alphabetical_order` in `crate::module`, which
/// this satisfies by construction rather than by a sort computed here.
pub const ALL_MODULES: &[&str] = &ALL_MODULE_NAMES;

/// Where `env_var.*` modules collectively sit in the default ordering.
const ENV_VAR_PROMPT_POSITION: u16 = 85;
/// Where `custom.*` modules collectively sit in the default ordering.
const CUSTOM_PROMPT_POSITION: u16 = 88;

/// The default order modules are drawn in, and what `$all` expands to.
///
/// Built once, from every [`BuiltinModule`] that has a
/// [`ModuleEntry::prompt_position`], plus the two prefixed-family placeholders,
/// sorted by that position. A `const` array would have to sort itself at compile
/// time for no real benefit — nothing here is on a hot path, and it is computed
/// once regardless — so this is a plain sort behind a [`LazyLock`].
pub static PROMPT_ORDER: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let mut positioned: Vec<(u16, &'static str)> = BuiltinModule::ALL
        .iter()
        .filter_map(|&module| {
            entry_for(module)
                .prompt_position
                .map(|position| (position, module.name()))
        })
        .collect();
    positioned.push((ENV_VAR_PROMPT_POSITION, "env_var"));
    positioned.push((CUSTOM_PROMPT_POSITION, "custom"));
    positioned.sort_by_key(|&(position, _)| position);
    positioned.into_iter().map(|(_, name)| name).collect()
});

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact ordering `PROMPT_ORDER` held before this table replaced its
    /// hand-maintained array, with `battery` included — this crate is built
    /// `--all-features` under test, so the cfg-gated entry is always present
    /// here.
    ///
    /// A golden list rather than a property (`is_sorted_by`, say) because the
    /// property that matters is the *specific* order a hand-maintained array
    /// used to encode by position alone: this is the one test that would have
    /// caught the position-scale bug an earlier draft of this table had, where
    /// builtin positions and the two placeholder positions were computed on
    /// two different numberings and silently reordered `env_var`, `custom` and
    /// their neighbours.
    const ORIGINAL_PROMPT_ORDER: &[&str] = &[
        "username",
        "hostname",
        "localip",
        "shlvl",
        "singularity",
        "kubernetes",
        "nats",
        "directory",
        "vcsh",
        "fossil_branch",
        "fossil_metrics",
        "git_branch",
        "git_commit",
        "git_state",
        "git_metrics",
        "git_status",
        "hg_branch",
        "hg_state",
        "pijul_channel",
        "docker_context",
        "package",
        "bun",
        "c",
        "cmake",
        "cobol",
        "cpp",
        "daml",
        "dart",
        "deno",
        "dotnet",
        "elixir",
        "elm",
        "erlang",
        "fennel",
        "fortran",
        "gleam",
        "golang",
        "gradle",
        "haskell",
        "haxe",
        "helm",
        "java",
        "julia",
        "kotlin",
        "lua",
        "maven",
        "mojo",
        "nim",
        "nodejs",
        "ocaml",
        "odin",
        "opa",
        "perl",
        "php",
        "pulumi",
        "purescript",
        "python",
        "quarto",
        "raku",
        "rlang",
        "red",
        "ruby",
        "rust",
        "scala",
        "solidity",
        "swift",
        "terraform",
        "typst",
        "vlang",
        "vagrant",
        "xmake",
        "zig",
        "buf",
        "guix_shell",
        "nix_shell",
        "conda",
        "pixi",
        "meson",
        "spack",
        "memory_usage",
        "aws",
        "gcloud",
        "openstack",
        "azure",
        "direnv",
        "env_var",
        "mise",
        "crystal",
        "custom",
        "sudo",
        "cmd_duration",
        "line_break",
        "jobs",
        #[cfg(feature = "battery")]
        "battery",
        "time",
        "status",
        "container",
        "netns",
        "os",
        "shell",
        "character",
    ];

    #[test]
    fn prompt_order_exactly_reproduces_the_ordering_it_replaced() {
        assert_eq!(ORIGINAL_PROMPT_ORDER, PROMPT_ORDER.as_slice());
    }

    #[test]
    fn all_modules_is_alphabetical_and_agrees_with_builtin_module_all() {
        let mut sorted: Vec<&str> = ALL_MODULES.to_vec();
        sorted.sort_unstable();
        assert_eq!(sorted, ALL_MODULES);
        assert_eq!(BuiltinModule::ALL.len(), ALL_MODULES.len());
    }

    /// Every [`BuiltinModule`] round-trips through its name — the property
    /// [`ModuleId::parse`] leans on to recognise a builtin at all.
    #[test]
    fn every_builtin_round_trips_through_its_name() {
        for &module in BuiltinModule::ALL {
            assert_eq!(
                Some(module),
                BuiltinModule::from_name(module.name()),
                "{module:?} does not round-trip through its own name {:?}",
                module.name()
            );
        }
    }

    #[test]
    fn module_id_parses_every_builtin() {
        for &module in BuiltinModule::ALL {
            assert_eq!(
                Some(ModuleId::Builtin(module)),
                ModuleId::parse(module.name())
            );
        }
    }

    #[test]
    fn module_id_parses_the_prefixed_families() {
        assert_eq!(Some(ModuleId::EnvVar(None)), ModuleId::parse("env_var"));
        assert_eq!(
            Some(ModuleId::EnvVar(Some("SHELL"))),
            ModuleId::parse("env_var.SHELL")
        );
        assert_eq!(
            Some(ModuleId::Custom("git_hash")),
            ModuleId::parse("custom.git_hash")
        );
    }

    /// The crux of `ModuleId`: nothing that merely *looks* like a builtin, a
    /// dotted family member, or a bare prefix parses as one.
    #[test]
    fn module_id_rejects_everything_else() {
        for unrecognised in [
            "not_a_module",
            // Bare `custom` never dispatched to anything before `ModuleId`
            // existed either; it must not gain a meaning now.
            "custom",
            // A string that merely starts with a builtin's name is not that
            // builtin — this is the "arbitrary string masquerading as a
            // builtin" case the type exists to rule out.
            "awsome",
            "aws2",
            "",
        ] {
            assert_eq!(
                None,
                ModuleId::parse(unrecognised),
                "{unrecognised:?} parsed as a module, but is not one"
            );
        }
    }

    #[test]
    fn description_agrees_with_entry_for() {
        for &module in BuiltinModule::ALL {
            let entry = entry_for(module);
            assert_eq!(entry.description, description(module.name()));
        }
    }
}
