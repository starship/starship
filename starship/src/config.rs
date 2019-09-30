#![allow(dead_code)]
use crate::configs::StarshipRootConfig;
use crate::utils;
use ansi_term::{Color, Style};

use std::clone::Clone;
use std::marker::Sized;

use dirs::home_dir;
use std::env;
use toml::Value;

/// Root config of a module.
pub trait RootModuleConfig<'a>
where
    Self: ModuleConfig<'a>,
{
    /// Create a new root module config with default values.
    fn new() -> Self;

    /// Load root module config from given Value and fill unset variables with default
    /// values.
    fn load(config: &'a Value) -> Self {
        Self::new().load_config(config)
    }

    /// Helper function that will call RootModuleConfig::load(config) if config is Some,
    /// or RootModuleConfig::new() if config is None.
    fn try_load(config: Option<&'a Value>) -> Self {
        if let Some(config) = config {
            Self::load(config)
        } else {
            Self::new()
        }
    }
}

/// Parsable config.
pub trait ModuleConfig<'a>
where
    Self: Sized + Clone,
{
    /// Construct a `ModuleConfig` from a toml value.
    fn from_config(_config: &'a Value) -> Option<Self> {
        None
    }

    /// Merge `self` with config from a toml table.
    fn load_config(&self, config: &'a Value) -> Self {
        Self::from_config(config).unwrap_or_else(|| self.clone())
    }
}

// TODO: Add logging to default implementations
impl<'a> ModuleConfig<'a> for &'a str {
    fn from_config(config: &'a Value) -> Option<Self> {
        config.as_str()
    }
}

impl<'a> ModuleConfig<'a> for Style {
    fn from_config(config: &Value) -> Option<Self> {
        parse_style_string(config.as_str()?)
    }
}

impl<'a> ModuleConfig<'a> for bool {
    fn from_config(config: &Value) -> Option<Self> {
        config.as_bool()
    }
}

impl<'a> ModuleConfig<'a> for i64 {
    fn from_config(config: &Value) -> Option<Self> {
        config.as_integer()
    }
}

impl<'a> ModuleConfig<'a> for f64 {
    fn from_config(config: &Value) -> Option<Self> {
        config.as_float()
    }
}

impl<'a, T> ModuleConfig<'a> for Vec<T>
where
    T: ModuleConfig<'a>,
{
    fn from_config(config: &'a Value) -> Option<Self> {
        config
            .as_array()?
            .iter()
            .map(|value| T::from_config(value))
            .collect()
    }
}

impl<'a, T> ModuleConfig<'a> for Option<T>
where
    T: ModuleConfig<'a> + Sized,
{
    fn from_config(config: &'a Value) -> Option<Self> {
        Some(T::from_config(config))
    }
}

/// Root config of starship.
pub struct StarshipConfig {
    pub config: Option<Value>,
}

impl StarshipConfig {
    /// Initialize the Config struct
    pub fn initialize() -> Self {
        if let Some(file_data) = Self::config_from_file() {
            StarshipConfig {
                config: Some(file_data),
            }
        } else {
            StarshipConfig {
                config: Some(Value::Table(toml::value::Table::new())),
            }
        }
    }

    /// Create a config from a starship configuration file
    fn config_from_file() -> Option<Value> {
        let file_path = if let Ok(path) = env::var("STARSHIP_CONFIG") {
            // Use $STARSHIP_CONFIG as the config path if available
            log::debug!("STARSHIP_CONFIG is set: \n{}", &path);
            path
        } else {
            // Default to using ~/.config/starship.toml
            log::debug!("STARSHIP_CONFIG is not set");
            let config_path = home_dir()?.join(".config/starship.toml");
            let config_path_str = config_path.to_str()?.to_owned();
            log::debug!("Using default config path: {}", config_path_str);
            config_path_str
        };

        let toml_content = match utils::read_file(&file_path) {
            Ok(content) => {
                log::trace!("Config file content: \n{}", &content);
                Some(content)
            }
            Err(e) => {
                log::debug!("Unable to read config file content: \n{}", &e);
                None
            }
        }?;

        let config = toml::from_str(&toml_content).ok()?;
        log::debug!("Config parsed: \n{:?}", &config);
        Some(config)
    }

