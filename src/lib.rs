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
pub mod formatter;
pub mod init;
pub mod logger;
pub mod module;
mod modules;
mod painted;
mod plan;
pub mod print;
mod segment;
pub mod stream;
mod utils;
mod workers;

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

/// Return the number of threads to render `modules` modules on, `waiting` of
/// which wait on subprocesses or the filesystem rather than compute, if that
/// is known.
///
/// Unless configured otherwise, one thread per logical core renders what
/// computes, and one more renders each module that waits; without knowing how
/// many do, as many are taken to wait as to compute.
fn num_module_threads(modules: usize, waiting: Option<usize>) -> usize {
    num_configured_starship_threads()
        .unwrap_or_else(|| {
            let cores = available_parallelism().map_or(1, usize::from);
            cores.saturating_add(waiting.unwrap_or(cores)).clamp(4, 64)
        })
        .max(1)
        .min(modules)
}
