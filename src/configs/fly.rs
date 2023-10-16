use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FlyConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for FlyConfig<'a> {
    fn default() -> Self {
        FlyConfig {
            // $version is excluded by default as flyctl version command takes too long
            // tracked in https://github.com/superfly/flyctl/issues/2908
            format: "via [$symbol($app )($primary_region )]($style)",
            version_format: "v${raw}",
            symbol: "🎈 ",
            style: "purple bold",
            disabled: false,
        }
    }
}
