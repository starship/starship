//! The on-disk caching functionality for Starship.
//!
//! This module contains the caching mechanism allowing Starship to reuse the
//! output of previously run commands when possible.
//!
//! The cache stores the output of commands, and the metadata of the binaries
//! being called at the time the command is run. When the binary's metadata
//! changes, the cache clears all the values of the commands calling that binary.
//!
//! The goals of this library are to be quick to cache outputs, quick to retreive
//! cached values, compatible with version-managed tools, and easy to troubleshoot.

pub mod errors;

pub use errors::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::{self, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
    process::Output,
    time::UNIX_EPOCH,
};

type FullCommand = String;
type BinaryPath = PathBuf;

const CURRENT_VERSION: u8 = 1;

/// An instance of the binary output cache
pub struct Cache {
    /// The path of the cache file the cache serializes to
    path: PathBuf,
    /// Whether the cache has been changed and requires writing to disk
    changed: bool,
    /// The cache's internal state
    contents: CacheContents,
}

impl Cache {
    /// Create or parse a cache file at the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .map_err(Error::OpenFile)?;
        let mut contents = String::new();

        // Clear the cache if it is not valid UTF-8
        file.read_to_string(&mut contents).unwrap_or_default();

        // Clear the cache if it unable to be parsed
        let mut cache: CacheContents = toml::from_str(&contents).unwrap_or_default();

        // Clear the cache if it is not using the current version
        if cache.version != CURRENT_VERSION {
            cache = CacheContents::default();
        }

        Ok(Self {
            path: path.as_ref().to_owned(),
            changed: false,
            contents: cache,
        })
    }

    /// Get the output of the given command if it has been previously cached
    pub fn get(&mut self, binary_path: &Path, command: &str) -> Option<&CachedOutput> {
        let bin = self.contents.binaries.get(binary_path)?;

        let current_metadata = BinaryMetadata::try_from(binary_path).ok()?;
        let is_stale = current_metadata != bin.metadata;
        if is_stale {
            return None;
        };

        bin.commands.get(command)
    }

    /// Set the cached output of the given command
    pub fn set<O: Into<CachedOutput>>(&mut self, binary_path: &Path, command: &str, output: O) {
        let current_metadata = match BinaryMetadata::try_from(binary_path) {
            Ok(metadata) => metadata,
            // Skip caching if unable to read binary metadata
            Err(_e) => return,
        };
        let mut bin = self
            .contents
            .binaries
            .entry(binary_path.to_path_buf())
            .or_insert(BinaryCache {
                metadata: current_metadata.clone(),
                commands: HashMap::new(),
            });

        let is_stale = current_metadata != bin.metadata;
        if is_stale {
            bin.metadata = current_metadata;
            bin.commands.clear();
        };

        bin.commands.insert(command.to_owned(), output.into());
        self.changed = true;
    }

    /// Write any cache updates to disk
    pub fn write(&self) -> Result<(), Error> {
        if !self.changed {
            return Ok(());
        };

        let contents = toml::to_string(&self.contents).map_err(Error::SerializeCache)?;
        fs::write(&self.path, contents).map_err(Error::WriteFile)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CacheContents {
    /// The version of the cache file
    version: u8,
    /// A mapping of binaries' paths and their caches
    binaries: HashMap<BinaryPath, BinaryCache>,
}

impl Default for CacheContents {
    fn default() -> Self {
        Self {
            version: CURRENT_VERSION,
            binaries: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BinaryCache {
    /// The metadata of the binary at the time it was last called
    /// If the binary's metadata changes, its cached data is cleared
    metadata: BinaryMetadata,
    /// A mapping of commands and their cached outputs
    commands: HashMap<FullCommand, CachedOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status: Option<i32>,
}

impl CachedOutput {
    pub fn success(&self) -> bool {
        self.status == Some(0)
    }
}

impl From<Output> for CachedOutput {
    fn from(output: Output) -> Self {
        Self {
            stdout: output.stdout,
            stderr: output.stderr,
            status: output.status.code()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct BinaryMetadata {
    size: u64,
    is_dir: bool,
    is_file: bool,
    readonly: bool,
    c_time: u64,
    m_time: u64,
}

impl TryFrom<&Path> for BinaryMetadata {
    type Error = crate::Error;

    fn try_from(path: &Path) -> Result<Self, Error> {
        let metadata = fs::metadata(path).map_err(Error::ReadMetadata)?;

        // If ctime or mtime are not provided, store `0` in their place
        let c_time = match metadata.created() {
            Err(_e) => 0,
            Ok(t) => t
                .duration_since(UNIX_EPOCH)
                .map(|t| t.as_secs())
                .unwrap_or(0),
        };

        let m_time = match metadata.modified() {
            Err(_e) => 0,
            Ok(t) => t
                .duration_since(UNIX_EPOCH)
                .map(|t| t.as_secs())
                .unwrap_or(0),
        };

        Ok(Self {
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            readonly: metadata.permissions().readonly(),
            c_time,
            m_time,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Write};
    use tempfile::tempdir;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn empty_cache_file_is_created() -> Result {
        let dir = tempdir()?;
        let cache_path = Path::join(dir.path(), "bin-cache");
        let cache = Cache::new(&cache_path)?;
        cache.write()?;

        assert!(Path::exists(&cache_path));
        Ok(())
    }

    #[test]
    fn retreive_from_populated_cache() -> Result {
        let dir = tempdir()?;
        let cache_path = dir.path().join("bin-cache");
        let mut cache = Cache::new(&cache_path)?;

        // Create "node" binary
        let bin_path = dir.path().join("node");
        File::create(&bin_path)?;

        // Populate cache with "node" output
        let expected = "v14.16.0";
        cache.set(&bin_path, "node --version", &expected);
        cache.write()?;

        // Retreive cached output
        let mut new_cache = Cache::new(&cache_path)?;
        let actual = new_cache.get(&bin_path, "node --version").unwrap();

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn overrites_stale_cache() -> Result {
        let dir = tempdir()?;
        let cache_path = dir.path().join("bin-cache");
        let mut cache = Cache::new(&cache_path)?;

        // Create "node" binary
        let bin_path = dir.path().join("node");
        File::create(&bin_path)?;

        // Populate cache with "node" output
        let expected = "v14.16.0";
        cache.set(&bin_path, "node -v", &expected);
        cache.set(&bin_path, "node --help", &expected);
        cache.set(&bin_path, "node --version", &expected);
        cache.write()?;

        // Update "node" binary
        File::create(&bin_path)?.write(b"updated")?;

        // Retreive cached output
        let mut new_cache = Cache::new(&cache_path)?;

        // Set a cached value again
        new_cache.set(&bin_path, "node -v", "v15.0.0");

        // The other, previously cached values, should be cleared as stale
        assert_eq!(new_cache.get(&bin_path, "node --version"), None);
        assert_eq!(new_cache.get(&bin_path, "node --help"), None);
        Ok(())
    }

    #[test]
    fn doesnt_retreive_stale_cache() -> Result {
        let dir = tempdir()?;
        let cache_path = dir.path().join("bin-cache");
        let mut cache = Cache::new(&cache_path)?;

        // Create "node" binary
        let bin_path = dir.path().join("node");
        File::create(&bin_path)?;

        // Populate cache with "node" output
        cache.set(&bin_path, "node --version", "v14.16.0");
        cache.write()?;

        // Update "node" binary
        File::create(&bin_path)?.write(b"updated")?;

        let mut new_cache = Cache::new(&cache_path)?;
        let actual = new_cache.get(&bin_path, "node --version");

        assert_eq!(None, actual);
        Ok(())
    }
}
