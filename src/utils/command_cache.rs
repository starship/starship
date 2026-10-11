use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::logger::get_log_dir;
use crate::utils::CommandOutput;

const CACHE_FILE: &str = "commands.json";
const LOCK_FILE: &str = "commands.json.lock";
// How old a lock marker must be before it is assumed abandoned (left behind by a crashed
// process) and reclaimed. Writers only hold the lock for milliseconds, so this short grace makes
// recovery prompt without evicting an active holder.
const LOCK_GRACE: Duration = Duration::from_secs(2);
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
    entries: HashMap<Box<str>, CacheEntry<Box<[u8]>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CacheEntry<T> {
    value: T,
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

fn serialize(output: &CommandOutput) -> Box<[u8]> {
    let stdout = output.stdout.as_bytes();
    let stderr = output.stderr.as_bytes();
    let stdout_len = u64::try_from(stdout.len()).unwrap_or(u64::MAX);
    let stderr_len = u64::try_from(stderr.len()).unwrap_or(u64::MAX);
    let mut data = Vec::with_capacity(16 + stdout.len() + stderr.len());
    // Length-prefixed encoding so that arbitrary output (including a NUL byte) round-trips
    // without ambiguity between stdout and stderr. Lengths are stored as 64-bit values so their
    // conversion never wraps on any supported platform.
    data.extend_from_slice(&stdout_len.to_le_bytes());
    data.extend_from_slice(stdout);
    data.extend_from_slice(&stderr_len.to_le_bytes());
    data.extend_from_slice(stderr);
    data.into_boxed_slice()
}

/// Decodes a value written by [`serialize`]. Returns `None` when the payload is truncated or
/// declares a length beyond its own data so callers can treat malformed entries as a cache miss
/// instead of silently returning empty output.
fn deserialize(data: &[u8]) -> Option<CommandOutput> {
    let mut offset = 0;
    let stdout_len = read_length(data, &mut offset)?;
    let stdout = data.get(offset..offset.checked_add(stdout_len)?)?;
    offset = offset.checked_add(stdout_len)?;
    let stderr_len = read_length(data, &mut offset)?;
    let stderr = data.get(offset..offset.checked_add(stderr_len)?)?;
    Some(CommandOutput {
        stdout: String::from_utf8_lossy(stdout).into_owned(),
        stderr: String::from_utf8_lossy(stderr).into_owned(),
    })
}

/// Reads a 64-bit little-endian length from `bytes` starting at `offset`, advancing `offset` past
/// it. Returns `None` on a truncated, out-of-range, or unrepresentable payload.
fn read_length(bytes: &[u8], offset: &mut usize) -> Option<usize> {
    let start = *offset;
    let end = start.checked_add(8)?;
    let raw = bytes.get(start..end)?;
    let len = u64::from_le_bytes(raw.try_into().ok()?);
    let len = usize::try_from(len).ok()?;
    *offset = end;
    Some(len)
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
    if now().saturating_sub(entry.written_at) > ttl {
        return None;
    }
    deserialize(&entry.value)
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
    let started = Instant::now();

    loop {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
        {
            Ok(file) => {
                // Record the owner PID so a peer can reason about ownership and (combined with the
                // age guard) recover promptly if this process crashes.
                let _ = std::fs::write(&lock_path, std::process::id().to_string());
                return Some(CacheLock {
                    path: lock_path,
                    _file: file,
                });
            }
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                // Ownership-aware recovery: a lock owned by the current process is never reclaimed
                // (it is an active re-entrant holder), and a recently-created marker is assumed to
                // belong to a live peer. Older markers are treated as abandoned and reclaimed
                // immediately, so a crashed process does not block the synchronous module for long.
                if lock_may_be_live(&lock_path) {
                    if started.elapsed() >= LOCK_TIMEOUT {
                        return None;
                    }
                    thread::sleep(Duration::from_millis(20));
                } else {
                    let _ = std::fs::remove_file(&lock_path);
                    continue;
                }
            }
            Err(_) => return None,
        }
    }
}

/// True while the current lock holder may still be active: either it is owned by this process, or
/// the marker is younger than the grace period. Otherwise the marker is assumed abandoned.
fn lock_may_be_live(lock_path: &Path) -> bool {
    let owns_lock = std::fs::read_to_string(lock_path)
        .ok()
        .and_then(|content| content.trim().parse::<u32>().ok())
        .is_some_and(|pid| pid == std::process::id());
    if owns_lock {
        return true;
    }
    std::fs::metadata(lock_path)
        .and_then(|meta| meta.modified())
        .map(|modified| modified.elapsed().unwrap_or_default() < LOCK_GRACE)
        .unwrap_or(true)
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
    cache.entries.insert(
        key.into(),
        CacheEntry {
            value: serialize(&output),
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
                    value: serialize(&sample_output()),
                    written_at: now() - 10_000,
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
    fn serialization_round_trips_nul_bytes() {
        let output = CommandOutput {
            stdout: "line\x00with nul".to_string(),
            stderr: "err\x00too".to_string(),
        };
        let data = serialize(&output);
        let back = deserialize(&data).unwrap();
        assert_eq!(back.stdout, "line\x00with nul");
        assert_eq!(back.stderr, "err\x00too");
    }

    #[test]
    fn serialization_round_trips_empty_stderr() {
        let output = CommandOutput {
            stdout: "out".to_string(),
            stderr: String::new(),
        };
        let back = deserialize(&serialize(&output)).unwrap();
        assert_eq!(back.stdout, "out");
        assert_eq!(back.stderr, "");
    }

    #[test]
    fn deserialize_rejects_truncated_payload() {
        let full = serialize(&sample_output());
        for cut in 0..full.len() {
            assert_eq!(deserialize(&full[..cut]), None, "cut at {cut}");
        }
    }

    #[test]
    fn deserialize_rejects_oversized_length() {
        // A header declaring more bytes than the payload holds must be a miss, not empty output.
        let mut data = vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        data.extend_from_slice(b"short");
        assert_eq!(deserialize(&data), None);
    }

    #[test]
    fn malformed_payload_is_a_cache_miss() {
        with_temp_path(|path| {
            let mut cache = Cache::default();
            cache.entries.insert(
                "/mvn".into(),
                CacheEntry {
                    value: vec![0xff, 0xff, 0xff, 0xff].into_boxed_slice(),
                    written_at: now(),
                },
            );
            std::fs::write(path, serde_json::to_vec(&cache).unwrap()).unwrap();

            assert_eq!(get("/mvn", 3600), None);
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
