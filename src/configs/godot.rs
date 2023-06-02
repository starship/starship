use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GodotConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub show_version: bool,
    pub version_command: Vec<&'a str>,
}

impl<'a> Default for GodotConfig<'a> {
    fn default() -> Self {
        GodotConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            style: "bold blue",
            symbol: "godot ",
            disabled: false,
            show_version: false,
            version_command: vec!["godot", "--version"],
        }
    }
}
