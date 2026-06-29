use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct MiseConfig<'a> {
    pub local_only: bool,
    pub health_check_enabled: bool,
    pub format: &'a str,
    pub symbol: &'a str,
    pub healthy_symbol: &'a str,
    pub unhealthy_symbol: &'a str,
    pub untrusted_symbol: &'a str,
    pub error_symbol: &'a str,
    pub style: &'a str,
    pub style_missing_some: &'a str,
    pub style_missing_all: &'a str,
    pub style_unhealthy: &'a str,
    pub style_untrusted: &'a str,
    pub style_error: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl Default for MiseConfig<'_> {
    fn default() -> Self {
        Self {
            local_only: true,
            health_check_enabled: false,
            format: "on [$symbol($status )($installed/$required )]($style)",
            symbol: "mise ",
            healthy_symbol: "healthy",
            unhealthy_symbol: "unhealthy",
            untrusted_symbol: "untrusted",
            error_symbol: "error",
            style: "bold purple",
            style_missing_some: "bold yellow",
            style_missing_all: "bold red",
            style_unhealthy: "bold red",
            style_untrusted: "bold red",
            style_error: "bold red",
            disabled: true,
            detect_extensions: vec![],
            detect_files: vec![
                "mise.toml",
                "mise.local.toml",
                ".mise.toml",
                ".mise.local.toml",
            ],
            detect_folders: vec![".mise"],
        }
    }
}
