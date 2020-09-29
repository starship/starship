use crate::config::{ModuleConfig, RootModuleConfig};
use std::collections::HashMap;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DirectoryConfig<'a> {
    pub truncation_length: i64,
    pub truncate_to_repo: bool,
    pub substitutions: HashMap<String, &'a str>,
    pub fish_style_pwd_dir_length: i64,
    pub use_logical_path: bool,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub read_only_symbol: &'a str,
    pub read_only_symbol_style: &'a str,
}

impl<'a> RootModuleConfig<'a> for DirectoryConfig<'a> {
    fn new() -> Self {
        DirectoryConfig {
            truncation_length: 3,
            truncate_to_repo: true,
            fish_style_pwd_dir_length: 0,
            substitutions: HashMap::new(),
            use_logical_path: true,
            format: "[$path]($style)[$read_only]($read_only_style) ",
            style: "cyan bold",
            disabled: false,
            read_only_symbol: "🔒",
            read_only_symbol_style: "red",
        }
    }
}
