use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct NixShellConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub impure_msg: &'a str,
    pub pure_msg: &'a str,
    pub disabled: bool,
}

/* The trailing double spaces in `symbol` are needed to work around issues with
multiwidth emoji support in some shells. Please do not file a PR to change this
unless you can show that your changes do not affect this workaround.  */
impl<'a> Default for NixShellConfig<'a> {
    fn default() -> Self {
        NixShellConfig {
            format: "via [$symbol$state( \\($name\\))]($style) ",
            symbol: "❄️  ",
            style: "bold blue",
            impure_msg: "impure",
            pure_msg: "pure",
            disabled: false,
        }
    }
}
