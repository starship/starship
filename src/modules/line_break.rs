use super::{Context, Module};

/// Creates a segment for the line break
pub fn segment(_context: &Context) -> Option<Module> {
    const LINE_ENDING: &str = "\n";

    let mut module = Module::new("line_break");
    
    let mut prefix = module.get_prefix();
    prefix.set_value("");

    let mut suffix = module.get_suffix();
    suffix.set_value("");

    let mut symbol = module.new_segment("character");
    symbol.set_value(LINE_ENDING);
    
    Some(module)
}
