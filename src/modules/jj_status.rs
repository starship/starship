use super::{Context, Module, ModuleConfig};
use crate::configs::jj_status::JjStatusConfig;
use crate::formatter::StringFormatter;
use crate::segment::Segment;
use crate::utils::create_command;
use std::path::Path;

const ALL_STATUS_FORMAT: &str = "$conflicted$deleted$renamed$modified$staged$untracked";

/// Creates a module with the Jujutsu repository status
///
/// Will display the status if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_status");
    let config: JjStatusConfig = JjStatusConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let repo_root = context.begin_ancestor_scan().set_folders(&[".jj"]).scan()?;

    let status_info = get_jj_status(&repo_root)?;

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
            .map_variables_to_segments(|variable: &str| {
                let segments = match variable {
                    "ahead_behind" => status_info.get_ahead_behind().and_then(|(ahead, behind)| {
                        if ahead > 0 && behind > 0 {
                            format_text(
                                config.diverged,
                                "jj_status.diverged",
                                context,
                                |variable| match variable {
                                    "ahead_count" => Some(ahead.to_string()),
                                    "behind_count" => Some(behind.to_string()),
                                    _ => None,
                                },
                            )
                        } else if ahead > 0 && behind == 0 {
                            format_count(config.ahead, "jj_status.ahead", context, ahead)
                        } else if behind > 0 && ahead == 0 {
                            format_count(config.behind, "jj_status.behind", context, behind)
                        } else {
                            format_symbol(config.up_to_date, "jj_status.up_to_date", context)
                        }
                    }),
                    "conflicted" => status_info.get_conflicted().and_then(|count| {
                        format_count(config.conflicted, "jj_status.conflicted", context, count)
                    }),
                    "deleted" => status_info.get_deleted().and_then(|count| {
                        format_count(config.deleted, "jj_status.deleted", context, count)
                    }),
                    "renamed" => status_info.get_renamed().and_then(|count| {
                        format_count(config.renamed, "jj_status.renamed", context, count)
                    }),
                    "modified" => status_info.get_modified().and_then(|count| {
                        format_count(config.modified, "jj_status.modified", context, count)
                    }),
                    "staged" => status_info.get_staged().and_then(|count| {
                        format_count(config.staged, "jj_status.staged", context, count)
                    }),
                    "untracked" => status_info.get_untracked().and_then(|count| {
                        format_count(config.untracked, "jj_status.untracked", context, count)
                    }),
                    _ => None,
                };
                segments.map(Ok)
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => {
            if segments.is_empty() {
                return None;
            }
            segments
        }
        Err(error) => {
            log::warn!("Error in module `jj_status`:\n{error}");
            return None;
        }
    });

    Some(module)
}

#[derive(Default, Debug)]
struct JjStatusInfo {
    ahead: usize,
    behind: usize,
    conflicted: usize,
    deleted: usize,
    renamed: usize,
    modified: usize,
    staged: usize,
    untracked: usize,
}

impl JjStatusInfo {
    fn get_ahead_behind(&self) -> Option<(usize, usize)> {
        if self.ahead > 0 || self.behind > 0 {
            Some((self.ahead, self.behind))
        } else {
            None
        }
    }

    fn get_conflicted(&self) -> Option<usize> {
        if self.conflicted > 0 {
            Some(self.conflicted)
        } else {
            None
        }
    }

    fn get_deleted(&self) -> Option<usize> {
        if self.deleted > 0 {
            Some(self.deleted)
        } else {
            None
        }
    }

    fn get_renamed(&self) -> Option<usize> {
        if self.renamed > 0 {
            Some(self.renamed)
        } else {
            None
        }
    }

    fn get_modified(&self) -> Option<usize> {
        if self.modified > 0 {
            Some(self.modified)
        } else {
            None
        }
    }

    fn get_staged(&self) -> Option<usize> {
        if self.staged > 0 {
            Some(self.staged)
        } else {
            None
        }
    }

    fn get_untracked(&self) -> Option<usize> {
        if self.untracked > 0 {
            Some(self.untracked)
        } else {
            None
        }
    }
}

fn get_jj_status(repo_root: &Path) -> Option<JjStatusInfo> {
    let mut command = create_command("jj").ok()?;
    command.args(["status", "--no-pager"]).current_dir(repo_root);

    let output = command.output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    parse_jj_status(&stdout)
}

fn parse_jj_status(output: &str) -> Option<JjStatusInfo> {
    let mut status = JjStatusInfo::default();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse ahead/behind from branch info
        if line.contains("ahead") || line.contains("behind") {
            // JJ status shows something like: (ahead by 2, behind by 1)
            // But actually, jj status doesn't show ahead/behind by default
            // We might need to run jj log or something, but for now, skip
            continue;
        }

        // Parse file status
        if let Some(status_char) = line.chars().next() {
            match status_char {
                'A' => status.staged += 1,
                'M' => status.modified += 1,
                'R' => status.renamed += 1,
                'D' => status.deleted += 1,
                'U' => status.conflicted += 1,
                '?' => status.untracked += 1,
                _ => {}
            }
        }
    }

    Some(status)
}

fn format_text<F>(
    format_str: &str,
    config_path: &str,
    context: &Context,
    mapper: F,
) -> Option<Vec<Segment>>
where
    F: Fn(&str) -> Option<String> + Send + Sync,
{
    if let Ok(formatter) = StringFormatter::new(format_str) {
        formatter
            .map(|variable| mapper(variable).map(Ok))
            .parse(None, Some(context))
            .ok()
    } else {
        log::warn!("Error parsing format string `{}`", &config_path);
        None
    }
}

fn format_count(
    format_str: &str,
    config_path: &str,
    context: &Context,
    count: usize,
) -> Option<Vec<Segment>> {
    if count == 0 {
        return None;
    }

    format_text(
        format_str,
        config_path,
        context,
        |variable| match variable {
            "count" => Some(count.to_string()),
            _ => None,
        },
    )
}

fn format_symbol(format_str: &str, config_path: &str, context: &Context) -> Option<Vec<Segment>> {
    format_text(format_str, config_path, context, |_variable| None)
}