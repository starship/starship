use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct StatusConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub success_symbol: &'a str,
    pub not_executable_symbol: &'a str,
    pub not_found_symbol: &'a str,
    pub sigint_symbol: &'a str,
    pub signal_symbol: &'a str,
    pub style: &'a str,
    pub map_symbol: bool,
    pub recognize_signal_code: bool,
    pub pipestatus: bool,
    pub pipestatus_separator: &'a str,
    pub pipestatus_format: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipestatus_segment_format: Option<&'a str>,
    pub disabled: bool,
}

impl<'a> Default for StatusConfig<'a> {
    fn default() -> Self {
        StatusConfig {
            format: "[$symbol$status]($style) ",
            symbol: "❌",
            success_symbol: "",
            not_executable_symbol: "🚫",
            not_found_symbol: "🔍",
            sigint_symbol: "🧱",
            signal_symbol: "⚡",
            style: "bold red",
            map_symbol: false,
            recognize_signal_code: true,
            pipestatus: false,
            pipestatus_separator: "|",
            pipestatus_format:
                "\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)",
            pipestatus_segment_format: None,
            disabled: true,
        }
    }
}
