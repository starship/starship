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
    pub bus: BusConfig,
}

impl Default for AsynchronousConfig {
    fn default() -> Self {
        Self {
            enabled: true,
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
    /// The longest, in milliseconds, the first paint waits for modules, and
    /// how much a redraw that soon is worth in waiting. Later redraws are worth
    /// less, the longer the module that makes them took.
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
