use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct HerokuConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub app_aliases: HashMap<String, &'a str>,
    pub account_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for HerokuConfig<'a> {
    fn default() -> Self {
        HerokuConfig {
            format: "[( $symbol on $app_name \\(via  $account\\) )]($style)",
            symbol: " ",
            style: "purple",
            disabled: false,
            app_aliases: HashMap::new(),
            account_aliases: HashMap::new(),
        }
    }
}
