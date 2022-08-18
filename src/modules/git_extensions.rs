use super::{Context, Module, ModuleConfig};
use crate::configs::git_extensions::GitExtensionsConfig;
use crate::formatter::StringFormatter;

/// Creates a module with any git extensions active in the repo at the current directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_extensions");
    let config: GitExtensionsConfig = GitExtensionsConfig::try_load(module.config);

    let git_config = context.get_repo().ok()?.open().ok()?.config().ok()?;

    let exts = config.extensions;
    let mut active_exts: Vec<&'a str> = vec![];

    // iterate over exts, running functions for each one that we know
    // about and appending to active_exts when a match is found
    for ext_configured in &exts {
        match *ext_configured {
            "lfs" => {
                // we don't care what the value is, merely that it exists.
                git_config.get_entry("lfs.repositoryformatversion").ok()?;
                active_exts.push("lfs")
            }
            "svn" => {
                git_config.get_entry("svn-remote.svn.url").ok()?;
                active_exts.push("svn")
            }
            _ => {
                log::error!("Unknown git extension '{ext_configured}'. If this isn't expected, please create a feature request.")
            }
        };
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "active_exts" => {
                    if active_exts.is_empty() {
                        None
                    } else {
                        Some(Ok(active_exts.join(", ")))
                    }
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_extensions`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::io;

    use crate::test::{create_repo, run_git_cmd, ModuleRenderer};

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_extensions")
            .config(toml::toml! {
                [git_extensions]
                disabled=false
            })
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn show_nothing_on_repo_without_extensions() -> io::Result<()> {
        let repo_dir = create_repo()?;

        let actual = ModuleRenderer::new("git_extensions")
            .config(toml::toml! {
                [git_extensions]
                disabled=false
            })
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn show_lfs() -> io::Result<()> {
        let repo_dir = add_lfs_to_repo(create_repo()?)?;

        let actual = ModuleRenderer::new("git_extensions")
            .config(toml::toml! {
                [git_extensions]
                disabled=false
                extensions=["lfs"]
            })
            .path(repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{}",
            Color::Fixed(149).bold().paint("git exts: lfs ")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn show_svn() -> io::Result<()> {
        let repo_dir = add_svn_to_repo(create_repo()?)?;

        let actual = ModuleRenderer::new("git_extensions")
            .config(toml::toml! {
                [git_extensions]
                disabled=false
                extensions=["svn"]
            })
            .path(repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{}",
            Color::Fixed(149).bold().paint("git exts: svn ")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn show_multiple() -> io::Result<()> {
        let repo_dir = add_svn_to_repo(add_lfs_to_repo(create_repo()?)?)?;

        let actual = ModuleRenderer::new("git_extensions")
            .config(toml::toml! {
                [git_extensions]
                disabled=false
                extensions=["lfs", "svn"]
            })
            .path(repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{}",
            Color::Fixed(149).bold().paint("git exts: lfs, svn ")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    fn add_lfs_to_repo(repo_dir: tempfile::TempDir) -> io::Result<tempfile::TempDir> {
        run_git_cmd(
            &["config", "lfs.repositoryformatversion", "0"],
            Some(repo_dir.path()),
            true,
        )?;

        Ok(repo_dir)
    }

    fn add_svn_to_repo(repo_dir: tempfile::TempDir) -> io::Result<tempfile::TempDir> {
        run_git_cmd(
            &["config", "svn-remote.svn.url", "0"],
            Some(repo_dir.path()),
            true,
        )?;

        Ok(repo_dir)
    }
}
