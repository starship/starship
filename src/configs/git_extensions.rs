use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct GitExtensionsConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub extensions: Vec<&'a str>,
}

impl<'a> Default for GitExtensionsConfig<'a> {
    fn default() -> Self {
        GitExtensionsConfig {
            format: "[($symbol$active_exts )]($style)",
            style: "149 bold",
            symbol: "git exts: ",
            disabled: true,
            extensions: vec!["lfs", "svn"],
        }
    }
}
