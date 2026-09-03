use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn default_profiles() -> IndexMap<String, String> {
    IndexMap::from_iter([(
        "claude-code".to_string(),
        "$claude_model$git_branch$claude_context$claude_cost".to_string(),
    )])
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct StarshipRootConfig {
    #[serde(rename = "$schema")]
    schema: String,
    pub format: String,
    pub right_format: String,
    pub continuation_prompt: String,
    pub scan_timeout: u64,
    pub command_timeout: u64,
    pub add_newline: bool,
    pub follow_symlinks: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palette: Option<String>,
    pub palettes: HashMap<String, Palette>,
    #[serde(rename = "profiles")]
    #[cfg_attr(feature = "config-schema", schemars(default = "default_profiles"))]
    pub user_profiles: IndexMap<String, String>,
    #[serde(skip)]
    pub internal_profiles: IndexMap<String, String>,
}

pub type Palette = HashMap<String, String>;

/// The default order modules are drawn in, and what `$all` expands to.
///
/// Generated from the module registry — see [`crate::modules::registry`] —
/// rather than hand-maintained here; each module's place in this ordering is
/// declared next to the rest of what the registry says about it.
///
/// NOTE: If this ordering changes, the default prompt order subheading inside
/// the prompt heading of the config docs needs to be updated to match.
pub use crate::modules::registry::PROMPT_ORDER;

// On changes please also update `Default` for the `FullConfig` struct in `mod.rs`
impl Default for StarshipRootConfig {
    fn default() -> Self {
        Self {
            schema: "https://starship.rs/config-schema.json".to_string(),
            format: "$all".to_string(),
            right_format: String::new(),
            continuation_prompt: "[∙](bright-black) ".to_string(),
            user_profiles: IndexMap::new(),
            internal_profiles: default_profiles(),
            scan_timeout: 30,
            command_timeout: 500,
            add_newline: true,
            follow_symlinks: true,
            palette: None,
            palettes: HashMap::default(),
        }
    }
}
