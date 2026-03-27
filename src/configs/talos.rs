use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct TalosConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub detect_env_vars: Vec<&'a str>,
    pub contexts: Vec<TalosContextConfig<'a>>,
}

pub const DEFAULT_FORMAT_STRING: &str = "[$symbol$context]($style bold)(\\([$cluster]($style)\\)) via [$endpoints]($style) (as [$roles]($style) )in ";

impl Default for TalosConfig<'_> {
    fn default() -> Self {
        Self {
            symbol: "󰰥 ",
            format: DEFAULT_FORMAT_STRING,
            style: "208",
            disabled: true,
            detect_extensions: vec![],
            detect_files: vec![],
            detect_folders: vec![],
            detect_env_vars: vec![],
            contexts: vec![],
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Default)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct TalosContextConfig<'a> {
    pub context_pattern: &'a str,
    pub context_alias: Option<&'a str>,
    pub symbol: Option<&'a str>,
    pub style: Option<&'a str>,
}
