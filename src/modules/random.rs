use rand::seq::SliceRandom;

use crate::{
    config::ModuleConfig,
    configs::random::RandomConfig,
    context::Context,
    formatter::{string_formatter::StringFormatterError, StringFormatter},
    module::Module,
};
const EMPTY_STYLES_MSG: &str = "`styles` is empty but a $style was encountered";

pub fn module<'ctx>(name: &str, ctx: &'ctx Context) -> Option<Module<'ctx>> {
    let cfg = ctx.config.get_random_module_config(name).or_else(|| {
        log::debug!("tried to load top level random config '{name}' but config doesn't exist");
        None
    })?;
    let mut module = Module::new(&format!("random.{name}"), "<random module>", Some(cfg));
    let cfg = RandomConfig::load(cfg);
    if cfg.disabled {
        log::debug!("Module `random.{name}` is disabled, skipping it");
        return None;
    }
    if cfg.symbols.is_empty() {
        log::warn!("No symbols found for random module `random.{name}`, skipping it");
        return None;
    }

    let mut rng = rand::thread_rng();
    let symbol = cfg.symbols.choose(&mut rng)?;
    let style = cfg.styles.choose(&mut rng);
    let parsed =
        StringFormatter::new(cfg.format).and_then(|f| {
            f.map_meta(|var, _| match var {
                "symbol" => Some(symbol),
                _ => None,
            })
            .map_style(|var| match var {
                "style" => Some(style.copied().map(Ok).unwrap_or_else(|| {
                    Err(StringFormatterError::Custom(EMPTY_STYLES_MSG.to_owned()))
                })),
                _ => None,
            })
            .parse(None, Some(ctx))
        });
    match parsed {
        Ok(segs) => module.set_segments(segs),
        Err(e) => {
            log::warn!("Error in module `random.{name}`:\n{e}");
        }
    }
    Some(module)
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Style;

    use crate::test::ModuleRenderer;

    #[test]
    fn disabled() {
        let result = ModuleRenderer::new("random.test")
            .config(toml::toml! {
                [random.test]
                symbols = [">"]
                disabled = true
            })
            .collect();
        assert_eq!(result, None);
    }
    #[test]
    fn empty_style_and_symbol() {
        let result = ModuleRenderer::new("random.test")
            .config(toml::toml! {
                format = "${random.test}"
                [random.test]
                styles = []
                symbols = []
                format = "[$symbol]($style)"
            })
            .collect();
        assert_eq!(result, None);
    }

    #[test]
    fn symbol_with_no_style() {
        let result = ModuleRenderer::new("random.test")
            .config(toml::toml! {
                format = "${random.test}"
                [random.test]
                styles = []
                symbols = [">"]
                format = "$symbol"
            })
            .collect();
        assert_eq!(result, Some(">".to_owned()));
    }

    #[test]
    fn symbol_with_no_style_but_tries_to_use_style() {
        let result = ModuleRenderer::new("random.test")
            .config(toml::toml! {
                format = "${random.test}"
                [random.test]
                styles = []
                symbols = [">"]
                format = "[$symbol]($style)"
            })
            .collect();
        assert_eq!(result, None);
    }
    #[test]
    fn style_with_no_symbol() {
        let result = ModuleRenderer::new("random.test")
            .config(toml::toml! {
                format = "${random.test}"
                [random.test]
                symbols = []
                styles = ["fg:blue"]
                format = "[$symbol]($style)"
            })
            .collect();
        assert_eq!(result, None);
    }
}
