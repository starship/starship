use crate::serde_utils::ValueDeserializer;
use crate::utils;
use ansi_term::Color;
use serde::{
    de::value::Error as ValueError, de::Error as SerdeError, Deserialize, Deserializer, Serialize,
};

use std::borrow::Cow;
use std::clone::Clone;
use std::io::ErrorKind;

use std::env;
use toml::Value;

/// Root config of a module.
pub trait ModuleConfig<'a, E>
where
    Self: Default,
    E: SerdeError,
{
    /// Construct a `ModuleConfig` from a toml value.
    fn from_config(config: &'a Value) -> Result<Self, E>;

    /// Loads the TOML value into the config.
    /// Missing values are set to their default values.
    /// On error, logs an error message.
    fn load(config: &'a Value) -> Self {
        match Self::from_config(config) {
            Ok(config) => config,
            Err(e) => {
                log::warn!("Failed to load config value: {}", e);
                Self::default()
            }
        }
    }

    /// Helper function that will call `ModuleConfig::from_config(config)  if config is Some,
    /// or `ModuleConfig::default()` if config is None.
    fn try_load(config: Option<&'a Value>) -> Self {
        config.map(Self::load).unwrap_or_default()
    }
}

impl<'a, T: Deserialize<'a> + Default> ModuleConfig<'a, ValueError> for T {
    /// Create `ValueDeserializer` wrapper and use it to call `Deserialize::deserialize` on it.
    fn from_config(config: &'a Value) -> Result<Self, ValueError> {
        let deserializer = ValueDeserializer::new(config);
        T::deserialize(deserializer)
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum Either<A, B> {
    First(A),
    Second(B),
}

/// A wrapper around `Vec<T>` that implements `ModuleConfig`, and either
/// accepts a value of type `T` or a list of values of type `T`.
#[derive(Clone, Default, Serialize)]
pub struct VecOr<T>(pub Vec<T>);

impl<'de, T> Deserialize<'de> for VecOr<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let either = Either::<Vec<T>, T>::deserialize(deserializer)?;
        match either {
            Either::First(v) => Ok(Self(v)),
            Either::Second(s) => Ok(Self(vec![s])),
        }
    }
}

#[cfg(feature = "config-schema")]
impl<T> schemars::JsonSchema for VecOr<T>
where
    T: schemars::JsonSchema + Sized,
{
    fn schema_name() -> String {
        Either::<T, Vec<T>>::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Either::<T, Vec<T>>::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        Either::<T, Vec<T>>::is_referenceable()
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
    /// Get the table of all the registered `env_var` modules, if any
    pub fn get_env_var_modules(&self) -> Option<&toml::value::Table> {
        self.get_config(&["env_var"])?.as_table()
    }
}

/// Deserialize a style string in the starship format with serde
pub fn deserialize_style<'de, D>(de: D) -> Result<ansi_term::Style, D::Error>
where
    D: Deserializer<'de>,
{
    Cow::<'_, str>::deserialize(de).and_then(|s| {
        parse_style_string(s.as_ref()).ok_or_else(|| D::Error::custom("Invalid style string"))
    })
}

