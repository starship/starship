use super::{Context, Module, ModuleConfig};

use crate::configs::fossil_metrics::FossilMetricsConfig;
use crate::formatter::StringFormatter;

/// Creates a module with currently added/deleted lines in the Fossil check-out in the current
/// directory.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_checkout = context
        .try_begin_scan()?
        .set_files(&[".fslckout"])
        .is_match();

    if !is_checkout {
        return None;
    }

    let mut module = context.new_module("fossil_metrics");
    let config = FossilMetricsConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    // Read the total number of added and deleted lines from "fossil diff --numstat"
    let output = context.exec_cmd("fossil", &["diff", "--numstat"])?.stdout;
    let stats = FossilDiff::try_parse(&output, config.only_nonzero_diffs)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "added_style" => Some(Ok(config.added_style)),
                "deleted_style" => Some(Ok(config.deleted_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "added" => Some(Ok(stats.added)),
                "deleted" => Some(Ok(stats.deleted)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `fossil_metrics`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

/// Represents the parsed output from a Fossil diff with the --numstat option enabled.
#[derive(Debug, PartialEq)]
struct FossilDiff<'a> {
    added: &'a str,
    deleted: &'a str,
}

impl<'a> FossilDiff<'a> {
    fn try_parse(diff_numstat: &'a str, only_nonzero_diffs: bool) -> Option<Self> {
        let mut split = match diff_numstat.lines().last() {
            Some(line) => line.split_whitespace(),
            None => {
                log::warn!("Error in module `fossil_metrics`:\nLast line of numstat diff missing.");
                return None;
            }
        };

        let added = match split.next() {
            Some("0") if only_nonzero_diffs => "",
            Some(s) => s,
            None => {
                log::warn!("Error in module `fossil_metrics`:\nNumber of added lines missing.");
                return None;
            }
        };

        let deleted = match split.next() {
            Some("0") if only_nonzero_diffs => "",
            Some(s) => s,
            None => {
                log::warn!("Error in module `fossil_metrics`:\nNumber of deleted lines missing.");
                return None;
            }
        };

        Some(Self { added, deleted })
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;

    use nu_ansi_term::{Color, Style};

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};

    use super::FossilDiff;

    enum Expect<'a> {
        Empty,
        Added(Option<&'a str>),
        AddedStyle(Style),
        Deleted(Option<&'a str>),
        DeletedStyle(Style),
    }

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let checkout_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("fossil_metrics")
            .path(checkout_dir.path())
            .collect();
        let expected = None;
        assert_eq!(expected, actual);

        checkout_dir.close()
    }

    #[test]
    fn test_fossil_metrics_disabled_per_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Fossil)?;
        let checkout_dir = tempdir.path();
        expect_fossil_metrics_with_config(
            checkout_dir,
            Some(toml::toml! {
                // no "disabled=false" in config!
                [fossil_metrics]
                only_nonzero_diffs = false
            }),
            &[Expect::Empty],
        );
        tempdir.close()
    }

    #[test]
    fn test_fossil_metrics_autodisabled() -> io::Result<()> {
        let tempdir = tempfile::tempdir()?;
        expect_fossil_metrics_with_config(tempdir.path(), None, &[Expect::Empty]);
        tempdir.close()
    }

    #[test]
    fn test_fossil_metrics() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Fossil)?;
        let checkout_dir = tempdir.path();
        expect_fossil_metrics_with_config(
            checkout_dir,
            None,
            &[Expect::Added(Some("3")), Expect::Deleted(Some("2"))],
        );
        tempdir.close()
    }

    #[test]
    fn test_fossil_metrics_configured() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Fossil)?;
        let checkout_dir = tempdir.path();
        expect_fossil_metrics_with_config(
            checkout_dir,
            Some(toml::toml! {
                [fossil_metrics]
                added_style = "underline blue"
                deleted_style = "underline purple"
                disabled = false
            }),
            &[
                Expect::Added(Some("3")),
                Expect::AddedStyle(Color::Blue.underline()),
                Expect::Deleted(Some("2")),
                Expect::DeletedStyle(Color::Purple.underline()),
            ],
        );
        tempdir.close()
    }

    #[test]
    fn parse_no_changes_discard_zeros() {
        let actual =
            FossilDiff::try_parse("         0          0 TOTAL over 0 changed files\n", true)
                .unwrap();
        let expected = FossilDiff {
            added: "",
            deleted: "",
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_no_changes_keep_zeros() {
        let actual =
            FossilDiff::try_parse("         0          0 TOTAL over 0 changed files\n", false)
                .unwrap();
        let expected = FossilDiff {
            added: "0",
            deleted: "0",
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_changes() {
        let actual = FossilDiff::try_parse(
            "         3          2 README.md\n         3          2 TOTAL over 1 changed files\n",
            true,
        )
        .unwrap();
        let expected = FossilDiff {
            added: "3",
            deleted: "2",
        };
        assert_eq!(expected, actual);
    }

    fn expect_fossil_metrics_with_config(
        checkout_dir: &Path,
        config: Option<toml::Value>,
        expectations: &[Expect],
    ) {
        let actual = ModuleRenderer::new("fossil_metrics")
            .path(checkout_dir.to_str().unwrap())
            .config(config.unwrap_or_else(|| {
                toml::toml! {
                    [fossil_metrics]
                    disabled = false
                }
            }))
            .collect();

        let mut expect_added = Some("3");
        let mut expect_added_style = Color::Green.bold();
        let mut expect_deleted = Some("2");
        let mut expect_deleted_style = Color::Red.bold();

        for expect in expectations {
            match expect {
                Expect::Empty => {
                    assert_eq!(None, actual);
                    return;
                }
                Expect::Added(added) => expect_added = *added,
                Expect::AddedStyle(style) => expect_added_style = *style,
                Expect::Deleted(deleted) => expect_deleted = *deleted,
                Expect::DeletedStyle(style) => expect_deleted_style = *style,
            }
        }

        let expected = Some(format!(
            "{}{}",
            expect_added
                .map(|added| format!("{} ", expect_added_style.paint(format!("+{added}"))))
                .unwrap_or(String::from("")),
            expect_deleted
                .map(|deleted| format!("{} ", expect_deleted_style.paint(format!("-{deleted}"))))
                .unwrap_or(String::from("")),
        ));
        assert_eq!(expected, actual);
    }
}
