use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct KubernetesConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub context_aliases: HashMap<String, &'a str>,
    pub user_aliases: HashMap<String, &'a str>,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub contexts: Vec<KubernetesContextConfig<'a>>,
}

impl<'a> Default for KubernetesConfig<'a> {
    fn default() -> Self {
        KubernetesConfig {
            symbol: "☸ ",
            format: "[$symbol$context( \\($namespace\\))]($style) in ",
            style: "cyan bold",
            disabled: true,
            context_aliases: HashMap::new(),
            user_aliases: HashMap::new(),
            detect_extensions: vec![],
            detect_files: vec![],
            detect_folders: vec![],
            contexts: vec![],
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Default)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct KubernetesContextConfig<'a> {
    pub context_pattern: &'a str,
    pub user_pattern: Option<&'a str>,
    pub symbol: Option<&'a str>,
    pub style: Option<&'a str>,
    pub context_alias: Option<&'a str>,
    pub user_alias: Option<&'a str>,
}
