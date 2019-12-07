use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PythonConfig<'a> {
    pub format: &'a str,
    pub pyenv_format: &'a str,
    pub pyenv_version_name: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PythonConfig<'a> {
    fn new() -> Self {
        PythonConfig {
            format: "via ${styled?value=🐍 &style=yellow bold}${version?style=yellow bold}${pyenv?style=yellow bold} ",
            pyenv_format: " (${name})",
            pyenv_version_name: false,
            disabled: false,
        }
    }
}
