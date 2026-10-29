use super::{Context, Module};

use crate::config::ModuleConfig;
use crate::configs::blank_fill::BlankFillConfig;
use crate::segment::Segment;

/// Creates a module that fills space with a single blank.
///
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("blank_fill");
    let config: BlankFillConfig = BlankFillConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    // no style, no configurable symbol — just one space
    module.set_segments(vec![Segment::fill(None, " ")]);

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;

    #[test]
    fn basic() {
        let actual = ModuleRenderer::new("blank_fill").collect();
        let expected = Some(" ".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn module_disabled() {
        let actual = ModuleRenderer::new("blank_fill")
            .config(toml::toml! {
                [blank_fill]
                disabled = true
            })
            .collect();
        let expected = Option::<String>::None;
        assert_eq!(expected, actual);
    }
}
