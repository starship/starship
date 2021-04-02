use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct OCamlConfig<'a> {
    pub global_switch_indicator: &'a str,
    pub local_switch_indicator: &'a str,
    pub format: &'a str,
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
            global_switch_indicator: "",
            local_switch_indicator: "*",
            format: "via [$symbol($version )(\\($switch_indicator$switch_name\\) )]($style)",
            symbol: "🐫 ",
            style: "bold yellow",
            disabled: false,
            detect_extensions: vec!["opam", "ml", "mli", "re", "rei"],
            detect_files: vec!["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"],
            detect_folders: vec!["_opam", "esy.lock"],
        }
    }
}
