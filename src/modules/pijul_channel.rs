use super::utils::truncate::truncate_text;
use super::{Context, Module, ModuleConfig};

use crate::configs::pijul_channel::PijulConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the Pijul channel in the current directory
///
/// Will display the channel lame if the current directory is a pijul repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_repo = context
        .try_begin_scan()?
        .set_folders(&[".pijul"])
        .is_match();

    if !is_repo {
        return None;
    }

    let mut module = context.new_module("pijul_channel");
    let config: PijulConfig = PijulConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    };

    let channel_name = get_pijul_current_channel(context)?;

    let truncated_text = truncate_text(
        &channel_name,
        config.truncation_length as usize,
        config.truncation_symbol,
    );

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
                "channel" => Some(Ok(&truncated_text)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `pijul_channel`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_pijul_current_channel(ctx: &Context) -> Option<String> {
    let output = ctx.exec_cmd("pijul", &["channel"])?.stdout;

    output
        .lines()
        .find_map(|l| l.strip_prefix("* "))
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::{Color, Style};
    use std::io;
    use std::path::Path;

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};

    enum Expect<'a> {
        ChannelName(&'a str),
        Empty,
        NoTruncation,
        Symbol(&'a str),
        Style(Style),
        TruncationSymbol(&'a str),
    }

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("pijul_channel")
            .path(repo_dir.path())
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_pijul_disabled_per_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Pijul)?;
        let repo_dir = tempdir.path();
        expect_pijul_with_config(
            repo_dir,
            Some(toml::toml! {
                [pijul_channel]
                truncation_length = 14
            }),
            &[Expect::Empty],
        );
        tempdir.close()
    }

    #[test]
    fn test_pijul_autodisabled() -> io::Result<()> {
        let tempdir = tempfile::tempdir()?;
        expect_pijul_with_config(tempdir.path(), None, &[Expect::Empty]);
        tempdir.close()
    }

    #[test]
    fn test_pijul_channel() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Pijul)?;
        let repo_dir = tempdir.path();
        run_pijul(&["channel", "new", "tributary-48198"], repo_dir)?;
        run_pijul(&["channel", "switch", "tributary-48198"], repo_dir)?;
        expect_pijul_with_config(
            repo_dir,
            None,
            &[Expect::ChannelName("tributary-48198"), Expect::NoTruncation],
        );
        tempdir.close()
    }

    #[test]
    fn test_pijul_configured() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Pijul)?;
        let repo_dir = tempdir.path();
        run_pijul(&["channel", "new", "tributary-48198"], repo_dir)?;
        run_pijul(&["channel", "switch", "tributary-48198"], repo_dir)?;
        expect_pijul_with_config(
            repo_dir,
            Some(toml::toml! {
                [pijul_channel]
                style = "underline blue"
                symbol = "P "
                truncation_length = 14
                truncation_symbol = "%"
                disabled = false
            }),
            &[
                Expect::ChannelName("tributary-4819"),
                Expect::Style(Color::Blue.underline()),
                Expect::Symbol("P"),
                Expect::TruncationSymbol("%"),
            ],
        );
        tempdir.close()
    }

    fn expect_pijul_with_config(
        repo_dir: &Path,
        config: Option<toml::Value>,
        expectations: &[Expect],
    ) {
        let actual = ModuleRenderer::new("pijul_channel")
            .path(repo_dir.to_str().unwrap())
            .config(config.unwrap_or_else(|| {
                toml::toml! {
                    [pijul_channel]
                    disabled = false
                }
            }))
            .collect();

        let mut expect_channel_name = "main";
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
                Expect::ChannelName(channel_name) => {
                    expect_channel_name = channel_name;
                }
                Expect::Style(style) => expect_style = *style,
            }
        }

        let expected = Some(format!(
            "on {} ",
            expect_style.paint(format!(
                "{expect_symbol} {expect_channel_name}{expect_truncation_symbol}"
            )),
        ));
        assert_eq!(expected, actual);
    }

    fn run_pijul(args: &[&str], _repo_dir: &Path) -> io::Result<()> {
        crate::utils::mock_cmd("pijul", args).ok_or(io::ErrorKind::Unsupported)?;
        Ok(())
    }
}
