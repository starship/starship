use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JJStatusConfig<'a> {
    /// The default format for `jj_status`.
    pub format: &'a str,
    /// The style for the module.
    pub style: &'a str,

    /// The format shown when the current change or a parent has merge conflicts.
    pub conflicted: &'a str,

    /// The format shown when the current change has no description.
    pub description_empty: &'a str,
    /// The format shown when the current change has a description.
    pub description_present: &'a str,

    /// The format shown when the current change is hidden.
    pub hidden: &'a str,
    /// The format shown when the current change is immutable.
    pub immutable: &'a str,

    /// The format shown when a new file has been added in the working directory.
    pub added: &'a str,
    /// The format shown when a new file has been copied in the working directory.
    pub copied: &'a str,
    /// The format shown when a file has been deleted in the working directory.
    pub deleted: &'a str,
    /// The format shown when a file has been modified in the working directory.
    pub modified: &'a str,
    /// The format shown when a file has been renamed in the working directory.
    pub renamed: &'a str,

    /// Disables the `jj_status` module.
    pub disabled: bool,
}

impl Default for JJStatusConfig<'_> {
    fn default() -> Self {
        Self {
            format: "([\\[$all\\]]($style) )",
            style: "bold red",
            conflicted: "!",
            description_empty: "◌",
            description_present: "",
            hidden: "",
            immutable: "◆",
            added: "+",
            copied: "=",
            deleted: "✘",
            modified: "~",
            renamed: "»",
            disabled: false,
        }
    }
}
