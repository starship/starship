use super::{Context, Module, RootModuleConfig};
use git2::Repository;

use crate::configs::git_commit::GitCommitConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the Git commit in the current directory
///
/// Will display the commit hash if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_commit");
    let config: GitCommitConfig = GitCommitConfig::try_load(module.config);

    let repo = context.get_repo().ok()?;
    let repo_root = repo.root.as_ref()?;
    let git_repo = Repository::open(repo_root).ok()?;

    let is_detached = git_repo.head_detached().ok()?;
    if config.only_detached && !is_detached {
        return None;
    };

    let git_head = git_repo.head().ok()?;
    let head_commit = git_head.peel_to_commit().ok()?;
    let commit_oid = head_commit.id();

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "hash" => Some(Ok(id_to_hex_abbrev(
                    commit_oid.as_bytes(),
                    config.commit_hash_length,
                ))),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_commit`:\n{}", error);
            return None;
        }
    });

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

/// len specifies length of hex encoded string
pub fn id_to_hex_abbrev(bytes: &[u8], len: usize) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
        .chars()
        .take(len)
        .collect()
}
