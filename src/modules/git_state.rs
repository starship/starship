use git2::RepositoryState;
use std::path::PathBuf;

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
    root: &'a std::path::PathBuf,
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

fn describe_rebase<'a>(root: &'a PathBuf, rebase_config: &'a str) -> StateDescription<'a> {
    /*
     *  Sadly, libgit2 seems to have some issues with reading the state of
     *  interactive rebases. So, instead, we'll poke a few of the .git files
     *  ourselves. This might be worth re-visiting this in the future...
     *
     *  The following is based heavily on: https://github.com/magicmonty/bash-git-prompt
     */

    let dot_git = root.join(".git");

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
    } else if has_path("rebase-merge/onto") {
        Some((1, 1))
    } else if has_path("rebase-apply") {
        paths_to_progress("rebase-apply/next", "rebase-apply/last")
    } else {
        None
    };

    StateDescription {
        label: rebase_config,
        current: Some(format!("{}", progress.unwrap().0)),
        total: Some(format!("{}", progress.unwrap().1)),
    }
}

struct StateDescription<'a> {
    label: &'a str,
    current: Option<String>,
    total: Option<String>,
}
