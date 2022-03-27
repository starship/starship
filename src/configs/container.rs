use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ContainerConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for ContainerConfig<'a> {
    fn default() -> Self {
        ContainerConfig {
            format: "[$symbol \\[$name\\]]($style) ",
            symbol: "⬢",
            style: "red bold dimmed",
            disabled: false,
        }
    }
}
