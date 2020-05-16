use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct EnvVarConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub variable: Option<&'a str>,
    pub default: Option<&'a str>,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for EnvVarConfig<'a> {
    fn new() -> Self {
        EnvVarConfig {
            symbol: "",
            style: "black bold dimmed",
            variable: None,
            default: None,
            format: "with [${env_value}]($style) ",
            disabled: false,
        }
    }
}
