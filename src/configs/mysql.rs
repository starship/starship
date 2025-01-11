use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Symbol {
    Database,
    Running,
    Stopped,
    NotApplicable,
}
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct MySQLConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub style_ok: &'a str,
    pub style_error: &'a str,
    pub style_not_applicable: &'a str,
    pub symbols: IndexMap<Symbol, &'a str>,
    pub lag_threshold_s: f64,
    pub disabled: bool,
    pub url: &'a str,
}

impl<'a> MySQLConfig<'a> {
    pub fn get_symbol(&self, key: &Symbol) -> Option<&'a str> {
        self.symbols.get(key).copied()
    }
}

impl Default for MySQLConfig<'_> {
    fn default() -> Self {
        MySQLConfig {
            format: "[$symbol]($style)[$up]($up_style)[/]($style)[$io]($io_style)[/]($style)[$sql]($sql_style) [$lag]($lag_style)",
            style: "bold white",
            style_ok: "bold green",
            style_error: "bold red",
            style_not_applicable: "bold white",
            symbols: indexmap! {
                Symbol::Database => "🐬 " ,
                Symbol::Running => "▶",
                Symbol::Stopped => "■",
                Symbol::NotApplicable => "●",
            },
            url: "mysql://root@localhost:3306/",
            lag_threshold_s: 10.0,
            disabled: true,
        }
    }
}
