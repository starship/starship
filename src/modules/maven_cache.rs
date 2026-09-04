use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::logger::get_log_dir;

const CACHE_FILE: &str = "maven-cache.json";
const LOCK_FILE: &str = "maven-cache.json.lock";
// How long (seconds) we keep retrying to acquire the cross-process lock before giving up and
// proceeding without it (best effort). Also the threshold past which a lock is considered stale.
const LOCK_TIMEOUT: Duration = Duration::from_secs(3);

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
        if let Some(path) = TEST_PATH.with(|p| p.borrow().clone()) {
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
pub fn get(binary: &Path, ttl: u64) -> Option<String> {
    let cache = load();
    let entry = cache.entries.get(&binary.to_string_lossy().into_owned())?;
    if now().saturating_sub(entry.written_at) > ttl {
        return None;
    }
    Some(entry.version.clone())
}

/// A best-effort cross-process lock backed by an atomic `create_new` lock file. Serializes
/// concurrent `set` calls so the read/merge/write sequence does not lose entries.
struct CacheLock {
    path: PathBuf,
    _file: std::fs::File,
}

impl Drop for CacheLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn acquire_lock(dir: &Path) -> Option<CacheLock> {
    let lock_path = dir.join(LOCK_FILE);
    let started = std::time::Instant::now();

    loop {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
        {
            Ok(file) => {
                return Some(CacheLock {
                    path: lock_path,
                    _file: file,
                });
            }
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                // Remove a stale lock left behind by a crashed process. Staleness is based on the
                // lock file's modification time (set atomically at creation), which avoids racing
                // with a peer that just created the file but has not written its payload yet.
                let stale = std::fs::metadata(&lock_path)
                    .and_then(|meta| meta.modified())
                    .map(|modified| modified.elapsed().unwrap_or_default() >= LOCK_TIMEOUT)
                    .unwrap_or(true);
                if stale {
                    let _ = std::fs::remove_file(&lock_path);
                    continue;
                }
                if started.elapsed() >= LOCK_TIMEOUT {
                    return None;
                }
                thread::sleep(Duration::from_millis(20));
            }
            Err(_) => return None,
        }
    }
}

/// Caches the version associated with the resolved binary path using a cross-process lock and an
/// atomic replace so that concurrent readers never observe a partially written file and concurrent
/// writers do not overwrite each other's entries.
pub fn set(binary: &Path, version: String) {
    let path = cache_path();
    let Some(dir) = path.parent() else {
        return;
    };
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }

    // Serialize the read/merge/write only when we can actually take the lock; otherwise fall back
    // to the best-effort unlocked path (a fresh `mvn --version` will be re-cached next time).
    let _lock = acquire_lock(dir);

    let mut cache = load();
    cache.entries.insert(
        binary.to_string_lossy().into_owned(),
        CacheEntry {
            version,
            written_at: now(),
        },
    );

    // Serialize to a unique temp file in the same directory, then atomically move it into place.
    let tmp = dir.join(format!("{CACHE_FILE}.tmp.{}", std::process::id()));
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
        let previous = TEST_PATH.with(|p| p.borrow().clone());
        TEST_PATH.with(|p| *p.borrow_mut() = Some(path.clone()));
        f(&path);
        TEST_PATH.with(|p| *p.borrow_mut() = previous);
        let _ = dir.close();
    }

    #[test]
    fn set_then_get_within_ttl() {
        with_temp_path(|_| {
            set(
                &PathBuf::from("/opt/mvn/4.0.0-rc-6"),
                "4.0.0-rc-6".to_string(),
            );
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

    #[test]
    fn concurrent_sets_do_not_lose_entries() {
        with_temp_path(|path| {
            let shared = path.clone();
            // Each spawned thread has its own thread-local cache path, so point every thread at
            // the same shared temp file to exercise the cross-process lock serialization.
            let handles: Vec<_> = (0..8)
                .map(|i| {
                    let target = shared.clone();
                    std::thread::spawn(move || {
                        TEST_PATH.with(|p| *p.borrow_mut() = Some(target));
                        set(&PathBuf::from(format!("/bin/{i}")), i.to_string());
                    })
                })
                .collect();
            for handle in handles {
                handle.join().unwrap();
            }

            for i in 0..8 {
                assert_eq!(
                    get(&PathBuf::from(format!("/bin/{i}")), 3600),
                    Some(i.to_string())
                );
            }
        });
    }

    #[test]
    fn lock_is_acquired_and_released() {
        with_temp_path(|_| {
            let dir = cache_path().parent().unwrap().to_path_buf();
            let lock = dir.join(LOCK_FILE);

            let guard = acquire_lock(&dir).unwrap();
            // Ownership of the lock file is exclusive as long as the guard is alive.
            assert!(lock.exists());
            drop(guard);
            // Dropping the guard releases the lock by removing the file.
            assert!(!lock.exists());
        });
    }
}
