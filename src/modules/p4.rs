use std::collections::HashMap;

use super::{Context, Module};

use crate::config::ModuleConfig;
use crate::configs::p4::P4Config;
use crate::formatter::StringFormatter;

/// Creates a module that displays Perforce repository currently in use
///
/// Will display the name of the current Perforce repository if one is active.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("p4");
    let config: P4Config = P4Config::try_load(module.config);

    if config.disabled {
        return None;
    }

    context.exec_cmd("p4", &["login", "-s"])?;

    let info_output = context.exec_cmd("p4", &["info"])?.stdout;
    let info_map = parse_p4_info(&info_output);

    let root_folder = info_map.get("Client root").unwrap_or(&&"");
    let current_dir = &context.current_dir;
 
    if !current_dir.starts_with(root_folder) {
        return None;
    }

    let user = info_map.get("User name").unwrap_or(&&"unknown");
    let client = info_map.get("Client name").unwrap_or(&&"unknown");

    let changelist_output = context.exec_cmd("p4", &["changes", "-m1", "#have"])?.stdout;
    let changelist = changelist_output.split(" ").nth(1).unwrap_or("?");

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "changelist" => Some(Ok(changelist)),
                "user" => Some(Ok(user)),
                "client" => Some(Ok(client)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `p4`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_p4_info(output: &str) -> HashMap<&str, &str> {
    let mut info_map = HashMap::new();

    for line in output.lines() {
        match line.split_once(":") {
            Some((k, v)) => {
                info_map.insert(k, v.trim());
            }
            None => {}
        };
    }

    info_map
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn empty_test() {
        assert_eq!("expected", "expected");
    }
}
