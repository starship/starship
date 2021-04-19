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

    let mut parsed;

    parsed = StringFormatter::new(config.format).and_then(|formatter| {
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
            parsed = StringFormatter::new(config.format).and_then(|formatter| {
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
                    .map(|variable| match variable {
                        "tag" => Some(Ok(format!(" {}{}", &config.tag_symbol, &tag_name))),
                        _ => None,
                    })
                    .parse(None)
            });
        }
    };

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

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::process::Command;
    use std::{io, str};

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};

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
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let mut git_output = Command::new("git")
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
            Color::Green
                .bold()
                .paint(format!("({})", expected_hash))
                .to_string()
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_len_override() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let mut git_output = Command::new("git")
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
            Color::Green
                .bold()
                .paint(format!("({})", expected_hash))
                .to_string()
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_branch() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let actual = ModuleRenderer::new("git_commit")
            .path(&repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_detached() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        Command::new("git")
            .args(&["checkout", "@~1"])
            .current_dir(&repo_dir.path())
            .output()?;

        let mut git_output = Command::new("git")
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
            Color::Green
                .bold()
                .paint(format!("({})", expected_hash))
                .to_string()
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_with_tag_enabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let mut git_commit = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let git_tag = Command::new("git")
            .args(&["describe", "--tags", "--exact-match", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{} {}", commit_output, tag_output);

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
                    tag_disabled = false
                    tag_symbol = ""
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_output.trim()))
                .to_string()
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_detached_with_tag_enabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        Command::new("git")
            .args(&["checkout", "@~1"])
            .current_dir(&repo_dir.path())
            .output()?;

        Command::new("git")
            .args(&["tag", "tagOnDetached", "-m", "Testing tags on detached"])
            .current_dir(&repo_dir.path())
            .output()?;

        let mut git_commit = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let git_tag = Command::new("git")
            .args(&["describe", "--tags", "--exact-match", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{} {}", commit_output, tag_output);

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    tag_disabled = false
                    tag_symbol = ""
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_output.trim()))
                .to_string()
        ));

        assert_eq!(expected, actual);
        Ok(())
    }
}
