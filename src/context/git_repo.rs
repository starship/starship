use std::ffi::OsStr;
use std::fmt::{Debug, Write as _};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

use gix::{Repository, ThreadSafeRepository, state as git_state};

use crate::utils::{self, CommandOutput, create_command, exec_timeout};
use gix::sec::Trust as TrustLevel;

use super::Context;

pub struct GitRepo {
    pub repo: ThreadSafeRepository,

    /// If `current_dir` is a git repository or is contained within one,
    /// this is the short name of the current branch name of that repo,
    /// i.e. `main`.
    pub branch: Option<String>,

    /// If `current_dir` is a git repository or is contained within one,
    /// this is the path to the root of that repo.
    pub workdir: Option<PathBuf>,

    /// The path of the repository's `.git` directory.
    pub path: PathBuf,

    /// State
    pub state: Option<git_state::InProgress>,

    /// Remote repository
    pub remote: Option<GitRemote>,

    /// Contains `true` if the value of `core.fsmonitor` is set to `true`.
    /// If not `true`, `fsmonitor` is explicitly disabled in git commands.
    pub(crate) fs_monitor_value_is_true: bool,

    /// The trust level of the repository's `.git` directory.
    pub trust: TrustLevel,
}

/// Remote repository
pub struct GitRemote {
    pub branch: Option<String>,
    pub name: Option<String>,
}

impl GitRepo {
    /// Opens the associated git repository.
    pub fn open(&self) -> Repository {
        self.repo.to_thread_local()
    }

    pub(crate) fn uses_reftables(&self) -> bool {
        self.open()
            .config_snapshot()
            .string("extensions.refstorage")
            .is_some_and(|kind| kind == "reftable")
    }

    /// Select the Git executable for status and metrics only in fully trusted repositories.
    pub(crate) fn use_git_executable(&self, enabled: bool) -> Option<bool> {
        // TODO: remove the sparse-index fallback once gitoxide supports tree-index comparisons.
        let use_git = enabled
            || self.fs_monitor_value_is_true
            || self.uses_reftables()
            || self.open().index_or_empty().ok()?.is_sparse();
        if use_git && self.trust != TrustLevel::Full {
            log::info!(
                "Not using the Git executable for status or metrics because repository trust is reduced."
            );
            return Some(false);
        }
        Some(use_git)
    }

    /// Wrapper to execute external git commands.
    /// Handles adding the appropriate `--git-dir` and `--work-tree` flags to the command.
    /// Also handles additional features required for security, such as disabling `fsmonitor`.
    /// Requires full repository trust.
    /// At this time, mocking is not supported.
    pub fn exec_git<T: AsRef<OsStr> + Debug>(
        &self,
        context: &Context,
        git_args: impl IntoIterator<Item = T>,
    ) -> Option<CommandOutput> {
        self.exec_git_with_required_trust(context, TrustLevel::Full, git_args)
    }

    /// Execute a Git command only if the repository meets the required trust level.
    /// Callers allowing reduced trust must ensure the command and its arguments cannot
    /// execute repository-configured external commands (such as filters or hooks).
    pub fn exec_git_with_required_trust<T: AsRef<OsStr> + Debug>(
        &self,
        context: &Context,
        minimum_trust: TrustLevel,
        git_args: impl IntoIterator<Item = T>,
    ) -> Option<CommandOutput> {
        let repo_trust = self.trust;
        if minimum_trust > repo_trust {
            log::warn!(
                "Not executing git command because the minimum trust level {minimum_trust:?} is greater than the repository trust level {repo_trust:?}."
            );
            return None;
        }

        let mut command = create_command("git").ok()?;

        // A value of `true` should not execute external commands.
        let fsm_config_value = if self.fs_monitor_value_is_true {
            "core.fsmonitor=true"
        } else {
            "core.fsmonitor="
        };

        command.env("GIT_OPTIONAL_LOCKS", "0").args([
            OsStr::new("-C"),
            context.current_dir.as_os_str(),
            OsStr::new("--git-dir"),
            self.path.as_os_str(),
            OsStr::new("-c"),
            OsStr::new(fsm_config_value),
        ]);

        // Bare repositories might not have a workdir, so we need to check for that.
        if let Some(wt) = self.workdir.as_ref() {
            command.args([OsStr::new("--work-tree"), wt.as_os_str()]);
        }

        command.args(git_args);
        log::trace!("Executing git command: {command:?}");

        exec_timeout(
            &mut command,
            Duration::from_millis(context.root_config.command_timeout),
        )
    }
}

/// Location of Git's installation-level config (e.g. `/opt/homebrew/etc/gitconfig`).
pub(crate) enum InstallationConfig {
    /// Unknown, let gix discover it by running `git config` (adds a process spawn per prompt).
    Discover,
    /// Known location, or `None` if Git reports no installation config.
    Known(Option<PathBuf>),
}

