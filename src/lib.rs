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
    num_module_threads_with(
        modules,
        waiting,
        num_configured_starship_threads(),
        available_parallelism().map_or(1, usize::from),
    )
}

fn num_module_threads_with(
    modules: usize,
    waiting: Option<usize>,
    configured: Option<usize>,
    cores: usize,
) -> usize {
    configured
        .unwrap_or_else(|| cores.saturating_add(waiting.unwrap_or(cores)).clamp(4, 64))
        .max(1)
        .min(modules)
}

#[cfg(test)]
mod scheduling_tests {
    use super::num_module_threads_with;

    #[test]
    fn configured_thread_count_is_clamped_to_available_modules() {
        assert_eq!(5, num_module_threads_with(5, None, Some(64), 8));
    }

    #[test]
    fn configured_thread_count_has_a_one_thread_floor() {
        assert_eq!(1, num_module_threads_with(3, None, Some(0), 8));
    }

    #[test]
    fn configured_thread_count_does_not_create_workers_without_modules() {
        assert_eq!(0, num_module_threads_with(0, None, Some(8), 8));
    }

    #[test]
    fn default_thread_count_accounts_for_waiters_and_resource_limits() {
        assert_eq!(16, num_module_threads_with(32, Some(15), None, 1));
        assert_eq!(4, num_module_threads_with(32, None, None, 1));
        assert_eq!(2, num_module_threads_with(2, None, None, 1));
        assert_eq!(0, num_module_threads_with(0, None, None, 1));
        assert_eq!(64, num_module_threads_with(128, None, None, 128));
    }
}
