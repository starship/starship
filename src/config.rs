use crate::utils;
use indexmap::IndexMap;
use owo_colors::{DynColors, Style};
use serde::Serialize;

use std::clone::Clone;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::marker::Sized;

use std::env;
use toml::Value;

/// Root config of a module.
pub trait RootModuleConfig<'a>
where
    Self: ModuleConfig<'a> + Default,
{
    /// Load root module config from given Value and fill unset variables with default
    /// values.
    fn load(config: &'a Value) -> Self {
        let mut out = Self::default();
        out.load_config(config);
        out
    }

    /// Helper function that will call RootModuleConfig::load(config) if config is Some,
    /// or RootModuleConfig::new() if config is None.
    fn try_load(config: Option<&'a Value>) -> Self {
        if let Some(config) = config {
            Self::load(config)
        } else {
            Self::default()
        }
    }
}

impl<'a, T: ModuleConfig<'a> + Default> RootModuleConfig<'a> for T {}

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
    fn load_config(&mut self, config: &'a Value) {
        if let Some(value) = Self::from_config(config) {
            let _ = std::mem::replace(self, value);
        }
    }
}

// TODO: Add logging to default implementations
impl<'a> ModuleConfig<'a> for &'a str {
    fn from_config(config: &'a Value) -> Option<Self> {
        config.as_str()
    }
}

