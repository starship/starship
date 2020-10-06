use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ThresholdStyle<'a> {
    pub threshold: i64,
    pub style: &'a str,
}

#[derive(Clone, ModuleConfig)]
pub struct DiskUsedConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub prefix: &'a str,
    pub separator: &'a str,
    pub disabled: bool,
    pub show_percentage: bool,
    pub current_threshold: Option<i64>,
    pub all_threshold: Option<i64>,
    pub show_current_name: bool,
    pub default_style: &'a str,
    pub threshold_styles: Vec<ThresholdStyle<'a>>,
}

impl<'a> RootModuleConfig<'a> for DiskUsedConfig<'a> {
    fn new() -> Self {
        DiskUsedConfig {
            format: "[($prefix )]($style)$symbol$current_storage(\\[$other_storage\\]) ",
            symbol: "💾 ",
            prefix: "",
            separator: "|",
            disabled: true,
            show_percentage: true,
            current_threshold: Some(30),
            all_threshold: Some(30),
            show_current_name: false,
            default_style: "white bold",
            threshold_styles: vec![
                ThresholdStyle {threshold: 50, style: "yellow bold"},
                ThresholdStyle {threshold: 80, style: "red bold"},
            ],
        }
    }
}