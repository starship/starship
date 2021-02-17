use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ShellConfig<'a> {
    pub format: &'a str,
    pub bash_indicator: &'a str,
    pub fish_indicator: &'a str,
    pub zsh_indicator: &'a str,
    pub powershell_indicator: &'a str,
    pub ion_indicator: &'a str,
    pub elvish_indicator: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ShellConfig<'a> {
    fn new() -> Self {
        ShellConfig {
            format: "$indicator ",
            bash_indicator: "bsh",
            fish_indicator: "fsh",
            zsh_indicator: "zsh",
            powershell_indicator: "psh",
            ion_indicator: "ion",
            elvish_indicator: "esh",
            disabled: true,
        }
    }
}
