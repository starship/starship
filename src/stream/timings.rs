//! What each module did for the previous prompt of a shell session.
//!
//! A stream reports it once every module has rendered, in a file of the
//! session's beside its log, which the next stream reads. After an internal
//! generation header, the body contains `name=milliseconds` for a module that
//! changed the prompt, and the bare name of one that did not, separated by
//! commas. It only ever decides when the prompt is drawn, never what is drawn,
//! so a stale or mangled report costs at most an extra redraw.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use indexmap::IndexSet;

use super::bus::{Estimate, Span};
use crate::context::Context;

const HEADER: &str = "# starship-timings-v1 ";

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
    read(body(report.as_deref().unwrap_or_default()), modules)
}

/// The stream allowed to report for a session. A newer stream claims it before
/// loading estimates, so an older, slower render cannot replace its report.
pub struct Writer {
    path: PathBuf,
    generation: u128,
}

impl Writer {
    pub fn claim(path: PathBuf) -> Option<Self> {
        let writer = Self {
            path,
            generation: rand::random(),
        };
        let claimed = (|| -> io::Result<()> {
            let _owner = writer.lock()?;
            let previous = match fs::read_to_string(&writer.path) {
                Ok(previous) => previous,
                Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
                Err(error) => return Err(error),
            };
            writer
                .prepare(body(&previous))?
                .persist(&writer.path)
                .map_err(|error| error.error)?;
            Ok(())
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
        let written = self.prepare(report)?;

        let _owner = self.lock()?;
        let previous = fs::read_to_string(&self.path)?;
        if unpack(&previous).0 == Some(self.generation) {
            written.persist(&self.path).map_err(|error| error.error)?;
        }
        Ok(())
    }

    fn prepare(&self, report: &str) -> io::Result<tempfile::NamedTempFile> {
        // A process may exit without dropping this file. The logger's existing
        // cleanup expires orphaned stages just like completed session reports.
        let mut written = tempfile::Builder::new()
            .prefix("session_")
            .suffix(".timings")
            .tempfile_in(self.directory())?;
        writeln!(written, "{HEADER}{:032x}", self.generation)?;
        written.write_all(report.as_bytes())?;
        Ok(written)
    }

    /// One lock serves the cache directory; each session keeps its ownership
    /// in its expiring report. The lock stays in place so contenders always
    /// lock the same inode, and is held only to claim or publish a report.
    /// Contention skips this optional cache work instead of delaying a prompt.
    fn lock(&self) -> io::Result<File> {
        let owner = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.directory().join(".timings.lock"))?;
        owner.try_lock()?;
        Ok(owner)
    }

    fn directory(&self) -> &Path {
        self.path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
    }
}

/// The module report, without storage metadata. Legacy reports have no header.
pub(super) fn body(stored: &str) -> &str {
    unpack(stored).1
}

fn unpack(stored: &str) -> (Option<u128>, &str) {
    let Some((header, body)) = stored.split_once('\n') else {
        return (None, stored);
    };
    let generation = header
        .strip_prefix(HEADER)
        .filter(|value| value.len() == 32)
        .and_then(|value| u128::from_str_radix(value, 16).ok());
    match generation {
        Some(generation) => (Some(generation), body),
        None => (None, stored),
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
    use std::fs::FileTimes;
    use std::sync::Barrier;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Instant, SystemTime};

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
        assert_eq!("rust", body(&fs::read_to_string(&path).unwrap()));
        assert_eq!(vec![None, None], load(None, &modules), "outside a session");
    }

    #[test]
    fn a_claim_preserves_legacy_reports_and_storage_headers_stay_internal() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session_key.timings");
        let report = "rust=12,character";
        fs::write(&path, report).unwrap();
        let writer = Writer::claim(path.clone()).unwrap();
        let stored = fs::read_to_string(&path).unwrap();

