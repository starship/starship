use super::{Context, Module, ModuleConfig};

use crate::configs::git_tags::GitTagsConfig;
use crate::formatter::StringFormatter;
use gix::Repository;

/// Creates a module with the Git tag in the current directory
///
/// Will display the tag name if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_tags");
    let config: GitTagsConfig = GitTagsConfig::try_load(module.config);

    let repo = context.get_repo().ok()?;
    let tag_names = all_tags(repo.open())?;

    if tag_names.is_empty() {
        return None;
    }

    let formatted_tags = tag_names.join(config.separator);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "tags" => Some(Ok(&formatted_tags)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_tags`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

/// Returns both annotated and lightweight tags of the current commit.
fn all_tags(mut repo: Repository) -> Option<Vec<String>> {
    // Increase the default object cache size to speed up operation for some repos
    repo.object_cache_size_if_unset(4 * 1024 * 1024);
    let commit = repo.head_commit().ok()?;
    let platform = repo.references().ok()?;
    let all_tags = platform.tags().ok()?;

    let mut tags = all_tags
        .filter_map(Result::ok)
        .filter_map(|mut r: gix::Reference<'_>| {
            // Filter out tags that don't belong to the commit ID.
            let peeled_id = r.peel_to_id().ok()?;
            if peeled_id != commit.id() {
                return None;
            }

            // Filter out tags that aren't valid UTF-8.
            r.inner.name.shorten().to_owned().try_into().ok()
        })
        .collect::<Vec<_>>();

    tags.sort();

    Some(tags)
}

#[cfg(test)]
mod tests {
    use std::io;

    use nu_ansi_term::Color;

    use crate::configs::git_tags::GitTagsConfig;
    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use crate::utils::create_command;

    #[test]
    fn test_show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_tags")
            .path(&repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_show_nothing_if_no_tags() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let actual = ModuleRenderer::new("git_tags")
            .path(&repo_dir.path())
            .collect();

        assert_eq!(None, actual);
        repo_dir.close()
    }

    #[test]
    fn test_annotated_tags_shown() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(&["tag", "-a", "tag1", "-m", "created tag 1"])
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(&["tag", "-a", "tag2", "-m", "created tag 1"])
            .current_dir(repo_dir.path())
            .output()?;

        let expected_tags = "tag1 tag2";
        let config = GitTagsConfig::default();
        let expected = Some(format!(
            "{} ",
            Color::Yellow
                .bold()
                .paint(format!("({}{})", config.symbol, expected_tags))
        ));

        let actual = ModuleRenderer::new("git_tags")
            .path(&repo_dir.path())
            .collect();

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_lightweight_tags_shown() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(["tag", "--no-sign", "v1"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:00 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(["tag", "--no-sign", "v0", "HEAD~1"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(["tag", "--no-sign", "v2"])
            .env("GIT_COMMITTER_DATE", "2022-01-01 00:00:01 +0000")
            .current_dir(repo_dir.path())
            .output()?;

        let actual = ModuleRenderer::new("git_tags")
            .config(toml::toml! {
                [git_tags]
                    symbol = "T "
            })
            .path(repo_dir.path())
            .collect();

        let expected_output = "T v1 v2";
        let expected = Some(format!(
            "{} ",
            Color::Yellow
                .bold()
                .paint(format!("({})", expected_output.trim()))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_annotated_and_lightweight_tags_shown() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(["tag", "lightweight"])
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(&["tag", "-a", "annotated", "-m", "created tag 1"])
            .current_dir(repo_dir.path())
            .output()?;

        let actual = ModuleRenderer::new("git_tags")
            .path(repo_dir.path())
            .collect();

        let expected_tags = "annotated lightweight";
        let config = GitTagsConfig::default();
        let expected = Some(format!(
            "{} ",
            Color::Yellow
                .bold()
                .paint(format!("({}{})", config.symbol, expected_tags))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }
}
