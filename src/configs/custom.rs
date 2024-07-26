use crate::config::{Either, VecOr};

use indexmap::IndexMap;

use serde::{self, Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct CustomConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub error: &'a str,
    pub command: &'a str,
    pub when: Either<bool, &'a str>,
    pub require_repo: bool,
    pub shell: VecOr<&'a str>,
    pub description: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    #[serde(alias = "files")]
    pub detect_files: Vec<&'a str>,
    #[serde(alias = "extensions")]
    pub detect_extensions: Vec<&'a str>,
    #[serde(alias = "directories")]
    pub detect_folders: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stdin: Option<bool>,
    #[serde(borrow)]
    pub formats: IndexMap<String, &'a str>,
    pub ignore_timeout: bool,
}

impl<'a> Default for CustomConfig<'a> {
    fn default() -> Self {
        CustomConfig {
            format: "[$symbol($output )]($style)",
            error: "Error $status",
            symbol: "",
            command: "",
            when: Either::First(false),
            require_repo: false,
            shell: VecOr::default(),
            description: "<custom config>",
            style: "green bold",
            disabled: false,
            detect_files: Vec::default(),
            detect_extensions: Vec::default(),
            detect_folders: Vec::default(),
            formats: IndexMap::new(),
            os: None,
            use_stdin: None,
            ignore_timeout: false,
        }
    }
}
