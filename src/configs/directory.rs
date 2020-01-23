use crate::config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DirectoryConfig<'a> {
    pub truncation_length: i64,
    pub truncate_to_repo: bool,
    pub fish_style_pwd_dir_length: i64,
    pub use_logical_path: bool,
    pub prefix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for DirectoryConfig<'a> {
    fn new() -> Self {
        DirectoryConfig {
            truncation_length: 3,
            truncate_to_repo: true,
            fish_style_pwd_dir_length: 0,
            use_logical_path: true,
            prefix: "in ",
            style: Color::Cyan.bold(),
            disabled: false,
        }
    }
}
