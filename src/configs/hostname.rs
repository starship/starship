use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct HostnameConfig<'a> {
    pub ssh_only: bool,
    pub trim_at: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for HostnameConfig<'a> {
    fn default() -> Self {
        HostnameConfig {
            ssh_only: true,
            trim_at: ".",
            format: "[$hostname]($style) in ",
            style: "green dimmed bold",
            disabled: false,
        }
    }
}
