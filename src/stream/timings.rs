//! What each module did for the previous prompt of a shell session.
//!
//! A stream reports it once every module has rendered, in a file of the
//! session's beside its log, which the next stream reads: `name=milliseconds`
//! for a module that changed the prompt, and the bare name of one that did
//! not, separated by commas. It only ever decides when the prompt is drawn,
//! never what is drawn, so a stale or mangled report costs at most an extra
//! redraw.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use indexmap::IndexSet;

use super::bus::{Estimate, Span};
use crate::context::Context;

/// The report of the session `context` belongs to, if it belongs to one.
pub fn path(context: &Context) -> Option<PathBuf> {
    let session = context
        .get_env("STARSHIP_SESSION_KEY")
        .filter(|key| !key.is_empty())?;
    Some(crate::logger::get_log_dir().join(format!("session_{session}.timings")))
}

/// What each of `modules` did for the previous prompt, as far as the report
/// at `path` says.
pub fn load(path: Option<&Path>, modules: &IndexSet<String>) -> Vec<Option<Estimate>> {
    let report = path.and_then(|path| fs::read_to_string(path).ok());
    read(report.as_deref().unwrap_or_default(), modules)
}

/// Keeps `report` at `path` for the next prompt.
pub fn save(path: &Path, report: &str) {
    if let Err(error) = fs::write(path, report) {
        log::debug!("Unable to keep the timings of this prompt: {error}");
    }
}

/// What each of `modules` did for the previous prompt, by position, as far
/// as `report` says.
pub(super) fn read(report: &str, modules: &IndexSet<String>) -> Vec<Option<Estimate>> {
    let mut estimates = vec![None; modules.len()];
    for (module, estimate) in entries(report) {
        if let Some(position) = modules.get_index_of(module) {
            estimates[position] = Some(estimate);
        }
    }
    estimates
}

/// A report of what each module did, to the millisecond, finer than any two
/// redraws could be told apart by.
pub fn report<'a>(estimates: impl IntoIterator<Item = (&'a str, Estimate)>) -> String {
    estimates
        .into_iter()
        // A name holding a separator could not be read back as one module.
        .filter(|(module, _)| !module.is_empty() && !module.contains([',', '=']))
        .map(|(module, estimate)| match estimate {
            Estimate::Changed(took) => format!("{module}={}", took.get().as_millis()),
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
                let took = Duration::from_millis(milliseconds.parse().ok()?);
                (module, Estimate::Changed(Span::from(took)))
            }
            None => (entry, Estimate::Unchanged),
        };
        (!module.is_empty()).then_some((module, estimate))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn handed_back(report: &str) -> String {
        super::report(entries(report))
    }

    #[test]
    fn malformed_entries_are_skipped_and_the_last_of_a_module_counts() {
        assert_eq!(
            "rust=12,git_status=40,nodejs",
            handed_back("rust=12,x=y,=3,,git_status=40,fine=0.5,negative=-1,nodejs"),
            "a bare name is a module that changed nothing"
        );
        let modules = IndexSet::from(["rust".to_owned(), "character".to_owned()]);
        assert_eq!(
            vec![
                Some(Estimate::Changed(Span::from(Duration::from_millis(12)))),
                Some(Estimate::Unchanged)
            ],
            read(
                "rust=3,rust=12,rust=invalid,character=4,character,missing=1",
                &modules
            )
        );
    }

    #[test]
    fn a_report_kept_is_read_back_for_the_next_prompt() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session_key.timings");
        let modules = IndexSet::from(["rust".to_owned(), "character".to_owned()]);

        assert_eq!(
            vec![None, None],
            load(Some(&path), &modules),
            "none kept yet"
        );
        save(&path, "rust=12,character");
        assert_eq!(
            vec![
                Some(Estimate::Changed(Span::from(Duration::from_millis(12)))),
                Some(Estimate::Unchanged)
            ],
            load(Some(&path), &modules)
        );
        assert_eq!(vec![None, None], load(None, &modules), "outside a session");
    }

    #[test]
    fn a_session_keeps_its_report_beside_its_log() {
        let mut context = crate::test::default_context();
        assert_eq!(None, path(&context));
        context.env.insert("STARSHIP_SESSION_KEY", "key".to_owned());
        assert_eq!(
            Some(crate::logger::get_log_dir().join("session_key.timings")),
            path(&context)
        );
    }

    #[test]
    fn an_estimate_is_at_most_a_day() {
        assert_eq!(
            "quick=12,slow=86400000",
            handed_back("quick=12,slow=86400001,forever=inf")
        );
    }

    #[test]
    fn a_name_holding_a_separator_is_not_reported() {
        let reported = report([
            ("custom.a,b", Estimate::Unchanged),
            ("custom.a=b", Estimate::Changed(Span::MAX)),
            ("", Estimate::Unchanged),
            ("kept", Estimate::Unchanged),
        ]);

        assert_eq!("kept", reported);
    }

    fn estimate() -> impl Strategy<Value = Estimate> {
        prop_oneof![
            Just(Estimate::Unchanged),
            (0..=Span::MAX.get().as_millis() as u64).prop_map(|milliseconds| {
                Estimate::Changed(Span::from(Duration::from_millis(milliseconds)))
            }),
        ]
    }

    proptest! {
        /// Whatever the modules did, the next stream reads exactly what the
        /// last reported, to the millisecond.
        #[test]
        fn a_report_reads_back_as_it_was_made(
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
