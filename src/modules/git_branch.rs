use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module, ModuleConfig};

use crate::configs::git_branch::GitBranchConfig;
use crate::formatter::StringFormatter;

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
        std::usize::MAX
    } else {
        config.truncation_length as usize
    };

    let repo = context.get_repo().ok()?;

    if config.only_attached && repo.open().head().ok()?.is_detached() {
        return None;
    }

    let branch_name = repo.branch.as_ref()?;
    let mut graphemes: Vec<&str> = branch_name.graphemes(true).collect();

    if config
        .ignore_branches
        .iter()
        .any(|ignored| branch_name.eq(ignored))
    {
        return None;
    }

    let mut remote_branch_graphemes: Vec<&str> = Vec::new();
    let mut remote_name_graphemes: Vec<&str> = Vec::new();
    if let Some(remote) = repo.remote.as_ref() {
        if let Some(branch) = &remote.branch {
            remote_branch_graphemes = branch.graphemes(true).collect()
        };
        if let Some(name) = &remote.name {
            remote_name_graphemes = name.graphemes(true).collect()
        };
    }

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
            log::warn!("Error in module `git_branch`: \n{}", error);
            return None;
        }
    });

    Some(module)
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

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use crate::utils::create_command;

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
        test_truncate_length("नमस्ते", 3, "नमस्", "…")
    }

    #[test]
    fn test_hindi_truncation2() -> io::Result<()> {
        test_truncate_length("नमस्त", 3, "नमस्", "…")
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
        let repo_dir = tempfile::tempdir()?;

        create_command("git")?
            .args(["init"])
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
        repo_dir.close()
    }

    #[test]
    fn test_render_branch_only_attached_on_branch() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

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
        repo_dir.close()
    }

    #[test]
    fn test_render_branch_only_attached_on_detached() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

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
        repo_dir.close()
    }

    #[test]
    fn test_works_in_bare_repo() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        create_command("git")?
            .args(["init", "--bare"])
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
        repo_dir.close()
    }

    #[test]
    fn test_ignore_branches() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

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
        repo_dir.close()
    }

    #[test]
    fn test_remote() -> io::Result<()> {
        let remote_dir = fixture_repo(FixtureProvider::Git)?;
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

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
        remote_dir.close()
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
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

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
        repo_dir.close()
    }

    fn test_format<T: Into<String>>(
        branch_name: &str,
        format: &str,
        config_options: &str,
        expected: T,
    ) -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

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

        assert_eq!(Some(expected.into()), actual);
        repo_dir.close()
    }
}
