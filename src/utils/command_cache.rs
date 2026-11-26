use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::{File, TryLockError};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::logger::get_log_dir;
use crate::utils::CommandOutput;

// Version the files because the payload and lock protocol differ from the old marker cache.
const CACHE_FILE: &str = "commands.v2.json";
const LOCK_FILE: &str = "commands.v2.lock";
// Hard cap on how long we keep polling a genuinely-contended lock before giving up and skipping
// the write. Live contention is released in milliseconds, so this rarely binds.
const LOCK_TIMEOUT: Duration = Duration::from_secs(10);

// Allows tests to redirect the cache location without sharing state across concurrently
// running test threads (each test thread has its own override).
#[cfg(test)]
std::thread_local! {
    static TEST_PATH: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Cache {
    #[serde(default)]
    entries: HashMap<Box<str>, CacheEntry<CommandOutput>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CacheEntry<T> {
    value: T,
    #[serde(default)]
    written_at: u64,
    #[serde(default)]
    expires_at: u64,
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
    crate::utils::read_file(cache_path())
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Builds a stable cache key for a command invocation.
///
/// The key incorporates the resolved binary path (following symlinks, bounded to avoid cycles) so
/// that a change of the underlying installation (e.g. via SDKMAN) invalidates the cached entry;
/// when the binary cannot be located the plain command string is used. When `cwd` is provided it is
/// prepended so that directory-dependent commands (e.g. `git branch`) do not collide across
/// projects. Callers whose output is context-independent (e.g. `mvn --version`) pass `None`.
///
/// Each part is encoded as `len:value` (with the byte length) so that distinct argument vectors
/// such as `['a b']` and `['a', 'b']` do not share a key. Paths and arguments are encoded
/// losslessly so directories differing only in non-UTF-8 bytes still produce distinct keys.
pub fn key<T: AsRef<OsStr> + Debug, U: AsRef<OsStr> + Debug>(
    cwd: Option<&Path>,
    cmd: T,
    args: &[U],
) -> String {
    let mut parts = Vec::new();
    if let Some(cwd) = cwd {
        parts.push(encode_part(cwd.as_os_str()));
    }
    match resolve_binary(cmd.as_ref()) {
        Some(path) => parts.push(encode_part(path.as_os_str())),
        None => parts.push(encode_part(cmd.as_ref())),
    }
    parts.extend(args.iter().map(|arg| encode_part(arg.as_ref())));
    parts
        .into_iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("")
}

/// Encodes an OS string into injective ASCII. Non-ASCII bytes (including bytes that have no UTF-8
/// interpretation, such as in a non-UTF-8 file name) are escaped as `\xHH`, and a literal backslash
/// is escaped the same way so two distinct inputs can never produce the same key.
fn encode_part(part: &OsStr) -> String {
    let mut encoded = String::new();
    for &byte in part.as_encoded_bytes() {
        if (0x20..0x7f).contains(&byte) && byte != b'\\' {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("\\x{byte:02x}"));
        }
    }
    encoded
}

/// Resolves `cmd` to its canonical real location, following up to 10 symlink hops.
fn resolve_binary(cmd: &OsStr) -> Option<PathBuf> {
    let found = which::which(cmd).ok()?;
    let mut current = found;
    for _ in 0..10 {
        match std::fs::read_link(&current) {
            Ok(target) => {
                current = if target.is_absolute() {
                    target
                } else {
                    match current.parent() {
                        Some(dir) => dir.join(target),
                        None => target,
                    }
                };
            }
            Err(_) => break,
        }
    }

    Some(dunce::canonicalize(&current).unwrap_or(current))
}

/// Returns a cached `CommandOutput` for `key` if it exists and has not expired, given a `ttl` in
/// seconds. A `ttl` of `0` (or less) disables the cache and always returns `None`.
pub fn get(key: &str, ttl: u64) -> Option<CommandOutput> {
    if ttl == 0 {
        return None;
    }
    let cache = load();
    let entry = cache.entries.get(key)?;
    let now = now();
    if now >= entry.expires_at || now.saturating_sub(entry.written_at) >= ttl {
        return None;
    }
    Some(entry.value.clone())
}

/// Serializes writers using an OS lock, released even if the process crashes.
/// Never unlink the file: a waiter may already have opened the same inode.
fn acquire_lock(dir: &Path) -> Option<File> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(dir.join(LOCK_FILE))
        .ok()?;
    let started = Instant::now();
    loop {
        match file.try_lock() {
            Ok(()) => return Some(file),
            Err(TryLockError::WouldBlock) => {
                if started.elapsed() >= LOCK_TIMEOUT {
                    return None;
                }
                thread::sleep(Duration::from_millis(20));
            }
            Err(TryLockError::Error(_)) => return None,
        }
    }
}

/// Caches the `CommandOutput` associated with `key`. A `ttl` of `0` (or less) disables the cache
/// and is a no-op. The write is serialized with a cross-process lock and published atomically so
/// concurrent readers never observe a partially written file and concurrent writers do not
/// overwrite each other's entries.
pub fn set(key: &str, output: CommandOutput, ttl: u64) {
    if ttl == 0 {
        return;
    }
    let path = cache_path();
    let Some(dir) = path.parent() else {
        return;
    };
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }

    // Serialize the read/merge/write under the cross-process lock. If the lock cannot be acquired
    // we skip the write entirely: writing unlocked could overwrite another process's entry.
    let Some(_lock) = acquire_lock(dir) else {
        return;
    };

    let mut cache = load();
    let now = now();
    cache.entries.retain(|_, entry| now < entry.expires_at);
    cache.entries.insert(
        key.into(),
        CacheEntry {
            value: output,
            written_at: now,
            expires_at: now.saturating_add(ttl),
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

/// Runs a test against its own cache, restoring the override even on panic.
#[cfg(test)]
pub fn with_temp_path<F: FnOnce(&PathBuf)>(f: F) {
    struct RestorePath(Option<PathBuf>);

    impl Drop for RestorePath {
        fn drop(&mut self) {
            TEST_PATH.with(|p| *p.borrow_mut() = self.0.take());
        }
    }

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(CACHE_FILE);
    let _restore = RestorePath(TEST_PATH.with(|p| p.replace(Some(path.clone()))));
    f(&path);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_output() -> CommandOutput {
        CommandOutput {
            stdout: String::from("out"),
            stderr: String::from("err"),
        }
    }

    #[test]
    fn set_then_get_within_ttl() {
        with_temp_path(|_| {
            set("mvn --version", sample_output(), 3600);
            let got = get("mvn --version", 3600).unwrap();
            assert_eq!(got.stdout, "out");
            assert_eq!(got.stderr, "err");
        });
    }

    #[test]
    fn ttl_zero_disables_cache() {
        with_temp_path(|_| {
            set("cmd", sample_output(), 0);
            assert_eq!(get("cmd", 0), None);
            assert_eq!(get("cmd", 3600), None);
        });
    }

    #[test]
    fn different_keys_are_isolated() {
        with_temp_path(|_| {
            set("/a", sample_output(), 3600);
            set(
                "/b",
                CommandOutput {
                    stdout: "2".to_string(),
                    stderr: String::new(),
                },
                3600,
            );
            assert_eq!(get("/a", 3600).unwrap().stdout, "out");
            assert_eq!(get("/b", 3600).unwrap().stdout, "2");
        });
    }

    #[test]
    fn stale_entry_is_a_miss() {
        with_temp_path(|path| {
            let mut cache = Cache::default();
            cache.entries.insert(
                "/mvn".into(),
                CacheEntry {
                    value: sample_output(),
                    written_at: now() - 10_000,
                    expires_at: now() - 6400,
                },
            );
            std::fs::write(path, serde_json::to_vec(&cache).unwrap()).unwrap();

            assert_eq!(get("/mvn", 3600), None);
        });
    }

    #[test]
    fn missing_entry_is_a_miss() {
        with_temp_path(|_| {
            assert_eq!(get("/nope", 3600), None);
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
                        set(
                            &format!("/bin/{i}"),
                            CommandOutput {
                                stdout: i.to_string(),
                                stderr: String::new(),
                            },
                            3600,
                        );
                    })
                })
                .collect();
            for handle in handles {
                handle.join().unwrap();
            }

            for i in 0..8 {
                let got = get(&format!("/bin/{i}"), 3600).unwrap();
                assert_eq!(got.stdout, i.to_string());
            }
        });
    }

    #[test]
    fn output_round_trips_special_characters() {
        with_temp_path(|_| {
            for (stdout, stderr) in [
                ("line\x00with nul", "err\x00too"),
                ("out", ""),
                ("", ""),
                ("\u{1f680}\n\"\\", "\u{e9}\r\t"),
            ] {
                let output = CommandOutput {
                    stdout: stdout.into(),
                    stderr: stderr.into(),
                };
                set("cmd", output.clone(), 3600);
                assert_eq!(get("cmd", 3600), Some(output));
            }
        });
    }

    #[test]
    fn output_is_stored_as_compact_strings() {
        with_temp_path(|path| {
            let output = CommandOutput {
                stdout: "x".repeat(4096),
                stderr: "y".repeat(4096),
            };
            set("cmd", output.clone(), 3600);
            let data = crate::utils::read_file(path).unwrap();
            assert!(data.len() < output.stdout.len() + output.stderr.len() + 200);
            let json: serde_json::Value = serde_json::from_str(&data).unwrap();
            assert_eq!(json["entries"]["cmd"]["value"]["stdout"], output.stdout);
            assert_eq!(get("cmd", 3600), Some(output));
        });
    }

    #[test]
    fn malformed_payload_is_a_cache_miss() {
        with_temp_path(|path| {
            for content in [
                r#"{"entries":{"cmd":{"value":[255,255],"written_at":0}}}"#,
                r#"{"entries":{"cmd":{"value":{"stdout":"out"},"written_at":0}}}"#,
                "not json",
            ] {
                std::fs::write(path, content).unwrap();
                assert_eq!(get("cmd", 3600), None);
                set("cmd", sample_output(), 3600);
                assert_eq!(get("cmd", 3600), Some(sample_output()));
            }
        });
    }

    #[test]
    fn writes_prune_expired_entries_using_their_own_ttl() {
        with_temp_path(|path| {
            let now = now();
            let mut cache = Cache::default();
            for (key, written_at, expires_at) in [
                ("expired", now - 100, now - 1),
                ("boundary", now - 100, now),
                ("long-lived", now - 100, now + 3600),
            ] {
                cache.entries.insert(
                    key.into(),
                    CacheEntry {
                        value: sample_output(),
                        written_at,
                        expires_at,
                    },
                );
            }
            std::fs::write(path, serde_json::to_vec(&cache).unwrap()).unwrap();
            assert_eq!(get("expired", 3600), None);
            assert_eq!(get("boundary", 3600), None);
            // A shorter caller TTL is still respected.
            assert_eq!(get("long-lived", 1), None);
            set("new", sample_output(), 1);
            let cache = load();
            assert_eq!(cache.entries.len(), 2);
            assert!(cache.entries.contains_key("new"));
            assert!(cache.entries.contains_key("long-lived"));
            assert_eq!(get("long-lived", 3600), Some(sample_output()));
        });
    }

    #[test]
    fn key_distinguishes_argument_boundaries() {
        // `['mvn', '--version']` must not collide with a single argument that contains a space.
        assert_ne!(
            key(None, "cmd", &["a b"] as &[&str; 1]),
            key(None, "cmd", &["a", "b"] as &[&str; 2]),
        );
    }

    #[test]
    fn key_includes_cwd_when_provided() {
        let a = key(Some(Path::new("/proj/a")), "git", &["branch"]);
        let b = key(Some(Path::new("/proj/b")), "git", &["branch"]);
        assert_ne!(a, b);
    }

    #[test]
    fn key_is_stable_without_cwd() {
        let a = key(None, "git", &["branch"]);
        let b = key(None, "git", &["branch"]);
        assert_eq!(a, b);
    }

    #[cfg(unix)]
    #[test]
    fn key_distinguishes_non_utf8_directories() {
        use std::os::unix::ffi::OsStrExt;

        let cwd_a = Path::new(OsStr::from_bytes(b"/proj/foo\xff"));
        let cwd_b = Path::new(OsStr::from_bytes(b"/proj/foo\xfe"));
        let a = key(Some(cwd_a), "git", &["branch"]);
        let b = key(Some(cwd_b), "git", &["branch"]);
        assert_ne!(a, b);
    }

    #[test]
    fn cache_lock_child() {
        let Some(dir) = std::env::var_os("STARSHIP_CACHE_LOCK_TEST_DIR") else {
            return;
        };
        let dir = PathBuf::from(dir);
        let _guard = acquire_lock(&dir).unwrap();
        std::fs::write(dir.join("ready"), b"").unwrap();
        loop {
            thread::sleep(Duration::from_millis(20));
        }
    }

    #[test]
    fn cache_lock_survives_live_owner_and_recovers_after_crash() {
        struct ChildGuard(std::process::Child);
        impl Drop for ChildGuard {
            fn drop(&mut self) {
                let _ = self.0.kill();
                let _ = self.0.wait();
            }
        }

        let dir = tempfile::tempdir().unwrap();
        let mut child = ChildGuard(
            crate::utils::create_command(std::env::current_exe().unwrap())
                .unwrap()
                .args(["--exact", "utils::command_cache::tests::cache_lock_child"])
                .env("STARSHIP_CACHE_LOCK_TEST_DIR", dir.path())
                .stdout(std::process::Stdio::null())
                .spawn()
                .unwrap(),
        );
        let started = Instant::now();
        while !dir.path().join("ready").exists() {
            assert!(child.0.try_wait().unwrap().is_none(), "child exited early");
            assert!(started.elapsed() < Duration::from_secs(10));
            thread::sleep(Duration::from_millis(20));
        }

        let target = dir.path().to_path_buf();
        let (tx, rx) = std::sync::mpsc::channel();
        let waiter = thread::spawn(move || {
            let guard = acquire_lock(&target);
            tx.send(guard.is_some()).unwrap();
        });
        // A live owner must not be evicted after the old two-second grace period.
        let early = rx.recv_timeout(Duration::from_secs(3));
        child.0.kill().unwrap();
        child.0.wait().unwrap();
        waiter.join().unwrap();
        assert!(matches!(
            early,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout)
        ));
        assert!(rx.recv_timeout(Duration::from_secs(1)).unwrap());
        // Process death releases the OS lock without needing to replace its file.
        assert!(acquire_lock(dir.path()).is_some());
    }

    #[test]
    fn lock_is_acquired_and_released() {
        with_temp_path(|_| {
            let dir = cache_path().parent().unwrap().to_path_buf();
            let guard = acquire_lock(&dir).unwrap();
            let peer = File::options()
                .read(true)
                .write(true)
                .open(dir.join(LOCK_FILE))
                .unwrap();
            assert!(matches!(peer.try_lock(), Err(TryLockError::WouldBlock)));
            drop(guard);
            // Retaining the same inode prevents a waiter from locking an unlinked file.
            assert!(dir.join(LOCK_FILE).exists());
            peer.try_lock().unwrap();
        });
    }
}