    /// Get the subset of the table for a module by its name
    pub fn get_module_config(&self, module_name: &str) -> Option<&Value> {
        let module_config = self.config.as_ref()?.as_table()?.get(module_name);
        if module_config.is_some() {
            log::debug!(
                "Config found for \"{}\": \n{:?}",
                &module_name,
                &module_config
            );
        } else {
            log::trace!("No config found for \"{}\"", &module_name);
        }
        module_config
    }

    pub fn get_root_config(&self) -> StarshipRootConfig {
        if let Some(root_config) = &self.config {
            StarshipRootConfig::load(root_config)
        } else {
            StarshipRootConfig::new()
        }
    }
}

#[derive(Clone)]
pub struct SegmentConfig<'a> {
    pub value: &'a str,
    pub style: Option<Style>,
}

impl<'a> ModuleConfig<'a> for SegmentConfig<'a> {
    fn from_config(config: &'a Value) -> Option<Self> {
        match config {
            Value::String(ref config_str) => Some(Self {
                value: config_str,
                style: None,
            }),
            Value::Table(ref config_table) => Some(Self {
                value: config_table.get("value")?.as_str()?,
                style: config_table.get("style").and_then(<Style>::from_config),
            }),
            _ => None,
        }
    }

    fn load_config(&self, config: &'a Value) -> Self {
        let mut new_config = self.clone();
        match config {
            Value::String(ref config_str) => {
                new_config.value = config_str;
            }
            Value::Table(ref config_table) => {
                if let Some(Value::String(value)) = config_table.get("value") {
                    new_config.value = value;
                };
                if let Some(style) = config_table.get("style") {
                    new_config.style = <Style>::from_config(style);
                };
            }
            _ => {}
        };
        new_config
    }
}

impl<'a> SegmentConfig<'a> {
    /// Mutably set value
    fn set_value(&mut self, value: &'a str) {
        self.value = value;
    }

    fn set_style(&mut self, style: Style) {
        self.style = Some(style);
    }
}

