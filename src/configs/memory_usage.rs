use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct MemoryConfig<'a> {
    pub threshold: i64,
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for MemoryConfig<'a> {
    fn default() -> Self {
        MemoryConfig {
            threshold: 75,
            format: "via $symbol[$ram( | $swap)]($style) ",
            style: "white bold dimmed",
            symbol: "🐏 ",
            disabled: true,
        }
    }
}
