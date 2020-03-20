use super::{Context, Module, RootModuleConfig};
use git2::Repository;

use crate::configs::git_commit::GitCommitConfig;

/// Creates a module with the Git commit in the current directory
///
/// Will display the commit hash if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_commit");
    let config = GitCommitConfig::try_load(module.config);

    module
        .get_prefix()
        .set_value(config.prefix)
        .set_style(config.style);
    module
        .get_suffix()
        .set_value(config.suffix)
        .set_style(config.style);
    module.set_style(config.style);

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
    module.create_segment(
        "hash",
        &config.hash.with_value(&id_to_hex_abbrev(
            commit_oid.as_bytes(),
            config.commit_hash_length,
        )),
    );

    if !config.tag_disabled {
        // Let's get repo tags names
        let tag_names = git_repo.tag_names(None).ok()?;
        let tag_and_refs = tag_names.iter().flat_map(|name| {
            let full_tag = format!("refs/tags/{}", name.unwrap());
            git_repo
                .find_reference(&full_tag)
                .map(|reference| (String::from(name.unwrap()), reference))
        });

        let mut tag_name = String::new();
        // Let's check if HEAD has some tag. If several, only gets first...
        for (name, reference) in tag_and_refs {
            if commit_oid == reference.peel_to_commit().ok()?.id() {
                tag_name = name;
                break;
            }
        }
        // If we have tag...
        if !tag_name.is_empty() {
            module.create_segment(
                "tag",
                &config
                    .tag
                    .with_value(&format!(" {}{}", &config.tag_symbol, &tag_name)),
            );
        }
    };

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
