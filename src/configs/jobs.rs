use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JobsConfig<'a> {
    pub threshold: i64,
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for JobsConfig<'a> {
    fn default() -> Self {
        JobsConfig {
            threshold: 1,
            format: "[$symbol$number]($style) ",
            symbol: "✦",
            style: "bold blue",
            disabled: false,
        }
    }
}
