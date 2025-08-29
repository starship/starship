use crate::configs::cc::CcConfig;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct CppConfigMarker;

pub type CppConfig<'a> = CcConfig<'a, CppConfigMarker>;

impl Default for CppConfig<'_> {
    fn default() -> Self {
        Self {
            marker: std::marker::PhantomData::<CppConfigMarker>,

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
