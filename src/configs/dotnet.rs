use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct DotnetConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub heuristic: bool,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for DotnetConfig<'a> {
    fn default() -> Self {
        DotnetConfig {
            format: "via [$symbol($version )(🎯 $tfm )]($style)",
            version_format: "v${raw}",
            symbol: ".NET ",
            style: "blue bold",
            heuristic: true,
            disabled: false,
            detect_extensions: vec!["csproj", "fsproj", "xproj"],
            detect_files: vec![
                "global.json",
                "project.json",
                "Directory.Build.props",
                "Directory.Build.targets",
                "Packages.props",
            ],
            detect_folders: vec![],
        }
    }
}
