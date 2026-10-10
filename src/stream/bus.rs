//! Schedule redraws while prompt modules render concurrently.
//!
//! A pending batch has a fixed deadline. Its redraw cost equals its maximum
//! wait, and its waiting cost is the delay of its oldest change. Slower changes
//! open smaller batches: `fallback² / (fallback + render_duration)`.
//!
//! Predictions determine the cheapest sequence of future redraws, with earlier
//! departures winning ties. Unknown or overdue predictions use the pending
//! deadline. The initial prompt waits for a predicted change when one fits;
//! otherwise it can be drawn immediately.
//!
//! Planning takes O(n log n) time, including sorting, and one O(n) working
//! buffer. Reuse a [`DeparturePlanner`] to retain that buffer between calls.

use std::time::{Duration, Instant};

/// The upper bound for an individual batch's wait and for sizing measurements.
pub const MAXIMUM_BATCH_WAIT: Duration = Duration::from_secs(60 * 60);

/// The existing public name for [`MAXIMUM_BATCH_WAIT`].
pub const FOREVER: Duration = MAXIMUM_BATCH_WAIT;

const PREVIOUS_ESTIMATE_WEIGHT: u128 = 3;
const MEASUREMENT_WEIGHT: u128 = 1;
const TOTAL_ESTIMATE_WEIGHT: u128 = PREVIOUS_ESTIMATE_WEIGHT + MEASUREMENT_WEIGHT;
const MINIMUM_WAITING_DURATION: Duration = Duration::from_millis(1);
const NANOSECONDS_PER_SECOND: u128 = 1_000_000_000;

/// A module's behavior on the previous prompt.
///
/// The variant order schedules changing modules first, shortest duration first.
/// An unmeasured module, represented by `None`, sorts before either variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Estimate {
    /// Changed the prompt after rendering for this duration.
    Changed(Duration),
    /// Produced nothing new, including producing no output at all.
    Unchanged,
}

impl Estimate {
    /// Update an estimate, giving a new duration one quarter of the weight.
    pub fn after(previous: Option<Self>, measured: Self) -> Self {
        match (previous, measured) {
            (Some(Self::Changed(previous)), Self::Changed(measured)) => {
                Self::Changed(smooth_duration(previous, measured))
            }
            _ => measured,
        }
    }

    /// Whether a module may wait on external work rather than compute briefly.
    pub fn waits(estimate: Option<Self>) -> bool {
        match estimate {
            Some(Self::Changed(duration)) => duration >= MINIMUM_WAITING_DURATION,
            Some(Self::Unchanged) => false,
            None => true,
        }
    }
}

fn smooth_duration(previous: Duration, measured: Duration) -> Duration {
    // The weighted sum fits u128 even when both durations are Duration::MAX.
    let nanoseconds = (previous.as_nanos() * PREVIOUS_ESTIMATE_WEIGHT
        + measured.as_nanos() * MEASUREMENT_WEIGHT)
        / TOTAL_ESTIMATE_WEIGHT;

    Duration::new(
        (nanoseconds / NANOSECONDS_PER_SECOND) as u64,
        (nanoseconds % NANOSECONDS_PER_SECOND) as u32,
    )
}

/// A module that is still rendering.
#[derive(Clone, Copy, Debug)]
pub struct Rendering {
    /// When this module started rendering.
    pub since: Instant,
    /// What it did on the previous prompt, if measured.
    pub estimate: Option<Estimate>,
}

impl Rendering {
    /// A strictly future completion; due, overdue, or unrepresentable is unknown.
    fn remaining_duration(self, now: Instant, render_duration: Duration) -> Option<Duration> {
        self.since
            .checked_add(render_duration)?
            .checked_duration_since(now)
            .filter(|remaining| !remaining.is_zero())
    }
}

/// Collect changes into batches and choose when to draw them.
pub struct Bus {
    fallback: Duration,
    pending: Option<PendingBatch>,
}

#[derive(Clone, Copy, Debug)]
struct PendingBatch {
    started: Instant,
    maximum_wait: Duration,
    passengers: Passengers,
}

