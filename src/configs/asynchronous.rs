use std::time::Duration;

use serde::{Deserialize, Serialize};

/// How a shell that streams prompts is sent them.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct AsynchronousConfig {
    /// Draw a prompt before every module has rendered, then refine it as they
    /// do. When disabled, the prompt is drawn once every module has rendered.
    pub enabled: bool,

    /// While the prompt is shown, render again the modules whose values go
    /// stale on their own, and redraw the prompt when they change. Each such
    /// module can be kept from it with its own `refresh` option.
    pub refresh: bool,
    pub bus: BusConfig,
}

impl Default for AsynchronousConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            refresh: true,
            bus: BusConfig::default(),
        }
    }
}

/// When a streamed prompt is redrawn.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct BusConfig {
    /// Plan redraws from what each module did for the previous prompt. When
    /// disabled, every module is waited for as one never measured is.
    pub adaptive: bool,

    /// How much waiting, in milliseconds, a redraw of a module that took no
    /// time is worth, shared by everything drawn with it: the more is ready,
    /// the sooner it is drawn. Redraws of modules that took longer are worth
    /// less. A module never measured is waited for this long.
    pub fallback: u64,
}

impl Default for BusConfig {
    fn default() -> Self {
        Self {
            adaptive: true,
            fallback: 50,
        }
    }
}

impl BusConfig {
    pub fn fallback(&self) -> Duration {
        Duration::from_millis(self.fallback)
    }
}
