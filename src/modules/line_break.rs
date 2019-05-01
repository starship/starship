use super::{Context, Module};

/// Creates a segment for the line break
pub fn segment(_context: &Context) -> Option<Module> {
    const LINE_ENDING: &str = "\n";

    let mut module = Module::new("line_break");
    
    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    module.new_segment("character", LINE_ENDING);

    Some(module)
}
