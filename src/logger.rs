use crate::utils;
use log::{Level, LevelFilter, Metadata, Record};
use nu_ansi_term::Color;
use once_cell::sync::OnceCell;
use std::{
    cmp,
    collections::HashSet,
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Mutex, RwLock},
};

pub struct StarshipLogger {
    log_file: OnceCell<Mutex<File>>,
    log_file_path: PathBuf,
    log_file_content: RwLock<HashSet<String>>,
    log_level: Level,
}

/// Returns the path to the log directory.
pub fn get_log_dir() -> PathBuf {
    env::var_os("STARSHIP_CACHE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            utils::home_dir()
                .map(|home| home.join(".cache"))
                .or_else(dirs_next::cache_dir)
                .unwrap_or_else(std::env::temp_dir)
                .join("starship")
        })
}

/// Deletes all log files in the log directory that were modified more than 24 hours ago.
pub fn cleanup_log_files<P: AsRef<Path>>(path: P) {
    let log_dir = path.as_ref();
    let log_files = match fs::read_dir(log_dir) {
        Ok(log_files) => log_files,
        // Avoid noisily handling errors in this cleanup function.
        Err(_) => return,
    };

    for file in log_files {
        // Skip files that can't be read.
        let file = match file {
            Ok(file) => file,
            Err(_) => continue,
        };

        // Avoid deleting files that don't look like log files.
        if !file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .starts_with("session_")
            || file.path().extension() != Some("log".as_ref())
        {
            continue;
        }

        // Read metadata to check file age.
        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        // Avoid handling anything that isn't a file.
        if !metadata.is_file() {
            continue;
        }

        // Get the file's modification time.
        let modified = match metadata.modified() {
            Ok(modified) => modified,
            Err(_) => continue,
        };

        // Delete the file if it hasn't changed in 24 hours.
        if modified.elapsed().unwrap_or_default().as_secs() > 60 * 60 * 24 {
            let _ = fs::remove_file(file.path());
        }
    }
}

impl Default for StarshipLogger {
    fn default() -> Self {
        let log_dir = get_log_dir();

        if let Err(err) = fs::create_dir_all(&log_dir) {
            eprintln!("Unable to create log dir {log_dir:?}: {err:?}!")
        };
        let session_log_file = log_dir.join(format!(
            "session_{}.log",
            env::var("STARSHIP_SESSION_KEY").unwrap_or_default()
        ));

        Self {
            log_file_content: RwLock::new(
                fs::read_to_string(&session_log_file)
                    .unwrap_or_default()
                    .lines()
                    .map(std::string::ToString::to_string)
                    .collect(),
            ),
            log_file: OnceCell::new(),
            log_file_path: session_log_file,
            log_level: env::var("STARSHIP_LOG")
                .map(|level| match level.to_ascii_lowercase().as_str() {
                    "trace" => Level::Trace,
                    "debug" => Level::Debug,
                    "info" => Level::Info,
                    "warn" => Level::Warn,
                    "error" => Level::Error,
                    _ => Level::Warn,
                })
                .unwrap_or_else(|_| Level::Warn),
        }
    }
}

impl StarshipLogger {
    /// Override the minimum log level
    pub fn set_log_level(&mut self, level: log::Level) {
        self.log_level = level;
    }

    /// Override the log level path
    /// This won't change anything if a log file was already opened
    pub fn set_log_file_path(&mut self, path: PathBuf) {
        let contents = fs::read_to_string(&path)
            .unwrap_or_default()
            .lines()
            .map(std::string::ToString::to_string)
            .collect();
        self.log_file_content = RwLock::new(contents);
        self.log_file_path = path;
    }
}

impl log::Log for StarshipLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        // Early return if the log level is not enabled
        if !self.enabled(record.metadata()) {
            return;
        }

        let to_print = format!(
            "[{}] - ({}): {}",
            record.level(),
            record.module_path().unwrap_or_default(),
            record.args()
        );

        // A log message is only printed or written to the log file,
        // if it's not already in the log file or has been printed in this session.
        // To help with debugging, duplicate detection only runs if the log level is warn or lower
        let is_debug = record.level() > Level::Warn;
        let is_duplicate = {
            !is_debug
                && self
                    .log_file_content
                    .read()
                    .map(|c| c.contains(to_print.as_str()))
                    .unwrap_or(false)
        };

        if is_duplicate {
            return;
        }

        // Write warning messages to the log file
        // If log level is error, only write error messages to the log file
        if record.level() <= cmp::min(Level::Warn, self.log_level) {
            let log_file = match self.log_file.get_or_try_init(|| {
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.log_file_path)
                    .map(Mutex::new)
            }) {
                Ok(log_file) => log_file,
                Err(err) => {
                    eprintln!(
                        "Unable to open session log file {:?}: {err:?}!",
                        self.log_file_path
                    );
                    return;
                }
            };

            let mut file_handle = match log_file.lock() {
                Ok(file_handle) => file_handle,
                Err(err) => {
                    eprintln!("Log file writer mutex was poisoned! {err:?}",);
                    return;
                }
            };
            if let Err(err) = writeln!(file_handle, "{to_print}") {
                eprintln!("Unable to write to session log file {err:?}!",);
            };
        }

        // Print messages to stderr
        eprintln!(
            "[{}] - ({}): {}",
            match record.level() {
                Level::Trace => Color::Blue.dimmed().paint(format!("{}", record.level())),
                Level::Debug => Color::Cyan.paint(format!("{}", record.level())),
                Level::Info => Color::White.paint(format!("{}", record.level())),
                Level::Warn => Color::Yellow.paint(format!("{}", record.level())),
                Level::Error => Color::Red.paint(format!("{}", record.level())),
            },
            record.module_path().unwrap_or_default(),
            record.args()
        );

        // Add to duplicate detection set
        if let Ok(mut c) = self.log_file_content.write() {
            c.insert(to_print);
        }
    }

    fn flush(&self) {
        if let Some(m) = self.log_file.get() {
            let result = match m.lock() {
                Ok(mut file) => file.flush(),
                Err(err) => return eprintln!("Log file writer mutex was poisoned: {err:?}"),
            };
            if let Err(err) = result {
                eprintln!("Unable to flush the log file: {err:?}");
            }
        }
    }
}

