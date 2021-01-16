use git2::RepositoryState;
use std::path::{Path, PathBuf};

use super::{Context, Module, RootModuleConfig};
use crate::configs::git_state::GitStateConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the state of the git repository at the current directory
///
/// During a git operation it will show: REBASING, BISECTING, MERGING, etc.
/// If the progress information is available (e.g. rebasing 3/10), it will show that too.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_state");
    let config: GitStateConfig = GitStateConfig::try_load(module.config);

    let repo = context.get_repo().ok()?;
    let repo_root = repo.root.as_ref()?;
    let repo_state = repo.state?;

    let state_description = get_state_description(repo_state, repo_root, &config)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "state" => Some(state_description.label),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "progress_current" => state_description.current.as_ref().map(Ok),
                "progress_total" => state_description.total.as_ref().map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_state`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

/// Returns the state of the current repository
///
/// During a git operation it will show: REBASING, BISECTING, MERGING, etc.
fn get_state_description<'a>(
    state: RepositoryState,
    root: &'a Path,
    config: &GitStateConfig<'a>,
) -> Option<StateDescription<'a>> {
    match state {
        RepositoryState::Clean => None,
        RepositoryState::Merge => Some(StateDescription {
            label: config.merge,
            current: None,
            total: None,
        }),
        RepositoryState::Revert => Some(StateDescription {
            label: config.revert,
            current: None,
            total: None,
        }),
        RepositoryState::RevertSequence => Some(StateDescription {
            label: config.revert,
            current: None,
            total: None,
        }),
        RepositoryState::CherryPick => Some(StateDescription {
            label: config.cherry_pick,
            current: None,
            total: None,
        }),
        RepositoryState::CherryPickSequence => Some(StateDescription {
            label: config.cherry_pick,
            current: None,
            total: None,
        }),
        RepositoryState::Bisect => Some(StateDescription {
            label: config.bisect,
            current: None,
            total: None,
        }),
        RepositoryState::ApplyMailbox => Some(StateDescription {
            label: config.am,
            current: None,
            total: None,
        }),
        RepositoryState::ApplyMailboxOrRebase => Some(StateDescription {
            label: config.am_or_rebase,
            current: None,
            total: None,
        }),
        RepositoryState::Rebase => Some(describe_rebase(root, config.rebase)),
        RepositoryState::RebaseInteractive => Some(describe_rebase(root, config.rebase)),
        RepositoryState::RebaseMerge => Some(describe_rebase(root, config.rebase)),
    }
}

fn describe_rebase<'a>(root: &'a Path, rebase_config: &'a str) -> StateDescription<'a> {
    /*
     *  Sadly, libgit2 seems to have some issues with reading the state of
     *  interactive rebases. So, instead, we'll poke a few of the .git files
     *  ourselves. This might be worth re-visiting this in the future...
     *
     *  The following is based heavily on: https://github.com/magicmonty/bash-git-prompt
     */

    let dot_git = root.join(".git");
    let dot_git = if let Ok(conf) = std::fs::read_to_string(&dot_git) {
        let gitdir_re = regex::Regex::new(r"(?m)^gitdir: (.*)$").unwrap();
        if let Some(caps) = gitdir_re.captures(&conf) {
            root.join(caps.get(1).unwrap().as_str())
        } else {
            dot_git
        }
    } else {
        dot_git
    };

    let has_path = |relative_path: &str| {
        let path = dot_git.join(PathBuf::from(relative_path));
        path.exists()
    };

    let file_to_usize = |relative_path: &str| {
        let path = dot_git.join(PathBuf::from(relative_path));
        let contents = crate::utils::read_file(path).ok()?;
        let quantity = contents.trim().parse::<usize>().ok()?;
        Some(quantity)
    };

    let paths_to_progress = |current_path: &str, total_path: &str| {
        let current = file_to_usize(current_path)?;
        let total = file_to_usize(total_path)?;
        Some((current, total))
    };

    let progress = if has_path("rebase-merge/msgnum") {
        paths_to_progress("rebase-merge/msgnum", "rebase-merge/end")
    } else if has_path("rebase-apply") {
        paths_to_progress("rebase-apply/next", "rebase-apply/last")
    } else {
        None
    };

    let (current, total) = if let Some((c, t)) = progress {
        (Some(format!("{}", c)), Some(format!("{}", t)))
    } else {
        (None, None)
    };

    StateDescription {
        label: rebase_config,
        current,
        total,
    }
}

struct StateDescription<'a> {
    label: &'a str,
    current: Option<String>,
    total: Option<String>,
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::ffi::OsStr;
    use std::fs::OpenOptions;
    use std::io::{self, Error, ErrorKind, Write};
    use std::path::Path;
    use std::process::{Command, Stdio};

    use crate::test::ModuleRenderer;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_state")
            .path(repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_rebasing() -> io::Result<()> {
        let repo_dir = create_repo_with_conflict()?;
        let path = repo_dir.path();

        run_git_cmd(&["rebase", "other-branch"], Some(path), false)?;

        let actual = ModuleRenderer::new("git_state").path(path).collect();

        let expected = Some(format!("({}) ", Color::Yellow.bold().paint("REBASING 1/1")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_merging() -> io::Result<()> {
        let repo_dir = create_repo_with_conflict()?;
        let path = repo_dir.path();

        run_git_cmd(&["merge", "other-branch"], Some(path), false)?;

        let actual = ModuleRenderer::new("git_state").path(path).collect();

        let expected = Some(format!("({}) ", Color::Yellow.bold().paint("MERGING")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_cherry_picking() -> io::Result<()> {
        let repo_dir = create_repo_with_conflict()?;
        let path = repo_dir.path();

        run_git_cmd(&["cherry-pick", "other-branch"], Some(path), false)?;

        let actual = ModuleRenderer::new("git_state").path(path).collect();

        let expected = Some(format!(
            "({}) ",
            Color::Yellow.bold().paint("CHERRY-PICKING")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_bisecting() -> io::Result<()> {
        let repo_dir = create_repo_with_conflict()?;
        let path = repo_dir.path();

        run_git_cmd(&["bisect", "start"], Some(path), false)?;

        let actual = ModuleRenderer::new("git_state").path(path).collect();

        let expected = Some(format!("({}) ", Color::Yellow.bold().paint("BISECTING")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_reverting() -> io::Result<()> {
        let repo_dir = create_repo_with_conflict()?;
        let path = repo_dir.path();

        run_git_cmd(&["revert", "--no-commit", "HEAD~1"], Some(path), false)?;

        let actual = ModuleRenderer::new("git_state").path(path).collect();

        let expected = Some(format!("({}) ", Color::Yellow.bold().paint("REVERTING")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    fn run_git_cmd<A, S>(args: A, dir: Option<&Path>, should_succeed: bool) -> io::Result<()>
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new("git");
        command
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());

        if let Some(dir) = dir {
            command.current_dir(dir);
        }

        let status = command.status()?;

        if should_succeed && !status.success() {
            Err(Error::from(ErrorKind::Other))
        } else {
            Ok(())
        }
    }

    fn create_repo_with_conflict() -> io::Result<tempfile::TempDir> {
        let repo_dir = tempfile::tempdir()?;
        let path = repo_dir.path();
        let conflicted_file = repo_dir.path().join("the_file");

        let write_file = |text: &str| {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&conflicted_file)?;
            write!(file, "{}", text)
        };

        // Initialize a new git repo
        run_git_cmd(
            &[
                "init",
                "--quiet",
                path.to_str().expect("Path was not UTF-8"),
            ],
            None,
            true,
        )?;

        // Set local author info
        run_git_cmd(
            &["config", "--local", "user.email", "starship@example.com"],
            Some(path),
            true,
        )?;
        run_git_cmd(
            &["config", "--local", "user.name", "starship"],
            Some(path),
            true,
        )?;

        // Ensure on the expected branch.
        // If build environment has `init.defaultBranch` global set
        // it will default to an unknown branch, so neeed to make & change branch
        run_git_cmd(
            &["checkout", "-b", "master"],
            Some(path),
            // command expected to fail if already on the expected branch
            false,
        )?;

        // Write a file on master and commit it
        write_file("Version A")?;
        run_git_cmd(&["add", "the_file"], Some(path), true)?;
        run_git_cmd(
            &["commit", "--message", "Commit A", "--no-gpg-sign"],
            Some(path),
            true,
        )?;

        // Switch to another branch, and commit a change to the file
        run_git_cmd(&["checkout", "-b", "other-branch"], Some(path), true)?;
        write_file("Version B")?;
        run_git_cmd(
            &["commit", "--all", "--message", "Commit B", "--no-gpg-sign"],
            Some(path),
            true,
        )?;

        // Switch back to master, and commit a third change to the file
        run_git_cmd(&["checkout", "master"], Some(path), true)?;
        write_file("Version C")?;
        run_git_cmd(
            &["commit", "--all", "--message", "Commit C", "--no-gpg-sign"],
            Some(path),
            true,
        )?;

        Ok(repo_dir)
    }
}
