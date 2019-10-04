use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::character::CharacterConfig;

/// Creates a module for the prompt character
///
/// The character segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `style_success`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `style_failure` (red by default)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    enum ShellEditMode {
        Normal,
        Insert,
    };
    const ASSUMED_MODE: ShellEditMode = ShellEditMode::Insert;
    // TODO: extend config to more modes

    let mut module = context.new_module("character");
    let config: CharacterConfig = CharacterConfig::try_load(module.config);
    module.get_prefix().set_value("");

    let arguments = &context.arguments;
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

    if exit_success {
        module.set_style(config.style_success);
    } else {
        module.set_style(config.style_failure);
    };

    /* If an error symbol is set in the config, use symbols to indicate
    success/failure, in addition to color */
    if config.use_symbol_for_status && !exit_success {
        module.create_segment("error_symbol", &config.error_symbol)
    } else {
        match mode {
            ShellEditMode::Normal => module.create_segment("vicmd_symbol", &config.vicmd_symbol),
            ShellEditMode::Insert => module.create_segment("symbol", &config.symbol),
        }
    };

    Some(module)
}
