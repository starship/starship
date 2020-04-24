use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ElixirConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ElixirConfig<'a> {
    fn new() -> Self {
        ElixirConfig {
            format: "via [💧 $version \\(OTP $otp_version\\)](purple bold) ",
            disabled: false,
        }
    }
}
