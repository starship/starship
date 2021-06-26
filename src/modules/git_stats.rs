use regex::Regex;

use crate::{
    config::RootModuleConfig, configs::git_stats::GitStatsConfig, formatter::StringFormatter,
    module::Module,
};

use super::Context;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_stats");
    let config: GitStatsConfig = GitStatsConfig::try_load(module.config);

    let diff = context
        .exec_cmd("git", &["diff", "--word-diff", "--unified=0"])?
        .stdout;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "added_style" => Some(Ok(config.added_style)),
                "modified_style" => Some(Ok(config.modified_style)),
                "deleted_style" => Some(Ok(config.deleted_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "added_lines" => Some(Ok(get_added_lines(&diff)?)),
                "modified_lines" => Some(Ok(get_modified_lines(&diff)?)),
                "deleted_lines" => Some(Ok(get_deleted_lines(&diff)?)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_stats`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_added_lines<'a>(diff: &'a str) -> Option<String> {
    let re = Regex::new(r"^\{\+.*\+\}$").unwrap();

    Some(
        diff.lines()
            .filter(|line| re.is_match(line))
            .count()
            .to_string(),
    )
}
fn get_modified_lines<'a>(diff: &'a str) -> Option<String> {
    let re = Regex::new(r"^.+(\[-|\{\+).*$").unwrap();

    Some(
        diff.lines()
            .filter(|line| re.is_match(line))
            .count()
            .to_string(),
    )
}
fn get_deleted_lines<'a>(diff: &'a str) -> Option<String> {
    let re = Regex::new(r"^\[-.*-\]$").unwrap();

    Some(
        diff.lines()
            .filter(|line| re.is_match(line))
            .count()
            .to_string(),
    )
}
