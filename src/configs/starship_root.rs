use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StarshipRootConfig<'a> {
    pub format: &'a str,
    pub scan_timeout: u64,
}

// List of default prompt order
// NOTE: If this const value is changed then Default prompt order subheading inside
// prompt heading of config docs needs to be updated according to changes made here.
pub const PROMPT_ORDER: &[&str] = &[
    "username",
    "hostname",
    "singularity",
    "kubernetes",
    "directory",
    "git_branch",
    "git_commit",
    "git_state",
    "git_status",
    "hg_branch",
    "docker_context",
    "package",
    // ↓ Toolchain version modules ↓
    // (Let's keep these sorted alphabetically)
    "cmake",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "golang",
    "helm",
    "java",
    "julia",
    "nim",
    "nodejs",
    "ocaml",
    "php",
    "purescript",
    "python",
    "ruby",
    "rust",
    "terraform",
    "zig",
    // ↑ Toolchain version modules ↑
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "env_var",
    "crystal",
    "cmd_duration",
    "custom",
    "line_break",
    "jobs",
    #[cfg(feature = "battery")]
    "battery",
    "time",
    "character",
];

impl<'a> RootModuleConfig<'a> for StarshipRootConfig<'a> {
    fn new() -> Self {
        StarshipRootConfig {
            format: "\n$all",
            scan_timeout: 30,
        }
    }
}
