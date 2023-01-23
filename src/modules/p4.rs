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

    if config.disabled || !is_p4_logged(context) {
        return None;
    }

    let info = get_p4_info(context)?;
 
    if !context.current_dir.starts_with(info.client_root) {
        return None;
    }

    let changelist = get_p4_last_changelist_number(context)?;

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
                "changelist" => Some(Ok(&changelist)),
                "user" => Some(Ok(&info.user_name)),
                "client" => Some(Ok(&info.client_name)),
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

struct P4Info {
    user_name: String,
    client_name: String,
    client_root: String
}

fn is_p4_logged(context: &Context) -> bool {
    match context.exec_cmd("p4", &["login", "-s"]) {
        Some(_) => true,
        None => false
    }
}

fn get_p4_info(context: &Context) -> Option<P4Info> {
    let info_output = context.exec_cmd("p4", &["info"])?.stdout;
    let info_map = parse_p4_info_output(&info_output);

    Some(P4Info {
        user_name: info_map.get("User name")?.to_string(),
        client_name: info_map.get("Client name")?.to_string(),
        client_root: info_map.get("Client root")?.to_string()
    })
}

fn get_p4_last_changelist_number<'a>(context: &Context) -> Option<String> {
    context.exec_cmd("p4", &["changes", "-m1", "#have"])?.stdout.split(" ").nth(1).and_then(|s| Some(s.to_string()))
}

fn parse_p4_info_output(output: &str) -> HashMap<&str, &str> {
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
