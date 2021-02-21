use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PerlConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for PerlConfig<'a> {
    fn new() -> Self {
        PerlConfig {
            symbol: "🐪 ",
            style: "149 bold",
            format: "via [$symbol($version )]($style)",
            disabled: false,
            detect_extensions: vec!["pl", "pm", "pod"],
            detect_files: vec![
                "Makefile.PL",
                "Build.PL",
                "cpanfile",
                "cpanfile.snapshot",
                "META.json",
                "META.yml",
                ".perl-version",
            ],
            detect_folders: vec![],
        }
    }
}
