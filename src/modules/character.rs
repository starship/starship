use super::{Context, Module};
use ansi_term::Color;

/// Creates a segment for the prompt character
///
/// The char segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `COLOR_SUCCESS`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `COLOR_FAILURE` (red by default)
pub fn segment<'a>(context: &'a Context) -> Option<Module<'a>> {
    const PROMPT_CHAR: &str = "➜";
    let color_success = Color::Green.bold();
    let color_failure = Color::Red.bold();

    let mut module = context.new_module("char");
    module.get_prefix().set_value("");

    let symbol = module.new_segment("symbol", PROMPT_CHAR);

    let arguments = &context.arguments;
    if arguments.value_of("status_code").unwrap_or("0") == "0" {
        symbol.set_style(color_success.bold());
    } else {
        symbol.set_style(color_failure.bold());
    };

    Some(module)
}