        assert_eq!((Some(writer.generation), report), unpack(&stored));
        assert_eq!(body(report), report, "legacy reports have no header");
        let modules = IndexSet::from(["rust".to_owned(), "character".to_owned()]);
        assert_eq!(read(report, &modules), load(Some(&writer), &modules));
        assert_eq!(
            handed_back(report),
            report,
            "the report format is unchanged"
        );
        for unrecognized in [
            "# starship-timings-v1 invalid\nrust=12",
            "# starship-timings-v1 1234\nrust=12",
            "# starship-timings-v2 00000000000000000000000000000001\nrust=12",
        ] {
            assert_eq!(body(unrecognized), unrecognized);
        }
    }

    #[test]
    fn sessions_share_one_lock_without_revoking_each_others_reports() {
        let directory = tempfile::tempdir().unwrap();
        let first_path = directory.path().join("session_first.timings");
        let second_path = directory.path().join("session_second.timings");
        let first = Writer::claim(first_path.clone()).unwrap();
        let second = Writer::claim(second_path.clone()).unwrap();

        second.save("second=2");
        first.save("first=1");

        assert_eq!(body(&fs::read_to_string(first_path).unwrap()), "first=1");
        assert_eq!(body(&fs::read_to_string(second_path).unwrap()), "second=2");
        assert_eq!(3, fs::read_dir(directory.path()).unwrap().count());
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
            body(&fs::read_to_string(&path).unwrap()),
            "a new stream retains the last complete report until it finishes"
        );
        newer.save("newer=3");
        older.save("older=4");
        previous.save("previous=5");
        assert_eq!("newer=3", body(&fs::read_to_string(&path).unwrap()));
    }

    #[test]
    fn a_busy_cache_lock_does_not_delay_claims_or_saves() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session_key.timings");
        let other_path = directory.path().join("session_other.timings");
        let writer = Writer::claim(path.clone()).unwrap();
        writer.save("rust=12");
        let before = fs::read_to_string(&path).unwrap();
        let owner = writer.lock().unwrap();

        thread::scope(|scope| {
            let (send, receive) = mpsc::channel();
            let other_path = &other_path;
            let writer = &writer;
            scope.spawn(move || {
                let skipped = Writer::claim(other_path.clone()).is_none();
                let error = writer.keep("rust=24").unwrap_err().kind();
                send.send((skipped, error)).unwrap();
            });
            let result = receive.recv_timeout(Duration::from_secs(5));
            // Release before asserting so a regression cannot hang scoped joins.
            drop(owner);
            assert_eq!(
                (true, io::ErrorKind::WouldBlock),
                result.expect("optional timing work waited for the busy cache lock")
            );
        });

        assert_eq!(before, fs::read_to_string(&path).unwrap());
        assert!(!other_path.exists(), "a failed claim changes no generation");
        writer.save("rust=36");
        assert_eq!("rust=36", body(&fs::read_to_string(&path).unwrap()));
    }

    #[test]
    fn overlapping_streams_and_readers_only_see_whole_reports() {
        const WRITERS: usize = 4;
        struct Running<'a>(&'a AtomicUsize);
        impl Drop for Running<'_> {
            fn drop(&mut self) {
                self.0.fetch_sub(1, Ordering::Release);
            }
        }
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
                    let _running = Running(remaining);
                    ready.wait();
                    for _ in 0..16 {
                        let Some(writer) = Writer::claim(path.clone()) else {
                            continue;
                        };
                        let deadline = Instant::now() + Duration::from_secs(5);
                        loop {
                            match writer.keep(report) {
                                Err(error)
                                    if error.kind() == io::ErrorKind::WouldBlock
                                        && Instant::now() < deadline =>
                                {
                                    thread::yield_now();
                                }
                                result => {
                                    result.unwrap();
                                    break;
                                }
                            }
                        }
                    }
                });
            }
            ready.wait();
            loop {
                let observed = fs::read_to_string(&path).unwrap();
                let (generation, observed) = unpack(&observed);
                assert!(generation.is_some(), "a reader saw a partial header");
                assert!(
                    observed == initial || reports.iter().any(|report| report == observed),
                    "a reader saw a partial or mixed report"
                );
                if remaining.load(Ordering::Acquire) == 0 {
                    break;
                }
                thread::yield_now();
            }
        });

        assert!(
            reports
                .iter()
                .any(|report| report == body(&fs::read_to_string(&path).unwrap()))
        );
    }

    #[test]
    fn a_failed_replacement_preserves_the_target_and_removes_its_temporary_file() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("session_key.timings");
        let writer = Writer::claim(path.clone()).unwrap();
        fs::remove_file(&path).unwrap();
        fs::create_dir(&path).unwrap();
        fs::write(path.join("existing"), "preserved").unwrap();

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
        assert!(
            Writer::claim(path).is_none(),
            "a failed claim preserves its target"
        );
    }

    #[test]
    fn closed_sessions_and_orphaned_stages_leave_only_one_shared_lock_after_cleanup() {
        let directory = tempfile::tempdir().unwrap();
        for session in 0..100 {
            let path = directory.path().join(format!("session_{session}.timings"));
            Writer::claim(path).unwrap().save("rust=12");
        }
        assert_eq!(
            101,
            fs::read_dir(directory.path()).unwrap().count(),
            "each session has only its expiring report, plus one shared lock"
        );

        let writer = Writer::claim(directory.path().join("session_0.timings")).unwrap();
        let (orphan, orphan_path) = writer.prepare("interrupted=1").unwrap().keep().unwrap();
        drop(orphan);
        let old = FileTimes::new().set_modified(SystemTime::UNIX_EPOCH);
        for entry in fs::read_dir(directory.path()).unwrap() {
            File::options()
                .write(true)
                .open(entry.unwrap().path())
                .unwrap()
                .set_times(old)
                .unwrap();
        }
        let owner = writer.lock().unwrap();
        crate::logger::cleanup_log_files(directory.path());
        assert!(!orphan_path.exists(), "interrupted writes also expire");
        let remaining: Vec<_> = fs::read_dir(directory.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(remaining, [".timings.lock"]);
        drop(owner);
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
