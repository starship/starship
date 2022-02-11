use crate::config::ModuleConfig;
use serde::Serialize;
use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct HerokuConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub app_aliases: HashMap<String, &'a str>,
    pub account_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for HerokuConfig<'a> {
    fn default() -> Self {
        HerokuConfig {
            format: "[( $symbol on $app_name \\(via  $account\\) )]($style)",
            symbol: " ",
            style: "purple",
            disabled: false,
            app_aliases: HashMap::new(),
            account_aliases: HashMap::new(),
        }
    }
}
