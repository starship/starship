use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
/// The `scaleway` module shows the current Scaleway project, region, and profile.
/// The module will display when any of the following environment variables are set:
/// - `SCW_ACCESS_KEY`
/// - `SCW_SECRET_KEY`
/// - `SCW_DEFAULT_PROJECT_ID`
/// - `SCW_DEFAULT_REGION`
/// - `SCW_DEFAULT_ZONE`
/// - `SCW_PROFILE`
/// - `SCW_API_URL`
///
/// When using temporary credentials or when `force_display` is set to `true`,
/// the module will display even if no credentials are detected.
pub struct ScalewayConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The symbol used before displaying the Scaleway information.
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// Table of project aliases to display in addition to the project ID.
    pub project_aliases: HashMap<String, &'a str>,
    /// Table of region aliases to display in addition to the region name.
    pub region_aliases: HashMap<String, &'a str>,
    /// Table of API URL aliases to display in addition to the API URL.
    pub api_url_aliases: HashMap<String, &'a str>,
    /// If true, displays info even if no Scaleway credentials are detected.
    pub force_display: bool,
}

impl Default for ScalewayConfig<'_> {
    fn default() -> Self {
        Self {
            format: "on [$symbol$project($region)]($style)",
            symbol: "☁️ ",
            style: "violet bold",
            project_aliases: HashMap::new(),
            region_aliases: HashMap::new(),
            api_url_aliases: HashMap::new(),
            force_display: false,
        }
    }
}
