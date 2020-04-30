use crate::configs::MessagesConfig;
use crate::formatter::StringFormatter;
use crate::segment::Segment;
use std::collections::BTreeSet;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::core::Message;

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
fn temp_dir() -> Result<PathBuf, io::Error> {
    let dir = if cfg!(windows) {
        r"C:\Windows\Temp\starship-messages"
    } else {
        "/tmp/starship-messages"
    };
    let dir = PathBuf::from(dir);

    if !dir.exists() {
        create_dir_all(&dir).map(move |_| dir)
    } else {
        Ok(dir)
    }
}

/// Get path to the store
fn store_path() -> Result<PathBuf, io::Error> {
    let session_key = env::var("STARSHIP_SESSION_KEY").unwrap_or_else(|_| "default".to_owned());

    let mut store_path = temp_dir()?;
    store_path.push(session_key);

    Ok(store_path)
}

/// Get hash of viewed messages
fn get_viewed_hash() -> Vec<u64> {
    let store_path = match store_path() {
        Ok(store_path) => store_path,
        Err(error) => {
            log::warn!(
                "Unable to get the location of current starship session: {}",
                error
            );
            return Vec::new();
        }
    };

    File::open(store_path)
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
        .unwrap_or_default()
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
                        "level" => Some(Ok(message.level.to_str())),
                        "message" => Some(Ok(message.message)),
                        _ => None,
                    })
                    .parse(None)
                    .unwrap(),
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
    let mut prev: Vec<String> = get_viewed_hash()
        .iter()
        .map(|hash| hash.to_string())
        .collect();

    File::create(store_path()?).and_then(|mut file| {
        let mut messages_hash: Vec<String> = get()
            .iter()
            .map(|message| message.get_hash().to_string())
            .collect();
        messages_hash.append(&mut prev);
        let text = messages_hash.join(" ");
        file.write_all(&text.into_bytes())?;
        Ok(())
    })
}
