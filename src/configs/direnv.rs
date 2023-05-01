use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct DirenvConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub allowed_msg: &'a str,
    pub denied_msg: &'a str,
    pub loaded_msg: &'a str,
    pub unloaded_msg: &'a str,
}

impl<'a> Default for DirenvConfig<'a> {
    fn default() -> Self {
        Self {
            format: "[$symbol$loaded/$allowed]($style) ",
            symbol: "direnv ",
            style: "bold orange",
            disabled: true,
            detect_extensions: vec![],
            detect_files: vec![".envrc"],
            detect_folders: vec![],
            allowed_msg: "allowed",
            denied_msg: "denied",
            loaded_msg: "loaded",
            unloaded_msg: "not loaded",
        }
    }
}
