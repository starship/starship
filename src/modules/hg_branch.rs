use std::io::{Error, ErrorKind};
use std::path::Path;

use super::utils::truncate::truncate_text;
use super::{Context, Module, ModuleConfig};

use crate::configs::hg_branch::HgBranchConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::path::PathExt;
use crate::utils::read_file;

/// Creates a module with the Hg bookmark or branch in the current directory
///
/// Will display the bookmark or branch name if the current directory is an hg repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("hg_branch");
    let config: HgBranchConfig = HgBranchConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    let len = if config.truncation_length <= 0 {
        log::warn!(
            "\"truncation_length\" should be a positive value, found {}",
            config.truncation_length
        );
        std::usize::MAX
    } else {
        config.truncation_length as usize
    };

    let repo_root = get_hg_repo_root(context).ok()?;
    let branch_name = get_hg_current_bookmark(repo_root).unwrap_or_else(|_| {
        get_hg_branch_name(repo_root).unwrap_or_else(|_| String::from("default"))
    });

    let branch_graphemes = truncate_text(&branch_name, len, config.truncation_symbol);
    let topic_graphemes = if let Ok(topic) = get_hg_topic_name(repo_root) {
        truncate_text(&topic, len, config.truncation_symbol)
    } else {
        String::from("")
    };

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
                "branch" => Some(Ok(branch_graphemes.as_str())),
                "topic" => Some(Ok(topic_graphemes.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `hg_branch`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_hg_repo_root<'a>(ctx: &'a Context) -> Result<&'a Path, Error> {
    let dir = ctx.current_dir.as_path();
    let dev_id = dir.device_id();
    for root_dir in dir.ancestors() {
        if dev_id != root_dir.device_id() {
            break;
        }
        if root_dir.join(".hg").is_dir() {
            return Ok(root_dir);
        }
    }
    Err(Error::new(ErrorKind::Other, "No .hg found!"))
}

fn get_hg_branch_name(hg_root: &Path) -> Result<String, Error> {
    match read_file(hg_root.join(".hg").join("branch")) {
        Ok(b) => Ok(b.trim().to_string()),
        Err(e) => Err(e),
    }
}

fn get_hg_current_bookmark(hg_root: &Path) -> Result<String, Error> {
    read_file(hg_root.join(".hg").join("bookmarks.current"))
}

