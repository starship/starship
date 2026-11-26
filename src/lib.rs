#![warn(clippy::disallowed_methods)]

#[macro_use]
extern crate shadow_rs;

use std::sync::Once;
use std::thread::available_parallelism;

shadow!(shadow);

// Lib is present to allow for benchmarking
pub mod bug_report;
pub mod config;
pub mod configs;
pub mod configure;
pub mod context;
pub mod formatter;
pub mod init;
pub mod logger;
pub mod module;
mod modules;
mod painted;
pub mod print;
mod segment;
pub mod stream;
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
        .unwrap_or_else(|| available_parallelism().map_or(1, usize::from).min(8))
}

/// Builds the global `rayon` thread pool the first time a module needs it:
/// prompts render their modules on threads of their own, and seldom do.
pub fn init_global_threadpool() {
    static BUILT: Once = Once::new();
    BUILT.call_once(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_rayon_threads())
            .build_global()
            .expect("Failed to initialize worker thread pool");
    });
}
