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
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const SUCCESS_CHAR: &str = "❯";
    const FAILURE_CHAR: &str = "✖";
    const VICMD_CHAR: &str = "❮";
    enum ShellEditMode {
        Normal,
        Insert,
    };
    const ASSUMED_MODE: ShellEditMode = ShellEditMode::Insert;
    // TODO: extend config to more modes

    let color_success = Color::Green.bold();
    let color_failure = Color::Red.bold();

    let mut module = context.new_module("character");
    module.get_prefix().set_value("");

    let arguments = &context.arguments;
    let use_symbol = module
        .config_value_bool("use_symbol_for_status")
        .unwrap_or(false);
    let exit_success = arguments.value_of("status_code").unwrap_or("0") == "0";
    let shell = std::env::var("STARSHIP_SHELL").unwrap_or_default();
    let keymap = arguments.value_of("keymap").unwrap_or("viins");

    // Match shell "keymap" names to normalized vi modes
    // NOTE: in vi mode, fish reports normal mode as "default".
    // Unfortunately, this is also the name of the non-vi default mode.
    // We do some environment detection in src/init.rs to translate.
    // The result: in non-vi fish, keymap is always reported as "insert"
    let mode = match (shell.as_str(), keymap) {
        ("fish", "default") | ("zsh", "vicmd") => ShellEditMode::Normal,
        _ => ASSUMED_MODE,
    };

    /* If an error symbol is set in the config, use symbols to indicate
    success/failure, in addition to color */
    let symbol = if use_symbol && !exit_success {
        module.new_segment("error_symbol", FAILURE_CHAR)
    } else {
        match mode {
            ShellEditMode::Normal => module.new_segment("vicmd_symbol", VICMD_CHAR),
            ShellEditMode::Insert => module.new_segment("symbol", SUCCESS_CHAR),
        }
    };

    if exit_success {
        symbol.set_style(color_success.bold());
    } else {
        symbol.set_style(color_failure.bold());
    };

    Some(module)
}
