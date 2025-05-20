use super::{Context, Module};
use crate::{config::ModuleConfig, configs::line_break::LineBreakConfig, segment::Segment};

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("line_break");
    let config = LineBreakConfig::try_load(module.config);

    let should_break = match config.break_below_width {
        Some(break_below_width) => context.width < break_below_width,
        None => true,
    };
    if should_break {
        module.set_segments(vec![Segment::LineTerm]);
    }

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

    fn check_break_below_width_behaviour(
        break_below_width: usize,
        term_width: usize,
        expected: Option<String>,
    ) {
        let config = toml::toml! {
            [line_break]
            break_below_width = break_below_width
        };
        let renderer = ModuleRenderer::new("line_break")
            .config(config)
            .width(term_width);
        let actual = renderer.collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn break_below_width_wide_terminal() {
        check_break_below_width_behaviour(80, 80, None);
    }

    #[test]
    fn break_below_width_narrow_terminal() {
        check_break_below_width_behaviour(80, 79, Some(String::from("\n")));
    }

    #[test]
    fn break_below_width_undetected_width() {
        check_break_below_width_behaviour(80, 0, Some(String::from("\n")));
    }
}
