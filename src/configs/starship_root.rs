use crate::config::{ModuleConfig, RootModuleConfig};
use crate::messages::LogLevel;
use toml::Value;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StarshipRootConfig<'a> {
    pub format: &'a str,
    pub scan_timeout: u64,
    pub messages: MessagesConfig<'a>,
}

pub const PROMPT_ORDER: [&str; 37] = [
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
    "dotnet",
    "elixir",
    "elm",
    "golang",
    "haskell",
    "java",
    "julia",
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
    "custom",
    "\n",
    "jobs",
    #[cfg(feature = "battery")]
    "battery",
    "time",
    "character",
];

impl<'a> RootModuleConfig<'a> for StarshipRootConfig<'a> {
    fn new() -> Self {
        StarshipRootConfig {
            // List of default prompt order
            // NOTE: If this const value is changed then Default prompt order subheading inside
            // prompt heading of config docs needs to be updated according to changes made here.
            format: "\n$all",
            scan_timeout: 30,
            messages: MessagesConfig::new(),
        }
    }
}

#[derive(Clone, ModuleConfig)]
pub struct MessagesConfig<'a> {
    pub format: &'a str,
    pub level: LogLevel,
}

impl<'a> MessagesConfig<'a> {
    fn new() -> Self {
        MessagesConfig {
            format: "\\[$level\\]: $message\n",
            level: LogLevel::Warning,
        }
    }
}

impl<'a> ModuleConfig<'a> for LogLevel {
    fn from_config(config: &'a Value) -> Option<Self> {
        let level_string = config.as_str()?.to_lowercase();
        match level_string.as_str() {
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "warning" => Some(LogLevel::Warning),
            "error" => Some(LogLevel::Error),
            _ => None,
        }
    }
}
