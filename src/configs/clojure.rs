use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ClojureConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ClojureConfig<'a> {
    fn default() -> Self {
        ClojureConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            disabled: false,
            style: "red dimmed",
            symbol: "λ̮̮̑̑ ",
            detect_extensions: vec!["clj", "cljc", "cljs", "cljd", "clj_kondo"],
            detect_files: vec![
                "deps.edn",
                "project.clj",
                "build.boot",
                "bb.edn",
                "shadow-cljs.edn",
            ],
            detect_folders: vec![],
        }
    }
}