const INSTALLATION_CONFIG_CACHE: &str = "git-installation-config";
/// How long to trust a cached "no installation config" result.
const NEGATIVE_CACHE_TTL: Duration = Duration::from_secs(60 * 60);

/// Resolve Git's installation config path without spawning `git` on every prompt.
///
/// gix finds this file by running `git config --show-origin`, which costs 15-30ms per prompt.
/// The result only changes when the `git` binary does, so it is cached on disk keyed by the
/// path and modification time of the `git` found in `PATH`.
pub(crate) fn installation_config(context: &Context) -> InstallationConfig {
    // These change which file gix picks, so leave discovery to gix.
    // With `EXEPATH` (Git for Windows), gix derives the path without spawning `git` anyway.
    if ["GIT_CONFIG_SYSTEM", "GIT_CONFIG_NOSYSTEM", "DEVELOPER_DIR"]
        .iter()
        .chain(cfg!(windows).then_some(&"EXEPATH"))
        .any(|var| context.get_env_os(var).is_some())
    {
        return InstallationConfig::Discover;
    }
    let (Some(cache_file), Some(key)) = (installation_config_cache_file(context), git_binary_key())
    else {
        return InstallationConfig::Discover;
    };

    if let Ok(content) = utils::read_file(&cache_file)
        && let Some(path) = parse_installation_config_cache(&content, &key)
        // A config file can be created without the git binary changing, so recheck its absence periodically
        && (path.is_some() || modified_within(&cache_file, NEGATIVE_CACHE_TTL))
    {
        log::trace!("Using cached git installation config {path:?}");
        return InstallationConfig::Known(path);
    }

    let path = gix::path::env::installation_config().map(Path::to_path_buf);
    log::debug!("Discovered git installation config {path:?}, caching it in {cache_file:?}");
    if let Err(e) = write_installation_config_cache(&cache_file, &key, path.as_deref()) {
        log::debug!("Unable to cache git installation config: {e}");
    }
    InstallationConfig::Known(path)
}

fn modified_within(path: &Path, ttl: Duration) -> bool {
    fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .is_some_and(|age| age < ttl)
}

fn installation_config_cache_file(context: &Context) -> Option<PathBuf> {
    let dir = match context.get_env_os("STARSHIP_CACHE") {
        Some(dir) => PathBuf::from(dir),
        // Never write to the real home directory from tests
        None if cfg!(test) => return None,
        None => context.get_home()?.join(".cache").join("starship"),
    };
    Some(dir.join(INSTALLATION_CONFIG_CACHE))
}

/// Identify the `git` binary gix would run: its path, plus the modification time of the
/// binary it resolves to (and on macOS of the active developer dir, which `/usr/bin/git` forwards to).
fn git_binary_key() -> Option<String> {
    fn mtime(path: &Path, follow_symlinks: bool) -> Option<u128> {
        let meta = if follow_symlinks {
            fs::metadata(path)
        } else {
            fs::symlink_metadata(path)
        };
        Some(
            meta.ok()?
                .modified()
                .ok()?
                .duration_since(UNIX_EPOCH)
                .ok()?
                .as_nanos(),
        )
    }

    let git = which::which("git").ok()?;
    let mut key = format!("{}\t{}", git.to_str()?, mtime(&git, true)?);
    if cfg!(target_os = "macos")
        && let Some(xcode) = mtime(Path::new("/var/db/xcode_select_link"), false)
    {
        let _ = write!(key, "\t{xcode}");
    }
    Some(key)
}

/// Cache format: the git binary key on the first line, the config path (or nothing) on the second.
/// Returns `None` if the cache is stale or invalid.
fn parse_installation_config_cache(content: &str, key: &str) -> Option<Option<PathBuf>> {
    let (cached_key, path) = content.split_once('\n')?;
    if cached_key != key {
        return None;
    }
    let path = path.strip_suffix('\n')?;
    if path.is_empty() {
        return Some(None);
    }
    let path = PathBuf::from(path);
    (path.is_absolute() && path.is_file()).then_some(Some(path))
}

