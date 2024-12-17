use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct RandomConfig<'a> {
    pub format: &'a str,
    pub symbols: Vec<&'a str>,
    pub styles: Vec<&'a str>,
    pub disabled: bool,
    pub description: &'a str,
}

impl<'a> Default for RandomConfig<'a> {
    fn default() -> Self {
        Self {
            format: "[$symbol ]($style)",
            styles: vec!["bold", "italic"],
            disabled: false,
            symbols: vec![">", "$", "%"],
            description: "<random module>",
        }
    }
}
