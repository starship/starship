use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub conflicted: SegmentConfig<'a>,
    pub ahead: SegmentConfig<'a>,
    pub behind: SegmentConfig<'a>,
    pub diverged: SegmentConfig<'a>,
    pub untracked: SegmentConfig<'a>,
    pub stashed: SegmentConfig<'a>,
    pub modified: SegmentConfig<'a>,
    pub staged: SegmentConfig<'a>,
    pub renamed: SegmentConfig<'a>,
    pub deleted: SegmentConfig<'a>,
    pub show_sync_count: bool,
    pub prefix: SegmentConfig<'a>,
    pub suffix: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitStatusConfig<'a> {
    fn new() -> Self {
        GitStatusConfig {
            // This is the order that the sections will appear in
            conflicted: SegmentConfig::new("="),
            ahead: SegmentConfig::new("⇡"),
            behind: SegmentConfig::new("⇣"),
            diverged: SegmentConfig::new("⇕"),
            untracked: SegmentConfig::new("?"),
            stashed: SegmentConfig::new("$"),
            modified: SegmentConfig::new("!"),
            staged: SegmentConfig::new("+"),
            renamed: SegmentConfig::new("»"),
            deleted: SegmentConfig::new("✘"),
            show_sync_count: false,
            prefix: SegmentConfig::new("["),
            suffix: SegmentConfig::new("] "),
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
