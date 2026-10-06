//! What each module did for the previous prompt, as a shell carries it from
//! one prompt to the next.
//!
//! A shell keeps what a stream reports as a string that it hands the next
//! stream unread: `name=milliseconds` for a module that changed the prompt, and
//! the bare name of one that did not, separated by commas. It only ever
//! decides when the prompt is drawn, never what is drawn, so a stale or
//! mangled report costs at most an extra redraw.

use std::fmt;
use std::time::Duration;

use indexmap::IndexSet;

use super::bus::{Estimate, FOREVER};

/// What each of `modules` did for the previous prompt, by position, as far
/// as `report` says.
pub fn read(report: &str, modules: &IndexSet<String>) -> Vec<Option<Estimate>> {
    let mut estimates = vec![None; modules.len()];
    for (module, estimate) in entries(report) {
        if let Some(position) = modules.get_index_of(module) {
            estimates[position] = Some(estimate);
        }
    }
    estimates
}

/// A report of what each module did, which [`read`] reads back exactly.
pub fn report<'a>(estimates: impl IntoIterator<Item = (&'a str, Estimate)>) -> String {
    estimates
        .into_iter()
        // A name holding a separator could not be read back as one module.
        .filter(|(module, _)| !module.is_empty() && !module.contains([',', '=']))
        .map(|(module, estimate)| match estimate {
            Estimate::Changed(took) => format!("{module}={}", Milliseconds(took)),
            Estimate::Unchanged => module.to_owned(),
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Every well-formed entry of `report`, skipping any other.
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

/// A duration in milliseconds, to the microsecond: finer than any two modules
/// could be told apart by.
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

/// The microseconds in a decimal number of milliseconds with at most three
/// decimal places.
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
        /// Whatever the modules did, a shell hands back exactly what the
        /// stream reported, to the microsecond.
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
}
