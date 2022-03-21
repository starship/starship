use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct PulumiConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for PulumiConfig<'a> {
    fn default() -> Self {
        PulumiConfig {
            format: "via [$symbol($username@)$stack]($style) ",
            version_format: "v${raw}",
            symbol: " ",
            style: "bold 5",
            disabled: false,
        }
    }
}
