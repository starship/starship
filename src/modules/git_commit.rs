use super::{Context, Module, ModuleConfig};

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
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "hash" => Some(Ok(git_hash(context.get_repo().ok()?, &config)?)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_commit`:\n{error}");
            return None;
        }
    });

    Some(module)
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

    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use crate::utils::create_command;

    // TODO: Support reftable and switch to `crate::test::COMMON_GIT_PROVIDERS`, e.g. via git-cli
    static COMMON_GIT_PROVIDERS: &[FixtureProvider] = &[FixtureProvider::Git {
        bare: false,
        reftable: false,
    }];

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
    fn test_render_commit_hash() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(provider)?;

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
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_render_commit_hash_len_override() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(provider)?;

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
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_branch() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(provider)?;

            let actual = ModuleRenderer::new("git_commit")
                .path(repo_dir.path())
                .collect();

            let expected = None;

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_detached() -> io::Result<()> {
        for &provider in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(provider)?;

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
            repo_dir.close()?;
        }
        Ok(())
    }
}
