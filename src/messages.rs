use crate::configs::MessagesConfig;
use crate::formatter::StringFormatter;
use crate::segment::Segment;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    fn to_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "Error",
            LogLevel::Warning => "Warning",
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
        }
    }
}

#[derive(Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct Message {
    message: &'static str,
    level: LogLevel,
}

impl Message {
    fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        s.write(&self.message.to_string().into_bytes());
        s.finish()
    }
}

pub mod messages {
    use super::{LogLevel, Message};
    pub const DEPRECATED_USE_FORMAT: Message = Message {
        message: "Starship uses `format` key to customize the appearance of modules from v0.41.0,\
                  Check https://starship.rs for more info on updating your config.",
        level: LogLevel::Warning,
    };
}

lazy_static! {
    static ref MESSAGES: Arc<Mutex<BTreeSet<Message>>> = Default::default();
}

/// Add a message
pub fn add(message: Message) {
    let messages_cloned = Arc::clone(&MESSAGES);
    let mut messages_cloned = messages_cloned.lock().unwrap();
    messages_cloned.insert(message);
}

/// Get messages
fn get() -> Vec<Message> {
    let messages = Arc::clone(&MESSAGES);
    let messages = messages.lock().unwrap();
    messages.clone().into_iter().collect()
}

/// Get temporary directory
fn temp_dir() -> &'static str {
    if cfg!(windows) {
        r"C:\Windows\Temp"
    } else {
        "/tmp"
    }
}

/// Get path to the store
fn store_path() -> PathBuf {
    // There is no way to check if starship has run in the current session.
    // Instead, this will get a path to a temporary directory, which will make
    // the message displays per boot.
    const STORAGE_FILE: &str = "starship_messages";

    let mut store_path = PathBuf::from(temp_dir());
    store_path.push(STORAGE_FILE);

    store_path
}

/// Get hash of viewed messages
fn get_viewed_hash() -> Vec<u64> {
    File::open(store_path())
        .and_then(|mut file| {
            let mut text = String::new();
            file.read_to_string(&mut text)?;
            Ok(text)
        })
        .map(|text: String| {
            text.split_whitespace()
                .map(|s| s.parse().ok())
                .flatten()
                .collect()
        })
        .unwrap_or(Vec::new())
}

/// Get messages not viewed
pub fn get_not_viewed() -> Vec<Message> {
    let viewed = get_viewed_hash();
    get()
        .into_iter()
        .filter(|message| !viewed.contains(&message.get_hash()))
        .collect()
}

/// Get not viewed messages as segments
pub fn get_segments(config: &MessagesConfig) -> Vec<Segment> {
    let messages = get_not_viewed();
    messages
        .iter()
        .flat_map(|message| match StringFormatter::new(config.format) {
            Ok(formatter) => Some(
                formatter
                    .map(|variable| match variable {
                        "level" => Some(message.level.to_str().to_owned()),
                        "message" => Some(message.message.to_string()),
                        _ => None,
                    })
                    .parse(None),
            ),
            Err(error) => {
                log::error!("Error parsing `messages.format`:\n{}", error);
                None
            }
        })
        .flatten()
        .collect()
}

/// Update viewed messages
pub fn update_viewed_hash() -> Result<(), std::io::Error> {
    File::create(store_path()).and_then(|mut file| {
        let messages_hash: Vec<String> = get()
            .iter()
            .map(|message| message.get_hash().to_string())
            .collect();
        let text = messages_hash.join(" ");
        file.write(&text.into_bytes())?;
        Ok(())
    })
}
