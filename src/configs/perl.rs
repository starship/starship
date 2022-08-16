use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct PerlConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for PerlConfig<'a> {
    fn default() -> Self {
        PerlConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🐪 ",
            style: "149 bold",
            disabled: false,
            detect_extensions: vec!["pl", "pm", "pod"],
            detect_files: vec![
                "Makefile.PL",
                "Build.PL",
                "cpanfile",
                "cpanfile.snapshot",
                "META.json",
                "META.yml",
                ".perl-version",
            ],
            detect_folders: vec![],
        }
    }
}
