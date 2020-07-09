use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PythonConfig<'a> {
    pub pyenv_version_name: bool,
    pub python_binary: &'a str,
    pub scan_for_pyfiles: bool,
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PythonConfig<'a> {
    fn new() -> Self {
        PythonConfig {
            pyenv_version_name: false,
            python_binary: "python",
            scan_for_pyfiles: true,
            format: "via [$symbol$version( \\($virtualenv\\))]($style) ",
            style: "yellow bold",
            symbol: "🐍 ",
            disabled: false,
        }
    }
}
