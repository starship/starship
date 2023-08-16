use super::{Context, Module};

use crate::config::ModuleConfig;
use crate::configs::vcsh::VcshConfig;
use crate::formatter::StringFormatter;

/// Creates a module that displays VCSH repository currently in use
///
/// Will display the name of the current VCSH repository if one is active.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let repo = context.get_env("VCSH_REPO_NAME").unwrap_or_default();
    if repo.trim().is_empty() {
        return None;
    }

    let mut module = context.new_module("vcsh");
    let config: VcshConfig = VcshConfig::try_load(module.config);

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
            log::warn!("Error in module `vcsh`:\n{}", error);
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
    fn not_in_env() {
        let actual = ModuleRenderer::new("vcsh").collect();

        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn env_set() {
        let actual = ModuleRenderer::new("vcsh")
            .env("VCSH_REPO_NAME", "astronauts")
            .collect();

        let expected = Some(format!(
            "vcsh {} ",
            Color::Yellow.bold().paint("astronauts")
        ));

        assert_eq!(expected, actual);
    }
}
