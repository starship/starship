use crate::config::Either;
use indexmap::IndexMap;
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
    pub follow_symlinks: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palette: Option<String>,
    pub palettes: HashMap<String, Palette>,
    pub profiles: IndexMap<String, Either<String, Profile>>,
}

/// Profiles are either a string containing the format, or a struct containing the format and right_format
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct Profile {
    pub format: String,
    pub right_format: String,
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
    "pijul_channel",
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
    "fennel",
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
    "zig",
    // ↑ Toolchain version modules ↑
    "buf",
    "guix_shell",
    "nix_shell",
    "conda",
    "meson",
    "spack",
    "memory_usage",
    "aws",
    "gcloud",
    "openstack",
    "azure",
    "direnv",
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
    "os",
    "shell",
    "character",
];

impl Default for StarshipRootConfig {
    fn default() -> Self {
        let mut profiles = IndexMap::new();
        profiles.insert(
            "transient".to_string(),
            Either::Second(Profile {
                format: "$character".to_string(),
                right_format: String::new(),
            }),
        );
        Self {
            schema: "https://starship.rs/config-schema.json".to_string(),
            format: "$all".to_string(),
            right_format: String::new(),
            continuation_prompt: "[∙](bright-black) ".to_string(),
            profiles,
            scan_timeout: 30,
            command_timeout: 500,
            add_newline: true,
            follow_symlinks: true,
            palette: None,
            palettes: HashMap::default(),
        }
    }
}
