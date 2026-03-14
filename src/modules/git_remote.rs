use super::{Context, Module, ModuleConfig};

use crate::configs::git_remote::GitRemoteConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the Git remote provider symbol
///
/// Will display a symbol for the git remote provider (GitHub, GitLab, Bitbucket, etc.)
/// based on the remote URL of the current branch. Shows a fallback symbol when
/// inside a git repo with no remote configured.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_remote");
    let config = GitRemoteConfig::try_load(module.config);

    let repo = context.get_repo().ok()?;

    // Try to get the URL from the repo's remote info
    let remote_url = get_remote_url(context, repo);

    // Determine the symbol based on the remote URL (or lack thereof)
    let symbol = match &remote_url {
        Some(url) => resolve_symbol(url, &config),
        None => config.no_remote_symbol.to_string(),
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(symbol.as_str()),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "url" => Some(Ok(remote_url.clone().unwrap_or_default())),
                "remote_name" => Some(Ok(repo
                    .remote
                    .as_ref()
                    .and_then(|r| r.name.clone())
                    .unwrap_or_default())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_remote`: \n{error}");
            return None;
        }
    });

    Some(module)
}

/// Get the remote URL, first from cached Remote info, then by opening the gix repo.
fn get_remote_url(_context: &Context, repo: &crate::context::Repo) -> Option<String> {
    // First try the cached URL from context
    if let Some(url) = repo.remote.as_ref().and_then(|r| r.url.clone()) {
        return Some(url);
    }

    // Fallback: open gix repo and try to find the "origin" remote URL
    let gix_repo = repo.open();
    let remote = gix_repo.find_remote("origin").ok().or_else(|| {
        // If no "origin", try the first available remote
        gix_repo
            .remote_names()
            .into_iter()
            .next()
            .and_then(|name| gix_repo.find_remote(name.as_ref()).ok())
    })?;

    let url = remote
        .url(gix::remote::Direction::Fetch)?
        .to_bstring()
        .to_string();

    if url.is_empty() { None } else { Some(url) }
}

/// Normalize an SSH-style remote URL so that provider keys using `/` also match.
/// e.g. `git@github.com:user/repo.git` → `github.com/user/repo.git`
fn normalize_url(url: &str) -> String {
    if let Some(rest) = url.strip_prefix("git@") {
        rest.replacen(':', "/", 1)
    } else {
        url.to_string()
    }
}

/// Resolve a provider symbol from the remote URL using the configured providers map.
/// When multiple keys match, the longest (most specific) key wins.
fn resolve_symbol<'a>(url: &str, config: &'a GitRemoteConfig<'a>) -> String {
    let normalized = normalize_url(url);
    config
        .providers
        .iter()
        .filter(|(domain, _)| normalized.contains(domain.as_str()))
        .max_by_key(|(domain, _)| domain.len())
        .map(|(_, symbol)| symbol.clone())
        .unwrap_or_else(|| config.symbol.to_string())
}

#[cfg(test)]
mod tests {
    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use crate::utils::create_command;
    use std::io;

    const NORMAL_AND_REFTABLE: [FixtureProvider; 2] =
        [FixtureProvider::Git, FixtureProvider::GitReftable];

    /// Helper: replace the fixture's "origin" remote with a custom URL
    fn set_remote_url(path: &std::path::Path, url: &str) -> io::Result<()> {
        create_command("git")?
            .args(["remote", "set-url", "origin", url])
            .current_dir(path)
            .output()?;
        Ok(())
    }

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_remote")
            .config(toml::toml! {
                [git_remote]
                disabled = false
            })
            .path(repo_dir.path())
            .collect();

        assert_eq!(None, actual);
        repo_dir.close()
    }

    #[test]
    fn show_no_remote_symbol_without_remote() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;

            // Remove the origin remote that the fixture creates
            create_command("git")?
                .args(["remote", "remove", "origin"])
                .current_dir(repo_dir.path())
                .output()?;

            let actual = ModuleRenderer::new("git_remote")
                .config(toml::toml! {
                    [git_remote]
                    disabled = false
                    format = "$symbol"
                })
                .path(repo_dir.path())
                .collect();

            assert_eq!(Some("\u{f1d3}".to_string()), actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_github_symbol() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;
            set_remote_url(repo_dir.path(), "https://github.com/user/repo.git")?;

            let actual = ModuleRenderer::new("git_remote")
                .config(toml::toml! {
                    [git_remote]
                    disabled = false
                    format = "$symbol"
                })
                .path(repo_dir.path())
                .collect();

            assert_eq!(Some("\u{eb00}".to_string()), actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_gitlab_symbol() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;
            set_remote_url(repo_dir.path(), "git@gitlab.com:user/repo.git")?;

            let actual = ModuleRenderer::new("git_remote")
                .config(toml::toml! {
                    [git_remote]
                    disabled = false
                    format = "$symbol"
                })
                .path(repo_dir.path())
                .collect();

            assert_eq!(Some("\u{f0ba0}".to_string()), actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_bitbucket_symbol() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;
            set_remote_url(repo_dir.path(), "git@bitbucket.org:user/repo.git")?;

            let actual = ModuleRenderer::new("git_remote")
                .config(toml::toml! {
                    [git_remote]
                    disabled = false
                    format = "$symbol"
                })
                .path(repo_dir.path())
                .collect();

            assert_eq!(Some("\u{f00a8}".to_string()), actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_default_symbol_for_unknown_remote() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;
            set_remote_url(
                repo_dir.path(),
                "https://my-custom-git.example.com/repo.git",
            )?;

            let actual = ModuleRenderer::new("git_remote")
                .config(toml::toml! {
                    [git_remote]
                    disabled = false
                    format = "$symbol"
                })
                .path(repo_dir.path())
                .collect();

            assert_eq!(Some("\u{f02a2}".to_string()), actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_custom_provider_symbol() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;
            set_remote_url(repo_dir.path(), "https://git.example.com/repo.git")?;

            let actual = ModuleRenderer::new("git_remote")
                .config(toml::toml! {
                    [git_remote]
                    disabled = false
                    format = "$symbol"
                    [git_remote.providers]
                    "git.example.com" = "EX "
                })
                .path(repo_dir.path())
                .collect();

            assert_eq!(Some("EX ".to_string()), actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_specific_account_symbol() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;
            set_remote_url(repo_dir.path(), "https://github.com/work-org/repo.git")?;

            let actual = ModuleRenderer::new("git_remote")
                .config(toml::toml! {
                    [git_remote]
                    disabled = false
                    format = "$symbol"
                    [git_remote.providers]
                    "github.com" = "GH "
                    "github.com/work-org" = "WORK "
                })
                .path(repo_dir.path())
                .collect();

            assert_eq!(Some("WORK ".to_string()), actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_url_variable() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;
            set_remote_url(repo_dir.path(), "https://git.example.org/myuser/myrepo.git")?;

            let actual = ModuleRenderer::new("git_remote")
                .config(toml::toml! {
                    [git_remote]
                    disabled = false
                    format = "$url"
                })
                .path(repo_dir.path())
                .collect();

            let actual_str = actual.expect("expected url output");
            assert!(
                actual_str.contains("git.example.org"),
                "Expected URL to contain 'git.example.org', got: {actual_str}"
            );
            repo_dir.close()?;
        }
        Ok(())
    }
}
