//! What each module did for the previous prompt of a shell session.
//!
//! A stream reports it once every module has rendered, in a file of the
//! session's beside its log, which the next stream reads: `name=milliseconds`
//! for a module that changed the prompt, and the bare name of one that did
//! not, separated by commas. It only ever decides when the prompt is drawn,
//! never what is drawn, so a stale or mangled report costs at most an extra
//! redraw.

use std::fs::{self, File};
use std::io::{self, Read, Write};
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
/// of `writer` says.
pub fn load(writer: Option<&Writer>, modules: &IndexSet<String>) -> Vec<Option<Estimate>> {
    let report = writer.and_then(|writer| fs::read_to_string(&writer.path).ok());
    read(report.as_deref().unwrap_or_default(), modules)
}

/// The stream allowed to report for a session. A newer stream claims it before
/// loading estimates, so an older, slower render cannot replace its report.
pub struct Writer {
    path: PathBuf,
    generation: [u8; 16],
}

impl Writer {
    pub fn claim(path: PathBuf) -> Option<Self> {
        let writer = Self {
            path,
            generation: rand::random(),
        };
        let claimed = (|| -> io::Result<()> {
            let mut owner = writer.lock()?;
            owner.set_len(0)?;
            owner.write_all(&writer.generation)
        })();
        match claimed {
            Ok(()) => Some(writer),
            Err(error) => {
                log::debug!("Unable to claim the timings of this prompt: {error}");
                None
            }
        }
    }

    /// Keeps `report` whole for the next prompt, if no newer stream has started.
    pub fn save(&self, report: &str) {
        if let Err(error) = self.keep(report) {
            log::debug!("Unable to keep the timings of this prompt: {error}");
        }
    }

    fn keep(&self, report: &str) -> io::Result<()> {
        let directory = self
            .path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let mut written = tempfile::NamedTempFile::new_in(directory)?;
        written.write_all(report.as_bytes())?;

        let mut owner = self.lock()?;
        let mut generation = [0; 16];
        owner.read_exact(&mut generation)?;
        if generation == self.generation {
            written.persist(&self.path).map_err(|error| error.error)?;
        }
        Ok(())
    }

    /// This file stays in place: replacing or removing it would let contenders
    /// lock different inodes. Locks are held only to claim or publish a report.
    fn lock(&self) -> io::Result<File> {
        let owner = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.path.with_added_extension("lock"))?;
        owner.lock()?;
        Ok(owner)
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
    use std::sync::Barrier;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

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
        let writer = Writer::claim(path.clone()).unwrap();
        let modules = IndexSet::from(["rust".to_owned(), "character".to_owned()]);

        assert_eq!(
            vec![None, None],
            load(Some(&writer), &modules),
            "none kept yet"
        );
        writer.save("rust=12,character");
        assert_eq!(
            vec![
                Some(Estimate::Changed(Span::from(Duration::from_millis(12)))),
                Some(Estimate::Unchanged)
            ],
            load(Some(&writer), &modules)
        );
        writer.save("rust");
        assert_eq!("rust", fs::read_to_string(&path).unwrap());
        assert_eq!(vec![None, None], load(None, &modules), "outside a session");
    }

    #[test]
    fn a_newer_stream_owns_the_report_even_if_an_older_stream_finishes_last() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session_key.timings");
        let previous = Writer::claim(path.clone()).unwrap();
        previous.save("previous=1");
        let older = Writer::claim(path.clone()).unwrap();
        let newer = Writer::claim(path.clone()).unwrap();

        older.save("older=2");
        assert_eq!(
            "previous=1",
            fs::read_to_string(&path).unwrap(),
            "a new stream retains the last complete report until it finishes"
        );
        newer.save("newer=3");
        older.save("older=4");
        previous.save("previous=5");
        assert_eq!("newer=3", fs::read_to_string(&path).unwrap());
    }

    #[test]
    fn overlapping_streams_and_readers_only_see_whole_reports() {
        const WRITERS: usize = 4;
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session_key.timings");
        let initial = "initial=0";
        Writer::claim(path.clone()).unwrap().save(initial);
        let reports: Vec<_> = (0..WRITERS)
            .map(|writer| {
                (0..(512 + writer * 128))
                    .map(|module| format!("custom.writer{writer}.{module}={writer}"))
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect();
        let ready = Barrier::new(WRITERS + 1);
        let remaining = AtomicUsize::new(WRITERS);

        thread::scope(|scope| {
            let ready = &ready;
            let remaining = &remaining;
            for report in &reports {
                let path = path.clone();
                scope.spawn(move || {
                    ready.wait();
                    for _ in 0..16 {
                        let writer = Writer::claim(path.clone()).unwrap();
                        writer.keep(report).unwrap();
                    }
                    remaining.fetch_sub(1, Ordering::Release);
                });
            }
            ready.wait();
            loop {
                let observed = fs::read_to_string(&path).unwrap();
                assert!(
                    observed == initial || reports.contains(&observed),
                    "a reader saw a partial or mixed report"
                );
                if remaining.load(Ordering::Acquire) == 0 {
                    break;
                }
                thread::yield_now();
            }
        });

        assert!(reports.contains(&fs::read_to_string(&path).unwrap()));
    }

    #[test]
    fn a_failed_replacement_preserves_the_target_and_removes_its_temporary_file() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session_key.timings");
        fs::create_dir(&path).unwrap();
        fs::write(path.join("existing"), "preserved").unwrap();
        let writer = Writer::claim(path.clone()).unwrap();

        assert!(writer.keep("rust=12").is_err());
        assert_eq!(
            "preserved",
            fs::read_to_string(path.join("existing")).unwrap()
        );
        assert_eq!(
            2,
            fs::read_dir(directory.path()).unwrap().count(),
            "only the target and stable ownership file remain"
        );
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
