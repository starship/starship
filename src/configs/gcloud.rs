use crate::config::{ModuleConfig, RootModuleConfig};
use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;

#[derive(Clone, ModuleConfig)]
pub struct GcloudConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub region_aliases: HashMap<String, &'a str>,
}

impl<'a> RootModuleConfig<'a> for GcloudConfig<'a> {
    fn new() -> Self {
        GcloudConfig {
            format: "on [$symbol$account(\\($region\\))]($style) ",
            symbol: "☁️ ",
            style: "bold blue",
            disabled: false,
            region_aliases: HashMap::new(),
        }
    }
}
