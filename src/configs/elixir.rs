use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct ElixirConfig<'a> {
    pub format: &'a str,
    pub version_extract_map: HashMap<&'a str, &'a str>,
    pub version_format_map: HashMap<&'a str, &'a str>,
    pub version_command: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ElixirConfig<'a> {
    fn default() -> Self {
        ElixirConfig {
            format: "via [$symbol($version \\(OTP $otp_version\\) )]($style)",
            version_extract_map: HashMap::from([
                ("version", "Elixir\\s([\\d\\.\\-\\w]+)"),
                ("otp_version", "Erlang/OTP\\s([\\d\\.\\-\\w]+)"),
            ]),
            version_format_map: HashMap::from([("version", "v${raw}")]),
            version_command: "--version",
            symbol: "💧 ",
            style: "bold purple",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["mix.exs"],
            detect_folders: vec![],
        }
    }
}
