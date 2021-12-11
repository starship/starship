use super::{Context, Module, RootModuleConfig};

use crate::configs::git_commit::GitCommitConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the Git commit in the current directory
///
/// Will display the commit hash if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_commit");
    let config: GitCommitConfig = GitCommitConfig::try_load(module.config);

    let repo = context.get_repo().ok()?;
    let git_repo = repo.open().ok()?;

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
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_commit`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

/// len specifies length of hex encoded string
fn id_to_hex_abbrev(bytes: &[u8], len: usize) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
        .chars()
        .take(len)
        .collect()
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::{io, str};

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use crate::utils::create_command;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_commit")
            .path(&repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut git_output = create_command("git")?
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_output.truncate(7);
        let expected_hash = str::from_utf8(&git_output).unwrap();

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green.bold().paint(format!("({})", expected_hash))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_len_override() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut git_output = create_command("git")?
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_output.truncate(14);
        let expected_hash = str::from_utf8(&git_output).unwrap();

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
                    commit_hash_length = 14
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green.bold().paint(format!("({})", expected_hash))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_branch() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let actual = ModuleRenderer::new("git_commit")
            .path(&repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_detached() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(&["checkout", "@~1"])
            .current_dir(&repo_dir.path())
            .output()?;

        let mut git_output = create_command("git")?
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_output.truncate(7);
        let expected_hash = str::from_utf8(&git_output).unwrap();

        let actual = ModuleRenderer::new("git_commit")
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green.bold().paint(format!("({})", expected_hash))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }
}
