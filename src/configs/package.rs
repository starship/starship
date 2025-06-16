use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct PackageConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub display_private: bool,
    pub disabled: bool,
    pub version_format: &'a str,
}

impl Default for PackageConfig<'_> {
    fn default() -> Self {
        PackageConfig {
            format: "is [$symbol$version]($style) ",
            symbol: "📦 ",
            style: "208 bold",
            display_private: false,
            disabled: false,
            version_format: "v${raw}",
        }
    }
}