/** Parse a style string which represents an ansi style. Valid tokens in the style
 string include the following:
 - 'fg:<color>'    (specifies that the color read should be a foreground color)
 - 'bg:<color>'    (specifies that the color read should be a background color)
 - 'underline'
 - 'bold'
 - 'italic'
 - '<color>'        (see the parse_color_string doc for valid color strings)
*/
fn parse_style_string(style_string: &str) -> Option<ansi_term::Style> {
    style_string
        .split_whitespace()
        .fold(Some(ansi_term::Style::new()), |maybe_style, token| {
            maybe_style.and_then(|style| {
                let token = token.to_lowercase();

                // Check for FG/BG identifiers and strip them off if appropriate
                // If col_fg is true, color the foreground. If it's false, color the background.
                let (token, col_fg) = if token.as_str().starts_with("fg:") {
                    (token.trim_start_matches("fg:").to_owned(), true)
                } else if token.as_str().starts_with("bg:") {
                    (token.trim_start_matches("bg:").to_owned(), false)
                } else {
                    (token, true) // Bare colors are assumed to color the foreground
                };

                match token.as_str() {
                    "underline" => Some(style.underline()),
                    "bold" => Some(style.bold()),
                    "italic" => Some(style.italic()),
                    "dimmed" => Some(style.dimmed()),
                    "none" => None,

                    // Try to see if this token parses as a valid color string
                    color_string => parse_color_string(color_string).map(|ansi_color| {
                        if col_fg {
                            style.fg(ansi_color)
                        } else {
                            style.on(ansi_color)
                        }
                    }),
                }
            })
        })
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
    } else {
        log::debug!("Could not parse color in string: {}", color_string);
    }
    predefined_color
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
    fn test_load_optional_config() {
        #[derive(Clone, ModuleConfig)]
        struct TestConfig<'a> {
            pub optional: Option<&'a str>,
            pub hidden: Option<&'a str>,
        }

        let config = toml::toml! {
            optional = "test"
        };
        let default_config = TestConfig {
            optional: None,
            hidden: None,
        };
        let rust_config = default_config.load_config(&config);

        assert_eq!(rust_config.optional, Some("test"));
        assert_eq!(rust_config.hidden, None);
    }

    #[test]
    fn test_load_enum_config() {
        #[derive(Clone, ModuleConfig)]
        struct TestConfig {
            pub switch_a: Switch,
            pub switch_b: Switch,
            pub switch_c: Switch,
        }

        #[derive(Debug, PartialEq, Clone)]
        enum Switch {
            ON,
            OFF,
        }

        impl<'a> ModuleConfig<'a> for Switch {
            fn from_config(config: &'a Value) -> Option<Self> {
                match config.as_str()? {
                    "on" => Some(Self::ON),
                    "off" => Some(Self::OFF),
                    _ => None,
                }
            }
        }

        let config = toml::toml! {
            switch_a = "on"
            switch_b = "any"
        };
        let default_config = TestConfig {
            switch_a: Switch::OFF,
            switch_b: Switch::OFF,
            switch_c: Switch::OFF,
        };
        let rust_config = default_config.load_config(&config);

        assert_eq!(rust_config.switch_a, Switch::ON);
        assert_eq!(rust_config.switch_b, Switch::OFF);
        assert_eq!(rust_config.switch_c, Switch::OFF);
    }

    #[test]
    fn test_from_string() {
        let config = Value::String(String::from("S"));
        assert_eq!(<&str>::from_config(&config).unwrap(), "S");
    }

    #[test]
    fn test_from_bool() {
        let config = Value::Boolean(true);
        assert_eq!(<bool>::from_config(&config).unwrap(), true);
    }

    #[test]
    fn test_from_i64() {
        let config = Value::Integer(42);
        assert_eq!(<i64>::from_config(&config).unwrap(), 42);
    }

    #[test]
    fn test_from_style() {
        let config = Value::from("red bold");
        assert_eq!(<Style>::from_config(&config).unwrap(), Color::Red.bold());
    }

    #[test]
    fn test_from_vec() {
        let config: Value = Value::Array(vec![Value::from("S")]);
        assert_eq!(<Vec<&str>>::from_config(&config).unwrap(), vec!["S"]);
    }

    #[test]
    fn test_from_option() {
        let config: Value = Value::String(String::from("S"));
        assert_eq!(<Option<&str>>::from_config(&config).unwrap(), Some("S"));
    }

    #[test]
    fn table_get_styles_bold_italic_underline_green_dimmy_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD");
        let mystyle = <Style>::from_config(&config).unwrap();
        assert!(mystyle.is_bold);
        assert!(mystyle.is_italic);
        assert!(mystyle.is_underline);
        assert!(mystyle.is_dimmed);
        assert_eq!(
            mystyle,
            ansi_term::Style::new()
                .bold()
                .italic()
                .underline()
                .dimmed()
                .fg(Color::Green)
        );
    }

    #[test]
    fn table_get_styles_plain_and_broken_styles() {
        // Test a "plain" style with no formatting
        let config = Value::from("");
        let plain_style = <Style>::from_config(&config).unwrap();
        assert_eq!(plain_style, ansi_term::Style::new());

        // Test a string that's clearly broken
        let config = Value::from("djklgfhjkldhlhk;j");
        assert!(<Style>::from_config(&config).is_none());

        // Test a string that's nullified by `none`
        let config = Value::from("fg:red bg:green bold none");
        assert!(<Style>::from_config(&config).is_none());

        // Test a string that's nullified by `none` at the start
        let config = Value::from("none fg:red bg:green bold");
        assert!(<Style>::from_config(&config).is_none());
    }

    #[test]
    fn table_get_styles_ordered() {
        // Test a background style with inverted order (also test hex + ANSI)
        let config = Value::from("bg:#050505 underline fg:120");
        let flipped_style = <Style>::from_config(&config).unwrap();
        assert_eq!(
            flipped_style,
            Style::new()
                .underline()
                .fg(Color::Fixed(120))
                .on(Color::RGB(5, 5, 5))
        );

        // Test that the last color style is always the one used
        let config = Value::from("bg:120 bg:125 bg:127 fg:127 122 125");
        let multi_style = <Style>::from_config(&config).unwrap();
        assert_eq!(
            multi_style,
            Style::new().fg(Color::Fixed(125)).on(Color::Fixed(127))
        );
    }
}
