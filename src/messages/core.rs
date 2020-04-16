use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn to_str(&self) -> &'static str {
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
    pub message: &'static str,
    pub level: LogLevel,
}

impl Message {
    pub fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        s.write(&self.message.to_string().into_bytes());
        s.finish()
    }
}
