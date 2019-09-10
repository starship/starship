use crate::utils;
use std::env;

use dirs::home_dir;
use toml::value::Table;

use ansi_term::Color;

pub trait Config {
    fn initialize() -> Table;
    fn config_from_file() -> Option<Table>;
    fn get_module_config(&self, module_name: &str) -> Option<&Table>;

    // Config accessor methods
    fn get_as_bool(&self, key: &str) -> Option<bool>;
    fn get_as_str(&self, key: &str) -> Option<&str>;
    fn get_as_i64(&self, key: &str) -> Option<i64>;
    fn get_as_array(&self, key: &str) -> Option<&Vec<toml::value::Value>>;
    fn get_as_ansi_style(&self, key: &str) -> Option<ansi_term::Style>;

    // Internal implementation for accessors
    fn get_config(&self, key: &str) -> Option<&toml::value::Value>;
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

    /// Get the subset of the table for a module by its name
    fn get_module_config(&self, module_name: &str) -> Option<&toml::value::Table> {
        log::trace!("{}",module_name);
        let module_config = self.get(module_name).and_then(toml::Value::as_table);

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

    /// Get the config value for a given key
    fn get_config(&self, key: &str) -> Option<&toml::value::Value> {
        log::trace!("Looking for config key \"{}\"", key);
        let config_value = self.get(key);

        if config_value.is_some() {
            log::trace!("Config found for \"{}\": {:?}", key, &config_value);
        } else {
            log::trace!("No value found for \"{}\"", key);
        }

        config_value
    }

    /// Get a key from a module's configuration as a boolean
    fn get_as_bool(&self, key: &str) -> Option<bool> {
        let value = self.get_config(key)?;
        let bool_value = value.as_bool();

        if bool_value.is_none() {
            log::debug!(
                "Expected \"{}\" to be a boolean. Instead received {} of type {}.",
                key,
                value,
                value.type_str()
            );
        }

        bool_value
    }

    /// Get a key from a module's configuration as a string
    fn get_as_str(&self, key: &str) -> Option<&str> {
        let value = self.get_config(key)?;
        let str_value = value.as_str();

        if str_value.is_none() {
            log::debug!(
                "Expected \"{}\" to be a string. Instead received {} of type {}.",
                key,
                value,
                value.type_str()
            );
        }

        str_value
    }

    /// Get a key from a module's configuration as an integer
    fn get_as_i64(&self, key: &str) -> Option<i64> {
        let value = self.get_config(key)?;
        let i64_value = value.as_integer();

        if i64_value.is_none() {
            log::debug!(
                "Expected \"{}\" to be an integer. Instead received {} of type {}.",
                key,
                value,
                value.type_str()
            );
        }

        i64_value
    }

    /// Get a key from a module's configuration as a vector
    fn get_as_array(&self, key: &str) -> Option<&Vec<toml::value::Value>> {
        let value = self.get_config(key)?;
        let array_value = value.as_array();
        if array_value.is_none() {
            log::debug!(
                "Expected \"{}\" to be a array. Instead received {} of type {}.",
                key,
                value,
                value.type_str()
            );
        }
        array_value
    }

    /// Get a text key and attempt to interpret it into an ANSI style.
    fn get_as_ansi_style(&self, key: &str) -> Option<ansi_term::Style> {
        let style_string = self.get_as_str(key)?;
        parse_style_string(style_string)
    }
}

/** Parse a style string which represents an ansi style. Valid tokens in the style
 string include the following:
 - 'fg:<color>'    (specifies that the color read should be a foreground color)
 - 'bg:<color>'    (specifies that the color read should be a background color)
 - 'underline'
 - 'bold'
 - '<color>'        (see the parse_color_string doc for valid color strings)
*/
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
    use ansi_term::Style;

    #[test]
    fn table_get_as_bool() {
        let mut table = toml::value::Table::new();

        // Use with boolean value
        table.insert(String::from("boolean"), toml::value::Value::Boolean(true));
        assert_eq!(table.get_as_bool("boolean"), Some(true));

        // Use with string value
        table.insert(
            String::from("string"),
            toml::value::Value::String(String::from("true")),
        );
        assert_eq!(table.get_as_bool("string"), None);
    }

    #[test]
    fn table_get_as_str() {
        let mut table = toml::value::Table::new();

        // Use with string value
        table.insert(
            String::from("string"),
            toml::value::Value::String(String::from("hello")),
        );
        assert_eq!(table.get_as_str("string"), Some("hello"));

        // Use with boolean value
        table.insert(String::from("boolean"), toml::value::Value::Boolean(true));
        assert_eq!(table.get_as_str("boolean"), None);
    }

    #[test]
    fn table_get_as_i64() {
        let mut table = toml::value::Table::new();

        // Use with integer value
        table.insert(String::from("integer"), toml::value::Value::Integer(82));
        assert_eq!(table.get_as_i64("integer"), Some(82));

        // Use with string value
        table.insert(
            String::from("string"),
            toml::value::Value::String(String::from("82")),
        );
        assert_eq!(table.get_as_bool("string"), None);
    }

    #[test]
    fn table_get_styles_simple() {
        let mut table = toml::value::Table::new();

        // Test for a bold underline green module (with SiLlY cApS)
        table.insert(
            String::from("mystyle"),
            toml::value::Value::String(String::from("bOlD uNdErLiNe GrEeN")),
        );
        assert!(table.get_as_ansi_style("mystyle").unwrap().is_bold);
        assert!(table.get_as_ansi_style("mystyle").unwrap().is_underline);
        assert_eq!(
            table.get_as_ansi_style("mystyle").unwrap(),
            ansi_term::Style::new().bold().underline().fg(Color::Green)
        );

        // Test a "plain" style with no formatting
        table.insert(
            String::from("plainstyle"),
            toml::value::Value::String(String::from("")),
        );
        assert_eq!(
            table.get_as_ansi_style("plainstyle").unwrap(),
            ansi_term::Style::new()
        );

        // Test a string that's clearly broken
        table.insert(
            String::from("broken"),
            toml::value::Value::String(String::from("djklgfhjkldhlhk;j")),
        );
        assert_eq!(
            table.get_as_ansi_style("broken").unwrap(),
            ansi_term::Style::new()
        );

        // Test a string that's nullified by `none`
        table.insert(
            String::from("nullified"),
            toml::value::Value::String(String::from("fg:red bg:green bold none")),
        );
        assert_eq!(
            table.get_as_ansi_style("nullified").unwrap(),
            ansi_term::Style::new()
        );
    }

    #[test]
    fn table_get_styles_ordered() {
        let mut table = toml::value::Table::new();

        // Test a background style with inverted order (also test hex + ANSI)
        table.insert(
            String::from("flipstyle"),
            toml::value::Value::String(String::from("bg:#050505 underline fg:120")),
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
            toml::value::Value::String(String::from("bg:120 bg:125 bg:127 fg:127 122 125")),
        );
        assert_eq!(
            table.get_as_ansi_style("multistyle").unwrap(),
            Style::new().fg(Color::Fixed(125)).on(Color::Fixed(127))
        );
    }
}
