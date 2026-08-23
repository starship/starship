#![warn(clippy::disallowed_methods)]

#[macro_use]
extern crate shadow_rs;

use std::num::NonZeroUsize;
use std::thread::available_parallelism;

shadow!(shadow);

// Lib is present to allow for benchmarking
pub mod bug_report;
pub mod config;
pub mod configs;
pub mod configure;
pub mod context;
pub mod escaping;
pub mod formatter;
pub mod init;
pub mod logger;
pub mod module;
pub mod print;
pub mod stream;
pub use frame::FrameEncoding;
pub use transport::{StreamingTransport, TransportMismatch};

// `stream` is the only part of the streaming prompt anything outside this
// crate needs to name — a prompt is served by one call — so the protocol,
// plan and repaint diff stay private and free to change shape without that
// being a breaking change.
mod damage;
mod frame;
mod modules;
mod plan;
mod render;
mod segment;
mod transport;
mod utils;

#[cfg(test)]
mod test;

/// A validated count of worker threads.
///
/// Guaranteed to be at least one, so a thread pool can never be built with no
/// workers at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadCount(NonZeroUsize);

impl ThreadCount {
    /// Build a count from an arbitrary number, raising zero to one.
    ///
    /// Zero is not a usable pool size, and it is reachable both from a
    /// user-supplied `STARSHIP_NUM_THREADS=0` and from arithmetic on a
    /// processor count, so it is corrected rather than rejected.
    const fn at_least_one(requested: usize) -> Self {
        match NonZeroUsize::new(requested) {
            Some(count) => Self(count),
            None => Self(NonZeroUsize::MIN),
        }
    }

    /// The count as a plain number, for interfaces that take one.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

/// Blocking threads to allocate for every logical processor.
///
/// Module rendering is I/O-bound (waiting on subprocesses up to
/// `command_timeout`), so the classic `processors * (1 + waiting/computing)`
/// sizing rule argues for a much higher multiplier than four; four is a
/// deliberately conservative choice, paired with the floor and ceiling below.
const BLOCKING_THREADS_PER_PROCESSOR: usize = 4;

/// The fewest blocking threads to allocate, whatever the processor count.
///
/// A low processor count says nothing about how many subprocess waits can
/// usefully overlap; sixteen covers the modules that typically reach a
/// subprocess in one prompt, so their timeouts run concurrently instead of
/// queueing behind one another.
const MINIMUM_BLOCKING_THREADS: usize = 16;

/// The most blocking threads to allocate, whatever the processor count.
///
/// The pool is built eagerly on every prompt, so each thread costs startup
/// time (measured: 8 to 44 threads added ~1ms on an 11-processor machine) —
/// cheap against what a slow module saves, but not free. Sixty-four also
/// exceeds the number of modules a single prompt could plausibly block on, so
/// raising it further would only add startup cost.
const MAXIMUM_BLOCKING_THREADS: usize = 64;

/// Return the number of threads starship should use, if configured.
pub fn num_configured_starship_threads() -> Option<usize> {
    std::env::var("STARSHIP_NUM_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
}

/// The number of logical processors, or one if it cannot be determined.
fn processor_count() -> usize {
    available_parallelism().map_or(1, usize::from)
}

/// The sizing policy, as a pure function of its two inputs.
fn thread_count_from(configured: Option<usize>, processors: usize) -> ThreadCount {
    if let Some(configured) = configured {
        return ThreadCount::at_least_one(configured);
    }

    ThreadCount::at_least_one(
        processors
            .saturating_mul(BLOCKING_THREADS_PER_PROCESSOR)
            .clamp(MINIMUM_BLOCKING_THREADS, MAXIMUM_BLOCKING_THREADS),
    )
}

fn thread_count() -> ThreadCount {
    thread_count_from(num_configured_starship_threads(), processor_count())
}

/// Return the maximum number of threads for the global thread-pool.
///
/// Module rendering is overwhelmingly blocking I/O (only 26 of 104 modules are
/// [`Cadence::Instant`]), so sizing the pool to the processor count would force
/// a prompt with more deferred modules than cores through several rounds of
/// `command_timeout` in sequence; it is sized for blocking work instead.
pub fn num_rayon_threads() -> usize {
    thread_count().get()
}

#[cfg(test)]
mod thread_pool_tests {
    use super::*;

    #[test]
    fn the_ceiling_binds_above_the_uncapped_range() {
        for processors in (MAXIMUM_BLOCKING_THREADS / BLOCKING_THREADS_PER_PROCESSOR)..=1024 {
            assert_eq!(
                thread_count_from(None, processors).get(),
                MAXIMUM_BLOCKING_THREADS,
                "the module pool must stop growing with {processors} processors"
            );
        }
    }

    #[test]
    fn the_module_pool_stays_within_its_bounds() {
        for processors in 1..=256 {
            let blocking = thread_count_from(None, processors).get();
            assert!(
                (MINIMUM_BLOCKING_THREADS..=MAXIMUM_BLOCKING_THREADS).contains(&blocking),
                "with {processors} processors, blocking got {blocking} threads"
            );
        }
    }

    #[test]
    fn a_configured_thread_count_overrides_the_policy() {
        assert_eq!(thread_count_from(Some(3), 64).get(), 3);
    }

    #[test]
    fn a_pool_always_has_at_least_one_thread() {
        assert_eq!(thread_count_from(Some(0), 0).get(), 1);
        assert!(thread_count_from(None, 0).get() >= 1);
    }

    #[test]
    fn the_environment_variable_is_still_honoured_end_to_end() {
        assert_eq!(
            thread_count(),
            thread_count_from(num_configured_starship_threads(), processor_count())
        );
    }
}
