use crate::config::ModuleConfig;
use indexmap::IndexMap;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct DirectoryConfig<'a> {
    pub truncation_length: i64,
    pub truncate_to_repo: bool,
    pub repo_markers: Vec<&'a str>,
    pub substitutions: IndexMap<String, &'a str>,
    pub fish_style_pwd_dir_length: i64,
    pub use_logical_path: bool,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub read_only: &'a str,
    pub read_only_style: &'a str,
    pub truncation_symbol: &'a str,
    pub home_symbol: &'a str,
}

impl<'a> Default for DirectoryConfig<'a> {
    fn default() -> Self {
        DirectoryConfig {
            truncation_length: 3,
            truncate_to_repo: true,
            repo_markers: Vec::new(),
            fish_style_pwd_dir_length: 0,
            use_logical_path: true,
            substitutions: IndexMap::new(),
            format: "[$path]($style)[$read_only]($read_only_style) ",
            style: "cyan bold",
            disabled: false,
            read_only: "🔒",
            read_only_style: "red",
            truncation_symbol: "",
            home_symbol: "~",
        }
    }
}
