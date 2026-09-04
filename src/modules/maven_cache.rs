use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::logger::get_log_dir;

const CACHE_FILE: &str = "maven-cache.json";

// Allows tests to redirect the cache location without sharing state across concurrently
// running test threads (each test thread has its own override).
#[cfg(test)]
std::thread_local! {
    static TEST_PATH: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Cache {
    #[serde(default)]
    entries: HashMap<String, CacheEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CacheEntry {
    version: String,
    #[serde(default)]
    written_at: u64,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default()
}

/// The persistent cache file location.
fn cache_path() -> PathBuf {
    #[cfg(test)]
    {
        if let Some(path) = TEST_PATH.with(|p| *p.borrow()) {
            return path;
        }
    }

    get_log_dir().join(CACHE_FILE)
}

fn load() -> Cache {
    std::fs::read_to_string(cache_path())
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Returns the cached version for the given resolved binary path if it is still fresh.
pub fn get(binary: &PathBuf, ttl: u64) -> Option<String> {
    let cache = load();
    let entry = cache.entries.get(&binary.to_string_lossy().into_owned())?;
    if now().saturating_sub(entry.written_at) > ttl {
        return None;
    }
    Some(entry.version.clone())
}

/// Caches the version associated with the resolved binary path using an atomic replace so that
/// concurrent readers never observe a partially written file.
pub fn set(binary: &PathBuf, version: String) {
    let path = cache_path();
    let Some(dir) = path.parent() else {
        return;
    };
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }

    let mut cache = load();
    cache
        .entries
        .insert(binary.to_string_lossy().into_owned(), CacheEntry { version, written_at: now() });

    // Serialize to a unique temp file in the same directory, then atomically move it into place.
    let tmp = dir.join(format!(
        "{CACHE_FILE}.tmp.{}",
        std::process::id()
    ));
    if std::fs::write(&tmp, serde_json::to_vec(&cache).unwrap_or_default()).is_err() {
        return;
    }

    if std::fs::rename(&tmp, &path).is_err() {
        // On Windows `rename` fails when the destination exists; fall back to a replace.
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::rename(&tmp, &path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_temp_path<F: FnOnce(&PathBuf)>(f: F) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(CACHE_FILE);
        // Set the override for this thread while the closure runs, then restore it.
        let previous = TEST_PATH.with(|p| p.borrow_mut().replace(Some(path.clone())));
        f(&path);
        TEST_PATH.with(|p| *p.borrow_mut() = previous);
        let _ = dir.close();
    }

    #[test]
    fn set_then_get_within_ttl() {
        with_temp_path(|_| {
            set(&PathBuf::from("/opt/mvn/4.0.0-rc-6"), "4.0.0-rc-6".to_string());
            assert_eq!(
                get(&PathBuf::from("/opt/mvn/4.0.0-rc-6"), 3600),
                Some("4.0.0-rc-6".to_string())
            );
        });
    }

    #[test]
    fn different_binary_is_isolated() {
        with_temp_path(|_| {
            set(&PathBuf::from("/a"), "1".to_string());
            set(&PathBuf::from("/b"), "2".to_string());
            assert_eq!(get(&PathBuf::from("/a"), 3600), Some("1".to_string()));
            assert_eq!(get(&PathBuf::from("/b"), 3600), Some("2".to_string()));
        });
    }

    #[test]
    fn stale_entry_is_a_miss() {
        with_temp_path(|path| {
            std::fs::write(
                path,
                serde_json::to_vec(&Cache {
                    entries: HashMap::from([(
                        "/mvn".to_string(),
                        CacheEntry {
                            version: "old".to_string(),
                            written_at: now() - 10_000,
                        },
                    )]),
                })
                .unwrap(),
            )
            .unwrap();

            assert_eq!(get(&PathBuf::from("/mvn"), 3600), None);
        });
    }

    #[test]
    fn missing_entry_is_a_miss() {
        with_temp_path(|_| {
            assert_eq!(get(&PathBuf::from("/nope"), 3600), None);
        });
    }
}