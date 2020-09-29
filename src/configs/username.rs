use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct UsernameConfig<'a> {
    pub format: &'a str,
    pub style_root: &'a str,
    pub style_user: &'a str,
    pub show_always: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for UsernameConfig<'a> {
    fn new() -> Self {
        UsernameConfig {
            format: "[$user]($style) in ",
            style_root: "red bold",
            style_user: "yellow bold",
            show_always: false,
            disabled: false,
        }
    }
}
