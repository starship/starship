use crate::config::{ModuleConfig, RootModuleConfig};
use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;

#[derive(Clone, ModuleConfig)]
pub struct AwsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub region_aliases: HashMap<String, &'a str>,
}

impl<'a> RootModuleConfig<'a> for AwsConfig<'a> {
    fn new() -> Self {
        AwsConfig {
            format: "on [$symbol($profile )(\\($region\\) )]($style)",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
            region_aliases: HashMap::new(),
        }
    }
}
