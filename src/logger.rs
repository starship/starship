use ansi_term::Color;
use log::{Level, LevelFilter, Metadata, Record};
use once_cell::sync::OnceCell;
use std::{
    collections::HashSet,
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};

pub struct StarshipLogger {
    log_file: OnceCell<Mutex<File>>,
    log_file_path: PathBuf,
    log_file_content: HashSet<String>,
    log_level: Level,
}

impl StarshipLogger {
    fn new() -> Self {
        let log_dir = env::var_os("STARSHIP_CACHE")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                dirs_next::home_dir()
                    .expect("Unable to find home directory")
                    .join(".cache/starship")
            });

        fs::create_dir_all(&log_dir)
            .unwrap_or_else(|err| panic!("Unable to create log dir {:?}: {:?}!", log_dir, err));
        let session_log_file = log_dir.join(format!(
            "session_{}.log",
            env::var("STARSHIP_SESSION_KEY").unwrap_or_default()
        ));

        Self {
            log_file_content: fs::read_to_string(&session_log_file)
                .unwrap_or_default()
                .lines()
                .map(|line| line.to_string())
                .collect(),
            log_file: OnceCell::new(),
            log_file_path: session_log_file,
            log_level: env::var("STARSHIP_LOG")
                .map(|level| match level.to_lowercase().as_str() {
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

impl log::Log for StarshipLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        let to_print = format!(
            "[{}] - ({}): {}",
            record.level(),
            record.module_path().unwrap_or_default(),
            record.args()
        );

        if record.metadata().level() <= Level::Warn {
            self.log_file
                .get_or_try_init(|| {
                    let m = Mutex::new(
                        OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&self.log_file_path)?,
                    );
                    Ok(m)
                })
                .unwrap_or_else(|err: std::io::Error| {
                    panic!(
                        "Unable to open session log file {:?}: {:?}!",
                        self.log_file_path, err
                    )
                })
                .lock()
                .map(|mut file| writeln!(file, "{}", to_print))
                .expect("Log file writer mutex was poisoned!")
                .expect("Unable to write to the log file!");
        }

        if self.enabled(record.metadata()) && !self.log_file_content.contains(to_print.as_str()) {
            eprintln!(
                "[{}] - ({}): {}",
                match record.level() {
                    Level::Trace => Color::Black.dimmed().paint(format!("{}", record.level())),
                    Level::Debug => Color::Black.paint(format!("{}", record.level())),
                    Level::Info => Color::White.paint(format!("{}", record.level())),
                    Level::Warn => Color::Yellow.paint(format!("{}", record.level())),
                    Level::Error => Color::Red.paint(format!("{}", record.level())),
                },
                record.module_path().unwrap_or_default(),
                record.args()
            );
        }
    }

    fn flush(&self) {
        if let Some(m) = self.log_file.get() {
            m.lock()
                .map(|mut writer| writer.flush())
                .expect("Log file writer mutex was poisoned!")
                .expect("Unable to flush the log file!");
        }
    }
}

pub fn init() {
    log::set_boxed_logger(Box::new(StarshipLogger::new())).unwrap();
    log::set_max_level(LevelFilter::Trace);
}
