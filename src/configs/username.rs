use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct UserOverrideConfig<'a> {
    pub format: Option<&'a str>,
    pub style: Option<&'a str>,
    pub disabled: bool,
}

impl<'a> Default for UserOverrideConfig<'a> {
    fn default() -> Self {
        UserOverrideConfig {
            format: None,
            style: None,
            disabled: false,
        }
    }
}

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
    pub style: &'a str,
    #[serde(borrow)]
    pub user_overrides: HashMap<String, UserOverrideConfig<'a>>,
    pub show_always: bool,
    pub show_if_root: bool,
    pub show_if_different: bool,
    pub show_if_ssh: bool,
    pub disabled: bool,
    pub aliases: IndexMap<String, &'a str>,
}

impl<'a> Default for UsernameConfig<'a> {
    fn default() -> Self {
        UsernameConfig {
            detect_env_vars: vec![],
            format: "[$user]($style) in ",
            style: "yellow bold",
            user_overrides: HashMap::new(),
            show_always: false,
            show_if_root: true,
            show_if_different: true,
            show_if_ssh: true,
            disabled: false,
            aliases: IndexMap::new(),
        }
    }
}
