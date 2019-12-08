use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PackageConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PackageConfig<'a> {
    fn new() -> Self {
        PackageConfig {
            format: "is ${styled?value=📦 &style=red bold}${version?style=red bold} ",
            disabled: false,
        }
    }
}
