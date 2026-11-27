use super::{Context, Module, ModuleConfig};

use gix::remote::Direction;

use crate::configs::git_host::GitHostConfig;
use crate::context::GitRepo;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_host");
    let config: GitHostConfig = GitHostConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let repo = context.get_git_repo().ok()?;
    let gix_repo = repo.open();

    let remote = tracking_remote_name(context, repo)
        .and_then(|name| gix_repo.find_remote(name.as_str()).ok())
        .or_else(|| default_remote(&gix_repo))?;

    let url = remote.url(Direction::Fetch)?;
    let host = url.host()?;

    let symbol = config
        .symbols
        .get(host.to_lowercase().as_str())
        .copied()
        .unwrap_or(config.unknown);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_host`: \n{error}");
            return None;
        }
    });

    Some(module)
}

fn tracking_remote_name(context: &Context, repo: &GitRepo) -> Option<String> {
    let name = if repo.uses_reftables() {
        let branch = repo
            .exec_git(context, ["symbolic-ref", "HEAD"])
            .and_then(|out| {
                out.stdout
                    .trim()
                    .strip_prefix("refs/heads/")
                    .map(str::to_string)
            });

        branch.and_then(|b| {
            repo.exec_git(context, ["config", "--get", &format!("branch.{b}.remote")])
                .map(|out| out.stdout.trim().to_string())
                .filter(|r| !r.is_empty() && r != ".")
        })
    } else {
        repo.remote.as_ref().and_then(|r| r.name.clone())
    };

    name.filter(|n| !n.is_empty() && n != ".")
}

fn default_remote<'r>(repo: &'r gix::Repository) -> Option<gix::Remote<'r>> {
    if let Ok(remote) = repo.find_remote("origin") {
        return Some(remote);
    }
    let names = repo.remote_names();
    if names.len() == 1 {
        let name = names.iter().next()?;
        return repo.find_remote(name).ok();
    }
    None
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;
    use std::ffi::OsStr;
    use std::io;
    use std::path::Path;
    use std::process::Stdio;

    use crate::test::{COMMON_GIT_PROVIDERS, ModuleRenderer, fixture_repo, fixture_repo_with_hash};
    use crate::utils::create_command;

    #[test]
    fn show_nothing_when_disabled() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(
                [
                    "remote",
                    "set-url",
                    "origin",
                    "https://github.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .collect();
            let expected = None;

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_host")
            .path(repo_dir.path())
            .config(toml::toml! {
                [git_host]
                disabled = false
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()?;
        Ok(())
    }

    #[test]
    fn show_nothing_when_repo_has_no_remote() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(["remote", "remove", "origin"], repo_dir.path())?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false
                })
                .collect();
            let expected = None;

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_icon_when_upstream() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(
                [
                    "remote",
                    "set-url",
                    "origin",
                    "https://github.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            run_git_cmd(
                [
                    "remote",
                    "add",
                    "upstream",
                    "https://gitlab.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            run_git_cmd(
                ["update-ref", "refs/remotes/upstream/main", "HEAD"],
                repo_dir.path(),
            )?;

            run_git_cmd(["checkout", "-b", "feature"], repo_dir.path())?;

            run_git_cmd(
                ["branch", "--set-upstream-to=upstream/main"],
                repo_dir.path(),
            )?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false

                    [git_host.symbols]
                    "gitlab.com" = ""
                })
                .collect();

            let expected = Some(format!("{} ", Color::Purple.bold().paint("")));

            assert_eq!(actual, expected);

            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_icon_when_single_remote_is_not_named_origin() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(["remote", "rename", "origin", "backup"], repo_dir.path())?;

            run_git_cmd(
                [
                    "remote",
                    "set-url",
                    "backup",
                    "https://github.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false

                    [git_host.symbols]
                    "github.com" = ""
                })
                .collect();

            let expected = Some(format!("{} ", Color::Purple.bold().paint("")));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_nothing_when_tracking_remote_url_is_local_path() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let sha256 = rand::random();
            let remote_dir = fixture_repo_with_hash(mode, sha256)?;
            let repo_dir = fixture_repo_with_hash(mode, sha256)?;

            let remote_path = remote_dir
                .path()
                .to_str()
                .expect("temp path is not valid UTF-8");

            run_git_cmd(["checkout", "-b", "test_branch"], repo_dir.path())?;

            run_git_cmd(
                ["remote", "add", "--fetch", "remote_repo", remote_path],
                repo_dir.path(),
            )?;

            run_git_cmd(
                ["branch", "--set-upstream-to", "remote_repo/master"],
                repo_dir.path(),
            )?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false
                })
                .collect();

            let expected = None;

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_icon_of_default_remote_when_upstream_is_local_branch() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(
                [
                    "remote",
                    "set-url",
                    "origin",
                    "https://github.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            run_git_cmd(["checkout", "-b", "feature"], repo_dir.path())?;

            run_git_cmd(["branch", "--set-upstream-to=master"], repo_dir.path())?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false

                    [git_host.symbols]
                    "github.com" = ""
                })
                .collect();
            let expected = Some(format!("{} ", Color::Purple.bold().paint("")));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }

        Ok(())
    }

    #[test]
    fn show_icon_of_fallback_remote_on_detached_head() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(
                [
                    "remote",
                    "set-url",
                    "origin",
                    "https://github.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            run_git_cmd(
                [
                    "remote",
                    "add",
                    "upstream",
                    "https://gitlab.com/org/repo.git",
                ],
                repo_dir.path(),
            )?;

            run_git_cmd(
                ["update-ref", "refs/remotes/upstream/main", "HEAD"],
                repo_dir.path(),
            )?;

            run_git_cmd(
                ["branch", "--set-upstream-to=upstream/main"],
                repo_dir.path(),
            )?;

            run_git_cmd(["checkout", "--detach"], repo_dir.path())?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false

                    [git_host.symbols]
                    "github.com" = ""
                })
                .collect();

            let expected = Some(format!("{} ", Color::Purple.bold().paint("")));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_github_icon_with_uppercase_host() -> io::Result<()> {
        test_with_remote_provider_url("https://GitHub.com/user/repo.git", "")
    }

    #[test]
    fn show_github_icon_using_https() -> io::Result<()> {
        test_with_remote_provider_url("https://github.com/user/repo.git", "")
    }

    #[test]
    fn show_github_icon_using_ssh() -> io::Result<()> {
        test_with_remote_provider_url("ssh://git@github.com/user/repo.git", "")
    }

    #[test]
    fn show_github_icon_using_scp() -> io::Result<()> {
        test_with_remote_provider_url("git@github.com:user/repo.git", "")
    }

    #[test]
    fn show_gitlab_icon_using_https() -> io::Result<()> {
        test_with_remote_provider_url("https://gitlab.com/user/repo.git", "")
    }

    #[test]
    fn show_gitlab_icon_using_ssh() -> io::Result<()> {
        test_with_remote_provider_url("ssh://git@gitlab.com/user/repo.git", "")
    }

    #[test]
    fn show_gitlab_icon_using_ssh_with_port() -> io::Result<()> {
        test_with_remote_provider_url("ssh://git@gitlab.com:2222/user/repo.git", "")
    }

    #[test]
    fn show_gitlab_icon_using_scp() -> io::Result<()> {
        test_with_remote_provider_url("git@gitlab.com:user/repo.git", "")
    }

    #[test]
    fn custom_symbol_matches_host_with_port() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(
                [
                    "remote",
                    "set-url",
                    "origin",
                    "ssh://git@git.example.com:2222/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false
                    [git_host.symbols]
                    "git.example.com" = "X"
                })
                .collect();

            let expected = Some(format!("{} ", Color::Purple.bold().paint("X")));
            assert_eq!(actual, expected);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_unknown_symbol_with_unknown_url() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(
                [
                    "remote",
                    "set-url",
                    "origin",
                    "https://git.example.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false
                })
                .collect();

            let expected = Some(format!("{} ", Color::Purple.bold().paint("")));
            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn show_nothing_when_remote_have_no_url() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(["remote", "set-url", "origin", ""], repo_dir.path())?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false
                })
                .collect();

            let expected = None;

            assert_eq!(actual, expected);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn branch_and_tag_with_the_same_name() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(
                [
                    "remote",
                    "set-url",
                    "origin",
                    "https://github.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            run_git_cmd(
                [
                    "remote",
                    "add",
                    "upstream",
                    "https://gitlab.com/user/repo.git",
                ],
                repo_dir.path(),
            )?;

            // A tag with the same name as the current branch makes
            // `symbolic-ref --short HEAD` return `heads/master` on reftable repositories.
            run_git_cmd(["tag", "master"], repo_dir.path())?;

            run_git_cmd(
                ["update-ref", "refs/remotes/upstream/master", "HEAD"],
                repo_dir.path(),
            )?;

            run_git_cmd(
                ["branch", "--set-upstream-to=upstream/master"],
                repo_dir.path(),
            )?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false

                    [git_host.symbols]
                    "gitlab.com" = ""
                })
                .collect();

            let expected = Some(format!("{} ", Color::Purple.bold().paint("")));

            assert_eq!(actual, expected);

            repo_dir.close()?;
        }

        Ok(())
    }

    fn test_with_remote_provider_url(url: &str, expected_symbol: &str) -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;

            run_git_cmd(["remote", "set-url", "origin", url], repo_dir.path())?;

            let actual = ModuleRenderer::new("git_host")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_host]
                    disabled = false

                    [git_host.symbols]
                    "github.com" = ""
                    "gitlab.com" = ""
                })
                .collect();

            let expected = Some(format!("{} ", Color::Purple.bold().paint(expected_symbol)));

            assert_eq!(actual, expected);

            repo_dir.close()?;
        }
        Ok(())
    }

    fn run_git_cmd<A, S>(args: A, dir: &Path) -> io::Result<()>
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = create_command("git")?;
        cmd.args(args).current_dir(dir).stdin(Stdio::null());

        let output = cmd.output()?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::other(format!(
                "{cmd:?} failed ({}): {}",
                output.status,
                stderr.trim().replace('\n', " | ")
            )))
        }
    }
}
