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
//! the change aboard longest waits.
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

use std::time::{Duration, Instant};

/// Longer than any prompt waits for anything, which bounds every sum of
/// waiting the bus does.
pub const FOREVER: Duration = Duration::from_secs(60 * 60);

/// What a module did for the previous prompt.
///
/// Ordered as a stream renders modules when they outnumber its workers, those
/// that changed the prompt soonest first. A module never measured comes before
/// any of these, being `None`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Estimate {
    /// Changed the prompt, after rendering for this long.
    Changed(Duration),
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
                Self::Changed((estimate * 3 + took) / 4)
            }
            _ => measured,
        }
    }

    /// Whether a module expected to do `estimate` waits on a subprocess or the
    /// filesystem, rather than computes, which takes microseconds: one that
    /// took a millisecond or more to change the prompt, or one never measured,
    /// which might.
    pub fn waits(estimate: Option<Self>) -> bool {
        match estimate {
            Some(Self::Changed(took)) => took >= Duration::from_millis(1),
            Some(Self::Unchanged) => false,
            None => true,
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
    passengers: Passengers,
}

/// What is aboard a bus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Passengers {
    /// The prompt alone, which has nothing new to show until a change lands.
    Prompt,
    /// Changes, the prompt's among them if it is the first bus.
    Changes,
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
                passengers: Passengers::Prompt,
            }),
        }
    }

    /// A change boards at `now`, having taken `took` to render.
    pub fn board(&mut self, now: Instant, took: Duration) {
        let size = size(self.fallback, took);
        let aboard = self.aboard.get_or_insert(Aboard {
            since: now,
            size,
            passengers: Passengers::Changes,
        });
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
        &self,
        now: Instant,
        rendering: impl IntoIterator<Item = Rendering>,
    ) -> Option<Instant> {
        let aboard = self.aboard?;
        let latest = aboard.since + aboard.size;
        let mut landings = Vec::new();
        for module in rendering {
            match module.estimate {
                Some(Estimate::Unchanged) => {}
                Some(Estimate::Changed(took)) if module.since + took > now => {
                    landings.push(Landing {
                        after: module.since + took - now,
                        size: size(self.fallback, took),
                    });
                }
                // Could land at any moment.
                _ => return Some(latest),
            }
        }
        landings.sort_unstable_by_key(|landing| landing.after);
        Some(
            now + cheapest(
                aboard.passengers,
                latest.saturating_duration_since(now),
                &landings,
            ),
        )
    }
}

/// The size of a bus opened for a change that took `took` to render.
pub fn size(fallback: Duration, took: Duration) -> Duration {
    let fallback = fallback.as_nanos();
    let total = fallback + took.min(FOREVER).as_nanos();
    // A fallback of an hour squared still fits a u128 many times over.
    let nanoseconds = (fallback * fallback).checked_div(total).unwrap_or(0);
    Duration::from_nanos(u64::try_from(nanoseconds).unwrap_or(u64::MAX))
}

/// A change expected to land `after` from now, which opens a bus of `size`
/// unless one is waiting for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Landing {
    pub after: Duration,
    pub size: Duration,
}

