use std::ffi::OsStr;
use std::fmt::Debug;
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

const INSTALLATION_CONFIG_CACHE: &str = "git-installation-config";
const APPLE_GIT_SHIM: &str = "/usr/bin/git";
const XCODE_SELECT_LINK: &str = "/var/db/xcode_select_link";

pub(crate) fn installation_config(context: &Context) -> Option<PathBuf> {
    if !cfg!(target_os = "macos")
        || ["GIT_CONFIG_SYSTEM", "GIT_CONFIG_NOSYSTEM", "DEVELOPER_DIR"]
            .iter()
            .any(|var| context.get_env_os(var).is_some())
    {
        return None;
    }
    let git = which::which_in("git", context.get_env_os("PATH"), &context.current_dir).ok()?;
    if git != Path::new(APPLE_GIT_SHIM) {
        return None;
    }
    let key = mtime(Path::new(XCODE_SELECT_LINK)).unwrap_or_default();
    cached_installation_config(&installation_config_cache_file(context)?, &key, || {
        gix::path::env::installation_config().map(Path::to_path_buf)
    })
}

fn cached_installation_config(
    cache_file: &Path,
    key: &str,
    discover: impl FnOnce() -> Option<PathBuf>,
) -> Option<PathBuf> {
    if let Some(path) = utils::read_file(cache_file)
        .ok()
        .and_then(|content| parse_installation_config_cache(&content, key))
    {
        log::trace!("Using cached git installation config {path:?}");
        return Some(path);
    }
    let path = discover()?;
    log::debug!("Caching git installation config {path:?} in {cache_file:?}");
    if let Err(e) = write_installation_config_cache(cache_file, key, &path) {
        log::debug!("Unable to cache git installation config: {e}");
    }
    Some(path)
}

fn installation_config_cache_file(context: &Context) -> Option<PathBuf> {
    let dir = match context.get_env_os("STARSHIP_CACHE") {
        Some(dir) => PathBuf::from(dir),
        None if cfg!(test) => return None,
        None => context.get_home()?.join(".cache").join("starship"),
    };
    Some(dir.join(INSTALLATION_CONFIG_CACHE))
}

fn mtime(path: &Path) -> Option<String> {
    let modified = fs::symlink_metadata(path).ok()?.modified().ok()?;
    Some(
        modified
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_nanos()
            .to_string(),
    )
}

fn parse_installation_config_cache(content: &str, key: &str) -> Option<PathBuf> {
    let (cached_key, path) = content.split_once('\n')?;
    let path = PathBuf::from(path.strip_suffix('\n')?);
    (cached_key == key && path.is_absolute() && path.is_file()).then_some(path)
}

fn write_installation_config_cache(
    cache_file: &Path,
    key: &str,
    path: &Path,
) -> std::io::Result<()> {
    let path = path
        .to_str()
        .filter(|path| !path.contains('\n'))
        .ok_or_else(|| std::io::Error::other("path is not valid UTF-8 or contains a newline"))?;
    let dir = cache_file
        .parent()
        .ok_or_else(|| std::io::Error::other("cache file has no parent directory"))?;
    fs::create_dir_all(dir)?;
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
    fn installation_config_is_cached() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let cache_file = dir.path().join(INSTALLATION_CONFIG_CACHE);
        let config = dir.path().join("gitconfig");
        fs::write(&config, "")?;

        let discovered = cached_installation_config(&cache_file, "1", || Some(config.clone()));
        assert_eq!(discovered.as_ref(), Some(&config));
        let cached = cached_installation_config(&cache_file, "1", || panic!("cache was not used"));
        assert_eq!(cached.as_ref(), Some(&config));

        let rediscovered = cached_installation_config(&cache_file, "2", || None);
        assert_eq!(
            rediscovered, None,
            "a new key means a different git installation"
        );

        fs::remove_file(&config)?;
        assert_eq!(cached_installation_config(&cache_file, "1", || None), None);
        dir.close()
    }

    #[test]
    fn missing_installation_config_is_not_cached() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let cache_file = dir.path().join(INSTALLATION_CONFIG_CACHE);
        assert_eq!(cached_installation_config(&cache_file, "1", || None), None);
        assert!(!cache_file.exists());
        dir.close()
    }

    #[test]
    fn installation_config_cache_rejects_invalid_content() {
        for content in [
            "",
            "1",
            "1\nrelative/gitconfig\n",
            "1\n/no-newline",
            "1\n\n",
        ] {
            assert_eq!(parse_installation_config_cache(content, "1"), None);
        }
    }

    #[test]
    fn installation_config_is_left_to_gix_off_the_apple_shim() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let bin = dir.path().join("bin");
        fs::create_dir(&bin)?;
        fs::copy(
            which::which("sh").expect("sh is installed"),
            bin.join("git"),
        )?;

        let mut context = crate::test::default_context();
        context
            .env
            .insert("STARSHIP_CACHE", dir.path().to_string_lossy().into_owned());
        context
            .env
            .insert("PATH", bin.to_string_lossy().into_owned());
        assert_eq!(installation_config(&context), None);
        assert!(!dir.path().join(INSTALLATION_CONFIG_CACHE).exists());
        dir.close()
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn apple_shim_installation_config_is_cached() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut context = crate::test::default_context();
        context
            .env
            .insert("STARSHIP_CACHE", dir.path().to_string_lossy().into_owned());
        context.env.insert("PATH", "/usr/bin".to_string());
        let discovered = installation_config(&context);
        assert_eq!(discovered.as_deref(), gix::path::env::installation_config());
        if let Some(config) = discovered {
            let content = utils::read_file(dir.path().join(INSTALLATION_CONFIG_CACHE))?;
            assert!(content.ends_with(&format!("\n{}\n", config.display())));
        }
        dir.close()
    }
}
