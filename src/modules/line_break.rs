use super::{Context, Module};
use crate::segment::Segment;

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("line_break");

    module.set_segments(vec![Segment::LineTerm]);

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
