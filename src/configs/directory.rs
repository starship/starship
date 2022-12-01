use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct DirectoryConfig<'a> {
    pub truncation_length: i64,
    pub truncate_to_repo: bool,
    pub substitutions: IndexMap<String, &'a str>,
    pub fish_style_pwd_dir_length: i64,
    pub use_logical_path: bool,
    pub format: &'a str,
    pub repo_root_format: &'a str,
    pub style: &'a str,
    pub repo_root_style: Option<&'a str>,
    pub before_repo_root_style: Option<&'a str>,
    pub disabled: bool,
    pub read_only: &'a str,
    pub read_only_style: &'a str,
    pub truncation_symbol: &'a str,
    pub home_symbol: &'a str,
    pub use_os_path_sep: bool,
}

impl<'a> Default for DirectoryConfig<'a> {
    fn default() -> Self {
        DirectoryConfig {
            truncation_length: 3,
            truncate_to_repo: true,
            fish_style_pwd_dir_length: 0,
            use_logical_path: true,
            substitutions: IndexMap::new(),
            format: "[$path]($style)[$read_only]($read_only_style) ",
            repo_root_format: "[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) ",
            style: "cyan bold",
            repo_root_style: None,
            before_repo_root_style: None,
            disabled: false,
            read_only: "🔒",
            read_only_style: "red",
            truncation_symbol: "",
            home_symbol: "~",
            use_os_path_sep: true,
        }
    }
}
