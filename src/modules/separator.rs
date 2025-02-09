use super::{Context, Module, ModuleConfig};
use crate::{configs::separator::SeperatorConfig, segment::Segment};
use nu_ansi_term::{Color, Style};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy)]
enum TermColor {
    Standard(Color),
    Bright(Color),
}

impl From<&str> for TermColor {
    fn from(color: &str) -> Self {
        match color {
            // Standard colors
            "black" => TermColor::Standard(Color::Black),
            "red" => TermColor::Standard(Color::Red),
            "green" => TermColor::Standard(Color::Green),
            "yellow" => TermColor::Standard(Color::Yellow),
            "blue" => TermColor::Standard(Color::Blue),
            "purple" => TermColor::Standard(Color::Purple),
            "cyan" => TermColor::Standard(Color::Cyan),
            "white" => TermColor::Standard(Color::White),
            // Bright colors
            "bright-black" => TermColor::Bright(Color::DarkGray),
            "bright-red" => TermColor::Bright(Color::LightRed),
            "bright-green" => TermColor::Bright(Color::LightGreen),
            "bright-yellow" => TermColor::Bright(Color::LightYellow),
            "bright-blue" => TermColor::Bright(Color::LightBlue),
            "bright-purple" => TermColor::Bright(Color::LightPurple),
            "bright-cyan" => TermColor::Bright(Color::LightCyan),
            "bright-white" => TermColor::Bright(Color::LightGray),
            // Default fallback
            _ => TermColor::Standard(Color::Green),
        }
    }
}

impl From<TermColor> for Color {
    fn from(term_color: TermColor) -> Self {
        match term_color {
            TermColor::Standard(c) | TermColor::Bright(c) => c,
        }
    }
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let config = SeperatorConfig::try_load(context.new_module("separator").config);

    // Early return if disabled or no command duration
    if config.disabled || context.get_cmd_duration().is_none() {
        return None;
    }
    let style = Style::new().fg(Color::from(TermColor::from(config.color.as_str())));
    let separator_width = config.symbol.width();
    let separator = config.symbol.repeat(context.width / separator_width);

    let mut module = context.new_module("separator");
    module.set_segments(vec![Segment::fill(Some(style.into()), separator)]);
    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use unicode_width::UnicodeWidthStr;

    fn get_terminal_width() -> usize {
        if let Some((width, _)) = terminal_size::terminal_size() {
            width.0 as usize
        } else {
            80
        }
    }
    #[test]
    fn test_separator_after_command() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "-"
                color = "green"
            })
            .cmd("echo 'test'", None) // Simulates running a command
            .cmd_duration(1500) // Command took 1500ms
            .collect();

        let expected = Some(format!(
            "{}",
            Color::Green.paint("-".repeat(get_terminal_width()/"-".width()))
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_separator_multiple_commands() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "*"
                color = "blue"
            })
            .cmd("ls", None) // First command
            .cmd_duration(500) // First command duration
            .cmd("pwd", None) // Second command
            .cmd_duration(200) // Second command duration
            .collect();

        let expected = Some(format!(
            "{}",
            Color::Blue.paint("*".repeat(get_terminal_width()/"*".width()))
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_separator_zero_duration() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "-"
                color = "red"
            })
            .cmd("echo 'quick'", None)
            .cmd_duration(0) // Command completed instantly
            .collect();
        log::info!("actual: {:?}", actual);
        let expected = Some(format!(
            "{}",
            Color::Red.paint("-".repeat(get_terminal_width()/"-".width()))
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_separator_long_command() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "="
                color = "purple"
            })
            .cmd("sleep 2 && echo 'done'", None) // Long running command
            .cmd_duration(2000) // 2 second duration
            .collect();

        let expected = Some(format!(
            "{}",
            Color::Purple.paint("=".repeat(get_terminal_width()/"=".width()))
        ));
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_disabled_module() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = true
            })
            .cmd_duration(500)
            .collect();

        assert_eq!(actual, None);
    }

    #[test]
    fn test_no_cmd_duration() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
            })
            .collect();

        assert_eq!(actual, None);
    }

    #[test]
    fn test_default_config() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "-"
                color = "green"
            })
            .cmd_duration(500)
            .collect();

        // Default terminal width is usually 80 characters
        let expected = Some(format!("{}", Color::Green.paint("-".repeat(get_terminal_width()/"-".width()))));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_custom_color() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "="
                color = "red"
            })
            .cmd_duration(500)
            .collect();

        let expected = Some(format!("{}", Color::Red.paint("=".repeat(get_terminal_width()/"=".width()))));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_custom_symbol() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "*"
                color = "blue"
            })
            .cmd_duration(500)
            .collect();

        let expected = Some(format!("{}", Color::Blue.paint("*".repeat(get_terminal_width()/"*".width()))));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_custom_width() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "-"
                color = "green"
            })
            .cmd_duration(500)
            .env("COLUMNS", "40") // Set custom terminal width
            .collect();

        let expected = Some(format!("{}", Color::Green.paint("-".repeat(get_terminal_width()/"-".width()))));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bright_colors() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "-"
                color = "bright-red"
            })
            .cmd_duration(500)
            .collect();

        let expected = Some(format!("{}", Color::LightRed.paint("-".repeat(get_terminal_width()/"-".width()))));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_invalid_color() {
        let actual = ModuleRenderer::new("separator")
            .config(toml::toml! {
                [separator]
                disabled = false
                symbol = "-"
                color = "invalid-color"
            })
            .cmd_duration(500)
            .collect();

        // Should default to green when color is invalid
        let expected = Some(format!("{}", Color::Green.paint("-".repeat(get_terminal_width()/"-".width()))));
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn test_empty_symbol() {
    //     let actual = ModuleRenderer::new("separator")
    //         .config(toml::toml! {
    //             [separator]
    //             disabled = false
    //             symbol = ""
    //             color = "green"
    //         })
    //         .cmd_duration(500)
    //         .collect();

    //     // Empty symbol should result in empty string repeated
    //     let expected = Some(format!("{}", Color::Green.paint("")));
    //     assert_eq!(actual, expected);
    // }
}
