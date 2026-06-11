use super::{Context, Module, ModuleConfig};
use crate::segment::Segment;

use crate::configs::line_break::LineBreakConfig;

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("line_break");
    let config: LineBreakConfig = LineBreakConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    module.set_segments(vec![Segment::LineTerm {
        max_width: config.max_width,
    }]);

    Some(module)
}

#[cfg(test)]
mod test {
    use crate::test::ModuleRenderer;

    #[test]
    fn produces_result() {
        let expected = Some(String::from("\n"));
        let actual = ModuleRenderer::new("line_break").collect();
        assert_eq!(expected, actual);
    }
}
