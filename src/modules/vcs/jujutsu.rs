use std::path::Path;

use crate::configs::vcs::jujutsu::{JjChangeConfig, JjConfig, JjStatusConfig};
use crate::formatter::string_formatter::StringFormatterError;

use super::{format_count, Context, Segment, StringFormatter};

const ALL_STATUS_FORMAT: &str = "$deleted$renamed$modified$added";

#[derive(Default)]
struct Status {
    added: usize,
    deleted: usize,
    modified: usize,
    renamed: usize,
}

pub(super) fn segments<'a>(
    context: &Context<'a>,
    root: &Path,
    config: &JjConfig<'a>,
) -> Option<Result<Vec<Segment>, StringFormatterError>> {
    // Prints something of the form:
    //
    // M src/configs/mod.rs
    // A src/configs/vcs/jujutsu.rs
    // vnyuwku
    let template = format!(
        "self.diff().summary() ++ '@ ' ++ change_id.shortest({})",
        config.change.change_id_length
    );
    let out = context.exec_cmd(
        "jj",
        &[
            "--repository".as_ref(),
            root.as_os_str(),
            "log".as_ref(),
            "--ignore-working-copy".as_ref(),
            "--no-graph".as_ref(),
            "--color".as_ref(),
            "never".as_ref(),
            "--revisions".as_ref(),
            "@".as_ref(), // Only display the current revision
            "--template".as_ref(),
            template.as_ref(),
        ],
    )?;

    let mut status = Status::default();
    let mut change_id = None;

    for line in out.stdout.lines() {
        if line.is_empty() {
            continue;
        }

        let (indic, rest) = line.split_once(' ')?;
        match indic {
            "A" => status.added += 1,
            "D" => status.deleted += 1,
            "M" => status.modified += 1,
            "R" => status.renamed += 1,
            "@" => change_id = Some(rest),
            _ => (),
        }
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_variables_to_segments(|variable| match variable {
                "change" => change_segments(context, &config.change, change_id),
                "status" => status_segments(context, &config.status, &status),
                _ => None,
            })
            .parse(None, Some(context))
    });

    Some(parsed)
}

fn change_segments(
    context: &Context,
    config: &JjChangeConfig,
    change_id: Option<&str>,
) -> Option<Result<Vec<Segment>, StringFormatterError>> {
    if config.disabled {
        return None;
    }
    let change_id = change_id?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable: &str| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| (variable == "change_id").then_some(Ok(change_id)))
            .parse(None, Some(context))
    });
    Some(parsed)
}

fn status_segments(
    context: &Context,
    config: &JjStatusConfig,
    status: &Status,
) -> Option<Result<Vec<Segment>, StringFormatterError>> {
    if config.disabled {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "all_status" => Some(ALL_STATUS_FORMAT),
                _ => None,
            })
            .map_style(|variable: &str| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map_variables_to_segments(|variable| {
                let segments = match variable {
                    "added" => {
                        format_count(config.added, "jujutsu.status.added", context, status.added)
                    }
                    "deleted" => format_count(
                        config.deleted,
                        "jujutsu.status.deleted",
                        context,
                        status.deleted,
                    ),
                    "modified" => format_count(
                        config.modified,
                        "jujutsu.status.modified",
                        context,
                        status.modified,
                    ),
                    "renamed" => format_count(
                        config.renamed,
                        "jujutsu.status.renamed",
                        context,
                        status.renamed,
                    ),
                    _ => None,
                };
                segments.map(Ok)
            })
            .parse(None, Some(context))
    });
    Some(parsed)
}
