use ansi_term::{Color, Style};

use std::clone::Clone;
use std::marker::Sized;

pub trait ModuleConfig<'a>
where
    Self: Sized + Clone,
{
    /// Construct a `ModuleConfig` from a toml value.
    fn from_config(_config: &'a toml::Value) -> Option<Self> {
        None
    }

    /// Merge `self` with config from a toml table.
    fn load_config(&self, config: &'a toml::Value) -> Self {
        Self::from_config(config).unwrap_or_else(|| self.clone())
    }
}

// TODO: Add logging to default implementations
impl<'a> ModuleConfig<'a> for &'a str {
    fn from_config(config: &'a toml::Value) -> Option<Self> {
        config.as_str()
    }
}

impl<'a> ModuleConfig<'a> for Style {
    fn from_config(config: &toml::Value) -> Option<Self> {
        parse_style_string(config.as_str()?)
    }
}

impl<'a> ModuleConfig<'a> for bool {
    fn from_config(config: &toml::Value) -> Option<Self> {
        config.as_bool()
    }
}

impl<'a> ModuleConfig<'a> for i64 {
    fn from_config(config: &toml::Value) -> Option<Self> {
        config.as_integer()
    }
}

impl<'a> ModuleConfig<'a> for f64 {
    fn from_config(config: &toml::Value) -> Option<Self> {
        config.as_float()
    }
}

impl<'a, T> ModuleConfig<'a> for Vec<T>
where
    T: ModuleConfig<'a>,
{
    fn from_config(config: &'a toml::Value) -> Option<Self> {
        config
            .as_array()?
            .iter()
            .map(|value| T::from_config(value))
            .collect()
    }
}

fn parse_style_string(style_string: &str) -> Option<ansi_term::Style> {
    let tokens = style_string.split_whitespace();
    let mut style = ansi_term::Style::new();

    // If col_fg is true, color the foreground. If it's false, color the background.
    let mut col_fg: bool;

    for token in tokens {
        let token = token.to_lowercase();

        // Check for FG/BG identifiers and strip them off if appropriate
        let token = if token.as_str().starts_with("fg:") {
            col_fg = true;
            token.trim_start_matches("fg:").to_owned()
        } else if token.as_str().starts_with("bg:") {
            col_fg = false;
            token.trim_start_matches("bg:").to_owned()
        } else {
            col_fg = true; // Bare colors are assumed to color the foreground
            token
        };

        match token.as_str() {
            "underline" => style = style.underline(),
            "bold" => style = style.bold(),
            "italic" => style = style.italic(),
            "dimmed" => style = style.dimmed(),
            "none" => return Some(ansi_term::Style::new()), // Overrides other toks

            // Try to see if this token parses as a valid color string
            color_string => {
                // Match found: set either fg or bg color
                if let Some(ansi_color) = parse_color_string(color_string) {
                    if col_fg {
                        style = style.fg(ansi_color);
                    } else {
                        style = style.on(ansi_color);
                    }
                } else {
                    // Match failed: skip this token and log it
                    log::debug!("Could not parse token in color string: {}", token)
                }
            }
        }
    }

    Some(style)
}

