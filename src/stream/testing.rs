//! Every timetable there is, to hold the planner to: the slowest, surest way
//! to find the cheapest.

use std::time::Duration;

use super::bus::{Departure, Landing};

/// Every timetable that keeps each bus within its size, as when its first
/// bus leaves and what it costs: a bus carrying `count` changes and holding
/// `room` more waiting leaves first, with what lands by then, and the rest of
/// `landings`, in order, ride buses that each open for a change, cost their
/// size, and take whatever lands before what they carry has waited, in all,
/// longer than it. Each bus costs, besides, how long everything aboard waits,
/// which for the first bus is counted from now.
pub(super) fn every_timetable(count: u32, room: Duration, landings: &[Landing]) -> Vec<Departure> {
    let mut timetables = Vec::new();
    for first in 0..=landings.len() {
        let leaves = first
            .checked_sub(1)
            .map_or(Duration::ZERO, |last| landings[last].after);
        let waited = leaves * count + waiting(&landings[..first], leaves);
        if waited > room {
            break;
        }
        later_buses(&landings[first..], leaves, waited, &mut timetables);
    }
    timetables
}

/// Every way `landings` can ride the buses after the first, which leaves at
/// `first`, with what each such timetable costs on top of `cost`.
fn later_buses(
    landings: &[Landing],
    first: Duration,
    cost: Duration,
    timetables: &mut Vec<Departure>,
) {
    let Some(opened) = landings.first() else {
        timetables.push(Departure { at: first, cost });
        return;
    };
    for (last, landing) in landings.iter().enumerate() {
        let waited = waiting(&landings[..=last], landing.after);
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

/// How long the changes of `landings`, each aboard once it lands, have waited
/// in all when their bus leaves, `leaves` from now.
fn waiting(landings: &[Landing], leaves: Duration) -> Duration {
    landings
        .iter()
        .map(|landing| (leaves - landing.after) * landing.count)
        .sum()
}