/** Parse a style string which represents an ansi style. Valid tokens in the style
 string include the following:
 - 'fg:<color>'    (specifies that the color read should be a foreground color)
 - 'bg:<color>'    (specifies that the color read should be a background color)
 - 'underline'
 - 'bold'
 - 'italic'
 - 'inverted'
 - '<color>'       (see the `parse_color_string` doc for valid color strings)
*/
pub fn parse_style_string(style_string: &str) -> Option<ansi_term::Style> {
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
                    "inverted" => Some(style.reverse()),
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
                                let mut new_style = style;
                                new_style.background = Option::None;
                                Some(new_style)
                            } else {
                                // Valid color, apply color to either bg or fg
                                parsed.map(|ansi_color| {
                                    if col_fg {
                                        style.fg(ansi_color)
                                    } else {
                                        style.on(ansi_color)
                                    }
                                })
                            }
                        }
                    }
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
        if color_string.len() != 7 {
            log::debug!("Could not parse hexadecimal string: {}", color_string);
            return None;
        }
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
    use ansi_term::Style;

    // Small wrapper to allow deserializing Style without a struct with #[serde(deserialize_with=)]
    #[derive(Default, Clone, Debug, PartialEq)]
    struct StyleWrapper(Style);

    impl<'de> Deserialize<'de> for StyleWrapper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_style(deserializer).map(Self)
        }
    }

    #[test]
    fn test_load_config() {
        #[derive(Clone, Default, Deserialize)]
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
        let rust_config = TestConfig::from_config(&config).unwrap();

        assert_eq!(rust_config.symbol, "T ");
        assert!(rust_config.disabled);
        assert_eq!(rust_config.some_array, vec!["A"]);
    }

    #[test]
    fn test_load_nested_config() {
        #[derive(Clone, Default, Deserialize)]
        #[serde(default)]
        struct TestConfig<'a> {
            #[serde(borrow)]
            pub untracked: SegmentDisplayConfig<'a>,
            #[serde(borrow)]
            pub modified: SegmentDisplayConfig<'a>,
        }

        #[derive(PartialEq, Debug, Clone, Default, Deserialize)]
        #[serde(default)]
        struct SegmentDisplayConfig<'a> {
            pub value: &'a str,
            #[serde(deserialize_with = "deserialize_style")]
            pub style: Style,
        }

        let config = toml::toml! {
            untracked.value = "x"
            modified = { value = "∙", style = "red" }
        };

        let git_status_config = TestConfig::from_config(&config).unwrap();

        assert_eq!(
            git_status_config.untracked,
            SegmentDisplayConfig {
                value: "x",
                style: Style::default(),
            }
        );
        assert_eq!(
            git_status_config.modified,
            SegmentDisplayConfig {
                value: "∙",
                style: Color::Red.normal(),
            }
        );
    }

    #[test]
    fn test_load_optional_config() {
        #[derive(Clone, Default, Deserialize)]
        #[serde(default)]
        struct TestConfig<'a> {
            pub optional: Option<&'a str>,
            pub hidden: Option<&'a str>,
        }

        let config = toml::toml! {
            optional = "test"
        };
        let rust_config = TestConfig::from_config(&config).unwrap();

        assert_eq!(rust_config.optional, Some("test"));
        assert_eq!(rust_config.hidden, None);
    }

    #[test]
    fn test_load_enum_config() {
        #[derive(Clone, Default, Deserialize)]
        #[serde(default)]
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

        impl<'de> Deserialize<'de> for Switch {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                match s.to_ascii_lowercase().as_str() {
                    "on" => Ok(Self::On),
                    _ => Ok(Self::Off),
                }
            }
        }

        impl Default for Switch {
            fn default() -> Self {
                Self::Off
            }
        }

        let config = toml::toml! {
            switch_a = "on"
            switch_b = "any"
        };
        let rust_config = TestConfig::from_config(&config).unwrap();

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
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Red.bold()
        );
    }

    #[test]
    fn test_from_hex_color_style() {
        let config = Value::from("#00000");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        let config = Value::from("#0000000");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        let config = Value::from("#NOTHEX");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        let config = Value::from("#a12BcD");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::RGB(0xA1, 0x2B, 0xCD).into()
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
        let mystyle = <StyleWrapper>::from_config(&config).unwrap().0;
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
    fn table_get_styles_bold_italic_underline_green_dimmed_inverted_silly_caps() {
        let config = Value::from("bOlD ItAlIc uNdErLiNe GrEeN diMMeD InVeRTed");
        let mystyle = <StyleWrapper>::from_config(&config).unwrap().0;
        assert!(mystyle.is_bold);
        assert!(mystyle.is_italic);
        assert!(mystyle.is_underline);
        assert!(mystyle.is_dimmed);
        assert!(mystyle.is_reverse);
        assert_eq!(
            mystyle,
            ansi_term::Style::new()
                .bold()
                .italic()
                .underline()
                .dimmed()
                .reverse()
                .fg(Color::Green)
        );
    }

    #[test]
    fn table_get_styles_plain_and_broken_styles() {
        // Test a "plain" style with no formatting
        let config = Value::from("");
        let plain_style = <StyleWrapper>::from_config(&config).unwrap().0;
        assert_eq!(plain_style, ansi_term::Style::new());

        // Test a string that's clearly broken
        let config = Value::from("djklgfhjkldhlhk;j");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test a string that's nullified by `none`
        let config = Value::from("fg:red bg:green bold none");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test a string that's nullified by `none` at the start
        let config = Value::from("none fg:red bg:green bold");
        assert!(<StyleWrapper>::from_config(&config).is_err());
    }

    #[test]
    fn table_get_styles_with_none() {
        // Test that none on the end will result in None, overriding bg:none
        let config = Value::from("fg:red bg:none none");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test that none in front will result in None, overriding bg:none
        let config = Value::from("none fg:red bg:none");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test that none in the middle will result in None, overriding bg:none
        let config = Value::from("fg:red none bg:none");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test that fg:none will result in None
        let config = Value::from("fg:none bg:black");
        assert!(<StyleWrapper>::from_config(&config).is_err());

        // Test that bg:none will yield a style
        let config = Value::from("fg:red bg:none");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Red.normal()
        );

        // Test that bg:none will yield a style
        let config = Value::from("fg:red bg:none bold");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Red.bold()
        );

        // Test that bg:none will overwrite the previous background colour
        let config = Value::from("fg:red bg:green bold bg:none");
        assert_eq!(
            <StyleWrapper>::from_config(&config).unwrap().0,
            Color::Red.bold()
        );
    }

    #[test]
    fn table_get_styles_ordered() {
        // Test a background style with inverted order (also test hex + ANSI)
        let config = Value::from("bg:#050505 underline fg:120");
        let flipped_style = <StyleWrapper>::from_config(&config).unwrap().0;
        assert_eq!(
            flipped_style,
            Style::new()
                .underline()
                .fg(Color::Fixed(120))
                .on(Color::RGB(5, 5, 5))
        );

        // Test that the last color style is always the one used
        let config = Value::from("bg:120 bg:125 bg:127 fg:127 122 125");
        let multi_style = <StyleWrapper>::from_config(&config).unwrap().0;
        assert_eq!(
            multi_style,
            Style::new().fg(Color::Fixed(125)).on(Color::Fixed(127))
        );
    }
}