impl<'a> ModuleConfig<'a> for String {
    fn from_config(config: &'a Value) -> Option<Self> {
        config.as_str().map(std::borrow::ToOwned::to_owned)
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

impl<'a> ModuleConfig<'a> for u64 {
    fn from_config(config: &Value) -> Option<Self> {
        match config {
            Value::Integer(value) => {
                // Converting i64 to u64
                if *value > 0 {
                    Some(*value as Self)
                } else {
                    None
                }
            }
            Value::String(value) => value.parse::<Self>().ok(),
            _ => None,
        }
    }
}

impl<'a> ModuleConfig<'a> for f64 {
    fn from_config(config: &Value) -> Option<Self> {
        config.as_float()
    }
}

impl<'a> ModuleConfig<'a> for u32 {
    fn from_config(config: &Value) -> Option<Self> {
        match config {
            Value::Integer(value) => {
                // Converting i64 to u32
                if *value > 0 && *value <= u32::MAX.into() {
                    Some(*value as Self)
                } else {
                    None
                }
            }
            Value::String(value) => value.parse::<Self>().ok(),
            _ => None,
        }
    }
}

impl<'a> ModuleConfig<'a> for usize {
    fn from_config(config: &Value) -> Option<Self> {
        match config {
            Value::Integer(value) => {
                if *value > 0 {
                    Some(*value as Self)
                } else {
                    None
                }
            }
            Value::String(value) => value.parse::<Self>().ok(),
            _ => None,
        }
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

impl<'a, T, S: ::std::hash::BuildHasher + Default> ModuleConfig<'a> for HashMap<String, T, S>
where
    T: ModuleConfig<'a>,
    S: Clone,
{
    fn from_config(config: &'a Value) -> Option<Self> {
        let mut hm = Self::default();

        for (x, y) in config.as_table()? {
            hm.insert(x.clone(), T::from_config(y)?);
        }

        Some(hm)
    }
}

impl<'a, T, S: ::std::hash::BuildHasher + Default> ModuleConfig<'a> for IndexMap<String, T, S>
where
    T: ModuleConfig<'a>,
    S: Clone,
{
    fn from_config(config: &'a Value) -> Option<Self> {
        let mut im = Self::default();

        for (x, y) in config.as_table()? {
            im.insert(x.clone(), T::from_config(y)?);
        }

        Some(im)
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

/// A wrapper around `Vec<T>` that implements `ModuleConfig`, and either
/// accepts a value of type `T` or a list of values of type `T`.
#[derive(Clone, Default, Serialize)]
pub struct VecOr<T>(pub Vec<T>);

impl<'a, T> ModuleConfig<'a> for VecOr<T>
where
    T: ModuleConfig<'a> + Sized,
{
    fn from_config(config: &'a Value) -> Option<Self> {
        if let Some(item) = T::from_config(config) {
            return Some(Self(vec![item]));
        }

        let vec = config
            .as_array()?
            .iter()
            .map(|value| T::from_config(value))
            .collect::<Option<Vec<T>>>()?;

        Some(Self(vec))
    }
}

/// Root config of starship.
pub struct StarshipConfig {
    pub config: Option<Value>,
}

pub fn get_config_path() -> Option<String> {
    if let Ok(path) = env::var("STARSHIP_CONFIG") {
        // Use $STARSHIP_CONFIG as the config path if available
        log::debug!("STARSHIP_CONFIG is set: {}", &path);
        Some(path)
    } else {
        // Default to using ~/.config/starship.toml
        log::debug!("STARSHIP_CONFIG is not set");
        let config_path = utils::home_dir()?.join(".config/starship.toml");
        let config_path_str = config_path.to_str()?.to_owned();
        log::debug!("Using default config path: {}", config_path_str);
        Some(config_path_str)
    }
}

impl StarshipConfig {
    /// Initialize the Config struct
    pub fn initialize() -> Self {
        if let Some(file_data) = Self::config_from_file() {
            Self {
                config: Some(file_data),
            }
        } else {
            Self {
                config: Some(Value::Table(toml::value::Table::new())),
            }
        }
    }

    /// Create a config from a starship configuration file
    fn config_from_file() -> Option<Value> {
        let file_path = get_config_path()?;

        let toml_content = match utils::read_file(&file_path) {
            Ok(content) => {
                log::trace!("Config file content: \"\n{}\"", &content);
                Some(content)
            }
            Err(e) => {
                let level = if e.kind() == ErrorKind::NotFound {
                    log::Level::Debug
                } else {
                    log::Level::Error
                };

                log::log!(level, "Unable to read config file content: {}", &e);
                None
            }
        }?;

        match toml::from_str(&toml_content) {
            Ok(parsed) => {
                log::debug!("Config parsed: {:?}", &parsed);
                Some(parsed)
            }
            Err(error) => {
                log::error!("Unable to parse the config file: {}", error);
                None
            }
        }
    }

    /// Get the subset of the table for a module by its name
    pub fn get_module_config(&self, module_name: &str) -> Option<&Value> {
        let module_config = self.get_config(&[module_name]);
        if module_config.is_some() {
            log::debug!(
                "Config found for \"{}\": {:?}",
                &module_name,
                &module_config
            );
        }
        module_config
    }

    /// Get the value of the config in a specific path
    pub fn get_config(&self, path: &[&str]) -> Option<&Value> {
        let mut prev_table = self.config.as_ref()?.as_table()?;

        assert_ne!(
            path.len(),
            0,
            "Starship::get_config called with an empty path"
        );

        let (table_options, _) = path.split_at(path.len() - 1);

        // Assumes all keys except the last in path has a table
        for option in table_options {
            match prev_table.get(*option) {
                Some(value) => match value.as_table() {
                    Some(value) => {
                        prev_table = value;
                    }
                    None => {
                        log::trace!(
                            "No config found for \"{}\": \"{}\" is not a table",
                            path.join("."),
                            &option
                        );
                        return None;
                    }
                },
                None => {
                    log::trace!(
                        "No config found for \"{}\": Option \"{}\" not found",
                        path.join("."),
                        &option
                    );
                    return None;
                }
            }
        }

        let last_option = path.last().unwrap();
        let value = prev_table.get(*last_option);
        if value.is_none() {
            log::trace!(
                "No config found for \"{}\": Option \"{}\" not found",
                path.join("."),
                &last_option
            );
        };
        value
    }

    /// Get the subset of the table for a custom module by its name
    pub fn get_custom_module_config(&self, module_name: &str) -> Option<&Value> {
        let module_config = self.get_config(&["custom", module_name]);
        if module_config.is_some() {
            log::debug!(
                "Custom config found for \"{}\": {:?}",
                &module_name,
                &module_config
            );
        }
        module_config
    }

    /// Get the table of all the registered custom modules, if any
    pub fn get_custom_modules(&self) -> Option<&toml::value::Table> {
        self.get_config(&["custom"])?.as_table()
    }
    /// Get the table of all the registered env_var modules, if any
    pub fn get_env_var_modules(&self) -> Option<&toml::value::Table> {
        self.get_config(&["env_var"])?.as_table()
    }
}

/** Parse a style string which represents an ansi style. Valid tokens in the style
 string include the following:
 - 'fg:<color>'    (specifies that the color read should be a foreground color)
 - 'bg:<color>'    (specifies that the color read should be a background color)
 - 'underline'
 - 'bold'
 - 'italic'
 - 'inverted'
 - '<color>'       (see the parse_color_string doc for valid color strings)
*/
pub fn parse_style_string(style_string: &str) -> Option<Style> {
    style_string
        .split_whitespace()
        .fold(Some(Style::new()), |maybe_style, token| {
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
                    "inverted" => Some(style.reversed()),
                    // When the string is supposed to be a color:
                    // Decide if we yield none, reset background or set color.
                    color_string => {
                        if color_string == "none" && col_fg {
                            None // fg:none yields no style.
                        } else {
                            // Either bg or valid color or both.
                            let parsed = parse_color_string(color_string);
                            // bg + invalid color = reset the background to default.
                            if !col_fg && parsed.is_none() {
                                Some(style.remove_bg())
                            } else {
                                // Valid color, apply color to either bg or fg
                                parsed.map(|ansi_color| {
                                    if col_fg {
                                        style.color(ansi_color)
                                    } else {
                                        style.on_color(ansi_color)
                                    }
                                })
                            }
                        }
                    }
                }
            })
        })
}

/// Parse a string that represents a predefined color name, returning None if this fails
fn parse_predefined_color(color_string: &str) -> Option<owo_colors::AnsiColors> {
    use owo_colors::AnsiColors::*;

    Some(match color_string.to_lowercase().as_str() {
        "black" => Black,
        "red" => Red,
        "green" => Green,
        "yellow" => Yellow,
        "blue" => Blue,
        "purple" => Magenta,
        "cyan" => Cyan,
        "white" => White,
        "bright-black" => BrightBlack, // "bright-black" is dark grey
        "bright-red" => BrightRed,
        "bright-green" => BrightGreen,
        "bright-yellow" => BrightYellow,
        "bright-blue" => BrightBlue,
        "bright-purple" => BrightMagenta,
        "bright-cyan" => BrightCyan,
        "bright-white" => BrightWhite,
        _ => return None,
    })
}

/** Parse a string that represents a color setting, returning None if this fails
 There are three valid color formats:
  - #RRGGBB      (a hash followed by an RGB hex)
  - u8           (a number from 0-255, representing an ANSI color)
  - colstring    (one of the 16 predefined color strings)
*/
fn parse_color_string(color_string: &str) -> Option<owo_colors::DynColors> {
    // Parse RGB hex values
    log::trace!("Parsing color_string: {}", color_string);
    if color_string.starts_with('#') {
        log::trace!(
            "Attempting to read hexadecimal color string: {}",
            color_string
        );
        if color_string.len() != 7 {
            log::debug!("Could not parse hexadecimal string: {}", color_string);
            return None;
        }
        let r: u8 = u8::from_str_radix(&color_string[1..3], 16).ok()?;
        let g: u8 = u8::from_str_radix(&color_string[3..5], 16).ok()?;
        let b: u8 = u8::from_str_radix(&color_string[5..7], 16).ok()?;
        log::trace!("Read RGB color string: {},{},{}", r, g, b);
        return Some(DynColors::Rgb(r, g, b));
    }

    // Parse a u8 (ansi 256-color)
    if let Result::Ok(ansi_color_num) = color_string.parse::<u8>() {
        log::trace!("Read ANSI color string: {}", ansi_color_num);
        return Some(DynColors::Xterm(ansi_color_num.into()));
    }

    // Check for any predefined color strings
    let predefined_color = parse_predefined_color(color_string).map(DynColors::Ansi);

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
    use owo_colors::XtermColors;
    use starship_module_config_derive::ModuleConfig;

    #[test]
    fn test_load_config() {
        #[derive(Clone, Default, ModuleConfig)]
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
        let mut rust_config = TestConfig {
            symbol: "S ",
            disabled: false,
            some_array: vec!["A", "B", "C"],
        };
        rust_config.load_config(&config);

        assert_eq!(rust_config.symbol, "T ");
        assert!(rust_config.disabled);
        assert_eq!(rust_config.some_array, vec!["A"]);
    }

    #[test]
    fn test_load_nested_config() {
        #[derive(Clone, Default, ModuleConfig)]
        struct TestConfig<'a> {
            pub untracked: SegmentDisplayConfig<'a>,
            pub modified: SegmentDisplayConfig<'a>,
        }

        #[derive(PartialEq, Debug, Clone, Default, ModuleConfig)]
        struct SegmentDisplayConfig<'a> {
            pub value: &'a str,
            pub style: Style,
        }

        let config = toml::toml! {
            untracked.value = "x"
            modified = { value = "∙", style = "red" }
        };

        let mut git_status_config = TestConfig {
            untracked: SegmentDisplayConfig {
                value: "?",
                style: Style::new().red().bold(),
            },
            modified: SegmentDisplayConfig {
                value: "!",
                style: Style::new().red().bold(),
            },
        };
        git_status_config.load_config(&config);

        assert_eq!(
            git_status_config.untracked,
            SegmentDisplayConfig {
                value: "x",
                style: Style::new().red().bold(),
            }
        );
        assert_eq!(
            git_status_config.modified,
            SegmentDisplayConfig {
                value: "∙",
                style: Style::new().red(),
            }
        );
    }

    #[test]
    fn test_load_optional_config() {
        #[derive(Clone, Default, ModuleConfig)]
        struct TestConfig<'a> {
            pub optional: Option<&'a str>,
            pub hidden: Option<&'a str>,
        }

        let config = toml::toml! {
            optional = "test"
        };
        let mut rust_config = TestConfig {
            optional: None,
            hidden: None,
        };
        rust_config.load_config(&config);

        assert_eq!(rust_config.optional, Some("test"));
        assert_eq!(rust_config.hidden, None);
    }

