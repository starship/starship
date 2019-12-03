use crate::config::{ModuleConfig, RootModuleConfig};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, PartialEq)]
pub enum AwsItems {
    All,
    Region,
    Profile,
}

#[derive(Clone, ModuleConfig)]
pub struct AwsConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
    pub displayed_items: AwsItems,
}

impl<'a> RootModuleConfig<'a> for AwsConfig<'a> {
    fn new() -> Self {
        AwsConfig {
            format: "on ${styled?value=☁️  &style=yellow bold}${items?style=yellow bold} ",
            disabled: false,
            displayed_items: AwsItems::All,
        }
    }
}

impl<'a> ModuleConfig<'a> for AwsItems {
    fn from_config(config: &toml::Value) -> Option<Self> {
        match config.as_str()? {
            "all" => Some(AwsItems::All),
            "region" => Some(AwsItems::Region),
            "profile" => Some(AwsItems::Profile),
            _ => None,
        }
    }
}
