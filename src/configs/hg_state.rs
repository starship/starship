use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct HgStateConfig<'a> {
    pub merge: &'a str,
    pub rebase: &'a str,
    pub update: &'a str,
    pub bisect: &'a str,
    pub shelve: &'a str,
    pub graft: &'a str,
    pub transplant: &'a str,
    pub histedit: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl Default for HgStateConfig<'_> {
    fn default() -> Self {
        HgStateConfig {
            merge: "MERGING",
            rebase: "REBASING",
            update: "UPDATING",
            bisect: "BISECTING",
            shelve: "SHELVING",
            graft: "GRAFTING",
            transplant: "TRANSPLANTING",
            histedit: "HISTEDITING",
            style: "bold yellow",
            format: "\\([$state]($style)\\) ",
            disabled: true,
        }
    }
}
