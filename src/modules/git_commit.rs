use super::{Context, Module, RootModuleConfig};
use crate::formatter::string_formatter::StringFormatterError;
use git2::Time;

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

    let mut tag_name = String::new();
    if !config.tag_disabled {
        // Let's get repo tags names
        let tag_names = git_repo.tag_names(None).ok()?;
        let tag_and_refs = tag_names.iter().flatten().flat_map(|name| {
            let full_tag = format!("refs/tags/{}", name);
            let tag_obj = git_repo.find_reference(&full_tag)?.peel_to_tag()?;
            let sig_obj = tag_obj.tagger();
            git_repo.find_reference(&full_tag).map(|reference| {
                (
                    String::from(name),
                    // fall back to oldest + 1s time if sig_obj is unavailable
                    sig_obj.map_or(git2::Time::new(1, 0), |s| s.when()),
                    reference,
                )
            })
        });

        let mut oldest = Time::new(0, 0);
        // Let's check if HEAD has some tag. If several, gets last created one...
        for (name, timestamp, reference) in tag_and_refs.rev() {
            if commit_oid == reference.peel_to_commit().ok()?.id() && timestamp > oldest {
                tag_name = name;
                oldest = timestamp;
            }
        }
    };

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
                "tag" => format_tag(config.tag_symbol, &tag_name),
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

fn format_tag(symbol: &str, tag_name: &str) -> Option<Result<String, StringFormatterError>> {
    if tag_name.is_empty() {
        None
    } else {
        Some(Ok(format!("{}{}", &symbol, &tag_name)))
    }
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

    #[test]
    fn test_render_commit_hash_with_tag_enabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut git_commit = create_command("git")?
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let git_tag = create_command("git")?
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
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_detached_with_tag_enabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(&["checkout", "@~1"])
            .current_dir(&repo_dir.path())
            .output()?;

        create_command("git")?
            .args(&["tag", "tagOnDetached", "-m", "Testing tags on detached"])
            .current_dir(&repo_dir.path())
            .output()?;

        let mut git_commit = create_command("git")?
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let git_tag = create_command("git")?
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
                    tag_symbol = " "
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_output.trim()))
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_latest_tag_shown_with_tag_enabled() -> io::Result<()> {
        use std::{thread, time};

        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut git_commit = create_command("git")?
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        create_command("git")?
            .args(&["tag", "v2", "-m", "Testing tags v2"])
            .current_dir(&repo_dir.path())
            .output()?;

        // Wait one second between tags
        thread::sleep(time::Duration::from_millis(1000));

        create_command("git")?
            .args(&["tag", "v0", "-m", "Testing tags v0", "HEAD~1"])
            .current_dir(&repo_dir.path())
            .output()?;

        create_command("git")?
            .args(&["tag", "v1", "-m", "Testing tags v1"])
            .current_dir(&repo_dir.path())
            .output()?;

        let git_tag = create_command("git")?
            .args(&[
                "for-each-ref",
                "--contains",
                "HEAD",
                "--sort=-taggerdate",
                "--count=1",
                "--format",
                "%(refname:short)",
                "refs/tags",
            ])
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
                    tag_symbol = " "
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_output.trim()))
        ));

        assert_eq!(expected, actual);
        Ok(())
    }
}
