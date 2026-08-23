//! What each module has been costing, remembered by the shell between prompts.
//!
//! Kept in a shell variable, not a file: these estimates only ever affect when
//! refinements are batched, never what is drawn, so a stale one costs nothing
//! but extra repaints and needs no invalidation story.

use std::convert::Infallible;
use std::num::NonZeroU64;
use std::time::Duration;

use crate::frame::{Microseconds, Timings};

/// Integer weight (one part in `parts`) so blending never needs float rounding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmoothingWeight(NonZeroU64);

impl SmoothingWeight {
    #[must_use]
    pub const fn one_part_in(parts: NonZeroU64) -> Self {
        Self(parts)
    }

    /// `estimate` moved one step towards `measured`.
    fn blend(self, estimate: Microseconds, measured: Microseconds) -> Microseconds {
        let parts = u128::from(self.0.get());
        // In `u128` because a `u64` of microseconds is over half a million
        // years and multiplying two of them is not: the arithmetic below cannot
        // overflow, so it needs no saturating behaviour to reason about.
        let blended = (u128::from(measured.0) + u128::from(estimate.0) * (parts - 1)) / parts;
        Microseconds(u64::try_from(blended).unwrap_or(u64::MAX))
    }
}

/// The weight the newest measurement carries.
///
/// A quarter. Three prompts of a genuinely slower machine move the estimate
/// most of the way there, which is fast enough to follow a change of working
/// directory into a large repository, while one anomalous prompt moves it by a
/// quarter and is undone by the next ordinary one.
const NEWEST_MEASUREMENT_WEIGHT: SmoothingWeight =
    SmoothingWeight::one_part_in(NonZeroU64::new(4).expect("four is not zero"));

/// The session's running estimate of what each module costs.
///
/// Empty on the first prompt of a session, and empty for a shell whose
/// transport does not carry the estimates back. Both are ordinary: see
/// [`Self::is_empty`], whose callers fall back to a fixed bus window.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LatencyEstimates(Timings);

impl LatencyEstimates {
    /// No estimates at all: the first prompt of a session.
    #[must_use]
    pub fn none() -> Self {
        Self::default()
    }

    /// The estimates a `--timings` argument carries.
    ///
    /// Never fails. The argument is a value this process wrote itself and a
    /// shell handed back, so anything malformed in it is a bug in the transport
    /// — but the estimates are advisory, and refusing to draw a prompt because
    /// an advisory input was truncated would turn a slightly worse repaint
    /// schedule into no prompt at all.
    ///
    /// # Errors
    ///
    /// None ever; the signature exists so that this can be a `clap` value
    /// parser.
    pub fn parse_argument(argument: &str) -> Result<Self, Infallible> {
        if argument.trim().is_empty() {
            return Ok(Self::none());
        }
        match Timings::from_json(argument.as_bytes()) {
            Some(timings) => Ok(Self(timings)),
            None => {
                log::warn!("Ignoring a --timings argument that is not a timing payload");
                Ok(Self::none())
            }
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// How long `module` is expected to take, if it has ever been measured.
    #[must_use]
    pub fn of(&self, module: &str) -> Option<Duration> {
        self.0.get(module).map(Microseconds::duration)
    }

    /// These estimates brought up to date with what this prompt measured.
    ///
    /// A module measured this time moves a quarter of the way towards what it
    /// cost; a module that was not measured — one this prompt does not show, or
    /// one whose slot the format string dropped — keeps the estimate it had.
    /// Keeping it is what makes a prompt in a repository still remember what
    /// `git_status` costs after a few prompts spent outside one.
    #[must_use]
    pub fn updated_with(&self, measured: &Timings) -> Self {
        let mut updated = self.0.clone();
        for (module, cost) in measured.iter() {
            let blended = match self.0.get(module) {
                Some(estimate) => NEWEST_MEASUREMENT_WEIGHT.blend(estimate, cost),
                // Nothing to smooth against: the first measurement *is* the
                // estimate.
                None => cost,
            };
            updated.set(module, blended);
        }
        Self(updated)
    }

    /// The payload to hand back to the shell.
    #[must_use]
    pub fn timings(&self) -> &Timings {
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
            timings.set(module, Microseconds(*microseconds));
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

        assert_eq!(
            Microseconds(400),
            weight.blend(Microseconds(100), Microseconds(400))
        );
    }
}
