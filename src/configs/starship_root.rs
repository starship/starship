use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StarshipRootConfig<'a> {
    pub add_newline: bool,
    pub prompt_order: Vec<&'a str>,
    pub scan_timeout: u64,
    pub split_prompt: bool,
}

impl<'a> RootModuleConfig<'a> for StarshipRootConfig<'a> {
    fn new() -> Self {
        StarshipRootConfig {
            add_newline: true,
            // List of default prompt order
            // NOTE: If this const value is changed then Default prompt order subheading inside
            // prompt heading of config docs needs to be updated according to changes made here.
            prompt_order: vec![
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
                "package",
                // ↓ Toolchain version modules ↓
                // (Let's keep these sorted alphabetically)
                "dotnet",
                "elixir",
                "elm",
                "golang",
                "haskell",
                "java",
                "nodejs",
                "php",
                "python",
                "ruby",
                "rust",
                "terraform",
                // ↑ Toolchain version modules ↑
                "nix_shell",
                "conda",
                "memory_usage",
                "aws",
                "env_var",
                "cmd_duration",
                "line_break",
                "jobs",
                #[cfg(feature = "battery")]
                "battery",
                "time",
                "character",
            ],
            scan_timeout: 30,
            split_prompt: false, // TODO: PLEASE CHANGE BEFORE UPLOADING
        }
    }
}
