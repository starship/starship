use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::Style;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, Default, PartialEq)]
pub struct Files<'a>(pub Vec<&'a str>);

#[derive(Clone, Default, PartialEq)]
pub struct Extensions<'a>(pub Vec<&'a str>);

#[derive(Clone, Default, PartialEq)]
pub struct Directories<'a>(pub Vec<&'a str>);

#[derive(Clone, ModuleConfig)]
pub struct CustomConfig<'a> {
    pub symbol: Option<SegmentConfig<'a>>,
    pub command: &'a str,
    pub when: Option<&'a str>,
    pub shell: Option<&'a str>,
    pub description: &'a str,
    pub style: Option<Style>,
    pub disabled: bool,
    pub prefix: Option<&'a str>,
    pub suffix: Option<&'a str>,
    pub files: Files<'a>,
    pub extensions: Extensions<'a>,
    pub directories: Directories<'a>,
}

impl<'a> RootModuleConfig<'a> for CustomConfig<'a> {
    fn new() -> Self {
        CustomConfig {
            symbol: None,
            command: "",
            when: None,
            shell: None,
            description: "<custom config>",
            style: None,
            disabled: false,
            prefix: None,
            suffix: None,
            files: Files::default(),
            extensions: Extensions::default(),
            directories: Directories::default(),
        }
    }
}

impl<'a> ModuleConfig<'a> for Files<'a> {
    fn from_config(config: &'a toml::Value) -> Option<Self> {
        let mut files = Vec::new();

        for item in config.as_array()? {
            if let Some(file) = item.as_str() {
                files.push(file);
            } else {
                log::debug!("Unexpected file {:?}", item);
            }
        }

        Some(Files(files))
    }
}

impl<'a> ModuleConfig<'a> for Extensions<'a> {
    fn from_config(config: &'a toml::Value) -> Option<Self> {
        let mut extensions = Vec::new();

        for item in config.as_array()? {
            if let Some(file) = item.as_str() {
                extensions.push(file);
            } else {
                log::debug!("Unexpected extension {:?}", item);
            }
        }

        Some(Extensions(extensions))
    }
}

impl<'a> ModuleConfig<'a> for Directories<'a> {
    fn from_config(config: &'a toml::Value) -> Option<Self> {
        let mut directories = Vec::new();

        for item in config.as_array()? {
            if let Some(file) = item.as_str() {
                directories.push(file);
            } else {
                log::debug!("Unexpected directory {:?}", item);
            }
        }

        Some(Directories(directories))
    }
}
