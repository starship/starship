use regex::Regex;

use super::{Context, Module, ModuleConfig};

use crate::configs::fossil_metrics::FossilMetricsConfig;
use crate::formatter::StringFormatter;

/// Creates a module with currently added/deleted lines in the Fossil check-out in the current
/// directory.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("fossil_metrics");
    let config = FossilMetricsConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    let checkout_db = if cfg!(windows) {
        "_FOSSIL_"
    } else {
        ".fslckout"
    };
    // See if we're in a check-out by scanning upwards for a directory containing the checkout_db file
    context
        .begin_ancestor_scan()
        .set_files(&[checkout_db])
        .scan()?;

    // Read the total number of added and deleted lines from "fossil diff --numstat"
    let output = context.exec_cmd("fossil", &["diff", "--numstat"])?.stdout;
    let stats = FossilDiff::parse(&output, config.only_nonzero_diffs);

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
    /// Parses the output of `fossil diff --numstat` as a `FossilDiff` struct.
    pub fn parse(diff_numstat: &'a str, only_nonzero_diffs: bool) -> Self {
        // Fossil formats the last line of the output as "%10d %10d TOTAL over %d changed files\n"
        // where the 1st and 2nd placeholders are the number of added and deleted lines respectively
        let re = Regex::new(r"^\s*(\d+)\s+(\d+) TOTAL over \d+ changed files$").unwrap();

        let (added, deleted) = diff_numstat
            .lines()
            .last()
            .and_then(|s| re.captures(s))
            .and_then(|caps| {
                let added = match caps.get(1)?.as_str() {
                    "0" if only_nonzero_diffs => "",
                    s => s,
                };

                let deleted = match caps.get(2)?.as_str() {
                    "0" if only_nonzero_diffs => "",
                    s => s,
                };

                Some((added, deleted))
            })
            .unwrap_or_default();

        Self { added, deleted }
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
    fn test_fossil_metrics_subdir() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Fossil)?;
        let checkout_dir = tempdir.path();
        expect_fossil_metrics_with_config(
            &checkout_dir.join("subdir"),
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
        let actual = FossilDiff::parse("         0          0 TOTAL over 0 changed files\n", true);
        let expected = FossilDiff {
            added: "",
            deleted: "",
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_no_changes_keep_zeros() {
        let actual = FossilDiff::parse("         0          0 TOTAL over 0 changed files\n", false);
        let expected = FossilDiff {
            added: "0",
            deleted: "0",
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_with_changes() {
        let actual = FossilDiff::parse(
            "         3          2 README.md\n         3          2 TOTAL over 1 changed files\n",
            true,
        );
        let expected = FossilDiff {
            added: "3",
            deleted: "2",
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_ignore_empty() {
        let actual = FossilDiff::parse("", true);
        let expected = FossilDiff {
            added: "",
            deleted: "",
        };
        assert_eq!(expected, actual);
    }

    /// Tests output as produced by Fossil v2.3 to v2.14, i.e. without the summary line.
    #[test]
    fn parse_ignore_when_missing_total_line() {
        let actual = FossilDiff::parse("         3          2 README.md\n", true);
        let expected = FossilDiff {
            added: "",
            deleted: "",
        };
        assert_eq!(expected, actual);
    }

    fn expect_fossil_metrics_with_config(
        checkout_dir: &Path,
        config: Option<toml::Table>,
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
                .unwrap_or_default(),
            expect_deleted
                .map(|deleted| format!("{} ", expect_deleted_style.paint(format!("-{deleted}"))))
                .unwrap_or_default(),
        ));
        assert_eq!(expected, actual);
    }
}
