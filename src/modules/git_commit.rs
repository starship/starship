use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};
use git2::Repository;

use crate::configs::git_commit::GitCommitConfig;
use crate::segment::Segment;

/// Creates a module with the Git commit in the current directory
///
/// Will display the commit hash if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_commit");
    let config = GitCommitConfig::try_load(module.config);
    if config.disabled {
        return None;
    };

    let repo = context.get_repo().ok()?;
    let repo_root = repo.root.as_ref()?;
    let git_repo = Repository::open(repo_root).ok()?;

    let git_head = git_repo.head().ok()?;
    let head_commit = git_head.peel_to_commit().ok()?;
    let commit_oid = head_commit.id();

    let hash = id_to_hex_abbrev(commit_oid.as_bytes(), config.commit_hash_length);

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "hash" => Some(Segment {
                _name: "hash".to_string(),
                value: hash.clone(),
                style,
            }),
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

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
