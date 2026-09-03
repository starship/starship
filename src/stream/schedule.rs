//! Predicted refinement batches.

use std::time::Duration;

use super::bus::BusWindow;

/// A module expected to finish after this much elapsed time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PredictedArrival(Duration);

impl PredictedArrival {
    #[must_use]
    pub const fn after(elapsed: Duration) -> Self {
        Self(elapsed)
    }

    const fn elapsed(self) -> Duration {
        self.0
    }
}

/// The portion of a prediction relevant to the next actual arrival.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ExpectedBatch {
    pub(crate) closes: Duration,
    pub(crate) has_later_arrival: bool,
}

/// The minimal partition of predicted arrivals into latency-bounded batches.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArrivalSchedule(Vec<Batch>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Batch {
    closes: Duration,
    arrivals: Vec<PredictedArrival>,
}

impl ArrivalSchedule {
    #[must_use]
    pub fn of(arrivals: impl IntoIterator<Item = PredictedArrival>, window: BusWindow) -> Self {
        let mut arrivals: Vec<_> = arrivals.into_iter().collect();
        arrivals.sort_unstable();

        let mut batches: Vec<Batch> = Vec::new();
        for arrival in arrivals {
            match batches.last_mut() {
                Some(batch) if arrival.elapsed() <= batch.closes => batch.arrivals.push(arrival),
                _ => batches.push(Batch {
                    closes: arrival.elapsed().saturating_add(window.duration()),
                    arrivals: vec![arrival],
                }),
            }
        }
        Self(batches)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn expected_batch_at(&self, elapsed: Duration) -> Option<ExpectedBatch> {
        let batch = self.0.iter().find(|batch| batch.closes >= elapsed)?;
        Some(ExpectedBatch {
            closes: batch.closes,
            has_later_arrival: batch
                .arrivals
                .iter()
                .filter(|arrival| arrival.elapsed() >= elapsed)
                .nth(1)
                .is_some(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW: BusWindow = BusWindow::from_milliseconds(100);

    fn schedule(milliseconds: &[u64]) -> ArrivalSchedule {
        ArrivalSchedule::of(
            milliseconds
                .iter()
                .map(|milliseconds| PredictedArrival::after(Duration::from_millis(*milliseconds))),
            WINDOW,
        )
    }

    #[test]
    fn greedily_groups_every_arrival_within_the_latency_budget() {
        let schedule = schedule(&[150, 10, 40, 111]);
        assert_eq!(
            Some(ExpectedBatch {
                closes: Duration::from_millis(110),
                has_later_arrival: true,
            }),
            schedule.expected_batch_at(Duration::from_millis(10))
        );
        assert_eq!(
            Some(ExpectedBatch {
                closes: Duration::from_millis(211),
                has_later_arrival: false,
            }),
            schedule.expected_batch_at(Duration::from_millis(150))
        );
    }

    #[test]
    fn expired_predictions_are_ignored() {
        assert_eq!(
            None,
            schedule(&[10]).expected_batch_at(Duration::from_millis(111))
        );
    }
}
