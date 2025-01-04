use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct JjConfig<'a> {
    pub format: &'a str,
    #[serde(borrow, default)]
    pub change: JjChangeConfig<'a>,
    #[serde(borrow, default)]
    pub status: JjStatusConfig<'a>,
}

impl<'a> Default for JjConfig<'a> {
    fn default() -> Self {
        Self {
            format: "$change$status",
            change: Default::default(),
            status: Default::default(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct JjChangeConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
    pub style: &'a str,

    pub change_id_length: usize,
}

impl<'a> Default for JjChangeConfig<'a> {
    fn default() -> Self {
        Self {
            disabled: false,
            format: "on [$change_id]($style) ",
            style: "purple",

            change_id_length: 7,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct JjStatusConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
    pub style: &'a str,

    pub added: &'a str,
    pub deleted: &'a str,
    pub modified: &'a str,
    pub renamed: &'a str,
}

impl<'a> Default for JjStatusConfig<'a> {
    fn default() -> Self {
        Self {
            disabled: false,
            format: "[\\[$all_status\\]]($style) ",
            style: "yellow",

            added: "+",
            deleted: "✘",
            modified: "!",
            renamed: "»",
        }
    }
}
