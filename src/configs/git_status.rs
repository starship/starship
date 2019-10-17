use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub conflicted: SegmentConfig<'a>,
    pub conflicted_count_disabled: bool,
    pub deleted: SegmentConfig<'a>,
    pub deleted_count_disabled: bool,
    pub renamed: SegmentConfig<'a>,
    pub renamed_count_disabled: bool,
    pub modified: SegmentConfig<'a>,
    pub modified_count_disabled: bool,
    pub staged: SegmentConfig<'a>,
    pub staged_count_disabled: bool,
    pub untracked: SegmentConfig<'a>,
    pub untracked_count_disabled: bool,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitStatusConfig<'a> {
    fn new() -> Self {
        GitStatusConfig {
            conflicted: SegmentConfig::new("="),
            conflicted_count_disabled: true,
            deleted: SegmentConfig::new("✘"),
            deleted_count_disabled: true,
            renamed: SegmentConfig::new("»"),
            renamed_count_disabled: true,
            modified: SegmentConfig::new("!"),
            modified_count_disabled: true,
            staged: SegmentConfig::new("+"),
            staged_count_disabled: true,
            untracked: SegmentConfig::new("?"),
            untracked_count_disabled: true,
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
