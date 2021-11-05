use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct RubyConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub detect_variables: Vec<&'a str>,
}

impl<'a> Default for RubyConfig<'a> {
    fn default() -> Self {
        RubyConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "💎 ",
            style: "bold red",
            disabled: false,
            detect_extensions: vec!["rb"],
            detect_files: vec!["Gemfile", ".ruby-version"],
            detect_folders: vec![],
            detect_variables: vec!["RUBY_VERSION", "RBENV_VERSION"],
        }
    }
}
