use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct EnvVarConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<&'a str>,
    pub format: &'a str,
    pub description: &'a str,
    pub disabled: bool,
}

impl<'a> Default for EnvVarConfig<'a> {
    fn default() -> Self {
        EnvVarConfig {
            symbol: "",
            style: "black bold dimmed",
            variable: None,
            default: None,
            format: "with [$env_value]($style) ",
            description: "<env_var module>",
            disabled: false,
        }
    }
}
