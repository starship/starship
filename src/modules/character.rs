use super::{Context, Module, RootModuleConfig};

use crate::configs::character::CharacterConfig;
use crate::modules::utils::query_parser::*;
use crate::segment::Segment;

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

    let props = &context.properties;
    let exit_code_default = std::string::String::from("0");
    let exit_code = props.get("status_code").unwrap_or(&exit_code_default);
    let shell = std::env::var("STARSHIP_SHELL").unwrap_or_default();
    let keymap_default = std::string::String::from("viins");
    let keymap = props.get("keymap").unwrap_or(&keymap_default);
    let exit_success = exit_code == "0";

    // Match shell "keymap" names to normalized vi modes
    // NOTE: in vi mode, fish reports normal mode as "default".
    // Unfortunately, this is also the name of the non-vi default mode.
    // We do some environment detection in src/init.rs to translate.
    // The result: in non-vi fish, keymap is always reported as "insert"
    let mode = match (shell.as_str(), keymap.as_str()) {
        ("fish", "default") | ("zsh", "vicmd") => ShellEditMode::Normal,
        _ => ASSUMED_MODE,
    };

    let symbol_style = if exit_success {
        config.style_success
    } else {
        config.style_failure
    };

    /* If an error symbol is set in the config, use symbols to indicate
    success/failure, in addition to color */
    let prompt_symbol = if config.use_symbol_for_status && !exit_success {
        &config.error_symbol
    } else {
        match mode {
            ShellEditMode::Normal => &config.vicmd_symbol,
            ShellEditMode::Insert => &config.symbol,
        }
    };

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query).or(Some(symbol_style));
        match name {
            "prompt_symbol" => Some(Segment {
                _name: "prompt_symbol".to_string(),
                value: prompt_symbol.to_string(),
                style,
            }),
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}
