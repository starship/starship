use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct LuaConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub lua_binary: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for LuaConfig<'a> {
    fn default() -> Self {
        LuaConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🌙 ",
            style: "bold blue",
            lua_binary: "lua",
            disabled: false,
            detect_extensions: vec!["lua"],
            detect_files: vec![".lua-version"],
            detect_folders: vec!["lua"],
        }
    }
}
