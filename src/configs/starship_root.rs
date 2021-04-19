use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StarshipRootConfig<'a> {
    pub format: &'a str,
    pub scan_timeout: u64,
    pub add_newline: bool,
}

// List of default prompt order
// NOTE: If this const value is changed then Default prompt order subheading inside
// prompt heading of config docs needs to be updated according to changes made here.
pub const PROMPT_ORDER: &[&str] = &[
    "username",
    "hostname",
    "shlvl",
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
    "dart",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "golang",
    "helm",
    "java",
    "julia",
    "lua",
    "nim",
    "nodejs",
    "ocaml",
    "perl",
    "php",
    "purescript",
    "python",
    "r",
    "ruby",
    "rust",
    "swift",
    "terraform",
    "zig",
    // ↑ Toolchain version modules ↑
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "gcloud",
    "openstack",
    "env_var",
    "crystal",
    "cmd_duration",
    "custom",
    "line_break",
    "jobs",
    #[cfg(feature = "battery")]
    "battery",
    "time",
    "status",
    "character",
];

impl<'a> RootModuleConfig<'a> for StarshipRootConfig<'a> {
    fn new() -> Self {
        StarshipRootConfig {
            format: "$all",
            scan_timeout: 30,
            add_newline: true,
        }
    }
}
