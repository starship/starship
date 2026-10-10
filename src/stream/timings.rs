//! Timing hints carried unchanged between streams: comma-separated
//! `name=milliseconds` for changed modules, bare names for unchanged ones.
//! Invalid entries are ignored; hints affect redraw timing, not content.

use std::fmt;
use std::time::Duration;

use indexmap::IndexSet;

use super::bus::{Estimate, FOREVER};

/// Decode estimates in plan order; missing modules have no estimate.
pub fn read(report: &str, modules: &IndexSet<String>) -> Vec<Option<Estimate>> {
    let mut estimates = vec![None; modules.len()];
    for (module, estimate) in entries(report) {
        if let Some(position) = modules.get_index_of(module) {
            estimates[position] = Some(estimate);
        }
    }
    estimates
}

/// Encode estimates with microsecond precision.
pub fn report<'a>(estimates: impl IntoIterator<Item = (&'a str, Estimate)>) -> String {
    estimates
        .into_iter()
        .filter(|(module, _)| !module.is_empty() && !module.contains([',', '=']))
        .map(|(module, estimate)| match estimate {
            Estimate::Changed(took) => format!("{module}={}", Milliseconds(took)),
            Estimate::Unchanged => module.to_owned(),
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn entries(report: &str) -> impl Iterator<Item = (&str, Estimate)> {
    report.split(',').filter_map(|entry| {
        let (module, estimate) = match entry.split_once('=') {
            Some((module, milliseconds)) => {
                let took = Duration::from_micros(microseconds(milliseconds)?);
                (module, Estimate::Changed(took.min(FOREVER)))
            }
            None => (entry, Estimate::Unchanged),
        };
        (!module.is_empty()).then_some((module, estimate))
    })
}

/// Decimal milliseconds with up to three fractional digits.
struct Milliseconds(Duration);

impl fmt::Display for Milliseconds {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let microseconds = self.0.as_micros();
        write!(formatter, "{}", microseconds / 1000)?;
        match microseconds % 1000 {
            0 => Ok(()),
            fraction => write!(formatter, ".{fraction:03}"),
        }
    }
}

fn microseconds(milliseconds: &str) -> Option<u64> {
    let (whole, fraction) = milliseconds.split_once('.').unwrap_or((milliseconds, ""));
    if fraction.len() > 3 || !fraction.bytes().all(|digit| digit.is_ascii_digit()) {
        return None;
    }
    let fraction: u64 = format!("{fraction:0<3}").parse().ok()?;
    whole
        .parse::<u64>()
        .ok()?
        .checked_mul(1000)?
        .checked_add(fraction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn handed_back(report: &str) -> String {
        super::report(entries(report))
    }

    #[test]
    fn malformed_entries_are_skipped() {
        assert_eq!(
            "rust=12.500,git_status=40,nodejs",
            handed_back("rust=12.5,x=y,=3,,git_status=40,fine=0.0001,negative=-1,nodejs"),
            "a bare name is a module that changed nothing"
        );
    }

    #[test]
    fn an_estimate_is_at_most_an_hour() {
        assert_eq!(
            "quick=12,slow=3600000",
            handed_back("quick=12,slow=3600001,forever=inf")
        );
    }

    #[test]
    fn a_name_holding_a_separator_is_not_reported() {
        let reported = report([
            ("custom.a,b", Estimate::Unchanged),
            ("custom.a=b", Estimate::Changed(Duration::from_millis(1))),
            ("", Estimate::Unchanged),
            ("kept", Estimate::Unchanged),
        ]);

        assert_eq!("kept", reported);
    }

    #[test]
    fn only_the_modules_planned_are_read() {
        let modules = IndexSet::from(["character".to_owned(), "rust".to_owned()]);

        assert_eq!(
            vec![None, Some(Estimate::Changed(Duration::from_millis(12)))],
            read("directory=1,rust=12", &modules)
        );
    }

    fn estimate() -> impl Strategy<Value = Estimate> {
        prop_oneof![
            Just(Estimate::Unchanged),
            (0..=FOREVER.as_micros() as u64)
                .prop_map(|microseconds| Estimate::Changed(Duration::from_micros(microseconds))),
        ]
    }

    proptest! {
        #[test]
        fn a_report_survives_being_handed_back(
            reported in prop::collection::btree_map("[a-z_.]{1,12}", estimate(), 0..8)
        ) {
            let report = report(reported.iter().map(|(module, &estimate)| (module.as_str(), estimate)));

            prop_assert_eq!(
                reported.into_iter().collect::<Vec<_>>(),
                entries(&report).map(|(module, estimate)| (module.to_owned(), estimate)).collect::<Vec<_>>()
            );
        }
    }
    #[test]
    fn malformed_duplicate_timings_preserve_the_last_valid_estimate() {
        let modules = IndexSet::from(["rust".to_owned(), "character".to_owned()]);
        assert_eq!(
            vec![
                Some(Estimate::Changed(Duration::from_micros(12_500))),
                Some(Estimate::Unchanged)
            ],
            read(
                "rust=3,rust=12.5,rust=invalid,character=4,character,missing=1",
                &modules
            )
        );
    }
}
