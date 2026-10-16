//! Every timetable there is, to hold the planner to: the slowest, surest way
//! to find the cheapest.

use std::time::Duration;

use super::bus::{Landing, Passengers};

/// What every timetable that keeps each bus within its size costs, with when
/// its first bus leaves: a bus carrying `passengers` and waiting at most
/// `patience` longer leaves first, with what lands by then, and the rest of
/// `landings`, in order, ride buses that each open for a change, cost their
/// size, and wait for whatever lands within it. Each bus costs, besides, the
/// waiting of what is aboard it longest, which for the first bus is counted
/// from now. The prompt alone does not leave before a change lands, unless
/// none lands within its patience.
pub(super) fn every_timetable(
    passengers: Passengers,
    patience: Duration,
    landings: &[Landing],
) -> Vec<(Duration, Duration)> {
    let waits_for_a_change = passengers == Passengers::Prompt
        && landings
            .first()
            .is_some_and(|landing| landing.after <= patience);
    let mut timetables = Vec::new();
    for first in usize::from(waits_for_a_change)..=landings.len() {
        let leaves = first
            .checked_sub(1)
            .map_or(Duration::ZERO, |last| landings[last].after);
        if leaves > patience {
            break;
        }
        later_buses(&landings[first..], leaves, leaves, &mut timetables);
    }
    timetables
}

/// Every way `landings` can ride the buses after the first, which leaves at
/// `first`, with what each such timetable costs on top of `cost`.
fn later_buses(
    landings: &[Landing],
    first: Duration,
    cost: Duration,
    timetables: &mut Vec<(Duration, Duration)>,
) {
    let Some(opened) = landings.first() else {
        timetables.push((first, cost));
        return;
    };
    for (last, landing) in landings.iter().enumerate() {
        let waited = landing.after - opened.after;
        if waited > opened.size.get() {
            break;
        }
        later_buses(
            &landings[last + 1..],
            first,
            cost + opened.size.get() + waited,
            timetables,
        );
    }
}
