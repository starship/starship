use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Duration;

use gix::{Repository, ThreadSafeRepository, state as git_state};

use crate::utils::{CommandOutput, create_command, exec_timeout};

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

    /// Wrapper to execute external git commands.
    /// Handles adding the appropriate `--git-dir` and `--work-tree` flags to the command.
    /// Also handles additional features required for security, such as disabling `fsmonitor`.
    /// At this time, mocking is not supported.
    pub fn exec_git<T: AsRef<OsStr> + Debug>(
        &self,
        context: &Context,
        git_args: impl IntoIterator<Item = T>,
    ) -> Option<CommandOutput> {
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
