use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitHostConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub symbols: HashMap<String, &'a str>,
    // pub github: &'a str,
    // pub gitlab: &'a str,
    // pub bitbucket: &'a str,
    // pub azure_devops: &'a str,
    // pub codecommit: &'a str,
    // pub codeberg: &'a str,
    pub unknown: &'a str,
    pub disabled: bool,
}

impl Default for GitHostConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$symbol]($style) ",
            style: "bold purple",
            symbols: HashMap::from([
                ("github.com".to_string(), "\u{f408}"),
                ("gitlab.com".to_string(), "\u{f296}"),
            ]),
            // github: "",
            // gitlab: "",
            // bitbucket: "",
            // azure_devops: "",
            // codecommit: "",
            // codeberg: "",
            unknown: "\u{e5fb}",
            disabled: true,
        }
    }
}
