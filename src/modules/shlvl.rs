use super::{Context, Module};

use crate::config::ModuleConfig;
use crate::configs::shlvl::ShLvlConfig;
use crate::formatter::StringFormatter;
use std::borrow::Cow;
use std::convert::TryInto;

const SHLVL_ENV_VAR: &str = "SHLVL";

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let shlvl = context.get_env(SHLVL_ENV_VAR)?.parse::<i64>().ok()?;

    let mut module = context.new_module("shlvl");
    let config: ShLvlConfig = ShLvlConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled || shlvl < config.threshold {
        return None;
    }

    let shlvl_str = &shlvl.to_string();

    let repeat_count = if config.repeat {
        shlvl.try_into().unwrap_or(1)
    } else {
        1
    };
    let symbol = if repeat_count != 1 {
        Cow::Owned(config.symbol.repeat(repeat_count))
    } else {
        Cow::Borrowed(config.symbol)
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(symbol.as_ref()),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "shlvl" => Some(Ok(shlvl_str)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `shlvl`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::{Color, Style};

    use crate::test::ModuleRenderer;

    use super::SHLVL_ENV_VAR;

    fn style() -> Style {
        // default style
        Color::Yellow.bold()
    }

    #[test]
    fn empty_config() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
            })
            .env(SHLVL_ENV_VAR, "2")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn enabled() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                disabled = false
            })
            .env(SHLVL_ENV_VAR, "2")
            .collect();
        let expected = Some(format!("{} ", style().paint("↕️  2")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn no_level() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                disabled = false
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn enabled_config_level_1() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                disabled = false
            })
            .env(SHLVL_ENV_VAR, "1")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn lower_threshold() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                threshold = 1
                disabled = false
            })
            .env(SHLVL_ENV_VAR, "1")
            .collect();
        let expected = Some(format!("{} ", style().paint("↕️  1")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn higher_threshold() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                threshold = 3
                disabled = false
            })
            .env(SHLVL_ENV_VAR, "1")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn custom_style() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                style = "Red Underline"
                disabled = false
            })
            .env(SHLVL_ENV_VAR, "2")
            .collect();
        let expected = Some(format!("{} ", Color::Red.underline().paint("↕️  2")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn custom_symbol() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                symbol = "shlvl is "
                disabled = false
            })
            .env(SHLVL_ENV_VAR, "2")
            .collect();
        let expected = Some(format!("{} ", style().paint("shlvl is 2")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn formatting() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                format = "$symbol going down [$shlvl]($style) GOING UP "
                disabled = false
            })
            .env(SHLVL_ENV_VAR, "2")
            .collect();
        let expected = Some(format!("↕️   going down {} GOING UP ", style().paint("2")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn repeat() {
        let actual = ModuleRenderer::new("shlvl")
            .config(toml::toml! {
                [shlvl]
                format = "[$symbol>]($style) "
                symbol = "~"
                repeat = true
                disabled = false
            })
            .env(SHLVL_ENV_VAR, "3")
            .collect();
        let expected = Some(format!("{} ", style().paint("~~~>")));

        assert_eq!(expected, actual);
    }
}
