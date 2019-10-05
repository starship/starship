use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StatusConfig<'a> {
    pub display_mode: DisplayMode,

    pub success: Style,
    pub error: Style,

    pub pipe_prefix: &'a str,
    pub pipeline: SegmentConfig<'a>,
    pub pipe_suffix: &'a str,

    pub exit_prefix: &'a str,
    pub exitcode: SegmentConfig<'a>,
    pub exit_suffix: &'a str,

    pub use_symbols: bool,
    pub success_symbol: &'a str,
    pub error_symbol: &'a str,

    pub simple_pipeline: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for StatusConfig<'a> {
    fn new() -> Self {
        StatusConfig {
            display_mode: DisplayMode::OnErrorOrMismatch,

            success: Color::White.dimmed(),
            error: Color::Red.into(),

            pipe_prefix: "(",
            pipeline: SegmentConfig {
                value: "",
                style: None,
            },
            pipe_suffix: ")",

            exit_prefix: "[",
            exitcode: SegmentConfig {
                value: "",
                style: None,
            },
            exit_suffix: "]",

            use_symbols: false,
            success_symbol: "✔",
            error_symbol: "✖",

            simple_pipeline: true,
            disabled: false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum DisplayMode {
    Always,      // Always shows all exit codes
    OnExitError, // Show all exit codes if there's any error in the pipeline
    OnError,     // Show all exit codes only on error
    OnErrorOrMismatch, // Show exit codes when pipeline and exit code don't match
                 //     or there's an error in the pipeline
}

impl<'a> ModuleConfig<'a> for DisplayMode {
    fn from_config(config: &toml::Value) -> Option<Self> {
        Some(match config.as_str()? {
            "always" => DisplayMode::Always,
            "error" => DisplayMode::OnExitError,
            "any error" => DisplayMode::OnError,
            /*"mismatch" |*/ _ => DisplayMode::OnErrorOrMismatch,
        })
    }
}