impl PendingBatch {
    fn deadline(self) -> Instant {
        self.started + self.maximum_wait
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Passengers {
    Prompt,
    Changes,
}

impl Bus {
    /// Start with the prompt waiting for its first paint.
    pub fn new(start: Instant, fallback: Duration) -> Self {
        let fallback = fallback.min(MAXIMUM_BATCH_WAIT);
        Self {
            fallback,
            pending: Some(PendingBatch {
                started: start,
                maximum_wait: fallback,
                passengers: Passengers::Prompt,
            }),
        }
    }

    /// Queue a change, retaining an existing batch's start and maximum wait.
    pub fn board(&mut self, now: Instant, render_duration: Duration) {
        let fallback = self.fallback;
        let pending = self.pending.get_or_insert_with(|| PendingBatch {
            started: now,
            maximum_wait: size(fallback, render_duration),
            passengers: Passengers::Changes,
        });
        pending.passengers = Passengers::Changes;
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_none()
    }

    /// Mark the pending batch as drawn.
    pub fn leave(&mut self) {
        self.pending = None;
    }

    /// Choose a departure using temporary planning storage.
    #[cfg(test)]
    pub fn departure(
        &self,
        now: Instant,
        rendering: impl IntoIterator<Item = Rendering>,
    ) -> Option<Instant> {
        self.departure_with_planner(now, rendering, &mut DeparturePlanner::with_capacity(0))
    }

    /// Choose a departure while retaining the planner's allocation for reuse.
    pub fn departure_with_planner(
        &self,
        now: Instant,
        rendering: impl IntoIterator<Item = Rendering>,
        planner: &mut DeparturePlanner,
    ) -> Option<Instant> {
        let pending = self.pending?;
        let deadline = pending.deadline();
        let Some(predictions) = planner.predictions(now, self.fallback, rendering) else {
            return Some(deadline);
        };

        let wait = cheapest_departure(
            pending.passengers,
            deadline.saturating_duration_since(now),
            predictions,
        );
        Some(now + wait)
    }
}

/// Calculate a batch's maximum wait from its opening change's render duration.
pub fn size(fallback: Duration, render_duration: Duration) -> Duration {
    if fallback.is_zero() {
        return Duration::ZERO;
    }

    let fallback_nanoseconds = fallback.as_nanos();
    let render_nanoseconds = render_duration.min(MAXIMUM_BATCH_WAIT).as_nanos();
    let total_nanoseconds = fallback_nanoseconds + render_nanoseconds;

    // Square only the smaller operand, which is bounded by MAXIMUM_BATCH_WAIT.
    // f² / (f + r) = f - r + r² / (f + r) when f >= r.
    let window_nanoseconds = if fallback_nanoseconds >= render_nanoseconds {
        fallback_nanoseconds - render_nanoseconds
            + render_nanoseconds * render_nanoseconds / total_nanoseconds
    } else {
        fallback_nanoseconds * fallback_nanoseconds / total_nanoseconds
    };

    // Preserve the public function's existing u64-nanosecond output limit.
    Duration::from_nanos(window_nanoseconds.min(u128::from(u64::MAX)) as u64)
}

/// Reusable scratch space. Once capacity is sufficient, planning allocates nothing.
#[derive(Default)]
pub struct DeparturePlanner {
    entries: Vec<TimedCost>,
}

impl DeparturePlanner {
    pub fn with_capacity(module_count: usize) -> Self {
        Self {
            entries: Vec::with_capacity(module_count),
        }
    }

    /// Collect sorted predictions, or return None when a change is unpredictable.
    fn predictions(
        &mut self,
        now: Instant,
        fallback: Duration,
        rendering: impl IntoIterator<Item = Rendering>,
    ) -> Option<&mut [TimedCost]> {
        self.entries.clear();
        for module in rendering {
            match module.estimate {
                Some(Estimate::Unchanged) => {}
                Some(Estimate::Changed(render_duration)) => {
                    self.entries.push(TimedCost {
                        after: module.remaining_duration(now, render_duration)?,
                        cost_nanoseconds: size(fallback, render_duration).as_nanos(),
                    });
                }
                None => return None,
            }
        }
        self.entries.sort_unstable_by_key(|entry| entry.after);
        Some(&mut self.entries)
    }
}

/// A prediction starts with a redraw cost; planning replaces it with a timetable
/// cost. Keeping the same representation lets the processed suffix be reused.
/// Wide costs allow a delay plus future costs to exceed Duration::MAX safely.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TimedCost {
    after: Duration,
    cost_nanoseconds: u128,
}

/// Transform sorted predictions into undominated departure candidates in place.
fn cheapest_departure(
    passengers: Passengers,
    patience: Duration,
    entries: &mut [TimedCost],
) -> Duration {
    // Waiting can help only if a predicted change fits the current batch.
    if !entries
        .first()
        .is_some_and(|prediction| prediction.after <= patience)
    {
        return Duration::ZERO;
    }

    let mut frontier_start = entries.len();
    let mut remaining_cost_nanoseconds = 0_u128;

    for index in (0..entries.len()).rev() {
        let prediction = entries[index];
        let departure = TimedCost {
            after: prediction.after,
            cost_nanoseconds: prediction.after.as_nanos() + remaining_cost_nanoseconds,
        };

        frontier_start = insert_departure(entries, frontier_start, departure);
        let cheapest = cheapest_within(
            &entries[frontier_start..],
            prediction.after,
            prediction.cost_nanoseconds,
        )
        .expect("the newly inserted departure is within its own batch");
        remaining_cost_nanoseconds =
            prediction.cost_nanoseconds + (cheapest.cost_nanoseconds - prediction.after.as_nanos());
    }

    choose_first_departure(
        passengers,
        patience,
        remaining_cost_nanoseconds,
        &entries[frontier_start..],
    )
}

/// Earlier departures with equal or lower cost make later ones unnecessary.
fn insert_departure(
    entries: &mut [TimedCost],
    mut frontier_start: usize,
    departure: TimedCost,
) -> usize {
    while entries
        .get(frontier_start)
        .is_some_and(|existing| existing.cost_nanoseconds >= departure.cost_nanoseconds)
    {
        frontier_start += 1;
    }

    // There is always a free slot: only the already-read suffix is overwritten.
    frontier_start -= 1;
    entries[frontier_start] = departure;
    frontier_start
}

/// Frontier times increase and costs strictly decrease: the last eligible wins.
fn cheapest_within(
    frontier: &[TimedCost],
    batch_start: Duration,
    maximum_wait_nanoseconds: u128,
) -> Option<TimedCost> {
    let eligible_count = frontier.partition_point(|departure| {
        (departure.after - batch_start).as_nanos() <= maximum_wait_nanoseconds
    });
    eligible_count.checked_sub(1).map(|index| frontier[index])
}

fn choose_first_departure(
    passengers: Passengers,
    patience: Duration,
    immediate_cost_nanoseconds: u128,
    frontier: &[TimedCost],
) -> Duration {
    let Some(departure) = cheapest_within(frontier, Duration::ZERO, patience.as_nanos()) else {
        return Duration::ZERO;
    };

    if passengers == Passengers::Changes && immediate_cost_nanoseconds <= departure.cost_nanoseconds
    {
        Duration::ZERO
    } else {
        departure.after
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::testing::{Landing, every_timetable};
    use proptest::prelude::*;

    const FALLBACK: Duration = Duration::from_millis(50);

    fn plan_departure(
        passengers: Passengers,
        patience: Duration,
        landings: &[Landing],
    ) -> Duration {
        let mut entries: Vec<_> = landings
            .iter()
            .map(|landing| TimedCost {
                after: landing.after,
                cost_nanoseconds: landing.size.as_nanos(),
            })
            .collect();
        cheapest_departure(passengers, patience, &mut entries)
    }

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

    #[test]
    fn smoothing_preserves_nanosecond_rounding_and_handles_maximum_durations() {
        for (previous, measured, expected) in [(10, 9, 9), (9, 10, 9), (1, 0, 0)] {
            assert_eq!(
                Estimate::Changed(Duration::from_nanos(expected)),
                Estimate::after(
                    Some(Estimate::Changed(Duration::from_nanos(previous))),
                    Estimate::Changed(Duration::from_nanos(measured)),
                ),
            );
        }
        assert_eq!(
            Estimate::Changed(Duration::MAX),
            Estimate::after(
                Some(Estimate::Changed(Duration::MAX)),
                Estimate::Changed(Duration::MAX),
            ),
        );
    }

    #[test]
    fn public_sizing_keeps_large_fallbacks_without_overflow() {
        let larger_fallback = MAXIMUM_BATCH_WAIT * 2;
        assert_eq!(larger_fallback, size(larger_fallback, Duration::ZERO));
        assert_eq!(
            Duration::from_nanos(u64::MAX),
            size(Duration::MAX, Duration::MAX),
        );
        assert_eq!(Duration::ZERO, size(Duration::ZERO, Duration::MAX));
    }

    #[test]
    fn boarding_an_occupied_batch_keeps_its_deadline() {
        let start = Instant::now();
        let mut bus = Bus::new(start, FALLBACK);
        let unknown = Rendering {
            since: start,
            estimate: None,
        };

        bus.board(start + milliseconds(10), milliseconds(300));
        bus.board(start + milliseconds(30), milliseconds(1));

        assert_eq!(
            Some(start + FALLBACK),
            bus.departure(start + milliseconds(30), [unknown]),
        );
    }

    #[test]
    fn due_predictions_and_expired_deadlines_keep_the_existing_policy() {
        let start = Instant::now();
        let bus = Bus::new(start, FALLBACK);

        assert_eq!(
            Some(start + FALLBACK),
            bus.departure(start + milliseconds(20), changing(start, &[20])),
            "an estimate exactly due is unpredictable",
        );
        let after_deadline = start + milliseconds(80);
        assert_eq!(
            Some(start + FALLBACK),
            bus.departure(after_deadline, changing(start, &[20])),
            "an overdue module still returns the original deadline",
        );
        assert_eq!(
            Some(after_deadline),
            bus.departure(after_deadline, []),
            "without uncertain modules an expired batch departs now",
        );
    }

    #[test]
    fn future_module_start_times_contribute_to_the_prediction() {
        let now = Instant::now();
        let bus = Bus::new(now, FALLBACK);
        let future = Rendering {
            since: now + milliseconds(5),
            estimate: Some(Estimate::Changed(milliseconds(10))),
        };

        assert_eq!(Some(now + milliseconds(15)), bus.departure(now, [future]));
    }

    #[test]
    fn equal_arrival_order_and_earliest_departure_ties_are_preserved() {
        let predictions = [(1, 10), (1, 1), (2, 1)].map(|(after, size)| Landing {
            after: milliseconds(after),
            size: milliseconds(size),
        });
        assert_eq!(
            milliseconds(1),
            plan_departure(Passengers::Changes, milliseconds(1), &predictions),
        );
        let reordered = [predictions[1], predictions[0], predictions[2]];
        assert_eq!(
            Duration::ZERO,
            plan_departure(Passengers::Changes, milliseconds(1), &reordered),
        );
    }

    #[test]
    fn large_arrival_offsets_do_not_overflow_timetable_costs() {
        let distant_batch_wait = Duration::from_secs(1_800);
        let predictions = [
            Landing {
                after: milliseconds(10),
                size: FALLBACK,
            },
            Landing {
                after: Duration::from_secs(u64::MAX - 1),
                size: distant_batch_wait,
            },
            Landing {
                after: Duration::from_secs(u64::MAX),
                size: distant_batch_wait,
            },
        ];

        assert_eq!(
            milliseconds(10),
            plan_departure(Passengers::Changes, FALLBACK, &predictions),
        );
    }

    #[test]
    fn a_reused_planner_discards_previous_predictions() {
        let now = Instant::now();
        let bus = Bus::new(now, FALLBACK);
        let modules = changing(now, &[1, 7, 16]);
        let mut planner = DeparturePlanner::with_capacity(modules.len());
        let unknown = Rendering {
            since: now,
            estimate: None,
        };

        for (rendering, expected) in [
            (modules.clone(), milliseconds(16)),
            (vec![modules[0], unknown], FALLBACK),
            (changing(now, &[5]), milliseconds(5)),
            (Vec::new(), Duration::ZERO),
        ] {
            assert_eq!(
                Some(now + expected),
                bus.departure_with_planner(now, rendering, &mut planner),
            );
        }
    }

    impl Bus {
        /// A bus that has drawn the first paint, with nothing aboard.
        fn empty(start: Instant) -> Self {
            let mut bus = Self::new(start, FALLBACK);
            bus.leave();
            bus
        }
    }

    fn landing() -> impl Strategy<Value = Landing> {
        (0_u64..400, 0_u64..60).prop_map(|(after, size)| Landing {
            after: milliseconds(after),
            size: milliseconds(size),
        })
    }

    proptest! {
        #[test]
        fn the_bus_leaves_on_the_cheapest_timetable(
            passengers in prop_oneof![Just(Passengers::Prompt), Just(Passengers::Changes)],
            patience in 0_u64..80,
            mut landings in prop::collection::vec(landing(), 0..9),
        ) {
            landings.sort_unstable_by_key(|landing| landing.after);
            let patience = milliseconds(patience);
            let leaves = plan_departure(passengers, patience, &landings);

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
