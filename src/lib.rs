#![warn(clippy::disallowed_methods)]

#[macro_use]
extern crate shadow_rs;

use std::thread::available_parallelism;

shadow!(shadow);

// Lib is present to allow for benchmarking
pub mod bug_report;
pub mod config;
pub mod configs;
pub mod configure;
pub mod context;
pub mod context_env;
pub mod formatter;
pub mod init;
pub mod logger;
pub mod module;
mod modules;
pub mod print;
mod segment;
mod serde_utils;
mod utils;

#[cfg(test)]
mod test;

/// Return the number of threads starship should use, if configured.
pub fn num_configured_starship_threads() -> Option<usize> {
    std::env::var("STARSHIP_NUM_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
}

/// Return the maximum number of threads for the global thread-pool.
pub fn num_rayon_threads() -> usize {
    num_configured_starship_threads()
        // Default to the number of logical cores,
        // but restrict the number of threads to 8
        .unwrap_or_else(|| available_parallelism().map(usize::from).unwrap_or(1).min(8))
}
