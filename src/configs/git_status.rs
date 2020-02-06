use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;
use std::path::PathBuf;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub stashed: SegmentConfig<'a>,
    pub stashed_count: CountConfig,
    pub ahead: SegmentConfig<'a>,
    pub behind: SegmentConfig<'a>,
    pub diverged: SegmentConfig<'a>,
    pub show_sync_count: bool,
    pub conflicted: SegmentConfig<'a>,
    pub conflicted_count: CountConfig,
    pub deleted: SegmentConfig<'a>,
    pub deleted_count: CountConfig,
    pub renamed: SegmentConfig<'a>,
    pub renamed_count: CountConfig,
    pub modified: SegmentConfig<'a>,
    pub modified_count: CountConfig,
    pub staged: SegmentConfig<'a>,
    pub staged_count: CountConfig,
    pub untracked: SegmentConfig<'a>,
    pub untracked_count: CountConfig,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
    pub disabled_for: Vec<PathBuf>
}

impl<'a> RootModuleConfig<'a> for GitStatusConfig<'a> {
    fn new() -> Self {
        GitStatusConfig {
            stashed: SegmentConfig::new("$"),
            stashed_count: CountConfig::default(),
            ahead: SegmentConfig::new("⇡"),
            behind: SegmentConfig::new("⇣"),
            diverged: SegmentConfig::new("⇕"),
            conflicted: SegmentConfig::new("="),
            show_sync_count: false,
            conflicted_count: CountConfig::default(),
            deleted: SegmentConfig::new("✘"),
            deleted_count: CountConfig::default(),
            renamed: SegmentConfig::new("»"),
            renamed_count: CountConfig::default(),
            modified: SegmentConfig::new("!"),
            modified_count: CountConfig::default(),
            staged: SegmentConfig::new("+"),
            staged_count: CountConfig::default(),
            untracked: SegmentConfig::new("?"),
            untracked_count: CountConfig::default(),
            prefix: "[",
            suffix: "] ",
            style: Color::Red.bold(),
            disabled: false,
            disabled_for: Vec::new()
        }
    }
}

#[derive(Clone, Copy, ModuleConfig, Default)]
pub struct CountConfig {
    pub enabled: bool,
    pub style: Option<Style>,
}
