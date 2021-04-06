use super::{Context, Module, RootModuleConfig};

use crate::configs::toolbox::ToolboxConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current toolbox image
///
/// Will display the toolbox image if `$toolbox_NAME` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let toolbox_env = context.get_env("TOOLBOX_PATH")?;
    let toolbox_name = context.get_env("NAME")?;

    let mut module = context.new_module("toolbox");
    let config: ToolboxConfig = ToolboxConfig::try_load(module.config);

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
            .map(|variable| match variable {
                "env" => Some(Ok(&toolbox_name)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `toolbox`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;

    #[test]
    fn no_env_set() {
        let actual = ModuleRenderer::new("toolbox").collect();

        let expected = None;
        assert_eq!(expected, actual);
    }
    #[test]
    fn env_set() {
        let actual = ModuleRenderer::new("toolbox")
            .env("NAME", "centos.img")
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Blue.bold().dimmed().paint("[centos.img]")
        ));

        assert_eq!(expected, actual);
    }
}
