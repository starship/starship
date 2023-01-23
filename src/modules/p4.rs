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

    let user_cmd = context.exec_cmd("p4", &["info"]);
    let info_map = match &user_cmd {
        Some(output) => parse_key_values(&output.stdout),
        None => return None
    };

    let user = info_map.get("User name").unwrap_or(&"");
    let client = info_map.get("Client name").unwrap_or(&"unknown");

    if user.is_empty() {
        return None;
    }

    let changelist_cmd = context.exec_cmd("p4", &["changes", "-m1", "#have"]);
    let changelist = match &changelist_cmd {
        Some(output) => output.stdout.split(" ").nth(1).unwrap_or("?"),
        None => ""
    };

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

fn parse_key_values(output: &str) -> HashMap<&str, &str> {
    let mut info_map = HashMap::new();
    let pairs = output
        .split("\n")
        .map(|e| { e.split(":") });
            
    for mut pair in pairs {
        let k = pair.nth(0).unwrap_or_default().trim();
        let v = pair.nth(0).unwrap_or_default().trim();

        if !k.is_empty() {
            info_map.insert(k, v);
        }
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