fn write_installation_config_cache(
    cache_file: &Path,
    key: &str,
    path: Option<&Path>,
) -> std::io::Result<()> {
    let path = match path {
        Some(path) => path
            .to_str()
            .filter(|path| !path.contains('\n'))
            .ok_or_else(|| {
                std::io::Error::other("path is not valid UTF-8 or contains a newline")
            })?,
        None => "",
    };
    let dir = cache_file
        .parent()
        .ok_or_else(|| std::io::Error::other("cache file has no parent directory"))?;
    fs::create_dir_all(dir)?;
    // Write to a temporary file first, so concurrent prompts never read a partial cache
    let tmp = dir.join(format!(
        "{INSTALLATION_CONFIG_CACHE}.{}",
        std::process::id()
    ));
    fs::write(&tmp, format!("{key}\n{path}\n"))?;
    fs::rename(&tmp, cache_file).inspect_err(|_| {
        let _ = fs::remove_file(&tmp);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installation_config_cache_roundtrip() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let cache_file = dir.path().join(INSTALLATION_CONFIG_CACHE);
        let config = dir.path().join("gitconfig");
        fs::write(&config, "")?;

        write_installation_config_cache(&cache_file, "git\t1", Some(&config))?;
        let content = utils::read_file(&cache_file)?;
        assert_eq!(
            parse_installation_config_cache(&content, "git\t1"),
            Some(Some(config.clone()))
        );
        // Different git binary
        assert_eq!(parse_installation_config_cache(&content, "git\t2"), None);

        // Config file removed since it was cached
        fs::remove_file(&config)?;
        assert_eq!(parse_installation_config_cache(&content, "git\t1"), None);

        write_installation_config_cache(&cache_file, "git\t1", None)?;
        let content = utils::read_file(&cache_file)?;
        assert_eq!(
            parse_installation_config_cache(&content, "git\t1"),
            Some(None)
        );
        dir.close()
    }

    #[test]
    fn negative_cache_entries_expire() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let cache_file = dir.path().join(INSTALLATION_CONFIG_CACHE);
        write_installation_config_cache(&cache_file, "git\t1", None)?;
        assert!(modified_within(&cache_file, NEGATIVE_CACHE_TTL));

        let expired = std::time::SystemTime::now() - NEGATIVE_CACHE_TTL - Duration::from_secs(1);
        fs::File::options()
            .write(true)
            .open(&cache_file)?
            .set_modified(expired)?;
        assert!(!modified_within(&cache_file, NEGATIVE_CACHE_TTL));
        dir.close()
    }

    fn context_with_cache(cache_dir: &Path) -> Context<'static> {
        let mut context = crate::test::default_context();
        context
            .env
            .insert("STARSHIP_CACHE", cache_dir.to_string_lossy().into_owned());
        context
    }

    #[test]
    fn installation_config_is_cached() -> std::io::Result<()> {
        let Some(key) = git_binary_key() else {
            // No git in PATH, nothing to cache
            return Ok(());
        };
        let dir = tempfile::tempdir()?;
        let context = context_with_cache(dir.path());
        let cache_file = dir.path().join(INSTALLATION_CONFIG_CACHE);

        // First prompt discovers the config and caches it
        let InstallationConfig::Known(discovered) = installation_config(&context) else {
            panic!("expected the installation config to be resolved");
        };
        let content = utils::read_file(&cache_file)?;
        assert_eq!(
            parse_installation_config_cache(&content, &key),
            Some(discovered)
        );

        // Later prompts read the cache instead of asking git
        let cached = dir.path().join("gitconfig");
        fs::write(&cached, "")?;
        write_installation_config_cache(&cache_file, &key, Some(&cached))?;
        assert!(matches!(
            installation_config(&context),
            InstallationConfig::Known(Some(path)) if path == cached
        ));
        dir.close()
    }

    #[test]
    fn installation_config_defers_to_gix_on_env_overrides() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut context = context_with_cache(dir.path());
        context.env.insert("GIT_CONFIG_NOSYSTEM", "1".to_string());
        assert!(matches!(
            installation_config(&context),
            InstallationConfig::Discover
        ));
        assert!(!dir.path().join(INSTALLATION_CONFIG_CACHE).exists());
        dir.close()
    }

    #[test]
    fn cached_installation_config_is_loaded() -> std::io::Result<()> {
        let Some(key) = git_binary_key() else {
            return Ok(());
        };
        let repo_dir = crate::test::fixture_repo(crate::test::FixtureProvider::Git {
            reftable: false,
            bare: false,
        })?;
        let cache_dir = tempfile::tempdir()?;
        let config = cache_dir.path().join("gitconfig");
        fs::write(&config, "[starship]\n\tinstalled = yes\n")?;
        write_installation_config_cache(
            &cache_dir.path().join(INSTALLATION_CONFIG_CACHE),
            &key,
            Some(&config),
        )?;

        let mut context = context_with_cache(cache_dir.path());
        context.current_dir = repo_dir.path().to_path_buf();
        let repo = context
            .get_git_repo()
            .expect("fixture is a git repo")
            .open();
        assert_eq!(
            repo.config_snapshot()
                .string("starship.installed")
                .map(|value| value.to_string()),
            Some("yes".to_string())
        );
        repo_dir.close()?;
        cache_dir.close()
    }

    #[test]
    fn installation_config_cache_rejects_invalid_content() {
        for content in [
            "",
            "git\t1",
            "git\t1\nrelative/gitconfig\n",
            "git\t1\n/no-newline",
        ] {
            assert_eq!(parse_installation_config_cache(content, "git\t1"), None);
        }
    }
}
