use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;
use std::borrow::Cow;

#[derive(Clone, ModuleConfig)]
pub struct CondaConfig<'a> {
    pub truncation_length: usize,
    pub symbol: SegmentConfig<'a>,
    pub environment: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CondaConfig<'a> {
    fn new() -> Self {
        CondaConfig {
            truncation_length: 1,
            symbol: SegmentConfig {
                value: Cow::Borrowed("C "),
                style: None,
            },
            environment: SegmentConfig {
                value: Cow::Borrowed(""),
                style: None,
            },
            style: Color::Green.bold(),
            disabled: false,
        }
    }
}
