use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct OCamlConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for OCamlConfig<'a> {
    fn new() -> Self {
        OCamlConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🐫 ",
            style: "bold yellow",
            disabled: false,
            detect_extensions: vec!["opam", "ml", "mli", "re", "rei"],
            detect_files: vec!["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"],
            detect_folders: vec!["_opam", "esy.lock"],
        }
    }
}
