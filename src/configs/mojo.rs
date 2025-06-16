use nu_ansi_term::Color;
use serde::{Deserialize, Serialize};

pub const MOJO_DEFAULT_COLOR: Color = Color::Fixed(208);

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct MojoConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl Default for MojoConfig<'_> {
    fn default() -> Self {
        MojoConfig {
            format: "with [$symbol($version )]($style)",
            symbol: "🔥 ",
            style: "bold 208",
            disabled: false,
            detect_extensions: vec!["mojo", "🔥"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
