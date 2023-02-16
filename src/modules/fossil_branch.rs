use super::{Context, Module, ModuleConfig};

use crate::configs::fossil_branch::FossilBranchConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::truncate::truncate_text;

/// Creates a module with the Fossil branch of the check-out in the current directory
///
/// Will display the branch name if the current directory is a Fossil check-out
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("fossil_branch");
    let config = FossilBranchConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    // See if we're in a check-out by scanning upwards for a directory containing ".fslckout"
    context
        .begin_ancestor_scan()
        .set_files(&[".fslckout"])
        .scan()?;

    let len = if config.truncation_length <= 0 {
        log::warn!(
            "\"truncation_length\" should be a positive value, found {}",
            config.truncation_length
        );
        std::usize::MAX
    } else {
        config.truncation_length as usize
    };

    let truncated_branch_name = {
        let output = context.exec_cmd("fossil", &["branch", "current"])?.stdout;
        truncate_text(output.trim(), len, config.truncation_symbol)
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
                "branch" => Some(Ok(truncated_branch_name.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `fossil_branch`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;

    use nu_ansi_term::{Color, Style};

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};

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
        let checkout_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("fossil_branch")
            .path(checkout_dir.path())
            .collect();
        let expected = None;
        assert_eq!(expected, actual);

        checkout_dir.close()
    }

    #[test]
    fn test_fossil_branch_disabled_per_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Fossil)?;
        let checkout_dir = tempdir.path();
        expect_fossil_branch_with_config(
            checkout_dir,
            Some(toml::toml! {
                // no "disabled=false" in config!
                [fossil_branch]
                truncation_length = 14
            }),
            &[Expect::Empty],
        );
        tempdir.close()
    }

    #[test]
    fn test_fossil_branch_autodisabled() -> io::Result<()> {
        let tempdir = tempfile::tempdir()?;
        expect_fossil_branch_with_config(tempdir.path(), None, &[Expect::Empty]);
        tempdir.close()
    }

    #[test]
    fn test_fossil_branch() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Fossil)?;
        let checkout_dir = tempdir.path();
        run_fossil(&["branch", "new", "topic-branch", "trunk"], checkout_dir)?;
        run_fossil(&["update", "topic-branch"], checkout_dir)?;
        expect_fossil_branch_with_config(
            checkout_dir,
            None,
            &[Expect::BranchName("topic-branch"), Expect::NoTruncation],
        );
        tempdir.close()
    }

    #[test]
    fn test_fossil_branch_subdir() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Fossil)?;
        let checkout_dir = tempdir.path();
        expect_fossil_branch_with_config(
            &checkout_dir.join("subdir"),
            None,
            &[Expect::BranchName("topic-branch"), Expect::NoTruncation],
        );
        tempdir.close()
    }

    #[test]
    fn test_fossil_branch_configured() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Fossil)?;
        let checkout_dir = tempdir.path();
        run_fossil(&["branch", "new", "topic-branch", "trunk"], checkout_dir)?;
        run_fossil(&["update", "topic-branch"], checkout_dir)?;
        expect_fossil_branch_with_config(
            checkout_dir,
            Some(toml::toml! {
                [fossil_branch]
                style = "underline blue"
                symbol = "F "
                truncation_length = 10
                truncation_symbol = "%"
                disabled = false
            }),
            &[
                Expect::BranchName("topic-bran"),
                Expect::Style(Color::Blue.underline()),
                Expect::Symbol("F"),
                Expect::TruncationSymbol("%"),
            ],
        );
        tempdir.close()
    }

    fn expect_fossil_branch_with_config(
        checkout_dir: &Path,
        config: Option<toml::Table>,
        expectations: &[Expect],
    ) {
        let actual = ModuleRenderer::new("fossil_branch")
            .path(checkout_dir.to_str().unwrap())
            .config(config.unwrap_or_else(|| {
                toml::toml! {
                    [fossil_branch]
                    disabled = false
                }
            }))
            .collect();

        let mut expect_branch_name = "trunk";
        let mut expect_style = Color::Purple.bold();
        let mut expect_symbol = "\u{e0a0}";
        let mut expect_truncation_symbol = "…";

        for expect in expectations {
            match expect {
                Expect::Empty => {
                    assert_eq!(None, actual);
                    return;
                }
                Expect::Symbol(symbol) => expect_symbol = symbol,
                Expect::TruncationSymbol(truncation_symbol) => {
                    expect_truncation_symbol = truncation_symbol
                }
                Expect::NoTruncation => expect_truncation_symbol = "",
                Expect::BranchName(branch_name) => expect_branch_name = branch_name,
                Expect::Style(style) => expect_style = *style,
            }
        }

        let expected = Some(format!(
            "on {} ",
            expect_style.paint(format!(
                "{expect_symbol} {expect_branch_name}{expect_truncation_symbol}"
            ))
        ));
        assert_eq!(expected, actual);
    }

    fn run_fossil(args: &[&str], _checkout_dir: &Path) -> io::Result<()> {
        crate::utils::mock_cmd("fossil", args).ok_or(io::ErrorKind::Unsupported)?;
        Ok(())
    }
}
