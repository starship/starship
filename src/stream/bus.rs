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
//! as big for a change that took a fallback. A bus waits at most its size, and
//! running one costs as much as that much waiting. Waiting costs however long
//! the change aboard longest waits. Changes that land together open one bus,
//! as big as the biggest of them.
//!
//! What each module did for the previous prompt says when it should land, and
//! whether it will change anything, and the bus leaves on the cheapest
//! timetable for all of them: now, or once the next few have landed. While
//! those estimates hold, no timetable costs less, and the bus plans again
//! whenever a module lands. The prompt alone has nothing new to show, so the
//! first bus leaves no sooner than the first change lands, unless none lands
//! within its size. A module never measured, or later than expected, could
//! land at any moment, so the bus waits for it as long as it waits for
//! anything.

use std::ops::Add;
use std::time::{Duration, Instant};

/// A span of time a stream plans with: at most a day, longer than anything a
/// prompt waits for, and short enough that adding up as many as a stream ever
/// holds cannot overflow. Longer ones are taken as a day.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span(Duration);

impl Span {
    pub const MAX: Self = Self(Duration::from_secs(24 * 60 * 60));

    pub const fn get(self) -> Duration {
        self.0
    }
}

impl From<Duration> for Span {
    fn from(duration: Duration) -> Self {
        Self(duration.min(Self::MAX.0))
    }
}

impl Add<Span> for Instant {
    type Output = Self;

    fn add(self, span: Span) -> Self {
        self + span.0
    }
}

/// What a module did for the previous prompt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Estimate {
    /// Changed the prompt, after rendering for this long.
    Changed(Span),
    /// Left the prompt as it was: showed nothing, or what it already showed.
    Unchanged,
}

impl Estimate {
    /// What a module is expected to do, having done `measured` when it was
    /// expected to do `previous`. A quarter of the weight goes to a new
    /// measurement of how long it takes, so that one slow prompt moves the
    /// estimate without replacing it.
    pub fn after(previous: Option<Self>, measured: Self) -> Self {
        match (previous, measured) {
            (Some(Self::Changed(estimate)), Self::Changed(took)) => {
                Self::Changed(Span((estimate.0 * 3 + took.0) / 4))
            }
            _ => measured,
        }
    }
}

/// A module still rendering, as the bus sees it.
#[derive(Clone, Copy, Debug)]
pub struct Rendering {
    pub since: Instant,
    /// What it did for the previous prompt, if that is known.
    pub estimate: Option<Estimate>,
}

/// The changes waiting to be drawn, and what planning when takes.
pub struct Bus {
    fallback: Span,
    aboard: Option<Aboard>,
    /// Kept from one plan to the next, so that planning allocates nothing
    /// once it has planned for every module there is.
    landings: Vec<Landing>,
    departures: Vec<Departure>,
}

/// A bus with something aboard.
#[derive(Clone, Copy, Debug)]
struct Aboard {
    /// When what is aboard longest boarded.
    since: Instant,
    size: Span,
    passengers: Passengers,
}

/// What is aboard a bus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Passengers {
    /// The prompt alone, which has nothing new to show until a change lands.
    Prompt,
    /// Changes, the prompt's among them if it is the first bus.
    Changes,
}

impl Bus {
    /// The bus of a stream of `modules` modules started at `start`, with the
    /// prompt itself aboard.
    pub fn new(start: Instant, fallback: Span, modules: usize) -> Self {
        Self {
            fallback,
            aboard: Some(Aboard {
                since: start,
                size: fallback,
                passengers: Passengers::Prompt,
            }),
            landings: Vec::with_capacity(modules),
            departures: Vec::with_capacity(modules),
        }
    }

    /// A change boards at `now`, having taken `took` to render.
    pub fn board(&mut self, now: Instant, took: Span) {
        let size = size(self.fallback, took);
        let aboard = self.aboard.get_or_insert(Aboard {
            since: now,
            size,
            passengers: Passengers::Changes,
        });
        if aboard.since == now {
            aboard.size = aboard.size.max(size);
        }
        aboard.passengers = Passengers::Changes;
    }

    pub fn is_empty(&self) -> bool {
        self.aboard.is_none()
    }

    /// The bus leaves, and what was aboard is drawn.
    pub fn leave(&mut self) {
        self.aboard = None;
    }