fn get_hg_topic_name(hg_root: &Path) -> Result<String, Error> {
    read_file(hg_root.join(".hg").join("topic"))
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::{Color, Style};
    use std::fs;
    use std::io;
    use std::path::Path;

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use crate::utils::create_command;

    enum Expect<'a> {
        BranchName(&'a str),
        Empty,
        NoTruncation,
        Symbol(&'a str),
        Style(Style),
        TruncationSymbol(&'a str),
    }

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("hg_branch")
            .path(repo_dir.path())
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_disabled_per_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        run_hg(&["whatever", "blubber"], repo_dir)?;
        expect_hg_branch_with_config(
            repo_dir,
            // no "disabled=false" in config!
            Some(toml::toml! {
                [hg_branch]
                truncation_length = 14
            }),
            &[Expect::Empty],
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_get_branch_fails() -> io::Result<()> {
        let tempdir = tempfile::tempdir()?;

        // Create a fake corrupted mercurial repo.
        let hgdir = tempdir.path().join(".hg");
        fs::create_dir(&hgdir)?;
        fs::write(hgdir.join("requires"), "fake-corrupted-repo")?;

        expect_hg_branch_with_config(
            tempdir.path(),
            None,
            &[Expect::BranchName("default"), Expect::NoTruncation],
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_get_branch_autodisabled() -> io::Result<()> {
        let tempdir = tempfile::tempdir()?;
        expect_hg_branch_with_config(tempdir.path(), None, &[Expect::Empty]);
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_bookmark() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        run_hg(&["bookmark", "bookmark-101"], repo_dir)?;
        expect_hg_branch_with_config(
            repo_dir,
            None,
            &[Expect::BranchName("bookmark-101"), Expect::NoTruncation],
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_topic() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        fs::write(repo_dir.join(".hg").join("topic"), "feature")?;

        let actual = ModuleRenderer::new("hg_branch")
            .path(repo_dir.to_str().unwrap())
            .config(toml::toml! {
                [hg_branch]
                format = "$topic"
                disabled = false
            })
            .collect();

        assert_eq!(Some(String::from("feature")), actual);
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_default_truncation_symbol() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        run_hg(&["branch", "-f", "branch-name-101"], repo_dir)?;
        run_hg(
            &[
                "commit",
                "-m",
                "empty commit 101",
                "-u",
                "fake user <fake@user>",
            ],
            repo_dir,
        )?;
        expect_hg_branch_with_config(
            repo_dir,
            Some(toml::toml! {
                [hg_branch]
                truncation_length = 14
                disabled = false
            }),
            &[Expect::BranchName("branch-name-10")],
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_configured_symbols() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        run_hg(&["branch", "-f", "branch-name-121"], repo_dir)?;
        run_hg(
            &[
                "commit",
                "-m",
                "empty commit 121",
                "-u",
                "fake user <fake@user>",
            ],
            repo_dir,
        )?;
        expect_hg_branch_with_config(
            repo_dir,
            Some(toml::toml! {
                [hg_branch]
                symbol = "B "
                truncation_length = 14
                truncation_symbol = "%"
                disabled = false
            }),
            &[
                Expect::BranchName("branch-name-12"),
                Expect::Symbol("B"),
                Expect::TruncationSymbol("%"),
            ],
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_configured_style() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        run_hg(&["branch", "-f", "branch-name-131"], repo_dir)?;
        run_hg(
            &[
                "commit",
                "-m",
                "empty commit 131",
                "-u",
                "fake user <fake@user>",
            ],
            repo_dir,
        )?;

        expect_hg_branch_with_config(
            repo_dir,
            Some(toml::toml! {
                [hg_branch]
                style = "underline blue"
                disabled = false
            }),
            &[
                Expect::BranchName("branch-name-131"),
                Expect::Style(Color::Blue.underline()),
                Expect::TruncationSymbol(""),
            ],
        );
        tempdir.close()
    }

    fn expect_hg_branch_with_config(
        repo_dir: &Path,
        config: Option<toml::Table>,
        expectations: &[Expect],
    ) {
        let actual = ModuleRenderer::new("hg_branch")
            .path(repo_dir.to_str().unwrap())
            .config(config.unwrap_or_else(|| {
                toml::toml! {
                    [hg_branch]
                    disabled = false
                }
            }))
            .collect();

        let mut expect_branch_name = "default";
        let mut expect_style = Color::Purple.bold();
        let mut expect_symbol = "\u{e0a0}";
        let mut expect_truncation_symbol = "…";

        for expect in expectations {
            match expect {
                Expect::Empty => {
                    assert_eq!(None, actual);
                    return;
                }
                Expect::Symbol(symbol) => {
                    expect_symbol = symbol;
                }
                Expect::TruncationSymbol(truncation_symbol) => {
                    expect_truncation_symbol = truncation_symbol;
                }
                Expect::NoTruncation => {
                    expect_truncation_symbol = "";
                }
                Expect::BranchName(branch_name) => {
                    expect_branch_name = branch_name;
                }
                Expect::Style(style) => expect_style = *style,
            }
        }

        let expected = Some(format!(
            "on {} ",
            expect_style.paint(format!(
                "{expect_symbol} {expect_branch_name}{expect_truncation_symbol}"
            )),
        ));
        assert_eq!(expected, actual);
    }

    fn run_hg(args: &[&str], repo_dir: &Path) -> io::Result<()> {
        create_command("hg")?
            .args(args)
            .current_dir(repo_dir)
            .output()?;
        Ok(())
    }
}
