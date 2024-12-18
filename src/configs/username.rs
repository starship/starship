use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct UsernameConfig<'a> {
    pub detect_env_vars: Vec<&'a str>,
    pub format: &'a str,
    pub style_root: &'a str,
    pub style_user: &'a str,
    pub show_always: bool,
    pub disabled: bool,
    pub aliases: IndexMap<String, &'a str>,
}

impl Default for UsernameConfig<'_> {
    fn default() -> Self {
        UsernameConfig {
            detect_env_vars: vec![],
            format: "[$user]($style) in ",
            style_root: "red bold",
            style_user: "yellow bold",
            show_always: false,
            disabled: false,
            aliases: IndexMap::new(),
        }
    }
}
