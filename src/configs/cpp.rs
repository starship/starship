use crate::configs::cc::CcConfig;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(transparent)]
pub struct CppConfig<'a>(#[serde(borrow)] pub CcConfig<'a, CppConfig<'a>>);

impl Default for CcConfig<'_, CppConfig<'_>> {
    fn default() -> Self {
        CcConfig {
            marker: std::marker::PhantomData::<CppConfig<'_>>,
            format: "via [$symbol($version(-$name) )]($style)",
            version_format: "v${raw}",
            style: "149 bold",
            symbol: "C++ ",
            disabled: true,
            detect_extensions: vec!["cpp", "cc", "cxx", "c++", "hpp", "hh", "hxx", "h++", "tcc"],
            detect_files: vec![],
            detect_folders: vec![],
            commands: vec![
                vec!["c++", "--version"],
                vec!["g++", "--version"],
                vec!["clang++", "--version"],
            ],
        }
    }
}
