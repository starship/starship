use super::{Context, Module};

use crate::config::{parse_style_string, ModuleConfig};
use crate::configs::fill::FillConfig;
use crate::segment::Segment;

/// Creates a module that fills the any extra space on the line.
///
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("fill");
    let config: FillConfig = FillConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let style = parse_style_string(config.style);

    module.set_segments(vec![Segment::fill(style, config.symbol)]);

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn basic() {
        let actual = ModuleRenderer::new("fill")
            .config(toml::toml! {
                [fill]
                style = "bold green"
                symbol = "*-"
            })
            .collect();
        let expected = Some(format!("{}", Color::Green.bold().paint("*-")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn module_disabled() {
        let actual = ModuleRenderer::new("fill")
            .config(toml::toml! {
                [fill]
                disabled = true
            })
            .collect();
        let expected = Option::<String>::None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn module_enabled() {
        let actual = ModuleRenderer::new("fill")
            .config(toml::toml! {
                [fill]
                disabled = false
            })
            .collect();
        let expected = Some(format!("{}", Color::Black.bold().paint(".")));

        assert_eq!(expected, actual);
    }
}
