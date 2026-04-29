use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitRemoteConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub no_remote_symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub providers: IndexMap<String, String>,
}

impl Default for GitRemoteConfig<'_> {
    fn default() -> Self {
        Self {
            format: "on [$symbol]($style)",
            symbol: "\u{f02a2}",
            no_remote_symbol: "\u{f1d3}",
            style: "bold dimmed white",
            disabled: true,
            providers: IndexMap::from([
                ("github.com".to_string(), "\u{eb00}".to_string()),
                ("gitlab.com".to_string(), "\u{f0ba0}".to_string()),
                ("bitbucket.org".to_string(), "\u{f00a8}".to_string()),
                ("codeberg.org".to_string(), "\u{f08e0}".to_string()),
                ("sr.ht".to_string(), "\u{f4aa}".to_string()),
                ("dev.azure.com".to_string(), "\u{f0fd5}".to_string()),
            ]),
        }
    }
}
