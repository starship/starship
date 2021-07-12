use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct ShellConfig<'a> {
    pub format: &'a str,
    pub bash_indicator: &'a str,
    pub fish_indicator: &'a str,
    pub zsh_indicator: &'a str,
    pub powershell_indicator: &'a str,
    pub ion_indicator: &'a str,
    pub elvish_indicator: &'a str,
    pub tcsh_indicator: &'a str,
    pub nu_indicator: &'a str,
    pub unknown_indicator: &'a str,
    pub disabled: bool,
}

impl<'a> Default for ShellConfig<'a> {
    fn default() -> Self {
        ShellConfig {
            format: "$indicator ",
            bash_indicator: "bsh",
            fish_indicator: "fsh",
            zsh_indicator: "zsh",
            powershell_indicator: "psh",
            ion_indicator: "ion",
            elvish_indicator: "esh",
            tcsh_indicator: "tsh",
            nu_indicator: "nu",
            unknown_indicator: "",
            disabled: true,
        }
    }
}
