use crate::configs::c::CConfig;
use crate::configs::cc::LangConfig;
use crate::impl_lang_config;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(transparent)]
pub struct CppConfig<'a>(#[serde(borrow)] CConfig<'a>);

impl<'a> Deref for CppConfig<'a> {
    type Target = CConfig<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for CppConfig<'_> {
    fn default() -> Self {
        CppConfig(CConfig {
            format: "via [$symbol($version(-$name) )]($style)",
            version_format: "v${raw}",
            style: "149 bold",
            symbol: "C++ ",
            disabled: false,
            detect_extensions: vec![
                "cc", "cp", "cxx", "cpp", "CPP", "c++", "C", "hh", "H", "hp", "hxx", "hpp", "HPP",
                "h++", "tcc",
            ],
            detect_files: vec![],
            detect_folders: vec![],
            commands: vec![
                vec!["c++", "--version"],
                vec!["g++", "--version"],
                vec!["clang++", "--version"],
            ],
        })
    }
}

impl_lang_config!(CppConfig);
