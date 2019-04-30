use ansi_term::Color;
use super::{Context, Module};

/// Creates a segment for the prompt character
///
/// The char segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `COLOR_SUCCESS`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `COLOR_FAILURE` (red by default)
pub fn segment(context: &Context) -> Option<Module> {
    const PROMPT_CHAR: &str = "➜";
    const COLOR_SUCCESS: Color = Color::Green;
    const COLOR_FAILURE: Color = Color::Red;

    let arguments = &context.arguments;

    let mut module = Module::new("char");

    let prefix = module.get_prefix();
    prefix.set_value("");

    let symbol = module.new_segment("symbol");
    symbol.set_value(PROMPT_CHAR);

    if arguments.value_of("status_code").unwrap() == "0" {
        symbol.set_style(COLOR_SUCCESS);
    } else {
        symbol.set_style(COLOR_FAILURE);
    };

    Some(module)
}
