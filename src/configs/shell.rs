use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ShellConfig<'a> {
    pub format: &'a str,
    pub bash_indicator: &'a str,
    pub fish_indicator: &'a str,
    pub zsh_indicator: &'a str,
    pub powershell_indicator: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pwsh_indicator: Option<&'a str>,
    pub ion_indicator: &'a str,
    pub elvish_indicator: &'a str,
    pub tcsh_indicator: &'a str,
    pub nu_indicator: &'a str,
    pub xonsh_indicator: &'a str,
    pub cmd_indicator: &'a str,
    pub unknown_indicator: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl Default for ShellConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$indicator]($style) ",
            bash_indicator: "bsh",
            fish_indicator: "fsh",
            zsh_indicator: "zsh",
            powershell_indicator: "psh",
            pwsh_indicator: None,
            ion_indicator: "ion",
            elvish_indicator: "esh",
            tcsh_indicator: "tsh",
            nu_indicator: "nu",
            xonsh_indicator: "xsh",
            cmd_indicator: "cmd",
            unknown_indicator: "",
            style: "white bold",
            disabled: true,
        }
    }
}
