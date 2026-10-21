//! What each module did for the previous prompt of a shell session.
//!
//! Each stream adds its report beside the session's log, in a file of its own
//! named for the stream's generation, and the next stream reads the newest. A
//! report is never replaced, only added and later pruned, so no stream reads
//! one half written, and none that finishes late can undo the report of one
//! that started after it. Generations follow a hybrid logical clock: the wall
//! clock, but always after every generation a stream has read, so that a
//! clock set back cannot put a report behind the one it was measured against.
//!
//! A report holds `name=milliseconds` for a module that changed the prompt,
//! and the bare name of one that did not, separated by commas. It only ever
//! decides when the prompt is drawn, never what is drawn, so a stale or
//! missing one costs at most an extra redraw.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use indexmap::IndexSet;

use super::bus::{Estimate, Span};
use crate::context::Context;

/// The reports of a shell session, as a stream of it found them.
pub struct Timings {
    directory: PathBuf,
    /// What the session's reports are named after: `session_<key>.`, then
    /// their generation, then `.timings`.
    session: String,
    /// The newest report when the stream started.
    previous: String,
    generation: u128,
}

impl Timings {
    /// The reports of the session `context` belongs to, if it belongs to one.
    pub fn open(context: &Context) -> Option<Self> {
        let key = context
            .get_env("STARSHIP_SESSION_KEY")
            .filter(|key| !key.is_empty())?;
        Some(Self::at(
            crate::logger::get_log_dir(),
            &key,
            SystemTime::now(),
        ))
    }

    /// The reports of the session `key` kept in `directory`, for a stream
    /// starting `now`.
    pub(super) fn at(directory: PathBuf, key: &str, now: SystemTime) -> Self {
        let session = format!("session_{key}.");
        let newest = reports(&directory, &session).max();
        let previous = newest
            .as_ref()
            .and_then(|(_, path)| crate::utils::read_file(path).ok())
            .unwrap_or_default();
        let now = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self {
            generation: newest.map_or(now, |(newest, _)| now.max(newest.saturating_add(1))),
            directory,
            session,
            previous,
        }
    }

    pub fn previous(&self) -> &str {
        &self.previous
    }

    /// Keeps `report` for the next prompt, in place of every report before it.
    pub fn keep(self, report: &str) {
        if let Err(error) = self.add(report) {
            log::debug!("Unable to keep the timings of this prompt: {error}");
        }
    }

    fn add(&self, report: &str) -> io::Result<()> {
        fs::create_dir_all(&self.directory)?;
        // Named like a report, so that cleanup expires it too should this
        // process end before it is.
        let mut staged = tempfile::Builder::new()
            .prefix("session_")
            .suffix(".timings")
            .tempfile_in(&self.directory)?;
        staged.write_all(report.as_bytes())?;
        let name = format!("{}{}.timings", self.session, self.generation);
        staged
            .persist_noclobber(self.directory.join(name))
            .map_err(|error| error.error)?;
        for (generation, path) in reports(&self.directory, &self.session) {
            if generation < self.generation {
                // Another stream may have pruned it first.
                let _ = fs::remove_file(path);
            }
        }
        Ok(())
    }
}

