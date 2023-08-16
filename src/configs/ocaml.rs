use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct OCamlConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub global_switch_indicator: &'a str,
    pub local_switch_indicator: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for OCamlConfig<'a> {
    fn default() -> Self {
        OCamlConfig {
            format: "via [$symbol($version )(\\($switch_indicator$switch_name\\) )]($style)",
            version_format: "v${raw}",
            global_switch_indicator: "",
            local_switch_indicator: "*",
            symbol: "🐫 ",
            style: "bold yellow",
            disabled: false,
            detect_extensions: vec!["opam", "ml", "mli", "re", "rei"],
            detect_files: vec!["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"],
            detect_folders: vec!["_opam", "esy.lock"],
        }
    }
}
