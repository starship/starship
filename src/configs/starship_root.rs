use crate::{config::ModuleConfig, module::ALL_MODULES};

use serde::Serialize;
use std::cmp::Ordering;

// On changes please also update the `FullConfig` struct in `mod.rs`
#[derive(Clone, Serialize)]
pub struct StarshipRootConfig<'a> {
    pub format: &'a str,
    pub scan_timeout: u64,
    pub command_timeout: u64,
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
    "vcsh",
    "git_branch",
    "git_commit",
    "git_state",
    "git_stats",
    "git_status",
    "hg_branch",
    "docker_context",
    "package",
    // ↓ Toolchain version modules ↓
    // (Let's keep these sorted alphabetically)
    "cmake",
    "dart",
    "deno",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "golang",
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
    "purescript",
    "python",
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
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "gcloud",
    "openstack",
    "env_var",
    "crystal",
    "custom",
    "cmd_duration",
    "line_break",
    "jobs",
    #[cfg(feature = "battery")]
    "battery",
    "time",
    "status",
    "shell",
    "character",
];

// On changes please also update `Default` for the `FullConfig` struct in `mod.rs`
impl<'a> Default for StarshipRootConfig<'a> {
    fn default() -> Self {
        StarshipRootConfig {
            format: "$all",
            scan_timeout: 30,
            command_timeout: 500,
            add_newline: true,
        }
    }
}

impl<'a> ModuleConfig<'a> for StarshipRootConfig<'a> {
    fn load_config(&mut self, config: &'a toml::Value) {
        if let toml::Value::Table(config) = config {
            config.iter().for_each(|(k, v)| match k.as_str() {
                "format" => self.format.load_config(v),
                "scan_timeout" => self.scan_timeout.load_config(v),
                "command_timeout" => self.command_timeout.load_config(v),
                "add_newline" => self.add_newline.load_config(v),
                unknown => {
                    if !ALL_MODULES.contains(&unknown) && unknown != "custom" {
                        log::warn!("Unknown config key '{}'", unknown);

                        let did_you_mean = &[
                            // Root options
                            "format",
                            "scan_timeout",
                            "command_timeout",
                            "add_newline",
                            // Modules
                            "custom",
                        ]
                        .iter()
                        .chain(ALL_MODULES.iter())
                        .filter_map(|field| {
                            let score = strsim::jaro_winkler(unknown, field);
                            (score > 0.8).then(|| (score, field))
                        })
                        .max_by(
                            |(score_a, _field_a), (score_b, _field_b)| {
                                score_a.partial_cmp(score_b).unwrap_or(Ordering::Equal)
                            },
                        );

                        if let Some((_score, field)) = did_you_mean {
                            log::warn!("Did you mean '{}'?", field);
                        }
                    }
                }
            });
        }
    }

    fn from_config(config: &'a toml::Value) -> Option<Self> {
        let mut out = Self::default();
        out.load_config(config);
        Some(out)
    }
}