/// The reports of `session` in `directory`, with their generations.
fn reports<'a>(directory: &Path, session: &'a str) -> impl Iterator<Item = (u128, PathBuf)> + 'a {
    fs::read_dir(directory)
        .into_iter()
        .flatten()
        .filter_map(move |entry| {
            let entry = entry.ok()?;
            let generation = entry
                .file_name()
                .to_str()?
                .strip_prefix(session)?
                .strip_suffix(".timings")?
                .parse()
                .ok()?;
            Some((generation, entry.path()))
        })
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
    use proptest::sample::Index;
    use std::thread;

    fn handed_back(report: &str) -> String {
        super::report(entries(report))
    }

    /// The reports of the session `key` in `directory`, for a stream starting
    /// `seconds` after the epoch.
    fn at(directory: &Path, seconds: u64) -> Timings {
        Timings::at(
            directory.to_path_buf(),
            "key",
            SystemTime::UNIX_EPOCH + Duration::from_secs(seconds),
        )
    }

    /// The session's reports in `directory`, oldest first.
    fn kept(directory: &Path) -> Vec<String> {
        let mut kept: Vec<_> = reports(directory, "session_key.").collect();
        kept.sort();
        kept.into_iter()
            .map(|(_, path)| crate::utils::read_file(path).unwrap())
            .collect()
    }

    #[test]
    fn malformed_entries_are_skipped_and_the_last_of_a_module_counts() {
        assert_eq!(
            "rust=12,git_status=40,nodejs,slow=86400000",
            handed_back(
                "rust=12,x=y,=3,,git_status=40,fine=0.5,negative=-1,nodejs,slow=86400001,forever=inf"
            ),
            "a bare name is a module that changed nothing, and none takes over a day"
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
    fn a_report_supersedes_the_one_it_read_however_far_the_clock_goes_back() {
        let directory = tempfile::tempdir().unwrap();
        at(directory.path(), 100).keep("before");
        let after = at(directory.path(), 1);
        assert_eq!("before", after.previous());
        after.keep("after");

        assert_eq!(["after"], kept(directory.path()).as_slice());
    }

    #[test]
    fn other_sessions_and_unfinished_reports_are_left_alone() {
        let directory = tempfile::tempdir().unwrap();
        let other = |seconds| {
            Timings::at(
                directory.path().to_path_buf(),
                "other",
                SystemTime::UNIX_EPOCH + Duration::from_secs(seconds),
            )
        };
        other(1).keep("other=1");
        let unfinished = directory.path().join("session_key.7.timings.partial");
        fs::write(&unfinished, "unfinished=1").unwrap();
        at(directory.path(), 1).keep("rust=12");
        at(directory.path(), 2).keep("rust=24");

        assert_eq!(["rust=24"], kept(directory.path()).as_slice());
        assert!(unfinished.exists());
        assert_eq!("other=1", other(3).previous());
    }

    #[test]
    fn streams_keeping_reports_at_once_are_only_ever_read_whole() {
        let directory = tempfile::tempdir().unwrap();
        let reports: Vec<String> = (0..4)
            .map(|stream| {
                (0..512)
                    .map(|module| format!("custom.stream{stream}.{module}={stream}"))
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect();

        thread::scope(|scope| {
            for report in &reports {
                let directory = directory.path();
                scope.spawn(move || {
                    for _ in 0..16 {
                        Timings::at(directory.to_path_buf(), "key", SystemTime::now()).keep(report);
                    }
                });
            }
            for _ in 0..256 {
                let read = at(directory.path(), 0).previous;
                assert!(
                    read.is_empty() || reports.contains(&read),
                    "a report read in part"
                );
            }
        });
    }

    #[test]
    fn a_session_keeps_its_reports_beside_its_log() {
        let mut context = crate::test::default_context();
        assert!(Timings::open(&context).is_none());
        context.env.insert("STARSHIP_SESSION_KEY", "key".to_owned());
        let timings = Timings::open(&context).unwrap();
        assert_eq!(crate::logger::get_log_dir(), timings.directory);
        assert_eq!("session_key.", timings.session);
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

        /// Streams start a second apart, and each keeps its report, if at
        /// all, at any point after it starts. However they overlap, the next
        /// stream reads the report of the last to start among those that kept
        /// one, and each report kept prunes every one started before it.
        #[test]
        fn the_last_stream_started_to_report_is_the_one_read(
            steps in prop::collection::vec((any::<bool>(), any::<Index>()), 0..12),
        ) {
            let directory = tempfile::tempdir().unwrap();
            let mut started = 0_u64;
            let mut running: Vec<(u64, Timings)> = Vec::new();
            let mut remaining = std::collections::BTreeSet::new();
            for (start, which) in steps {
                if start || running.is_empty() {
                    started += 1;
                    running.push((started, at(directory.path(), started)));
                } else {
                    let (stream, timings) = running.swap_remove(which.index(running.len()));
                    timings.keep(&stream.to_string());
                    remaining.insert(stream);
                    remaining.retain(|&kept| kept >= stream);
                }
            }

            let read = at(directory.path(), started + 1).previous;
            prop_assert_eq!(remaining.last().map(ToString::to_string).unwrap_or_default(), read);
            let remaining: Vec<_> = remaining.iter().map(ToString::to_string).collect();
            prop_assert_eq!(remaining, kept(directory.path()));
        }
    }
}
