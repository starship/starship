pub mod rust;

use crate::module_config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StarshipRootConfig<'a> {
    pub add_newline: bool,
    pub prompt_order: Vec<&'a str>,
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
                "directory",
                "git_branch",
                "git_state",
                "git_status",
                "package",
                "nodejs",
                "ruby",
                "rust",
                "python",
                "golang",
                "java",
                "nix_shell",
                "cmd_duration",
                "line_break",
                "jobs",
                #[cfg(feature = "battery")]
                "battery",
                "time",
                "character",
            ],
        }
    }
}
