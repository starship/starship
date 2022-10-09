use crate::config::VecOr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct DbtConfig<'a> {
    pub format: &'a str,
    pub python_binary: VecOr<&'a str>,
    pub project_dirs: Vec<&'a str>,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for DbtConfig<'a> {
    fn default() -> Self {
        DbtConfig {
            python_binary: VecOr(vec!["python", "python3"]),
            project_dirs: vec![],
            format: "via [$symbol($version )(\\($project\\) )]($style)",
            version_format: "v${raw}",
            symbol: "📊 ",
            style: "fg:#FF694A bold",
            disabled: false,
            detect_extensions: vec!["sql"],
            detect_files: vec!["dbt_project.yml"],
            detect_folders: vec![],
        }
    }
}
