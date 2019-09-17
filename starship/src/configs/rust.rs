use crate::module_config::ModuleConfig;

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Debug)]
pub struct RustConfig<'a> {
    pub symbol: &'a str,
    pub style: Style,
    pub disabled: bool,
}

/* This works
impl<'a> ModuleConfig<'a> for RustConfig<'a> {
    fn load_config(&self, config: &'a toml::Value) -> Self {
        let mut new_module_config = self.clone();
        if let toml::Value::Table(config) = config {
            if let Some(config_str) = config.get("symbol") {
                if let Some(symbol) = <&str>::from_config(config_str) {
                    new_module_config.symbol = symbol;
                }
            }
            if let Some(config_str) = config.get("disabled") {
                if let Some(disabled) = <bool>::from_config(config_str) {
                    new_module_config.symbol = disabled;
                }
            }
            if let Some(config_str) = config.get("style") {
                if let Some(style) = <Style>::from_config(config_str) {
                    new_module_config.symbol = style;
                }
            }
        }
        new_module_config
    }
}
*/

pub const DEFAULT_RUST_CONFIG: RustConfig = RustConfig {
    symbol: "🦀 ",
    style: Style {
        foreground: Some(Color::Red),
        background: None,
        is_bold: true,
        is_dimmed: false,
        is_italic: false,
        is_underline: false,
        is_blink: false,
        is_reverse: false,
        is_hidden: false,
        is_strikethrough: false,
    },
    disabled: false,
};

// TODO Move this test to ./module_config.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::module_config::ModuleConfig;
    use toml;

    #[test]
    fn test_load_config() {
        let config = toml::toml! {
            disabled = false
            symbol = "R "
            style = "red italic"
        };
        let rust_config = DEFAULT_RUST_CONFIG.load_config(&config);
        assert_eq!(rust_config.symbol, "R ");
    }
}
