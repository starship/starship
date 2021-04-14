use crate::config::ModuleConfig;
use serde::Serialize;
use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GcloudConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub region_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for GcloudConfig<'a> {
    fn default() -> Self {
        GcloudConfig {
            format: "on [$symbol$account$account_at(\\($region\\))]($style) ",
            symbol: "☁️ ",
            style: "bold blue",
            disabled: false,
            region_aliases: HashMap::new(),
        }
    }
}
