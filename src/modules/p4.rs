use super::{Context, Module};

use crate::config::ModuleConfig;
use crate::configs::p4::P4Config;
use crate::formatter::StringFormatter;

/// Creates a module that displays Perforce repository currently in use
///
/// Will display the name of the current Perforce repository if one is active.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let repo = context.get_env("VCSH_REPO_NAME").unwrap_or_default();
    if repo.trim().is_empty() {
        return None;
    }

    let mut module = context.new_module("p4");
    let config: P4Config = P4Config::try_load(module.config);

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
                "repo" => Some(Ok(&repo)),
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

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn empty_test() {
        assert_eq!("expected", "expected");
    }
}
