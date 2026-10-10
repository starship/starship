//! When a streamed prompt is drawn.
//!
//! Every module renders at once, and each change one makes to the prompt
//! boards a bus, which draws everything aboard when it leaves. Until the first
//! paint a shell shows nothing at all, so the prompt itself boards at the
//! start.
//!
//! What a redraw costs is how much it is seen to flash. Changes that land
//! close together look like one prompt appearing, so drawing them apart
//! flashes; a module that takes a while looks like a change of its own, so
//! drawing it as soon as it lands hardly does. So a bus is as big as the
//! fallback when it opens for a change that took no time to render, and
//! smaller the longer the change took: `fallback / (1 + took / fallback)`, half
//! as big for a change that took a fallback. A bus waits at most its size.
//!
//! A module still rendering could land at any moment, so while one is, the bus
//! waits as long as it waits for anything. Once none is, there is nothing to
//! wait for, and it leaves.

use std::time::{Duration, Instant};

/// Longer than any prompt waits for anything, which bounds every sum of
/// waiting the bus does.
const FOREVER: Duration = Duration::from_secs(60 * 60);

/// The changes waiting to be drawn.
pub struct Bus {
    fallback: Duration,
    aboard: Option<Aboard>,
}

/// A bus with something aboard.
#[derive(Clone, Copy, Debug)]
struct Aboard {
    /// When what is aboard longest boarded.
    since: Instant,
    size: Duration,
}

impl Bus {
    /// The bus of a stream started at `start`, with the prompt itself aboard.
    pub fn new(start: Instant, fallback: Duration) -> Self {
        let fallback = fallback.min(FOREVER);
        Self {
            fallback,
            aboard: Some(Aboard {
                since: start,
                size: fallback,
            }),
        }
    }

    /// A change boards at `now`, having taken `took` to render.
    pub fn board(&mut self, now: Instant, took: Duration) {
        let size = size(self.fallback, took);
        self.aboard.get_or_insert(Aboard { since: now, size });
    }

    pub fn is_empty(&self) -> bool {
        self.aboard.is_none()
    }

    /// The bus leaves, and what was aboard is drawn.
    pub fn leave(&mut self) {
        self.aboard = None;
    }

    /// When the bus leaves, if anything is aboard, given whether any module is
    /// still rendering.
    pub fn departure(&self, now: Instant, rendering: bool) -> Option<Instant> {
        let aboard = self.aboard?;
        Some(if rendering {
            aboard.since + aboard.size
        } else {
            now
        })
    }
}

/// The size of a bus opened for a change that took `took` to render.
fn size(fallback: Duration, took: Duration) -> Duration {
    let fallback = fallback.as_nanos();
    let total = fallback + took.min(FOREVER).as_nanos();
    // A fallback of an hour squared still fits a u128 many times over.
    let nanoseconds = (fallback * fallback).checked_div(total).unwrap_or(0);
    Duration::from_nanos(u64::try_from(nanoseconds).unwrap_or(u64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FALLBACK: Duration = Duration::from_millis(50);

    fn milliseconds(milliseconds: u64) -> Duration {
        Duration::from_millis(milliseconds)
    }

    #[test]
    fn a_bus_is_smaller_the_longer_the_change_it_opens_for_took() {
        let sizes = [0, 50, 150, 450].map(|took| size(FALLBACK, milliseconds(took)));

        assert_eq!(
            [50_000, 25_000, 12_500, 5_000].map(Duration::from_micros),
            sizes
        );
        assert_eq!(Duration::ZERO, size(Duration::ZERO, Duration::ZERO));
    }

    #[test]
    fn a_bus_waits_out_its_size_while_anything_is_rendering() {
        let start = Instant::now();
        let at = |milliseconds: u64| start + Duration::from_millis(milliseconds);
        let first = Bus::new(start, FALLBACK);
        let mut later = Bus::new(start, FALLBACK);
        later.leave();
        later.board(at(300), milliseconds(300));

        assert_eq!(Some(at(50)), first.departure(start, true));
        assert_eq!(Some(start), first.departure(start, false));
        assert_eq!(
            Some(at(300) + size(FALLBACK, milliseconds(300))),
            later.departure(at(300), true),
            "a change that took three hundred milliseconds waits seven for others"
        );
    }

    #[test]
    fn a_change_boarding_a_waiting_bus_does_not_hold_it_up() {
        let start = Instant::now();
        let mut bus = Bus::new(start, FALLBACK);
        bus.board(start + milliseconds(40), milliseconds(40));

        assert_eq!(
            Some(start + FALLBACK),
            bus.departure(start + milliseconds(40), true)
        );
    }

    #[test]
    fn an_empty_bus_never_leaves() {
        let start = Instant::now();
        let mut bus = Bus::new(start, FALLBACK);
        bus.leave();

        assert!(bus.is_empty());
        assert_eq!(None, bus.departure(start, true));
    }
}
