use super::{Context, Module};
use ansi_term::Color;

/// Creates a module for the prompt character
///
/// The character segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `COLOR_SUCCESS`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `COLOR_FAILURE` (red by default)
#[allow(clippy::collapsible_if)]
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const PROMPT_CHAR: &str = "➜";
    const FAIL_CHAR: &str = "✖";
    let color_success = Color::Green.bold();
    let color_failure = Color::Red.bold();

    let mut module = context.new_module("character")?;
    module.get_prefix().set_value("");

    /* If an error symbol is set in the config, use symbols to indicate
    success/failure. Otherwise, use colors  to indicate success/failure. */
    let use_color = module.config_value("error_symbol").is_none();
    let arguments = &context.arguments;

    if use_color {
        let symbol = module.new_segment("symbol", PROMPT_CHAR);
        if arguments.value_of("status_code").unwrap_or("0") == "0" {
            symbol.set_style(color_success.bold());
        } else {
            symbol.set_style(color_failure.bold());
        };
    } else {
        if arguments.value_of("status_code").unwrap_or("0") == "0" {
            module.new_segment("symbol", PROMPT_CHAR);
        } else {
            module.new_segment("error_symbol", FAIL_CHAR);
        };
    }

    Some(module)
}
