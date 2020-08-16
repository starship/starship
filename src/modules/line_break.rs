use super::{Context, Module};
use crate::segment::Segment;

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const LINE_ENDING: &str = "\n";

    let show_newline = context.config.get_root_config().add_newline;
    if show_newline == false {
        return None;
    }

    let mut module = context.new_module("line_break");

    module.set_segments(vec![Segment {
        _name: "line_break".to_string(),
        style: None,
        value: LINE_ENDING.to_string(),
    }]);

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;

    #[test]
    fn add_newline_by_default() {
        let expected = Some(String::from("\n"));
        let actual = ModuleRenderer::new("line_break").collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn dont_add_newline_when_disabled() {
        let expected = None;
        let config = toml::toml! {
            add_newline = false
        };
        let actual = ModuleRenderer::new("line_break").config(config).collect();
        assert_eq!(expected, actual);
    }
}
