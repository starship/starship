use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
//TODO docs
pub struct SalesforceConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The symbol used before displaying the current AWS profile.
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// Disables the AWS module.
    pub disabled: bool,
    /// Show alias if possible instead of full email address
    pub show_alias: bool,
    /// Use sfdx to determine current org
    pub use_sfdx:bool
}

impl<'a> Default for SalesforceConfig<'a> {
    fn default() -> Self {
        SalesforceConfig{
            format: "org [$symbol($org_name) ]($style)",
            symbol: "☁ ",
            style: "bold blue",
            disabled: false,
            show_alias: true,
            use_sfdx: false
        }
    }

}