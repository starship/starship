use crate::config::VecOr;

use serde::{self, Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct CustomConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub command: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<&'a str>,
    pub shell: VecOr<&'a str>,
    pub description: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub files: Vec<&'a str>,
    pub extensions: Vec<&'a str>,
    pub directories: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<&'a str>,
}

impl<'a> Default for CustomConfig<'a> {
    fn default() -> Self {
        CustomConfig {
            format: "[$symbol($output )]($style)",
            symbol: "",
            command: "",
            when: None,
            shell: VecOr::default(),
            description: "<custom config>",
            style: "green bold",
            disabled: false,
            files: Vec::default(),
            extensions: Vec::default(),
            directories: Vec::default(),
            os: None,
        }
    }
}
