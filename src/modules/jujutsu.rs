use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu::JujutsuConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu");
    let config = JujutsuConfig::try_load(module.config);
    let is_jj_repo = context
        .try_begin_scan()?
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_jj_repo {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map(|variable| match variable {
                "commit_info" => {
                    let output = &context.exec_cmd("jj", &["log", "-r@", "-n1", "--ignore-working-copy", "--no-graph", "--color","always", "-T", config.template])?.stdout;
                    Some(Ok(output.to_owned()))
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
