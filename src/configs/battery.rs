use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct BatteryConfig<'a> {
    pub full_symbol: SegmentConfig<'a>,
    pub charging_symbol: SegmentConfig<'a>,
    pub discharging_symbol: SegmentConfig<'a>,
    pub unknown_symbol: Option<SegmentConfig<'a>>,
    pub empty_symbol: Option<SegmentConfig<'a>>,
    pub display: Vec<BatteryDisplayConfig>,
    pub disabled: bool,
    pub percentage: SegmentConfig<'a>,
}

impl<'a> RootModuleConfig<'a> for BatteryConfig<'a> {
    fn new() -> Self {
        BatteryConfig {
            full_symbol: SegmentConfig::new("•"),
            charging_symbol: SegmentConfig::new("↑"),
            discharging_symbol: SegmentConfig::new("↓"),
            unknown_symbol: None,
            empty_symbol: None,
            display: vec![BatteryDisplayConfig {
                threshold: 10,
                style: Color::Red.bold(),
            }],
            disabled: false,
            percentage: SegmentConfig::default(),
        }
    }
}

#[derive(Clone, ModuleConfig)]
pub struct BatteryDisplayConfig {
    pub threshold: i64,
    pub style: Style,
}
