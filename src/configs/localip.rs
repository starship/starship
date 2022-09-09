use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct LocalipConfig<'a> {
    pub ssh_only: bool,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for LocalipConfig<'a> {
    fn default() -> Self {
        LocalipConfig {
            ssh_only: true,
            format: "[$localipv4]($style) ",
            style: "yellow bold",
            disabled: true,
        }
    }
}
