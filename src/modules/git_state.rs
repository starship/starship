use git2::RepositoryState;
use std::path::{Path, PathBuf};

use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig, SegmentConfig};
use crate::configs::git_state::GitStateConfig;
use crate::segment::Segment;

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

    let state_description = get_state_description(repo_state, repo_root, &config);

    let label = match &state_description {
        StateDescription::Label(label) => label,
        StateDescription::LabelAndProgress(label, _) => label,
        StateDescription::Clean => {
            return None;
        }
    };

    if let StateDescription::LabelAndProgress(_, progress) = &state_description {
        module.create_segment(
            "progress_current",
            &SegmentConfig::new(&format!(" {}", progress.current)),
        );
        module.create_segment("progress_divider", &SegmentConfig::new("/"));
        module.create_segment(
            "progress_total",
            &SegmentConfig::new(&format!("{}", progress.total)),
        );
    }

    let segments: Vec<Segment> = format_segments_nested(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "label" => format_segments(label.segment, style, |_, _| None).ok(),
            "progress" => {
                if let StateDescription::LabelAndProgress(_, progress) = &state_description {
                    format_segments(config.progress_format, style, |name, query| {
                        let style = get_style_from_query(&query).or(style);
                        match name {
                            "current" => Some(Segment {
                                _name: "progress_current".to_string(),
                                value: progress.current.to_string(),
                                style,
                            }),
                            "total" => Some(Segment {
                                _name: "progress_total".to_string(),
                                value: progress.total.to_string(),
                                style,
                            }),
                            _ => None,
                        }
                    })
                    .ok()
                } else {
                    None
                }
            }
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}

/// Returns the state of the current repository
///
/// During a git operation it will show: REBASING, BISECTING, MERGING, etc.
fn get_state_description<'a>(
    state: RepositoryState,
    root: &'a std::path::PathBuf,
    config: &'a GitStateConfig<'a>,
) -> StateDescription<'a> {
    match state {
        RepositoryState::Clean => StateDescription::Clean,
        RepositoryState::Merge => StateDescription::Label(StateLabel::new("merge", config.merge)),
        RepositoryState::Revert => {
            StateDescription::Label(StateLabel::new("revert", config.revert))
        }
        RepositoryState::RevertSequence => {
            StateDescription::Label(StateLabel::new("revert", config.revert))
        }
        RepositoryState::CherryPick => {
            StateDescription::Label(StateLabel::new("cherry_pick", config.cherry_pick))
        }
        RepositoryState::CherryPickSequence => {
            StateDescription::Label(StateLabel::new("cherry_pick", config.cherry_pick))
        }
        RepositoryState::Bisect => {
            StateDescription::Label(StateLabel::new("bisect", config.bisect))
        }
        RepositoryState::ApplyMailbox => StateDescription::Label(StateLabel::new("am", config.am)),
        RepositoryState::ApplyMailboxOrRebase => {
            StateDescription::Label(StateLabel::new("am_or_rebase", config.am_or_rebase))
        }
        RepositoryState::Rebase => describe_rebase(root, config.rebase),
        RepositoryState::RebaseInteractive => describe_rebase(root, config.rebase),
        RepositoryState::RebaseMerge => describe_rebase(root, config.rebase),
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
        let path = dot_git.join(Path::new(relative_path));
        path.exists()
    };

    let file_to_usize = |relative_path: &str| {
        let path = dot_git.join(Path::new(relative_path));
        let contents = crate::utils::read_file(path).ok()?;
        let quantity = contents.trim().parse::<usize>().ok()?;
        Some(quantity)
    };

    let paths_to_progress = |current_path: &str, total_path: &str| {
        let current = file_to_usize(current_path)?;
        let total = file_to_usize(total_path)?;
        Some(StateProgress { current, total })
    };

    let progress = if has_path("rebase-merge") {
        paths_to_progress("rebase-merge/msgnum", "rebase-merge/end")
    } else if has_path("rebase-apply") {
        paths_to_progress("rebase-apply/next", "rebase-apply/last")
    } else {
        None
    };

    match progress {
        None => StateDescription::Label(StateLabel::new("rebase", rebase_config)),
        Some(progress) => {
            StateDescription::LabelAndProgress(StateLabel::new("rebase", rebase_config), progress)
        }
    }
}

enum StateDescription<'a> {
    Clean,
    Label(StateLabel<'a>),
    LabelAndProgress(StateLabel<'a>, StateProgress),
}

struct StateLabel<'a> {
    name: &'static str,
    segment: &'a str,
}

struct StateProgress {
    current: usize,
    total: usize,
}

impl<'a> StateLabel<'a> {
    fn new(name: &'static str, segment: &'a str) -> Self {
        Self { name, segment }
    }
}
