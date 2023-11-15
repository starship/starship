use super::{Context, Module, ModuleConfig};

use crate::configs::vim_shell::VimShellConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vim_shell");
    let _ = context.get_env("VIMRUNTIME")?;

    let config = VimShellConfig::try_load(module.config);
    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `vim_shell`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;

    #[test]
    fn no_env_variables() {
        let actual = ModuleRenderer::new("vim_shell").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn has_env_variables() {
        let actual = ModuleRenderer::new("vim_shell")
            .env("VIMRUNTIME", "something")
            .collect();

        assert!(actual.is_some());
    }
}
