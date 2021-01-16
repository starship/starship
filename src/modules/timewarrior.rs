use super::{Context, Module};
use crate::config::RootModuleConfig;
use crate::configs::timewarrior::TimewarriorConfig;
use crate::formatter::StringFormatter;
use crate::utils;
use std::path::Path;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    if !timewarrior_installed() {
        log::debug!("Timewarrior is not installed");
        return None;
    }

    let mut module = context.new_module("timewarrior");
    let config: TimewarriorConfig = TimewarriorConfig::try_load(module.config);
    if config.disabled || !is_timewarrior_active() {
        return None;
    }

    let mut format = String::from("[$symbol]($symbol_style) ");
    if config.show_tags {
        format.push_str("[$tags]($tag_style) ");
    }

    let parsed = StringFormatter::new(&format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "symbol_style" => Some(Ok(config.symbol_style)),
                "tags_style" => Some(Ok(config.tags_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "tags" => Some(Ok(timewarrior_tags(config.max_tag_count))),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("{}", error);
            return None;
        }
    });

    Some(module)
}

fn timewarrior_installed() -> bool {
    match std::env::var("TIMEWARRIORDB") {
        Ok(val) => Path::new(&val).exists(),
        Err(_) => {
            let home_dir = dirs_next::home_dir().unwrap();
            Path::new(&home_dir).join(".timewarrior").exists()
        }
    }
}

fn is_timewarrior_active() -> bool {
    match utils::exec_cmd("timew", &["get", "dom.active"]) {
        Some(result) => result.stdout.trim().starts_with('1'),
        None => false,
    }
}

fn timewarrior_tags(count: i64) -> String {
    match utils::exec_cmd("timew", &["get", "dom.active.tag.count"]) {
        Some(result) => {
            let stdout = match result.stdout.trim().parse::<i64>() {
                Ok(result) => result,
                Err(error) => {
                    log::warn!("{}", error);
                    1 // Every entry should have at least 1 tag
                }
            };
            get_tags(std::cmp::min(count, stdout))
        }
        None => String::new(),
    }
}

fn get_tags(count: i64) -> String {
    let mut tags: Vec<String> = Vec::new();
    for n in 1..=count {
        if n > 1 {
            tags.push(String::from(", "));
        }

        let tag = match utils::exec_cmd("timew", &["get", format!("dom.active.tag.{}", n).as_str()])
        {
            Some(result) => String::from(result.stdout.trim()),
            None => String::new(),
        };

        if !tag.is_empty() {
            tags.push(tag);
        }
    }

    tags.into_iter().collect()
}
