use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct FillConfig<'a> {
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for FillConfig<'a> {
    fn default() -> Self {
        FillConfig {
            style: "bold black",
            symbol: ".",
            disabled: false,
        }
    }
}
