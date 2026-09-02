//! What each module has been costing, remembered by the shell between prompts.
//!
//! Kept in a shell variable, not a file: these estimates only ever affect when
//! refinements are batched, never what is drawn, so a stale one costs nothing
//! but extra repaints and needs no invalidation story.

use std::convert::Infallible;
use std::num::NonZeroU64;
use std::time::Duration;

use crate::frame::Timings;

const DEFAULT_SMOOTHING_PARTS: u64 = 4;
const ESTIMATE_WEIGHT_OFFSET: u64 = 1;

/// Integer weight (one part in `parts`) so blending never needs float rounding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmoothingWeight(NonZeroU64);

impl SmoothingWeight {
    #[must_use]
    pub const fn one_part_in(parts: NonZeroU64) -> Self {
        Self(parts)
    }

    #[must_use]
    fn blend(self, estimate: u64, measured: u64) -> u64 {
        let total_parts = u128::from(self.0.get());
        let previous_estimate_weight = total_parts - u128::from(ESTIMATE_WEIGHT_OFFSET);

        // u128 avoids overflow without saturating.
        let blended_microseconds =
            (u128::from(measured) + u128::from(estimate) * previous_estimate_weight) / total_parts;
        u64::try_from(blended_microseconds).unwrap_or(u64::MAX)
    }
}

const NEWEST_MEASUREMENT_WEIGHT: SmoothingWeight = SmoothingWeight::one_part_in(
    NonZeroU64::new(DEFAULT_SMOOTHING_PARTS).expect("smoothing parts must be non-zero"),
);

/// The session's running estimate of what each module costs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LatencyEstimates(Timings);

impl LatencyEstimates {
    /// No estimates at all: the first prompt of a session.
    #[must_use]
    pub const fn none() -> Self {
        Self(Timings::empty())
    }

    /// Parses the argument provided via the `--timings` command line flag.
    pub fn parse_argument(argument: &str) -> Result<Self, Infallible> {
        if argument.trim().is_empty() {
            return Ok(Self::none());
        }

        let parsed_timings = Timings::from_json(argument.as_bytes()).unwrap_or_else(|| {
            log::warn!("Ignoring a --timings argument that is not a timing payload");
            Timings::default()
        });

        Ok(Self(parsed_timings))
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// How long a specific module is expected to take, if recorded.
    #[must_use]
    pub fn of(&self, module_name: &str) -> Option<Duration> {
        self.0.get(module_name).map(Duration::from_micros)
    }

    /// Folds `measured_timings` in; a module missing from it keeps its prior estimate.
    #[must_use]
    pub fn updated_with(&self, measured_timings: &Timings) -> Self {
        let mut updated_timings = self.0.clone();

        for (module_name, measurement_cost) in measured_timings.iter() {
            let resolved_cost = self
                .0
                .get(module_name)
                .map_or(measurement_cost, |existing| {
                    NEWEST_MEASUREMENT_WEIGHT.blend(existing, measurement_cost)
                });
            updated_timings.set(module_name, resolved_cost);
        }

        Self(updated_timings)
    }

    /// The timings to hand back to the shell.
    #[must_use]
    pub const fn timings(&self) -> &Timings {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::ServerEvent;

    /// The `timings` payload bytes of a `ServerEvent::Complete` frame, as handed back as `--timings=<this>`.
    fn complete_payload(timings: &Timings) -> Vec<u8> {
        let mut written = Vec::new();
        ServerEvent::Complete(timings.clone())
            .write_to(&mut written)
            .expect("writing to a vector cannot fail");
        let keyword_length = written
            .iter()
            .position(|&byte| byte == 0)
            .expect("a NUL-terminated keyword field")
            + 1;
        let payload_length = written[keyword_length..]
            .iter()
            .position(|&byte| byte == 0)
            .expect("a NUL-terminated timings field");
        written[keyword_length..keyword_length + payload_length].to_vec()
    }

    /// Timings holding one measurement per named module.
    fn measured(measurements: &[(&str, u64)]) -> Timings {
        let mut timings = Timings::default();
        for (module, microseconds) in measurements {
            timings.set(module, *microseconds);
        }
        timings
    }

    #[test]
    fn a_session_with_nothing_measured_yet_has_no_estimates() {
        let estimates = LatencyEstimates::none();

        assert!(estimates.is_empty());
        assert_eq!(None, estimates.of("git_status"));
    }

    #[test]
    fn the_first_measurement_of_a_module_becomes_its_estimate() {
        let estimates = LatencyEstimates::none().updated_with(&measured(&[("git_status", 40_000)]));

        assert_eq!(Some(Duration::from_millis(40)), estimates.of("git_status"));
    }

    #[test]
    fn one_slow_prompt_does_not_take_the_estimate_with_it() {
        let settled = LatencyEstimates::none().updated_with(&measured(&[("git_status", 40_000)]));
        // Ten times as slow, once.
        let disturbed = settled.updated_with(&measured(&[("git_status", 400_000)]));

        let estimate = disturbed.of("git_status").expect("the module was measured");
        assert!(
            estimate < Duration::from_millis(140),
            "one outlier moved the estimate to {estimate:?}"
        );
        assert!(
            estimate > Duration::from_millis(40),
            "the outlier must still move the estimate at all, but it is {estimate:?}"
        );
    }

    #[test]
    fn a_machine_that_really_did_get_slower_is_followed() {
        let mut estimates =
            LatencyEstimates::none().updated_with(&measured(&[("git_status", 10_000)]));
        for _ in 0..8 {
            estimates = estimates.updated_with(&measured(&[("git_status", 200_000)]));
        }

        let estimate = estimates.of("git_status").expect("the module was measured");
        assert!(
            estimate > Duration::from_millis(150),
            "eight slow prompts should have moved the estimate, but it is {estimate:?}"
        );
    }

    #[test]
    fn a_module_this_prompt_did_not_run_keeps_its_estimate() {
        let estimates = LatencyEstimates::none()
            .updated_with(&measured(&[("git_status", 40_000), ("rust", 8_000)]))
            .updated_with(&measured(&[("rust", 8_000)]));

        assert_eq!(Some(Duration::from_millis(40)), estimates.of("git_status"));
    }

    #[test]
    fn the_estimates_survive_a_round_trip_through_the_shell() {
        let estimates = LatencyEstimates::none().updated_with(&measured(&[
            ("git_status", 40_000),
            ("custom.slow", 900_000),
        ]));

        // Mirrors the shell: keep the completion payload, hand it back as the next argument.
        let payload = complete_payload(estimates.timings());
        let variable = String::from_utf8(payload).expect("the payload is text");
        let handed_back = LatencyEstimates::parse_argument(&variable).expect("parsing never fails");

        assert_eq!(estimates, handed_back);
    }

    #[test]
    fn an_argument_that_is_not_a_payload_leaves_the_prompt_with_no_estimates() {
        for argument in ["", "   ", "not json", "{", "[1, 2, 3]"] {
            let estimates =
                LatencyEstimates::parse_argument(argument).expect("parsing never fails");
            assert!(
                estimates.is_empty(),
                "{argument:?} should have produced no estimates"
            );
        }
    }

    #[test]
    fn no_smoothing_at_all_takes_the_newest_measurement() {
        let weight = SmoothingWeight::one_part_in(NonZeroU64::new(1).expect("one is not zero"));

        assert_eq!(400, weight.blend(100, 400));
    }
}
