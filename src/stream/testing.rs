//! Enumerate valid schedules independently of the production planner.

use std::time::Duration;

use super::bus::Passengers;

#[derive(Clone, Copy, Debug)]
pub(super) struct Landing {
    pub(super) after: Duration,
    pub(super) size: Duration,
}

/// Enumerate complete valid schedules independently of the optimized planner.
/// Each result is (first departure, total remaining cost).
pub(super) fn every_timetable(
    passengers: Passengers,
    patience: Duration,
    landings: &[Landing],
) -> Vec<(Duration, Duration)> {
    let waits_for_change = passengers == Passengers::Prompt
        && landings
            .first()
            .is_some_and(|landing| landing.after <= patience);
    let minimum_changes = usize::from(waits_for_change);
    let mut timetables = Vec::new();

    for included_count in minimum_changes..=landings.len() {
        let departure = if included_count == 0 {
            Duration::ZERO
        } else {
            landings[included_count - 1].after
        };
        if departure > patience {
            break;
        }
        collect_remaining_timetables(
            &landings[included_count..],
            departure,
            departure,
            &mut timetables,
        );
    }
    timetables
}

fn collect_remaining_timetables(
    landings: &[Landing],
    first_departure: Duration,
    accumulated_cost: Duration,
    timetables: &mut Vec<(Duration, Duration)>,
) {
    let Some(opening_change) = landings.first() else {
        timetables.push((first_departure, accumulated_cost));
        return;
    };

    for (last_index, last_change) in landings.iter().enumerate() {
        let waited = last_change.after - opening_change.after;
        if waited > opening_change.size {
            break;
        }
        collect_remaining_timetables(
            &landings[last_index + 1..],
            first_departure,
            accumulated_cost + opening_change.size + waited,
            timetables,
        );
    }
}
