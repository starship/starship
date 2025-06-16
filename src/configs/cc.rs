use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields),
    schemars(bound = "CcConfig<'a, T>: std::default::Default, T: schemars::JsonSchema")
)]
#[serde(default)]
pub struct CcConfig<'a, T> {
    #[serde(skip)]
    pub marker: std::marker::PhantomData<T>,

    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub commands: Vec<Vec<&'a str>>,
}
