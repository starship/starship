use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct SpotifyConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SpotifyConfig<'a> {
    fn default() -> Self {
        SpotifyConfig {
            format: "[$symbol($song )]($style)",
            symbol: "阮",
            style: "green bold",
            disabled: false,
        }
    }
}
