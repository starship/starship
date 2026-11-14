use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
/// The `scaleway` module shows the current Scaleway configuration: active
/// profile, organization, project, region, zone and API URL.
///
/// The module will display when any of the following environment variables are set:
/// - `SCW_ACCESS_KEY` and `SCW_SECRET_KEY`
/// - `SCW_DEFAULT_ORGANIZATION_ID`
/// - `SCW_DEFAULT_PROJECT_ID`
/// - `SCW_DEFAULT_REGION`
/// - `SCW_DEFAULT_ZONE`
/// - `SCW_PROFILE`
/// - `SCW_API_URL`
///
/// or when one of these settings is present in a Scaleway config file, using
/// the active profile's values (following the scaleway SDK precedence:
/// `$SCW_CONFIG_PATH`, `$XDG_CONFIG_HOME`, then `$HOME/.config`).
pub struct ScalewayConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The symbol used before displaying the Scaleway information.
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// Table of aliases for organization IDs.
    pub organization_aliases: HashMap<String, &'a str>,
    /// Table of aliases for project IDs.
    pub project_aliases: HashMap<String, &'a str>,
    /// Table of aliases for regions.
    pub region_aliases: HashMap<String, &'a str>,
    /// Table of aliases for API URLs.
    pub api_url_aliases: HashMap<String, &'a str>,
    /// If true, displays info even if no Scaleway configuration is detected.
    pub force_display: bool,
}

impl Default for ScalewayConfig<'_> {
    fn default() -> Self {
        Self {
            format: "on [$symbol($profile )($project )(\\($zone\\) )]($style)",
            symbol: "☁️ ",
            style: "purple",
            organization_aliases: HashMap::new(),
            project_aliases: HashMap::new(),
            region_aliases: HashMap::new(),
            api_url_aliases: HashMap::new(),
            force_display: false,
        }
    }
}