pub fn init() {
    log::set_boxed_logger(Box::<StarshipLogger>::default()).unwrap();
    log::set_max_level(LevelFilter::Trace);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_file;
    use log::Log;
    use std::io;

    #[test]
    fn test_log_to_file() -> io::Result<()> {
        let log_dir = tempfile::tempdir()?;
        let log_file = log_dir.path().join("test.log");

        let mut logger = StarshipLogger::default();
        logger.set_log_file_path(log_file.clone());
        logger.set_log_level(Level::Warn);

        // Load at all log levels
        logger.log(
            &Record::builder()
                .level(Level::Error)
                .args(format_args!("error"))
                .build(),
        );
        logger.log(
            &Record::builder()
                .level(Level::Warn)
                .args(format_args!("warn"))
                .build(),
        );
        logger.log(
            &Record::builder()
                .level(Level::Info)
                .args(format_args!("info"))
                .build(),
        );
        logger.log(
            &Record::builder()
                .level(Level::Debug)
                .args(format_args!("debug"))
                .build(),
        );
        logger.log(
            &Record::builder()
                .level(Level::Trace)
                .args(format_args!("trace"))
                .build(),
        );

        // Print duplicate messages
        logger.log(
            &Record::builder()
                .level(Level::Warn)
                .args(format_args!("warn"))
                .build(),
        );
        logger.log(
            &Record::builder()
                .level(Level::Error)
                .args(format_args!("error"))
                .build(),
        );

        logger.flush();
        drop(logger);

        let content = read_file(log_file)?;

        assert_eq!(content, "[ERROR] - (): error\n[WARN] - (): warn\n");

        log_dir.close()
    }

    #[test]
    fn test_dedup_from_file() -> io::Result<()> {
        let log_dir = tempfile::tempdir()?;
        let log_file = log_dir.path().join("test.log");
        {
            let mut file = File::create(&log_file)?;
            file.write_all(b"[WARN] - (): warn\n")?;
            file.sync_all()?;
        }

        let mut logger = StarshipLogger::default();
        logger.set_log_file_path(log_file.clone());
        logger.set_log_level(Level::Warn);

        // This message should not be printed or written to the log file
        logger.log(
            &Record::builder()
                .level(Level::Warn)
                .args(format_args!("warn"))
                .build(),
        );
        // This message should be printed and written to the log file
        logger.log(
            &Record::builder()
                .level(Level::Warn)
                .args(format_args!("warn2"))
                .build(),
        );

        logger.flush();
        drop(logger);

        let content = read_file(log_file)?;

        assert_eq!(content, "[WARN] - (): warn\n[WARN] - (): warn2\n");

        log_dir.close()
    }

    #[test]
    #[cfg(unix)]
    fn test_cleanup() -> io::Result<()> {
        use nix::sys::{stat::utimes, time::TimeVal};
        use std::fs::File;

        let log_dir = tempfile::tempdir()?;

        // Should not be deleted
        let non_matching_file = log_dir.path().join("non-matching.log");
        let non_matching_file2 = log_dir.path().join("session_.exe");
        let new_file = log_dir.path().join("session_new.log");
        let directory = log_dir.path().join("session_dir.log");

        // Should be deleted
        let old_file = log_dir.path().join("session_old.log");

        for file in &[
            &non_matching_file,
            &non_matching_file2,
            &new_file,
            &old_file,
        ] {
            File::create(file)?;
        }
        fs::create_dir(&directory)?;

        // Set all files except the new file to be older than 24 hours
        for file in &[
            &non_matching_file,
            &non_matching_file2,
            &old_file,
            &directory,
        ] {
            utimes(file.as_path(), &TimeVal::new(0, 0), &TimeVal::new(0, 0))?;
            if let Ok(f) = File::open(file) {
                f.sync_all()?;
            }
        }

        cleanup_log_files(log_dir.path());

        for file in &[
            &non_matching_file,
            &non_matching_file2,
            &new_file,
            &directory,
        ] {
            assert!(file.exists(), "File {file:?} should exist");
        }

        assert!(!old_file.exists(), "File {old_file:?} should not exist");

        log_dir.close()
    }
}
