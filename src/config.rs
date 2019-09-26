use crate::utils;
use std::env;

use dirs::home_dir;
use toml::value::Table;
use toml::value::Value;

use ansi_term::Color;

pub trait Config {
    fn initialize() -> Table;
    fn config_from_file() -> Option<Table>;
    fn get_module_config(&self, module_name: &str) -> Option<&Table>;

    // Config accessor methods
    fn get_as_bool(&self, key: &str) -> Option<bool>;
    fn get_as_str(&self, key: &str) -> Option<&str>;
    fn get_as_i64(&self, key: &str) -> Option<i64>;
    fn get_as_array(&self, key: &str) -> Option<&Vec<Value>>;
    fn get_as_ansi_style(&self, key: &str) -> Option<ansi_term::Style>;
    fn get_as_segment_config(&self, key: &str) -> Option<SegmentConfig>;

    // Internal implementation for accessors
    fn get_config(&self, key: &str) -> Option<&Value>;
}

impl Config for Table {
    /// Initialize the Config struct
    fn initialize() -> Table {
        if let Some(file_data) = Self::config_from_file() {
            return file_data;
        }
        Self::new()
    }

    /// Create a config from a starship configuration file
    fn config_from_file() -> Option<Table> {
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

    /// Get the config value for a given key
    fn get_config(&self, key: &str) -> Option<&Value> {
        log::trace!("Looking for config key \"{}\"", key);
        let value = self.get(key);
        log_if_key_found(key, value);
        value
    }

    /// Get the subset of the table for a module by its name
    fn get_module_config(&self, key: &str) -> Option<&Table> {
        log::trace!("Looking for module key \"{}\"", key);
        let value = self.get(key);
        log_if_key_found(key, value);
        value.and_then(|value| {
            let casted = Value::as_table(value);
            log_if_type_correct(key, value, casted);
            casted
        })
    }

    /// Get a key from a module's configuration as a boolean
    fn get_as_bool(&self, key: &str) -> Option<bool> {
        log::trace!("Looking for boolean key \"{}\"", key);
        let value = self.get(key);
        log_if_key_found(key, value);
        value.and_then(|value| {
            let casted = Value::as_bool(value);
            log_if_type_correct(key, value, casted);
            casted
        })
    }

    /// Get a key from a module's configuration as a string
    fn get_as_str(&self, key: &str) -> Option<&str> {
        log::trace!("Looking for string key \"{}\"", key);
        let value = self.get(key);
        log_if_key_found(key, value);
        value.and_then(|value| {
            let casted = Value::as_str(value);
            log_if_type_correct(key, value, casted);
            casted
        })
    }

    /// Get a key from a module's configuration as an integer
    fn get_as_i64(&self, key: &str) -> Option<i64> {
        log::trace!("Looking for integer key \"{}\"", key);
        let value = self.get(key);
        log_if_key_found(key, value);
        value.and_then(|value| {
            let casted = Value::as_integer(value);
            log_if_type_correct(key, value, casted);
            casted
        })
    }

    /// Get a key from a module's configuration as a vector
    fn get_as_array(&self, key: &str) -> Option<&Vec<Value>> {
        log::trace!("Looking for array key \"{}\"", key);
        let value = self.get(key);
        log_if_key_found(key, value);
        value.and_then(|value| {
            let casted = Value::as_array(value);
            log_if_type_correct(key, value, casted);
            casted
        })
    }

    /// Get a text key and attempt to interpret it into an ANSI style.
    fn get_as_ansi_style(&self, key: &str) -> Option<ansi_term::Style> {
        // TODO: This should probably not unwrap to an empty new Style but inform the user about the problem
        self.get_as_str(key)
            .map(|x| parse_style_string(x).unwrap_or_default())
    }
}

fn log_if_key_found(key: &str, something: Option<&Value>) {
    if something.is_some() {
        log::trace!("Value found for \"{}\": {:?}", key, &something);
    } else {
        log::trace!("No value found for \"{}\"", key);
    }
}

fn log_if_type_correct<T: std::fmt::Debug>(
    key: &str,
    something: &Value,
    casted_something: Option<T>,
) {
    if let Some(casted) = casted_something {
        log::trace!(
            "Value under key \"{}\" has the expected type. Proceeding with {:?} which was build from {:?}.",
            key,
            casted,
            something
            );
    } else {
        log::debug!(
            "Value under key \"{}\" did not have the expected type. Instead received {} of type {}.",
            key,
            something,
            something.type_str()
            );
    }

    /// Get a key from a module's configuration as a segment config.
    ///
    /// The config can be
    ///
    /// - a string, will be interpreted as value.
    /// - a table with optional { value, style } keys.
    ///   If omitted, default value will be used.
    ///
    /// Returns `Some(SegmentConfig)` if key exists in the configuration, else `None`.
    fn get_as_segment_config(&self, key: &str) -> Option<SegmentConfig> {
        let segment_config = self.get_config(key)?;
        match segment_config {
            toml::Value::String(value) => Some(SegmentConfig {
                value: Some(value.as_str()),
                style: None,
            }),
            toml::Value::Table(config_table) => Some(SegmentConfig {
                value: config_table.get_as_str("value"),
                style: config_table.get_as_ansi_style("style"),
            }),
            _ => {
                log::debug!(
                    "Expected \"{}\" to be a string or config table. Instead received {} of type {}.",
                    key,
                    segment_config,
                    segment_config.type_str()
                );
                None
            }
        }
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

pub struct SegmentConfig<'a> {
    pub value: Option<&'a str>,
    pub style: Option<ansi_term::Style>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ansi_term::Style;

    #[test]
    fn table_get_nonexisting() {
        let table = toml::value::Table::new();
        assert_eq!(table.get_as_bool("boolean"), None);
    }

    #[test]
    fn table_get_config() {
        let mut table = toml::value::Table::new();
        table.insert(String::from("config"), Value::Boolean(true));
        assert_eq!(table.get_config("config"), Some(&Value::Boolean(true)));
    }

    #[test]
    fn table_get_as_bool() {
        let mut table = toml::value::Table::new();

        table.insert(String::from("boolean"), Value::Boolean(true));
        assert_eq!(table.get_as_bool("boolean"), Some(true));

        table.insert(String::from("string"), Value::String(String::from("true")));
        assert_eq!(table.get_as_bool("string"), None);
    }

    #[test]
    fn table_get_as_str() {
        let mut table = toml::value::Table::new();

        table.insert(String::from("string"), Value::String(String::from("hello")));
        assert_eq!(table.get_as_str("string"), Some("hello"));

        table.insert(String::from("boolean"), Value::Boolean(true));
        assert_eq!(table.get_as_str("boolean"), None);
    }

    #[test]
    fn table_get_as_i64() {
        let mut table = toml::value::Table::new();

        table.insert(String::from("integer"), Value::Integer(82));
        assert_eq!(table.get_as_i64("integer"), Some(82));

        table.insert(String::from("string"), Value::String(String::from("82")));
        assert_eq!(table.get_as_bool("string"), None);
    }

    #[test]
    fn table_get_as_array() {
        let mut table = toml::value::Table::new();

        table.insert(
            String::from("array"),
            Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
        );
        assert_eq!(
            table.get_as_array("array"),
            Some(&vec![Value::Integer(1), Value::Integer(2)])
        );

        table.insert(String::from("string"), Value::String(String::from("82")));
        assert_eq!(table.get_as_array("string"), None);
    }

    #[test]
    fn table_get_styles_bold_italic_underline_green_dimmy_silly_caps() {
        let mut table = toml::value::Table::new();

        table.insert(
            String::from("mystyle"),
            Value::String(String::from("bOlD ItAlIc uNdErLiNe GrEeN dimmed")),
        );
        assert!(table.get_as_ansi_style("mystyle").unwrap().is_bold);
        assert!(table.get_as_ansi_style("mystyle").unwrap().is_italic);
        assert!(table.get_as_ansi_style("mystyle").unwrap().is_underline);
        assert!(table.get_as_ansi_style("mystyle").unwrap().is_dimmed);
        assert_eq!(
            table.get_as_ansi_style("mystyle").unwrap(),
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
        let mut table = toml::value::Table::new();
        // Test a "plain" style with no formatting
        table.insert(String::from("plainstyle"), Value::String(String::from("")));
        assert_eq!(
            table.get_as_ansi_style("plainstyle").unwrap(),
            ansi_term::Style::new()
        );

        // Test a string that's clearly broken
        table.insert(
            String::from("broken"),
            Value::String(String::from("djklgfhjkldhlhk;j")),
        );
        assert_eq!(
            table.get_as_ansi_style("broken").unwrap(),
            ansi_term::Style::new()
        );

        // Test a string that's nullified by `none`
        table.insert(
            String::from("nullified"),
            Value::String(String::from("fg:red bg:green bold none")),
        );
        assert_eq!(
            table.get_as_ansi_style("nullified").unwrap(),
            ansi_term::Style::new()
        );

        // Test a string that's nullified by `none` at the start
        table.insert(
            String::from("nullified-start"),
            Value::String(String::from("none fg:red bg:green bold")),
        );
        assert_eq!(
            table.get_as_ansi_style("nullified-start").unwrap(),
            ansi_term::Style::new()
        );
    }

    #[test]
    fn table_get_styles_ordered() {
        let mut table = toml::value::Table::new();

        // Test a background style with inverted order (also test hex + ANSI)
        table.insert(
            String::from("flipstyle"),
            Value::String(String::from("bg:#050505 underline fg:120")),
        );
        assert_eq!(
            table.get_as_ansi_style("flipstyle").unwrap(),
            Style::new()
                .underline()
                .fg(Color::Fixed(120))
                .on(Color::RGB(5, 5, 5))
        );

        // Test that the last color style is always the one used
        table.insert(
            String::from("multistyle"),
            Value::String(String::from("bg:120 bg:125 bg:127 fg:127 122 125")),
        );
        assert_eq!(
            table.get_as_ansi_style("multistyle").unwrap(),
            Style::new().fg(Color::Fixed(125)).on(Color::Fixed(127))
        );
    }
}