    #[test]
    fn test_load_enum_config() {
        #[derive(Clone, Default, ModuleConfig)]
        struct TestConfig {
            pub switch_a: Switch,
            pub switch_b: Switch,
            pub switch_c: Switch,
        }

        #[derive(Debug, PartialEq, Clone)]
        enum Switch {
            On,
            Off,
        }

        impl Default for Switch {
            fn default() -> Self {
                Self::Off
            }
        }

        impl<'a> ModuleConfig<'a> for Switch {
            fn from_config(config: &'a Value) -> Option<Self> {
                match config.as_str()? {
                    "on" => Some(Self::On),
                    "off" => Some(Self::Off),
                    _ => None,
                }
            }
        }

        let config = toml::toml! {
            switch_a = "on"
            switch_b = "any"
        };
        let mut rust_config = TestConfig {
            switch_a: Switch::Off,
            switch_b: Switch::Off,
            switch_c: Switch::Off,
        };
        rust_config.load_config(&config);

        assert_eq!(rust_config.switch_a, Switch::On);
        assert_eq!(rust_config.switch_b, Switch::Off);
        assert_eq!(rust_config.switch_c, Switch::Off);
    }

    #[test]
    fn test_from_string() {
        let config = Value::String(String::from("S"));
        assert_eq!(<&str>::from_config(&config).unwrap(), "S");
    }

