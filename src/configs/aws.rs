use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AwsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub region_aliases: HashMap<String, &'a str>,
    pub profile_aliases: HashMap<String, &'a str>,
    pub expiration_symbol: &'a str,
}

impl<'a> Default for AwsConfig<'a> {
    fn default() -> Self {
        AwsConfig {
            format: "on [$symbol($profile )(\\($region\\) )(\\[$duration\\])]($style)",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
            region_aliases: HashMap::new(),
            profile_aliases: HashMap::new(),
            expiration_symbol: "X",
        }
    }
}
