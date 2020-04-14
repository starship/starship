use crate::configs::{MessageDef, Messages};
use crate::segment::Segment;
use crate::utils;
use dirs::audio_dir;
use serde::{Deserialize, Deserializer, Serialize};
use std::env;
use std::fs::{create_dir, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    DeprecatedUseFormat,
}

impl fmt::Display for MessageDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::DeprecatedUseFormat => write!(
                f,
                "Starship uses `format` key to customize the appearance of modules from v0.41.0,\
                Check https://starship.rs for more info on updating your config."
            ),
        }
    }
}

static messages: Arc<Mutex<Vec<Message>>> = Vec::new();

/// Add a message
pub fn add(&self, message: Message) {
    let messages: Arc<Mutex<Vec<Message>>> = Arc::clone(messages);
    let mut messages = messages.lock().unwrap();
    messages.push(Message);
}

/// Get messages
fn get() -> Vec<Message> {
    let messages: Arc<Mutex<Vec<Message>>> = Arc::clone(messages);
    let messages = messages.lock().unwrap();
    messages.clone()
}

/// Get temporary directory
fn temp_dir() -> &str {
    if cfg!(windows) {
        r"C:\Windows\Temp"
    } else {
        "/tmp"
    }
}

/// Get path to the store
fn store_path() -> PathBuf {
    const storage_dir: &str = "starship_messages";

    let ppid = env::var("PPID").unwrap_or("0");
    let mut store_path = PathBuf::from(temp_dir());

    store_path.push(storage_dir);
    if (!storage_dir.exists()) {
        create_dir(&storage_dir);
    }
    store_path.push(ppid);
    store_path.set_extension("json");
}

/// Get messages not viewed
pub fn get_not_viewed() -> Vec<Message> {
    if store_path.exists() {
        File::open(store_path())
            .map(|file| {
                let mut reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap()
            })
            .unwrap_or(Vec::new())
    } else {
        Vec::new()
    }
}

/// Update viewed messages
pub fn update_viewed() -> Result<()> {
    File::create(store_path()).map(|file| serde_json::to_writer(file, get()))
}
