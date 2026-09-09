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
    // need to bring in "Module" and "Segment" to populate prompt with test modules
    use crate::{module::Module, segment::Segment, test::ModuleRenderer};
    use nu_ansi_term::AnsiStrings;

    #[test]
    fn produces_result() {
        let expected = Some(String::from("\n"));
        let actual = ModuleRenderer::new("line_break").collect();
        assert_eq!(expected, actual);
    }

    // ModuleRenderer tests only one module in isolation. the functionality of max_width depends
    // upon other modules contributing to "used", hence why I am using Module/Segment over
    // ModuleRenderer.
    #[test]
    fn skip_break_under_max_width() {
        let expected = None; // line break skipped (no output)
        let actual = ModuleRenderer::new("line_break")
            .config(toml::toml! {
                [line_break]
                max_width = 200
            })
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn skip_break_at_max_width() {
        let expected = "test";
        let mut module = Module::new("test-module", expected, None);
        let mut segments = Segment::from_text(None, expected); // test of width 4
        segments.push(Segment::LineTerm { max_width: 4 });
        module.set_segments(segments);
        let result = module.ansi_strings();
        let output = AnsiStrings(&result).to_string(); // convert to plain text
        // output should be "test" with no newline
        assert_eq!(output, expected);
    }

    #[test]
    fn insert_break_over_max_width() {
        let input = "test";
        let expected = "test\n";
        let mut module = Module::new("test-module", input, None);
        let mut segments = Segment::from_text(None, input); // test of width 4
        segments.push(Segment::LineTerm { max_width: 3 });
        module.set_segments(segments);
        let result = module.ansi_strings();
        let output = AnsiStrings(&result).to_string(); // convert to plain text
        // output should be "test" with newline
        assert_eq!(output, expected);
    }
}
