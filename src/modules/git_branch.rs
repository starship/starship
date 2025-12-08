use gix::bstr::ByteSlice;
use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module, ModuleConfig};

use crate::configs::git_branch::GitBranchConfig;
use crate::context::Repo;
use crate::formatter::StringFormatter;
use crate::modules::git_status::uses_reftables;

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_branch");
    let config = GitBranchConfig::try_load(module.config);

    let truncation_symbol = get_first_grapheme(config.truncation_symbol);

    let len = if config.truncation_length <= 0 {
        log::warn!(
            "\"truncation_length\" should be a positive value, found {}",
            config.truncation_length
        );
        usize::MAX
    } else {
        config.truncation_length as usize
    };

    let repo = context.get_repo().ok()?;

    let gix_repo = repo.open();
    if config.ignore_bare_repo && gix_repo.is_bare() {
        return None;
    }

    // Get branch and remote information
    let (branch_name, remote_branch, remote_name) = if uses_reftables(&gix_repo) {
        // Use git executable for branch information
        match get_branch_info_from_git(context, repo) {
            Some((branch, remote_branch)) => {
                // Successfully got branch name, parse upstream into remote_name and remote_branch
                let remote_name = remote_branch
                    .as_ref()
                    .map(|remote_branch| {
                        find_longest_matching_remote_name(
                            remote_branch.as_ref(),
                            &gix_repo.remote_names(),
                        )
                    })
                    .unwrap_or_default();
                (
                    branch.shorten().to_string(),
                    remote_branch.zip(remote_name.as_deref()).map(
                        |(remote_branch, remote_name)| {
                            remote_branch
                                .shorten()
                                .to_str_lossy()
                                .strip_prefix(remote_name)
                                .expect("remote name determined by finding it as prefix before")
                                .trim_start_matches("/")
                                .to_string()
                        },
                    ),
                    remote_name,
                )
            }
            None => {
                // get_branch_info_from_git returns None when:
                // 1. HEAD is detached (git symbolic-ref fails)
                // 2. git command fails for other reasons (corrupted repo, missing git, etc.)
                // In both cases, if only_attached is set, we should return None
                if config.only_attached {
                    return None;
                }
                // Fallback to HEAD for detached state or when branch can't be determined
                ("HEAD".to_string(), None, None)
            }
        }
    } else {
        if config.only_attached && gix_repo.head().ok()?.is_detached() {
            return None;
        }

        let branch = repo.branch.clone().unwrap_or_else(|| "HEAD".to_string());
        let (remote_branch, remote_name) = if let Some(remote) = repo.remote.as_ref() {
            (remote.branch.clone(), remote.name.clone())
        } else {
            (None, None)
        };
        (branch, remote_branch, remote_name)
    };

    if config
        .ignore_branches
        .iter()
        .any(|&ignored| branch_name.eq(ignored))
    {
        return None;
    }

    let mut graphemes: Vec<&str> = branch_name.graphemes(true).collect();

    let remote_branch_string = remote_branch.unwrap_or_default();
    let mut remote_branch_graphemes: Vec<&str> = remote_branch_string.graphemes(true).collect();

    let remote_name_string = remote_name.unwrap_or_default();
    let mut remote_name_graphemes: Vec<&str> = remote_name_string.graphemes(true).collect();

    // Truncate fields if need be
    for e in &mut [
        &mut graphemes,
        &mut remote_branch_graphemes,
        &mut remote_name_graphemes,
    ] {
        let e = &mut **e;
        let trunc_len = len.min(e.len());
        if trunc_len < e.len() {
            // The truncation symbol should only be added if we truncate
            e[trunc_len] = truncation_symbol;
            e.truncate(trunc_len + 1);
        }
    }

    let show_remote = config.always_show_remote
        || (!graphemes.eq(&remote_branch_graphemes) && !remote_branch_graphemes.is_empty());

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
                "branch" => Some(Ok(graphemes.concat())),
                "remote_branch" => {
                    if show_remote && !remote_branch_graphemes.is_empty() {
                        Some(Ok(remote_branch_graphemes.concat()))
                    } else {
                        None
                    }
                }
                "remote_name" => {
                    if show_remote && !remote_name_graphemes.is_empty() {
                        Some(Ok(remote_name_graphemes.concat()))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_branch`: \n{error}");
            return None;
        }
    });

    Some(module)
}

/// Given `remote_names`, find the longest matching remote name in `remote_ref_name` and return it.
fn find_longest_matching_remote_name(
    remote_ref_name: &gix::refs::FullNameRef,
    remote_names: &gix::remote::Names<'_>,
) -> Option<String> {
    let (category, shorthand_name) = remote_ref_name.category_and_short_name()?;
    if !matches!(category, gix::refs::Category::RemoteBranch) {
        return None;
    }

    let longest_remote = remote_names
        .iter()
        .rfind(|reference_name| shorthand_name.starts_with(reference_name))?;
    Some(longest_remote.to_string())
}

/// Get branch information using the git executable.
/// Returns `None` if not on a branch (detached HEAD) or if the git command fails.
fn get_branch_info_from_git(
    context: &Context,
    repo: &Repo,
) -> Option<(gix::refs::FullName, Option<gix::refs::FullName>)> {
    // Get current branch name using git symbolic-ref
    let branch_output = repo.exec_git(context, ["symbolic-ref", "HEAD"])?;

    // Check if the command was successful (exit code 0)
    // symbolic-ref returns non-zero exit code when HEAD is detached
    let branch_name = branch_output.stdout.trim();
    if branch_name.is_empty() {
        return None;
    }

    let branch_name: gix::refs::FullName = branch_name.try_into().ok()?;
    // Get upstream tracking branch using git for-each-ref
    let upstream_output = repo.exec_git(
        context,
        [
            "for-each-ref",
            "--format=%(upstream)",
            branch_name.as_bstr().to_str_lossy().as_ref(),
        ],
    )?;
    let upstream = upstream_output.stdout.trim();
    let remote_info = if upstream.is_empty() {
        None
    } else {
        upstream.try_into().ok()
    };

    Some((branch_name, remote_info))
}

fn get_first_grapheme(text: &str) -> &str {
    UnicodeSegmentation::graphemes(text, true)
        .next()
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;
    use std::io;

    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use crate::utils::create_command;

    const NORMAL_AND_REFTABLE: [FixtureProvider; 2] =
        [FixtureProvider::Git, FixtureProvider::GitReftable];
    const BARE_AND_REFTABLE: [FixtureProvider; 2] =
        [FixtureProvider::GitBare, FixtureProvider::GitBareReftable];

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_branch")
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_changed_truncation_symbol() -> io::Result<()> {
        test_truncate_length_with_config(
            "1337_hello_world",
            15,
            "1337_hello_worl",
            "%",
            "truncation_symbol = \"%\"",
        )
    }

    #[test]
    fn test_no_truncation_symbol() -> io::Result<()> {
        test_truncate_length_with_config(
            "1337_hello_world",
            15,
            "1337_hello_worl",
            "",
            "truncation_symbol = \"\"",
        )
    }

    #[test]
    fn test_multi_char_truncation_symbol() -> io::Result<()> {
        test_truncate_length_with_config(
            "1337_hello_world",
            15,
            "1337_hello_worl",
            "a",
            "truncation_symbol = \"apple\"",
        )
    }

    #[test]
    fn test_ascii_boundary_below() -> io::Result<()> {
        test_truncate_length("1337_hello_world", 15, "1337_hello_worl", "…")
    }

    #[test]
    fn test_ascii_boundary_on() -> io::Result<()> {
        test_truncate_length("1337_hello_world", 16, "1337_hello_world", "")
    }

    #[test]
    fn test_ascii_boundary_above() -> io::Result<()> {
        test_truncate_length("1337_hello_world", 17, "1337_hello_world", "")
    }

    #[test]
    fn test_one() -> io::Result<()> {
        test_truncate_length("1337_hello_world", 1, "1", "…")
    }

    #[test]
    fn test_zero() -> io::Result<()> {
        test_truncate_length("1337_hello_world", 0, "1337_hello_world", "")
    }

    #[test]
    fn test_negative() -> io::Result<()> {
        test_truncate_length("1337_hello_world", -1, "1337_hello_world", "")
    }

    #[test]
    fn test_hindi_truncation() -> io::Result<()> {
        test_truncate_length("नमस्ते", 2, "नम", "…")
    }

    #[test]
    fn test_hindi_truncation2() -> io::Result<()> {
        test_truncate_length("नमस्त", 2, "नम", "…")
    }

    #[test]
    fn test_japanese_truncation() -> io::Result<()> {
        test_truncate_length("がんばってね", 4, "がんばっ", "…")
    }

    #[test]
    fn test_format_no_branch() -> io::Result<()> {
        test_format("1337_hello_world", "no_branch", "", "no_branch")
    }

    #[test]
    fn test_format_just_branch_name() -> io::Result<()> {
        test_format("1337_hello_world", "$branch", "", "1337_hello_world")
    }

    #[test]
    fn test_format_just_branch_name_color() -> io::Result<()> {
        test_format(
            "1337_hello_world",
            "[$branch](bold blue)",
            "",
            Color::Blue.bold().paint("1337_hello_world").to_string(),
        )
    }

    #[test]
    fn test_format_mixed_colors() -> io::Result<()> {
        test_format(
            "1337_hello_world",
            "branch: [$branch](bold blue) [THE COLORS](red) ",
            "",
            format!(
                "branch: {} {} ",
                Color::Blue.bold().paint("1337_hello_world"),
                Color::Red.paint("THE COLORS")
            ),
        )
    }

    #[test]
    fn test_format_symbol_style() -> io::Result<()> {
        test_format(
            "1337_hello_world",
            "$symbol[$branch]($style)",
            r#"
            symbol = "git: "
            style = "green"
        "#,
            format!("git: {}", Color::Green.paint("1337_hello_world"),),
        )
    }

    #[test]
    fn test_works_with_unborn_default_branch() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = tempfile::tempdir()?;

            create_command("git")?
                .arg("init")
                .args(maybe_reftable_format(mode))
                .current_dir(&repo_dir)
                .output()?;

            create_command("git")?
                .args(["symbolic-ref", "HEAD", "refs/heads/main"])
                .current_dir(&repo_dir)
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .path(repo_dir.path())
                .collect();

            let expected = Some(format!(
                "on {} ",
                Color::Purple.bold().paint(format!("\u{e0a0} {}", "main")),
            ));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_render_branch_only_attached_on_branch() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;

            create_command("git")?
                .args(["checkout", "-b", "test_branch"])
                .current_dir(repo_dir.path())
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .config(toml::toml! {
                    [git_branch]
                        only_attached = true
                })
                .path(repo_dir.path())
                .collect();

            let expected = Some(format!(
                "on {} ",
                Color::Purple
                    .bold()
                    .paint(format!("\u{e0a0} {}", "test_branch")),
            ));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_render_branch_only_attached_on_detached() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;

            create_command("git")?
                .args(["checkout", "@~1"])
                .current_dir(repo_dir.path())
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .config(toml::toml! {
                    [git_branch]
                        only_attached = true
                })
                .path(repo_dir.path())
                .collect();

            let expected = None;

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_works_in_bare_repo() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = tempfile::tempdir()?;

            create_command("git")?
                .arg("init")
                .args(maybe_reftable_format(mode))
                .arg("--bare")
                .current_dir(&repo_dir)
                .output()?;

            create_command("git")?
                .args(["symbolic-ref", "HEAD", "refs/heads/main"])
                .current_dir(&repo_dir)
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .path(repo_dir.path())
                .collect();

            let expected = Some(format!(
                "on {} ",
                Color::Purple.bold().paint(format!("\u{e0a0} {}", "main")),
            ));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_ignore_branches() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;

            create_command("git")?
                .args(["checkout", "-b", "test_branch"])
                .current_dir(repo_dir.path())
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .config(toml::toml! {
                    [git_branch]
                        ignore_branches = ["dummy", "test_branch"]
                })
                .path(repo_dir.path())
                .collect();

            let expected = None;

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_ignore_bare_repo() -> io::Result<()> {
        for mode in BARE_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;

            let actual = ModuleRenderer::new("git_branch")
                .config(toml::toml! {
                    [git_branch]
                        ignore_bare_repo = true

                })
                .path(repo_dir.path())
                .collect();

            let expected = None;

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_remote() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let remote_dir = fixture_repo(mode)?;
            let repo_dir = fixture_repo(mode)?;

            create_command("git")?
                .args(["checkout", "-b", "test_branch"])
                .current_dir(repo_dir.path())
                .output()?;

            create_command("git")?
                .args(["remote", "add", "--fetch", "remote_repo"])
                .arg(remote_dir.path())
                .current_dir(repo_dir.path())
                .output()?;

            create_command("git")?
                .args(["branch", "--set-upstream-to", "remote_repo/master"])
                .current_dir(repo_dir.path())
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .path(repo_dir.path())
                .config(toml::toml! {
                    [git_branch]
                    format = "$branch(:$remote_name/$remote_branch)"
                })
                .collect();

            let expected = Some("test_branch:remote_repo/master");

            assert_eq!(expected, actual.as_deref());
            repo_dir.close()?;
            remote_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_branch_fallback_on_detached() -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;

            create_command("git")?
                .args(["checkout", "@~1"])
                .current_dir(repo_dir.path())
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .config(toml::toml! {
                    [git_branch]
                    format = "$branch"
                })
                .path(repo_dir.path())
                .collect();

            let expected = Some("HEAD".into());

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    // This test is not possible until we switch to `git status --porcelain`
    // where we can mock the env for the specific git process. This is because
    // git2 does not care about our mocking and when we set the real `GIT_DIR`
    // variable it will interfere with the other tests.
    // #[test]
    // fn test_git_dir_env_variable() -> io::Result<()> {let repo_dir =
    // tempfile::tempdir()?;

    //     create_command("git")?
    //         .args(&["init"])
    //         .current_dir(&repo_dir)
    //         .output()?;

    //     // git2 does not care about our mocking
    //     std::env::set_var("GIT_DIR", repo_dir.path().join(".git"));

    //     let actual = ModuleRenderer::new("git_branch").collect();

    //     std::env::remove_var("GIT_DIR");

    //     let expected = Some(format!(
    //         "on {} ",
    //         Color::Purple.bold().paint(format!("\u{e0a0} {}", "master")),
    //     ));

    //     assert_eq!(expected, actual);
    //     repo_dir.close()
    // }

    fn test_truncate_length(
        branch_name: &str,
        truncate_length: i64,
        expected_name: &str,
        truncation_symbol: &str,
    ) -> io::Result<()> {
        test_truncate_length_with_config(
            branch_name,
            truncate_length,
            expected_name,
            truncation_symbol,
            "",
        )
    }

    fn test_truncate_length_with_config(
        branch_name: &str,
        truncate_length: i64,
        expected_name: &str,
        truncation_symbol: &str,
        config_options: &str,
    ) -> io::Result<()> {
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;

            create_command("git")?
                .args(["checkout", "-b", branch_name])
                .current_dir(repo_dir.path())
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .config(
                    toml::from_str(&format!(
                        "
                    [git_branch]
                        truncation_length = {truncate_length}
                        {config_options}
                "
                    ))
                    .unwrap(),
                )
                .path(repo_dir.path())
                .collect();

            let expected = Some(format!(
                "on {} ",
                Color::Purple
                    .bold()
                    .paint(format!("\u{e0a0} {expected_name}{truncation_symbol}")),
            ));

            assert_eq!(expected, actual);
            repo_dir.close()?;
        }
        Ok(())
    }

    fn test_format<T: Into<String>>(
        branch_name: &str,
        format: &str,
        config_options: &str,
        expected: T,
    ) -> io::Result<()> {
        let expected = expected.into();
        for mode in NORMAL_AND_REFTABLE {
            let repo_dir = fixture_repo(mode)?;

            create_command("git")?
                .args(["checkout", "-b", branch_name])
                .current_dir(repo_dir.path())
                .output()?;

            let actual = ModuleRenderer::new("git_branch")
                .config(
                    toml::from_str(&format!(
                        r#"
                    [git_branch]
                        format = "{format}"
                        {config_options}
                "#
                    ))
                    .unwrap(),
                )
                .path(repo_dir.path())
                .collect();

            assert_eq!(Some(&expected), actual.as_ref());
            repo_dir.close()?;
        }
        Ok(())
    }

    fn maybe_reftable_format(provider: FixtureProvider) -> Option<&'static str> {
        matches!(provider, FixtureProvider::GitReftable).then(|| "--ref-format=reftable")
    }
}
