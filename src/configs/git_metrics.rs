use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitMetricsConfig<'a> {
    pub added_style: &'a str,
    pub deleted_style: &'a str,
    pub only_nonzero_diffs: bool,
    pub format: &'a str,
    pub disabled: bool,
    pub ignore_submodules: bool,
    pub mode: GitMetricsMode,
}

impl Default for GitMetricsConfig<'_> {
    fn default() -> Self {
        Self {
            added_style: "bold green",
            deleted_style: "bold red",
            only_nonzero_diffs: true,
            format: "([+$added]($added_style) )([-$deleted]($deleted_style) )",
            disabled: true,
            ignore_submodules: false,
            mode: GitMetricsMode::Unstaged,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, serde::Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum GitMetricsMode {
    #[default]
    Unstaged,
    All,
}

// TODO: use derive (#3941)
impl<'de> Deserialize<'de> for GitMetricsMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "unstaged" => Ok(Self::Unstaged),
            "all" => Ok(Self::All),
            _ => Err(serde::de::Error::custom(format!(
                "invalid git_metrics mode: {s}"
            ))),
        }
    }
}
