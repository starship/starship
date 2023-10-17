use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct CudaConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub commands: Vec<Vec<&'a str>>,
}

impl<'a> Default for CudaConfig<'a> {
    fn default() -> Self {
        CudaConfig {
            format: "via [$symbol($version)]($style)",
            version_format: "v${raw}",
            style: "bold #76b900",
            symbol: "NV ",
            disabled: false,
            commands: vec![
                // the compiler is usually nvcc
                vec!["nvcc", "--version"],
                // For some users, they do not install the compiler
                // but install the driver only.
                vec!["nvidia-smi", "--query"],
            ],
        }
    }
}
