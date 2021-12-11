use super::{Context, Module, RootModuleConfig};

use crate::configs::git_tag::GitTagConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the Git tag in the current directory
///
/// Will display the tag name if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_tag");
    let config: GitTagConfig = GitTagConfig::try_load(module.config);

    let repo = context.get_repo().ok()?;
    let git_repo = repo.open().ok()?;

    let git_head = git_repo.head().ok()?;
    let head_commit = git_head.peel_to_commit().ok()?;
    let commit_oid = head_commit.id();

    // Let's get repo tags names
    let tag_names = git_repo.tag_names(None).ok()?;
    let tag_and_refs = tag_names.iter().flatten().flat_map(|name| {
        let full_tag = format!("refs/tags/{}", name);
        git_repo
            .find_reference(&full_tag)
            .map(|reference| (String::from(name), reference))
    });

    let mut tag_names = Vec::new();
    // Let's check if HEAD has some tags.
    for (name, reference) in tag_and_refs.rev() {
        if commit_oid == reference.peel_to_commit().ok()?.id() {
            tag_names.push(name);
        }
    }

    if tag_names.is_empty() {
        return None;
    }

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
                "tags" => Some(Ok(tag_names.join(config.separator))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_tag`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::{io, str};

    use crate::configs::git_tag::GitTagConfig;
    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use crate::utils::create_command;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_tag")
            .path(&repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }
    
    #[test]
    fn test_omit_if_no_tags() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let actual = ModuleRenderer::new("git_tag")
            .path(&repo_dir.path())
            .collect();

        assert_eq!(None, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_tag() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        create_command("git")?
            .args(&["tag", "tag1"])
            .current_dir(repo_dir.path())
            .output()?;

        create_command("git")?
            .args(&["tag", "tag2"])
            .current_dir(repo_dir.path())
            .output()?;

        let git_output = create_command("git")?
            .args(&["tag", "--list", "--contains", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        let expected_tags = str::from_utf8(&git_output)
            .unwrap()
            .lines()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ");

        let config = GitTagConfig::default();
        let expected = Some(format!(
            "{} ",
            Color::Yellow
                .bold()
                .paint(format!("({}{})", config.symbol, expected_tags))
        ));

        let actual = ModuleRenderer::new("git_tag")
            .path(&repo_dir.path())
            .collect();

        assert_eq!(expected, actual);
        repo_dir.close()
    }
}