    /// When the bus leaves, if anything is aboard, given the modules still
    /// rendering.
    pub fn departure(
        &mut self,
        now: Instant,
        rendering: impl IntoIterator<Item = Rendering>,
    ) -> Option<Instant> {
        let aboard = self.aboard?;
        let latest = aboard.since + aboard.size;
        self.landings.clear();
        for module in rendering {
            match module.estimate {
                Some(Estimate::Unchanged) => {}
                Some(Estimate::Changed(took)) if module.since + took > now => {
                    self.landings.push(Landing {
                        after: module.since + took - now,
                        size: size(self.fallback, took),
                    });
                }
                // Could land at any moment.
                _ => return Some(latest),
            }
        }
        self.landings.sort_unstable_by_key(|landing| landing.after);
        self.landings.dedup_by(|later, earlier| {
            let together = later.after == earlier.after;
            if together {
                earlier.size = earlier.size.max(later.size);
            }
            together
        });
        let patience = latest.saturating_duration_since(now);
        Some(
            now + cheapest(
                aboard.passengers,
                patience,
                &self.landings,
                &mut self.departures,
            ),
        )
    }
}

/// The size of a bus opened for a change that took `took` to render.
pub fn size(fallback: Span, took: Span) -> Span {
    let (fallback, took) = (fallback.0.as_nanos(), took.0.as_nanos());
    // A day squared, in nanoseconds, still fits a u128 many times over.
    let size = (fallback * fallback)
        .checked_div(fallback + took)
        .unwrap_or(0);
    Span(Duration::from_nanos(size.try_into().unwrap_or(u64::MAX)))
}

/// A change expected to land `after` from now, which opens a bus of `size`
/// unless one is waiting for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Landing {
    pub after: Duration,
    pub size: Span,
}

/// Leaving once the changes landing up to `at` from now have, at the `cost`
/// of all the waiting till then, and of the cheapest timetable after.
#[derive(Clone, Copy, Debug)]
struct Departure {
    at: Duration,
    cost: Duration,
}

