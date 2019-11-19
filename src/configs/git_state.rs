use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStateConfig<'a> {
    pub rebase: SegmentConfig<'a>,
    pub merge: SegmentConfig<'a>,
    pub revert: SegmentConfig<'a>,
    pub cherry_pick: SegmentConfig<'a>,
    pub bisect: SegmentConfig<'a>,
    pub am: SegmentConfig<'a>,
    pub am_or_rebase: SegmentConfig<'a>,
    pub progress_divider: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitStateConfig<'a> {
    fn new() -> Self {
        GitStateConfig {
            rebase: SegmentConfig::new("REBASING"),
            merge: SegmentConfig::new("MERGING"),
            revert: SegmentConfig::new("REVERTING"),
            cherry_pick: SegmentConfig::new("CHERRY-PICKING"),
            bisect: SegmentConfig::new("BISECTING"),
            am: SegmentConfig::new("AM"),
            am_or_rebase: SegmentConfig::new("AM/REBASE"),
            progress_divider: SegmentConfig::new("/"),
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