/** Parse a string that represents a color setting, returning None if this fails
 There are three valid color formats:
  - #RRGGBB      (a hash followed by an RGB hex)
  - u8           (a number from 0-255, representing an ANSI color)
  - colstring    (one of the 16 predefined color strings)
*/
fn parse_color_string(color_string: &str) -> Option<ansi_term::Color> {
    // Parse RGB hex values
    log::trace!("Parsing color_string: {}", color_string);
    if color_string.starts_with('#') {
        log::trace!(
            "Attempting to read hexadecimal color string: {}",
            color_string
        );
        let r: u8 = u8::from_str_radix(&color_string[1..3], 16).ok()?;
        let g: u8 = u8::from_str_radix(&color_string[3..5], 16).ok()?;
        let b: u8 = u8::from_str_radix(&color_string[5..7], 16).ok()?;
        log::trace!("Read RGB color string: {},{},{}", r, g, b);
        return Some(Color::RGB(r, g, b));
    }

    // Parse a u8 (ansi color)
    if let Result::Ok(ansi_color_num) = color_string.parse::<u8>() {
        log::trace!("Read ANSI color string: {}", ansi_color_num);
        return Some(Color::Fixed(ansi_color_num));
    }

    // Check for any predefined color strings
    // There are no predefined enums for bright colors, so we use Color::Fixed
    let predefined_color = match color_string.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "purple" => Some(Color::Purple),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "bright-black" => Some(Color::Fixed(8)), // "bright-black" is dark grey
        "bright-red" => Some(Color::Fixed(9)),
        "bright-green" => Some(Color::Fixed(10)),
        "bright-yellow" => Some(Color::Fixed(11)),
        "bright-blue" => Some(Color::Fixed(12)),
        "bright-purple" => Some(Color::Fixed(13)),
        "bright-cyan" => Some(Color::Fixed(14)),
        "bright-white" => Some(Color::Fixed(15)),
        _ => None,
    };

    if predefined_color.is_some() {
        log::trace!("Read predefined color: {}", color_string);
        return predefined_color;
    }

    // All attempts to parse have failed
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use starship_module_config_derive::ModuleConfig;
    use toml;

    #[test]
    fn test_load_config() {
        #[derive(Clone, ModuleConfig)]
        struct TestConfig<'a> {
            pub symbol: &'a str,
            pub disabled: bool,
            pub some_array: Vec<&'a str>,
        }

        let config = toml::toml! {
            symbol = "T "
            disabled = true
            some_array = ["A"]
        };
        let default_config = TestConfig {
            symbol: "S ",
            disabled: false,
            some_array: vec!["A", "B", "C"],
        };
        let rust_config = default_config.load_config(&config);

        assert_eq!(rust_config.symbol, "T ");
        assert_eq!(rust_config.disabled, true);
        assert_eq!(rust_config.some_array, vec!["A"]);
    }

    #[test]
    fn test_load_nested_config() {
        #[derive(Clone, ModuleConfig)]
        struct TestConfig<'a> {
            pub untracked: SegmentDisplayConfig<'a>,
            pub modified: SegmentDisplayConfig<'a>,
        }

        #[derive(PartialEq, Debug, Clone, ModuleConfig)]
        struct SegmentDisplayConfig<'a> {
            pub value: &'a str,
            pub style: Style,
        }

        let config = toml::toml! {
            untracked.value = "x"
            modified = { value = "•", style = "red" }
        };

        let default_config = TestConfig {
            untracked: SegmentDisplayConfig {
                value: "?",
                style: Color::Red.bold(),
            },
            modified: SegmentDisplayConfig {
                value: "!",
                style: Color::Red.bold(),
            },
        };
        let git_status_config = default_config.load_config(&config);

        assert_eq!(
            git_status_config.untracked,
            SegmentDisplayConfig {
                value: "x",
                style: Color::Red.bold(),
            }
        );
        assert_eq!(
            git_status_config.modified,
            SegmentDisplayConfig {
                value: "•",
                style: Color::Red.normal(),
            }
        );
    }

    #[test]
    fn test_from_string() {
        let config = toml::Value::String(String::from("S"));
        assert_eq!(<&str>::from_config(&config).unwrap(), "S")
    }

    #[test]
    fn test_from_bool() {
        let config = toml::Value::Boolean(true);
        assert_eq!(<bool>::from_config(&config).unwrap(), true)
    }

    #[test]
    fn test_from_vec() {
        let config: toml::Value = toml::Value::Array(vec![toml::Value::from("S")]);
        let default_config: Vec<&str> = vec!["S"];
        assert_eq!(<Vec<&str>>::from_config(&config).unwrap(), default_config)
    }
}
