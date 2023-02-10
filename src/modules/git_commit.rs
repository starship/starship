use super::{Context, Module, ModuleConfig};
use git_repository::commit::describe::SelectRef::AllTags;

use crate::configs::git_commit::GitCommitConfig;
use crate::context::Repo;
use crate::formatter::StringFormatter;

/// Creates a module with the Git commit in the current directory
///
/// Will display the commit hash if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_commit");
    let config: GitCommitConfig = GitCommitConfig::try_load(module.config);

    let repo = context.get_repo().ok()?;
    let git_repo = repo.open();
    let git_head = git_repo.head().ok()?;

    let is_detached = git_head.is_detached();
    if config.only_detached && !is_detached {
        return None;
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "hash" => Some(Ok(git_hash(context.get_repo().ok()?, &config)?)),
                "tag" if !config.tag_disabled => Some(Ok(format!(
                    "{}{}",
                    config.tag_symbol,
                    git_tag(context.get_repo().ok()?, &config)?
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

fn git_tag(repo: &Repo, config: &GitCommitConfig) -> Option<String> {
    let mut git_repo = repo.open();
    // Increase the default object cache size to speed up operation for some repos
    git_repo.object_cache_size_if_unset(4 * 1024 * 1024);
    let head_commit = git_repo.head_commit().ok()?;

    let describe_platform = head_commit
        .describe()
        .names(AllTags)
        .max_candidates(config.tag_max_candidates)
        .traverse_first_parent(true);
    let formatter = describe_platform.try_format().ok()??;

    Some(formatter.name?.to_string())
}

fn git_hash(repo: &Repo, config: &GitCommitConfig) -> Option<String> {
    let git_repo = repo.open();
    let head_id = git_repo.head_id().ok()?;

    Some(format!(
        "{}",
        head_id.to_hex_with_len(config.commit_hash_length)
    ))
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;
    use std::{io, str};

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use crate::utils::create_command;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_commit")
            .path(repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_ceiling_directory() -> io::Result<()> {
        use std::{env, ffi::OsString, path::Path};

        let repo_dir = fixture_repo(FixtureProvider::Git)?;
        let level1 = repo_dir.path().join("level1");
        let level2 = level1.join("level2");
        let level3 = level2.join("level3");
        std::fs::create_dir(&level1)?;
        std::fs::create_dir(&level2)?;
        std::fs::create_dir(&level3)?;

        // Set ceiling directory
        const CEILING_DIRECTORY_VAR: &str = "GIT_CEILING_DIRECTORIES";
        const CEILING_DIRECTORY_SEP: &str = if cfg!(windows) { ";" } else { ":" };

        let mut ceiling = match env::var_os(CEILING_DIRECTORY_VAR) {
            Some(mut var) => {
                var.push(CEILING_DIRECTORY_SEP);
                var
            }
            None => OsString::new(),
        };
        let parent_dir = repo_dir
            .path()
            .parent()
            .expect("tempdir should always have a parent");
        ceiling.push(parent_dir);
        ceiling.push(CEILING_DIRECTORY_SEP);
        ceiling.push(parent_dir);
        ceiling.push(CEILING_DIRECTORY_SEP);
        ceiling.push(&level1);

        #[allow(clippy::disallowed_methods)]
        env::set_var(CEILING_DIRECTORY_VAR, ceiling);

        let assert_discovery = |cwd: &Path| -> io::Result<()> {
            let mut git_output = create_command("git")?
                .args(["rev-parse", "HEAD"])
                .current_dir(cwd)
                .output()?
                .stdout;
            git_output.truncate(7);
            let expected_hash = str::from_utf8(&git_output).unwrap();

            let actual = ModuleRenderer::new("git_commit")
                .config(toml::toml! {
                    [git_commit]
                        only_detached = false
                })
                .path(cwd)
                .collect();

            let expected = Some(format!(
                "{} ",
                Color::Green.bold().paint(format!("({expected_hash})"))
            ));

            assert_eq!(expected, actual);
            Ok(())
        };

        let assert_no_discovery = |cwd: &Path| -> io::Result<()> {
            let actual = ModuleRenderer::new("git_commit")
                .config(toml::toml! {
                    [git_commit]
                        only_detached = false
                })
                .path(cwd)
                .collect();

            let expected = None;

            assert_eq!(expected, actual);
            Ok(())
        };

        assert_no_discovery(&level3)?;
        assert_no_discovery(&level2)?;
        assert_discovery(&level1)?;
        assert_discovery(repo_dir.path())?;

        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut git_output = create_command("git")?
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        git_output.truncate(7);
        let expected_hash = str::from_utf8(&git_output).unwrap();

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
            })
            .path(repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green.bold().paint(format!("({expected_hash})"))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_len_override() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut git_output = create_command("git")?
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir.path())
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
            .path(repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green.bold().paint(format!("({expected_hash})"))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_branch() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let actual = ModuleRenderer::new("git_commit")
            .path(repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_detached() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(["checkout", "@~1"])
            .current_dir(repo_dir.path())
            .output()?;

        let mut git_output = create_command("git")?
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        git_output.truncate(7);
        let expected_hash = str::from_utf8(&git_output).unwrap();

        let actual = ModuleRenderer::new("git_commit")
            .path(repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green.bold().paint(format!("({expected_hash})"))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_with_tag_disabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(["tag", "v1", "-m", "Testing tags"])
            .current_dir(repo_dir.path())
            .output()?;

        let mut git_commit = create_command("git")?
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
            })
            .path(repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", commit_output.trim()))
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_render_commit_hash_with_tag_enabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(["tag", "v1", "-m", "Testing tags"])
            .current_dir(repo_dir.path())
            .output()?;

        let mut git_commit = create_command("git")?
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let git_tag = create_command("git")?
            .args(["describe", "--tags", "--exact-match", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{commit_output} {tag_output}");

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
                    tag_disabled = false
                    tag_symbol = " "
            })
            .path(repo_dir.path())
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
            .args(["checkout", "@~1"])
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(["tag", "tagOnDetached", "-m", "Testing tags on detached"])
            .current_dir(repo_dir.path())
            .output()?;

        let mut git_commit = create_command("git")?
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let git_tag = create_command("git")?
            .args(["describe", "--tags", "--exact-match", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{commit_output} {tag_output}");

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    tag_disabled = false
                    tag_symbol = " "
            })
            .path(repo_dir.path())
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
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut git_commit = create_command("git")?
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        create_command("git")?
            .args(["tag", "v2", "-m", "Testing tags v2"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:00 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(["tag", "v0", "-m", "Testing tags v0", "HEAD~1"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(["tag", "v1", "-m", "Testing tags v1"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        // Annotaged tags are preferred over lightweight tags
        create_command("git")?
            .args(["tag", "l0"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:02 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        let git_tag = create_command("git")?
            .args(["describe", "--tags"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{commit_output} {tag_output}");

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
                    tag_disabled = false
                    tag_symbol = " "
            })
            .path(repo_dir.path())
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
    fn test_latest_tag_shown_with_tag_enabled_lightweight() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let mut git_commit = create_command("git")?
            .args(["rev-parse", "HEAD"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        // Lightweight tags are chosen lexicographically
        create_command("git")?
            .args(["tag", "v1"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:00 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(["tag", "v0", "HEAD~1"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(["tag", "v2"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        let git_tag = create_command("git")?
            .args(["describe", "--tags"])
            .current_dir(repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{commit_output} {tag_output}");

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
                    tag_disabled = false
                    tag_symbol = " "
            })
            .path(repo_dir.path())
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
