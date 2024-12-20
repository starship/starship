use serde::{Deserialize, Serialize};

static DEFAULT_TEMPLATE: &str = r#"
separate(" ",
  change_id.shortest(6),
  branches.map(|x| if(
    x.name().substr(0, 20).starts_with(x.name()),
    x.name().substr(0, 20),
    x.name().substr(0, 19) ++ "…")
  ).join(" "),
  if(
    description.first_line().substr(0, 24).starts_with(description.first_line()),
    description.first_line().substr(0, 24),
    description.first_line().substr(0, 23) ++ "…"
  ),
  if(conflict, "conflict"),
  if(divergent, "divergent"),
  if(hidden, "hidden"),
)"#;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JujutsuConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub template: &'a str,
    pub disabled: bool,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for JujutsuConfig<'a> {
    fn default() -> Self {
        Self {
            format: "$symbol $commit_info ",
            symbol: "jj",
            template: DEFAULT_TEMPLATE,
            disabled: false,
            detect_folders: vec![".jj"],
        }
    }
}