/// How long from now a bus carrying `passengers`, which waits at most
/// `patience` longer, leaves on the cheapest timetable for `landings`, in the
/// order they land: at once, or as the last of the next few lands; on equal
/// cost, sooner.
///
/// Planned from the last landing back, a departure at a landing costs its
/// time plus the cheapest timetable for what lands after it, and the bus a
/// landing opens leaves on the cheapest departure within its size. A departure
/// costing no less than a sooner one is never the cheapest, so `departures`
/// keeps those that are by time, latest first and so costing ever more: the
/// cheapest within any time is the first that time reaches.
fn cheapest(
    passengers: Passengers,
    patience: Duration,
    landings: &[Landing],
    departures: &mut Vec<Departure>,
) -> Duration {
    let reached = |departures: &[Departure], limit: Duration| {
        departures.partition_point(|departure| departure.at > limit)
    };
    departures.clear();
    // What the buses after a departure cost: none, after the last landing.
    let mut after = Duration::ZERO;
    for landing in landings.iter().rev() {
        let departure = Departure {
            at: landing.after,
            cost: landing.after + after,
        };
        while departures
            .last()
            .is_some_and(|last| last.cost >= departure.cost)
        {
            departures.pop();
        }
        departures.push(departure);
        // Its bus reaches the departure just made, at least.
        let leaves = departures[reached(departures, landing.after + landing.size.0)];
        after = landing.size.0 + leaves.cost - landing.after;
    }
    // Leaving now costs what every bus after it does.
    match (passengers, departures.get(reached(departures, patience))) {
        (Passengers::Prompt, Some(waits)) => waits.at,
        (Passengers::Changes, Some(waits)) if waits.cost < after => waits.at,
        _ => Duration::ZERO,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::testing::every_timetable;
    use proptest::prelude::*;

    const FALLBACK: Span = Span(Duration::from_millis(50));

    fn milliseconds(milliseconds: u64) -> Span {
        Span(Duration::from_millis(milliseconds))
    }

    /// Modules that have been rendering since `start`, expected to change the
    /// prompt `took` milliseconds after it.
    fn changing(start: Instant, took: &[u64]) -> Vec<Rendering> {
        took.iter()
            .map(|&took| Rendering {
                since: start,
                estimate: Some(Estimate::Changed(milliseconds(took))),
            })
            .collect()
    }

    impl Bus {
        /// A bus that has drawn the first paint, with nothing aboard.
        fn empty(start: Instant) -> Self {
            let mut bus = Self::new(start, FALLBACK, 0);
            bus.leave();
            bus
        }
    }

    #[test]
    fn a_bus_is_smaller_the_longer_the_change_it_opens_for_took() {
        let sizes = [0, 50, 150, 450].map(|took| size(FALLBACK, milliseconds(took)).get());

        assert_eq!(
            [50_000, 25_000, 12_500, 5_000].map(Duration::from_micros),
            sizes
        );
        assert_eq!(Span::default(), size(Span::default(), Span::default()));
        assert_eq!(Span::MAX, size(Span::MAX, Span::default()));
    }

    #[test]
    fn the_first_paint_waits_for_what_would_otherwise_flash() {
        let start = Instant::now();
        let mut bus = Bus::new(start, FALLBACK, 3);

        assert_eq!(
            Some(start + milliseconds(16)),
            bus.departure(start, changing(start, &[1, 7, 16])),
            "drawn at one millisecond, the prompt would flash at sixteen"
        );
        assert_eq!(
            Some(start),
            bus.departure(start, changing(start, &[300])),
            "a module that takes long enough is drawn on its own"
        );
        assert_eq!(
            Some(start + milliseconds(7)),
            bus.departure(start, changing(start, &[1, 7, 300]))
        );
        assert_eq!(
            Some(start + milliseconds(40)),
            bus.departure(start, changing(start, &[40, 55, 70])),
            "the fifty-five is worth waiting for, but beyond the bus"
        );
    }

    #[test]
    fn a_change_that_took_longer_waits_less_for_others() {
        let start = Instant::now();
        let at = |milliseconds: u64| start + Duration::from_millis(milliseconds);
        let mut early = Bus::empty(start);
        early.board(at(10), milliseconds(10));
        let mut late = Bus::empty(start);
        late.board(at(300), milliseconds(300));

        assert_eq!(
            Some(at(30)),
            early.departure(at(10), changing(start, &[30])),
            "twenty milliseconds after a change that took ten would flash"
        );
        assert_eq!(
            Some(at(300)),
            late.departure(at(300), changing(start, &[320])),
            "but not after one that took three hundred"
        );
        assert_eq!(
            Some(at(303)),
            late.departure(at(300), changing(start, &[303])),
            "which still waits for what lands within its size"
        );
    }

    #[test]
    fn what_cannot_be_predicted_is_waited_for_as_long_as_the_bus_waits() {
        let start = Instant::now();
        let mut bus = Bus::new(start, FALLBACK, 1);
        let never_measured = Rendering {
            since: start,
            estimate: None,
        };

        assert_eq!(
            Some(start + FALLBACK),
            bus.departure(start, [never_measured])
        );
        assert_eq!(
            Some(start + FALLBACK),
            bus.departure(start + milliseconds(5), changing(start, &[5])),
            "a module that is due, and so past its estimate, could land at any moment"
        );
        assert_eq!(
            Some(start),
            bus.departure(
                start,
                [Rendering {
                    since: start,
                    estimate: Some(Estimate::Unchanged)
                }]
            ),
            "but one that changes nothing is not waited for"
        );
        let unread = std::iter::from_fn(|| panic!("an empty bus plans nothing"));
        assert_eq!(None, Bus::empty(start).departure(start, unread));
    }

    #[test]
    fn changes_landing_together_take_the_biggest_bus_among_them() {
        let start = Instant::now();
        let at = |milliseconds: u64| start + Duration::from_millis(milliseconds);
        // Rendering again since three hundred milliseconds in, and expected
        // to change the prompt ten after.
        let refreshing = [Rendering {
            since: at(300),
            estimate: Some(Estimate::Changed(milliseconds(10))),
        }];
        let mut slow = Bus::empty(start);
        slow.board(at(300), milliseconds(300));
        let mut together = Bus::empty(start);
        together.board(at(300), milliseconds(300));
        together.board(at(300), milliseconds(0));

        assert_eq!(
            Some(at(300)),
            slow.departure(at(300), refreshing),
            "ten milliseconds is beyond the bus of a change that took three hundred"
        );
        assert_eq!(
            Some(at(310)),
            together.departure(at(300), refreshing),
            "but within that of one that took none"
        );
    }

    #[test]
    fn one_slow_prompt_moves_an_estimate_without_replacing_it() {
        let measured = Estimate::after(
            Some(Estimate::Changed(milliseconds(40))),
            Estimate::Changed(milliseconds(440)),
        );

        assert_eq!(Estimate::Changed(milliseconds(140)), measured);
        assert_eq!(
            Estimate::Changed(milliseconds(440)),
            Estimate::after(
                Some(Estimate::Unchanged),
                Estimate::Changed(milliseconds(440))
            ),
            "a module that changes the prompt again is measured afresh"
        );
    }

    /// Any duration at all, as a module could take or a shell could report.
    fn any_span() -> impl Strategy<Value = Span> {
        any::<u64>().prop_map(|nanoseconds| Span::from(Duration::from_nanos(nanoseconds)))
    }

    proptest! {
        /// No timetable costs less than the one the bus leaves on, whatever
        /// is aboard, its patience and whenever the rest is expected, and none
        /// costing as little leaves sooner.
        #[test]
        fn the_bus_leaves_on_the_cheapest_timetable(
            passengers in prop_oneof![Just(Passengers::Prompt), Just(Passengers::Changes)],
            patience in 0_u64..80,
            landings in prop::collection::btree_map(0_u64..400, 0_u64..60, 0..9),
        ) {
            let landings: Vec<_> = landings
                .into_iter()
                .map(|(after, size)| Landing {
                    after: milliseconds(after).get(),
                    size: milliseconds(size),
                })
                .collect();
            let patience = milliseconds(patience).get();
            let leaves = cheapest(passengers, patience, &landings, &mut Vec::new());

            let timetables = every_timetable(passengers, patience, &landings);
            let cheapest_cost = timetables.iter().map(|&(_, cost)| cost).min();
            let soonest = timetables
                .iter()
                .filter(|&&(_, cost)| Some(cost) == cheapest_cost)
                .map(|&(leaves, _)| leaves)
                .min();
            prop_assert_eq!(soonest, Some(leaves));
        }

        /// Whatever durations come in, planning is total: nothing overflows,
        /// and the bus leaves no later than it waits, or than now if that is
        /// later.
        #[test]
        fn planning_is_total_over_every_duration(
            fallback in any_span(),
            since in any_span(),
            estimates in prop::collection::vec(prop::option::of(any_span()), 0..6),
        ) {
            let start = Instant::now();
            let now = start + since;
            let mut bus = Bus::new(start, fallback, estimates.len());
            let rendering: Vec<_> = estimates
                .iter()
                .map(|&took| Rendering {
                    since: now,
                    estimate: took.map(Estimate::Changed),
                })
                .collect();

            let departure = bus.departure(now, rendering.iter().copied());
            prop_assert!(departure.is_some_and(|departure| departure <= (start + fallback).max(now)));
            bus.leave();
            bus.board(now, since);
            prop_assert!(bus.departure(now, rendering).is_some_and(|departure| departure <= now + fallback));
        }

        /// Modules expected to land together, from renders started at
        /// different times, plan the same in whatever order they come.
        #[test]
        fn the_order_modules_come_in_never_matters(
            mut modules in prop::collection::vec((0_u64..4, 0_u64..4), 0..6),
            aboard in prop::option::of(0_u64..40),
        ) {
            let start = Instant::now();
            let now = start + milliseconds(4);
            let mut bus = Bus::new(start, FALLBACK, modules.len());
            if let Some(took) = aboard {
                bus.board(now, milliseconds(took));
            }
            // Each lands between 26 and 30 milliseconds in, from a render
            // started up to three in: the later it started, the less it took,
            // and the bigger the bus it opens.
            let rendering = |modules: &[(u64, u64)]| -> Vec<Rendering> {
                modules
                    .iter()
                    .map(|&(started, early)| Rendering {
                        since: start + milliseconds(started),
                        estimate: Some(Estimate::Changed(milliseconds(30 - started - early))),
                    })
                    .collect()
            };
            let forward = bus.departure(now, rendering(&modules));
            modules.reverse();
            prop_assert_eq!(forward, bus.departure(now, rendering(&modules)));
        }

        /// Blending any two durations never overflows, and lands between them.
        #[test]
        fn an_estimate_blends_between_the_last_and_the_new(
            previous in any_span(),
            measured in any_span(),
        ) {
            let Estimate::Changed(blended) = Estimate::after(
                Some(Estimate::Changed(previous)),
                Estimate::Changed(measured),
            ) else {
                unreachable!("two durations blend into one")
            };
            prop_assert!((previous.min(measured)..=previous.max(measured)).contains(&blended));
        }
    }
}
