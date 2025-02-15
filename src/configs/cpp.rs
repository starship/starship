use crate::configs::c::CConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(transparent)]
pub struct CppConfig<'a>(#[serde(borrow)] pub CConfig<'a>);

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
