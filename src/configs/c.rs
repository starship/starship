use crate::configs::cc::CcConfig;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(transparent)]
pub struct CConfig<'a>(#[serde(borrow)] pub CcConfig<'a, CConfig<'a>>);

impl Default for CcConfig<'_, CConfig<'_>> {
    fn default() -> Self {
        CcConfig {
            marker: std::marker::PhantomData::<CConfig<'_>>,
            format: "via [$symbol($version(-$name) )]($style)",
            version_format: "v${raw}",
            style: "149 bold",
            symbol: "C ",
            disabled: false,
            detect_extensions: vec!["c", "h"],
            detect_files: vec![],
            detect_folders: vec![],
            commands: vec![
                // the compiler is usually cc, and --version works on gcc and clang
                vec!["cc", "--version"],
                // but on some platforms gcc is installed as *gcc*, not cc
                vec!["gcc", "--version"],
                // for completeness, although I've never seen a clang that wasn't cc
                vec!["clang", "--version"],
            ],
        }
    }
}
