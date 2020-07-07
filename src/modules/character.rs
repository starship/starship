use super::{Context, Module, RootModuleConfig, Shell};
use crate::configs::character::CharacterConfig;
use crate::formatter::StringFormatter;

/// Creates a module for the prompt character
///
/// The character segment prints an arrow character in a color dependant on the
/// exit-code of the last executed command:
/// - If the exit-code was "0", it will be formatted with `success_symbol`
///   (green arrow by default)
/// - If the exit-code was anything else, it will be formatted with
///   `error_symbol` (red arrow by default)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    enum ShellEditMode {
        Normal,
        Insert,
    };
    const ASSUMED_MODE: ShellEditMode = ShellEditMode::Insert;
    // TODO: extend config to more modes

    let mut module = context.new_module("character");
    let config: CharacterConfig = CharacterConfig::try_load(module.config);

    let props = &context.properties;
    let exit_code_default = String::from("0");
    let exit_code = props.get("status_code").unwrap_or(&exit_code_default);
    let keymap_default = String::from("viins");
    let keymap = props.get("keymap").unwrap_or(&keymap_default);
    let exit_success = exit_code == "0";

    // Match shell "keymap" names to normalized vi modes
    // NOTE: in vi mode, fish reports normal mode as "default".
    // Unfortunately, this is also the name of the non-vi default mode.
    // We do some environment detection in src/init.rs to translate.
    // The result: in non-vi fish, keymap is always reported as "insert"
    let mode = match (&context.shell, keymap.as_str()) {
        (Shell::Fish, "default") | (Shell::Zsh, "vicmd") => ShellEditMode::Normal,
        _ => ASSUMED_MODE,
    };

    let symbol = match mode {
        ShellEditMode::Normal => config.vicmd_symbol,
        ShellEditMode::Insert => {
            if exit_success {
                config.success_symbol
            } else {
                config.error_symbol
            }
        }
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(symbol),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `character`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
