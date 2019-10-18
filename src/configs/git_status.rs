use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub stashed: SegmentConfig<'a>,
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
}

impl<'a> RootModuleConfig<'a> for GitStatusConfig<'a> {
    fn new() -> Self {
        GitStatusConfig {
            stashed: SegmentConfig::new("$"),
            ahead: SegmentConfig::new("⇡"),
            behind: SegmentConfig::new("⇣"),
            diverged: SegmentConfig::new("⇕"),
            conflicted: SegmentConfig::new("="),
            show_sync_count: false,
            conflicted_count: CountConfig {
                enabled: false,
                style: None,
            },
            deleted: SegmentConfig::new("✘"),
            deleted_count: CountConfig {
                enabled: false,
                style: None,
            },
            renamed: SegmentConfig::new("»"),
            renamed_count: CountConfig {
                enabled: false,
                style: None,
            },
            modified: SegmentConfig::new("!"),
            modified_count: CountConfig {
                enabled: false,
                style: None,
            },
            staged: SegmentConfig::new("+"),
            staged_count: CountConfig {
                enabled: false,
                style: None,
            },
            untracked: SegmentConfig::new("?"),
            untracked_count: CountConfig {
                enabled: false,
                style: None,
            },
            prefix: "[",
            suffix: "] ",
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}

#[derive(Clone, Copy, ModuleConfig)]
pub struct CountConfig {
    pub enabled: bool,
    pub style: Option<Style>,
}