/// How long from now a bus carrying `passengers`, which waits at most
/// `patience` longer, leaves on the cheapest timetable for the changes
/// `landings`, in the order they land: at once, or as the last of the next few
/// lands.
fn cheapest(passengers: Passengers, patience: Duration, landings: &[Landing]) -> Duration {
    // The cheapest timetable for landings[from..] once this bus has left:
    // each bus after it opens for a change, costs its size, and waits for
    // what lands within it.
    let mut rest = vec![Duration::ZERO; landings.len() + 1];
    for from in (0..landings.len()).rev() {
        let opened = landings[from];
        rest[from] = (from..landings.len())
            .map_while(|last| {
                let waited = landings[last].after - opened.after;
                (waited <= opened.size).then(|| opened.size + waited + rest[last + 1])
            })
            .min()
            .unwrap_or_default();
    }
    // Leaving now, or once landings[..=last] have; on equal cost, sooner.
    // The prompt alone leaves now only if nothing lands within its patience.
    let now = (passengers == Passengers::Changes).then_some((rest[0], Duration::ZERO));
    now.into_iter()
        .chain(landings.iter().enumerate().map_while(|(last, landing)| {
            (landing.after <= patience).then(|| (landing.after + rest[last + 1], landing.after))
        }))
        .min()
        .unwrap_or_default()
        .1
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use proptest::prelude::*;

    const FALLBACK: Duration = Duration::from_millis(50);

    fn milliseconds(milliseconds: u64) -> Duration {
        Duration::from_millis(milliseconds)
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
    fn the_first_paint_waits_for_what_would_otherwise_flash() {
        let start = Instant::now();
        let bus = Bus::new(start, FALLBACK);

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
            bus.departure(start, changing(start, &[1, 7, 300])),
        );
    }

    #[test]
    fn a_change_that_took_longer_waits_less_for_others() {
        let start = Instant::now();
        let at = |milliseconds: u64| start + Duration::from_millis(milliseconds);
        let mut early = Bus::new(start, FALLBACK);
        early.leave();
        early.board(at(10), milliseconds(10));
        let mut late = Bus::new(start, FALLBACK);
        late.leave();
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
        let bus = Bus::new(start, FALLBACK);
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
            bus.departure(start + milliseconds(20), changing(start, &[5])),
            "a module past its estimate could land at any moment"
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
        assert_eq!(None, Bus::empty(start).departure(start, [never_measured]));
    }

    #[test]
    fn no_change_waits_longer_than_its_bus() {
        let start = Instant::now();
        let bus = Bus::new(start, FALLBACK);

        assert_eq!(
            Some(start + milliseconds(40)),
            bus.departure(start, changing(start, &[40, 55, 70])),
            "the fifty-five is worth waiting for, but beyond the bus"
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

    #[test]
    fn modules_that_take_a_millisecond_or_were_never_measured_wait() {
        let waits = [
            Some(Estimate::Changed(Duration::from_micros(200))),
            Some(Estimate::Changed(milliseconds(1))),
            Some(Estimate::Unchanged),
            None,
        ]
        .map(Estimate::waits);

        assert_eq!([false, true, false, true], waits);
    }

    impl Bus {
        /// A bus that has drawn the first paint, with nothing aboard.
        fn empty(start: Instant) -> Self {
            let mut bus = Self::new(start, FALLBACK);
            bus.leave();
            bus
        }
    }

    /// What every timetable that keeps each bus within its size costs, with
    /// when its first bus leaves: a bus carrying `passengers` and waiting at
    /// most `patience` longer leaves first, with what lands by then, and the
    /// rest of `landings`, in order, ride buses that each open for a change,
    /// cost their size, and wait for whatever lands within it. Each bus costs,
    /// besides, the waiting of what is aboard it longest, which for the first
    /// bus is counted from now. The prompt alone does not leave before a change
    /// lands, unless none lands within its patience.
    pub(crate) fn every_timetable(
        passengers: Passengers,
        patience: Duration,
        landings: &[Landing],
    ) -> Vec<(Duration, Duration)> {
        let waits_for_a_change = passengers == Passengers::Prompt
            && landings
                .first()
                .is_some_and(|landing| landing.after <= patience);
        let count = landings.len();
        // The first bus leaves once landings[..first] have, which may be none
        // of them; every bus after it ends where `cuts` says.
        (0..=count)
            .flat_map(|first| {
                let later = count - first;
                (0..1_u32 << later.saturating_sub(1)).map(move |cuts| {
                    let ends =
                        (first + 1..count).filter(move |end| cuts >> (end - first - 1) & 1 == 1);
                    (
                        first,
                        ends.chain((later > 0).then_some(count)).collect::<Vec<_>>(),
                    )
                })
            })
            .filter_map(|(first, ends)| {
                let leaves = first
                    .checked_sub(1)
                    .map_or(Duration::ZERO, |last| landings[last].after);
                if leaves > patience || (first == 0 && waits_for_a_change) {
                    return None;
                }
                let mut cost = leaves;
                let mut from = first;
                for end in ends {
                    let opened = landings[from];
                    let waited = landings[end - 1].after - opened.after;
                    if waited > opened.size {
                        return None;
                    }
                    cost += opened.size + waited;
                    from = end;
                }
                Some((leaves, cost))
            })
            .collect()
    }

    fn landing() -> impl Strategy<Value = Landing> {
        (0_u64..400, 0_u64..60).prop_map(|(after, size)| Landing {
            after: milliseconds(after),
            size: milliseconds(size),
        })
    }

    proptest! {
        /// No timetable costs less than the one the bus leaves on, whatever
        /// is aboard, its patience and whenever the rest is expected, and none
        /// costing as little leaves sooner.
        #[test]
        fn the_bus_leaves_on_the_cheapest_timetable(
            passengers in prop_oneof![Just(Passengers::Prompt), Just(Passengers::Changes)],
            patience in 0_u64..80,
            mut landings in prop::collection::vec(landing(), 0..9),
        ) {
            landings.sort_unstable_by_key(|landing| landing.after);
            let patience = milliseconds(patience);
            let leaves = cheapest(passengers, patience, &landings);

            let timetables = every_timetable(passengers, patience, &landings);
            let cheapest_cost = timetables.iter().map(|&(_, cost)| cost).min();
            let soonest = timetables
                .iter()
                .filter(|&&(_, cost)| Some(cost) == cheapest_cost)
                .map(|&(leaves, _)| leaves)
                .min();
            prop_assert_eq!(soonest, Some(leaves));
        }
    }
}
