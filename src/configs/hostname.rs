use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct UserOverrideConfig<'a> {
    pub format: Option<&'a str>,
    pub style: Option<&'a str>,
    pub disabled: bool,
}

impl<'a> Default for UserOverrideConfig<'a> {
    fn default() -> Self {
        UserOverrideConfig {
            format: None,
            style: None,
            disabled: false,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct HostnameConfig<'a> {
    pub ssh_only: bool,
    pub ssh_symbol: &'a str,
    pub trim_at: &'a str,
    pub detect_env_vars: Vec<&'a str>,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub user_overrides: HashMap<String, UserOverrideConfig<'a>>,
}

impl<'a> Default for HostnameConfig<'a> {
    fn default() -> Self {
        HostnameConfig {
            ssh_only: true,
            ssh_symbol: "🌐 ",
            trim_at: ".",
            detect_env_vars: vec![],
            format: "[$ssh_symbol$hostname]($style) in ",
            style: "green dimmed bold",
            disabled: false,
            user_overrides: HashMap::new(),
        }
    }
}
