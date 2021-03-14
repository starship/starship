use crate::config::{ModuleConfig, VecOr};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, Default, PartialEq)]
pub struct Files<'a>(pub Vec<&'a str>);

#[derive(Clone, Default, PartialEq)]
pub struct Extensions<'a>(pub Vec<&'a str>);

#[derive(Clone, Default, PartialEq)]
pub struct Directories<'a>(pub Vec<&'a str>);

#[derive(Clone, ModuleConfig)]
pub struct CustomConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub command: &'a str,
    pub when: Option<&'a str>,
    pub shell: VecOr<&'a str>,
    pub description: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub files: Files<'a>,
    pub extensions: Extensions<'a>,
    pub directories: Directories<'a>,
}

impl<'a> Default for CustomConfig<'a> {
    fn default() -> Self {
        CustomConfig {
            format: "[$symbol($output )]($style)",
            symbol: "",
            command: "",
            when: None,
            shell: VecOr::default(),
            description: "<custom config>",
            style: "green bold",
            disabled: false,
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
                log::warn!("Unexpected file {:?}", item);
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
                log::warn!("Unexpected extension {:?}", item);
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
                log::warn!("Unexpected directory {:?}", item);
            }
        }

        Some(Directories(directories))
    }
}