    #[test]
    fn test_from_bool() {
        let config = Value::Boolean(true);
        assert!(<bool>::from_config(&config).unwrap());
    }

    #[test]
    fn test_from_i64() {
        let config = Value::Integer(42);
        assert_eq!(<i64>::from_config(&config).unwrap(), 42);
    }

    #[test]
    fn test_from_style() {
        let config = Value::from("red bold");
        assert_eq!(
            <Style>::from_config(&config).unwrap(),
            Style::new().red().bold()
        );
    }

    #[test]
    fn test_from_hex_color_style() {
        let config = Value::from("#00000");
        assert_eq!(<Style>::from_config(&config), None);

        let config = Value::from("#0000000");
        assert_eq!(<Style>::from_config(&config), None);

        let config = Value::from("#NOTHEX");
        assert_eq!(<Style>::from_config(&config), None);

        let config = Value::from("#a12BcD");
        assert_eq!(
            <Style>::from_config(&config).unwrap(),
            Style::new().fg_rgb::<0xA1, 0x2B, 0xCD>()
        );
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
    fn table_get_styles_bold_italic_underline_green_dimmed_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD");
        let mystyle = <Style>::from_config(&config).unwrap();
        assert_eq!(
            mystyle,
            Style::new().bold().italic().underline().dimmed().green()
        );
    }

