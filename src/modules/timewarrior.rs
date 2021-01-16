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

    let format = String::from("[$symbol]($symbol_style)");

    let parsed = StringFormatter::new(&format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "symbol_style" => Some(Ok(config.symbol_style)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `timewarrior`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn timewarrior_installed() -> bool {
    match std::env::var("TIMEWARRIORDB") {
        Ok(val)  => Path::new(&val).exists(),
        Err(_) => {
            let home_dir = dirs_next::home_dir().unwrap();
            Path::new(&home_dir).join(".timewarrior").exists()
        }
    }
}

fn is_timewarrior_active() -> bool {
    match utils::exec_cmd("timew", &["get", "dom.active"]) {
        Some(result) => return result.stdout.starts_with("1"),
        None => false,
    }
}
