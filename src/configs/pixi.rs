use serde::{Deserialize, Serialize};

use crate::config::VecOr;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct PixiConfig<'a> {
    pub pixi_binary: VecOr<&'a str>,
    pub show_default_environment: bool,
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_files: Vec<&'a str>,
}

impl<'a> Default for PixiConfig<'a> {
    fn default() -> Self {
        PixiConfig {
            pixi_binary: VecOr(vec!["pixi"]),
            show_default_environment: true,
            format: "via [$symbol($version )(\\($environment\\) )]($style)",
            version_format: "v${raw}",
            symbol: "📦 ",
            style: "yellow bold",
            disabled: false,
            detect_files: vec!["pixi.toml", "pixi.lock"],
        }
    }
}
