use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct StarshipRootConfig {
    #[serde(rename = "$schema")]
    schema: String,
    pub format: String,
    pub right_format: String,
    pub continuation_prompt: String,
    pub scan_timeout: u64,
    pub command_timeout: u64,
    pub add_newline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palette: Option<String>,
    pub palettes: HashMap<String, Palette>,
}

pub type Palette = HashMap<String, String>;

// List of default prompt order
// NOTE: If this const value is changed then Default prompt order subheading inside
// prompt heading of config docs needs to be updated according to changes made here.
pub const PROMPT_ORDER: &[&str] = &[
    "username",
    "hostname",
    "localip",
    "shlvl",
    "singularity",
    "kubernetes",
    "directory",
    "vcsh",
    "git_branch",
    "git_commit",
    "git_state",
    "git_metrics",
    "git_status",
    "hg_branch",
    "docker_context",
    "package",
    // ↓ Toolchain version modules ↓
    // (Let's keep these sorted alphabetically)
    "bun",
    "c",
    "cmake",
    "cobol",
    "daml",
    "dart",
    "deno",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "golang",
    "haskell",
    "helm",
    "java",
    "julia",
    "kotlin",
    "lua",
    "nim",
    "nodejs",
    "ocaml",
    "perl",
    "php",
    "pulumi",
    "purescript",
    "python",
    "raku",
    "rlang",
    "red",
    "ruby",
    "rust",
    "scala",
    "swift",
    "terraform",
    "vlang",
    "vagrant",
    "zig",
    // ↑ Toolchain version modules ↑
    "buf",
    "nix_shell",
    "conda",
    "meson",
    "spack",
    "memory_usage",
    "aws",
    "gcloud",
    "openstack",
    "azure",
    "env_var",
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
    "shell",
    "character",
];

// On changes please also update `Default` for the `FullConfig` struct in `mod.rs`
impl Default for StarshipRootConfig {
    fn default() -> Self {
        Self {
            schema: "https://starship.rs/config-schema.json".to_string(),
            format: "$all".to_string(),
            right_format: "".to_string(),
            continuation_prompt: "[∙](bright-black) ".to_string(),
            scan_timeout: 30,
            command_timeout: 500,
            add_newline: true,
            palette: None,
            palettes: HashMap::default(),
        }
    }
}