    #[test]
    fn table_get_styles_bold_italic_underline_green_dimmed_inverted_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD InVeRTed");
        let mystyle = <Style>::from_config(&config).unwrap();
        assert_eq!(
            mystyle,
            Style::new()
                .bold()
                .italic()
                .underline()
                .dimmed()
                .reversed()
                .green()
        );
    }

    #[test]
    fn table_get_styles_plain_and_broken_styles() {
        // Test a "plain" style with no formatting
        let config = Value::from("");
        let plain_style = <Style>::from_config(&config).unwrap();
        assert_eq!(plain_style, Style::new());

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
    fn table_get_styles_with_none() {
        // Test that none on the end will result in None, overriding bg:none
        let config = Value::from("fg:red bg:none none");
        assert!(<Style>::from_config(&config).is_none());

        // Test that none in front will result in None, overriding bg:none
        let config = Value::from("none fg:red bg:none");
        assert!(<Style>::from_config(&config).is_none());

        // Test that none in the middle will result in None, overriding bg:none
        let config = Value::from("fg:red none bg:none");
        assert!(<Style>::from_config(&config).is_none());

        // Test that fg:none will result in None
        let config = Value::from("fg:none bg:black");
        assert!(<Style>::from_config(&config).is_none());

        // Test that bg:none will yield a style
        let config = Value::from("fg:red bg:none");
        assert_eq!(<Style>::from_config(&config).unwrap(), Style::new().red());

        // Test that bg:none will yield a style
        let config = Value::from("fg:red bg:none bold");
        assert_eq!(
            <Style>::from_config(&config).unwrap(),
            Style::new().red().bold()
        );

        // Test that bg:none will overwrite the previous background colour
        let config = Value::from("fg:red bg:green bold bg:none");
        assert_eq!(
            <Style>::from_config(&config).unwrap(),
            Style::new().red().bold()
        );
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
                .color(XtermColors::from(120))
                .bg_rgb::<5, 5, 5>()
        );

        // Test that the last color style is always the one used
        let config = Value::from("bg:120 bg:125 bg:127 fg:127 122 125");
        let multi_style = <Style>::from_config(&config).unwrap();
        assert_eq!(
            multi_style,
            Style::new()
                .color(XtermColors::from(125))
                .on_color(XtermColors::from(127))
        );
    }
}
