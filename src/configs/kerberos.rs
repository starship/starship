use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct KerberosConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub expired_symbol: &'a str,
    pub style: &'a str,
    pub expired_style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for KerberosConfig<'a> {
    fn default() -> Self {
        KerberosConfig {
            format: "[krb5_$symbol]($style)",
            symbol: "🎫 ",
            expired_symbol: "❌ ",
            style: "bold green",
            expired_style: "bold red",
            disabled: true,
        }
    }
}
