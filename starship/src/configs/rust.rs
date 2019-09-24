use crate::module_config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Debug)]
pub struct RustConfig<'a> {
    pub symbol: &'a str,
    pub style: Style,
    pub disabled: bool,
}

/* This is what the macro adds.
impl<'a> ModuleConfig<'a> for RustConfig<'a> {
    fn load_config(&self, config: &'a toml::Value) -> Self {
        let mut new_module_config = self.clone();
        if let toml::Value::Table(config) = config {
            if let Some(config_str) = config.get("symbol") {
                new_module_config.symbol = new_module_config.symbol.load_config(config_str);
            }
            if let Some(config_str) = config.get("disabled") {
                new_module_config.disabled = new_module_config.disabled.load_config(config_str);
            }
            if let Some(config_str) = config.get("style") {
                new_module_config.style = new_module_config.style.load_config(config_str);
            }
        }
        new_module_config
    }
}
*/

impl<'a> RootModuleConfig<'a> for RustConfig<'a> {
    fn new() -> Self {
        RustConfig {
            symbol: "🦀 ",
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}

// TODO Move this test to ./module_config.rs
#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn test_load_config() {
        let config = toml::toml! {
            disabled = false
            symbol = "R "
            style = "red italic"
        };
        let rust_config = RustConfig::load(&config);
        assert_eq!(rust_config.symbol, "R ");
    }
}
