use crate::segment::Segment;
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};
use std::fmt;
use std::string::ToString;

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module<'a> {
    /// The module's configuration map if available
    config: Option<&'a toml::value::Table>,

    /// The module's name, to be used in configuration and logging.
    name: String,

    /// The styling to be inherited by all segments contained within this module.
    style: Style,

    /// The prefix used to separate the current module from the previous one.
    prefix: ModuleAffix,

    /// The collection of segments that compose this module.
    segments: Vec<Segment>,

    /// The suffix used to separate the current module from the next one.
    suffix: ModuleAffix,
}

impl<'a> Module<'a> {
    /// Creates a module with no segments.
    pub fn new(name: &str, config: Option<&'a toml::value::Table>) -> Module<'a> {
        Module {
            config,
            name: name.to_string(),
            style: Style::default(),
            prefix: ModuleAffix::default_prefix(name.to_string()),
            segments: Vec::new(),
            suffix: ModuleAffix::default_suffix(name.to_string()),
        }
    }

    /// Get a reference to a newly created segment in the module
    pub fn new_segment<T>(&mut self, name: &str, value: T) -> &mut Segment
    where
        T: Into<String>,
    {
        let mut segment = Segment::new(name);
        segment.set_style(self.style);
        // Use the provided value unless overwritten by config
        segment.set_value(self.config_value(name).unwrap_or_else(|| value.into()));
        self.segments.push(segment);

        self.segments.last_mut().unwrap()
    }

    /// Whether a module has any segments
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get the module's prefix
    pub fn get_prefix(&mut self) -> &mut ModuleAffix {
        &mut self.prefix
    }

    /// Get the module's suffix
    pub fn get_suffix(&mut self) -> &mut ModuleAffix {
        &mut self.suffix
    }

    /// Sets the style of the segment.
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut Module<'a>
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Returns a vector of colored ANSIString elements to be later used with
    /// `ANSIStrings()` to optimize ANSI codes
    pub fn ansi_strings(&self) -> Vec<ANSIString> {
        let mut ansi_strings = self
            .segments
            .iter()
            .map(|s| s.ansi_string())
            .collect::<Vec<ANSIString>>();

        ansi_strings.insert(0, self.prefix.ansi_string());
        ansi_strings.push(self.suffix.ansi_string());

        ansi_strings
    }

    pub fn to_string_without_prefix(&self) -> String {
        ANSIStrings(&self.ansi_strings()[1..]).to_string()
    }

    /// Get a module's config value as a string
    fn config_value(&self, key: &str) -> Option<String> {
        self.config
            // Find the config value by its key
            .map(|config| config.get(key)).unwrap_or(None)
            // Get the config value as a `&str`
            .map(toml::Value::as_str).unwrap_or(None)
            // Convert it to a String
            .map(str::to_string)
    }
}

impl<'a> fmt::Display for Module<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi_strings = self.ansi_strings();
        write!(f, "{}", ANSIStrings(&ansi_strings))
    }
}

/// Module affixes are to be used for the prefix or suffix of a module.
pub struct ModuleAffix {
    /// The affix's name, to be used in configuration and logging.
    name: String,

    /// The affix's style.
    style: Style,

    /// The string value of the affix.
    value: String,
}

impl ModuleAffix {
    pub fn default_prefix(name: String) -> ModuleAffix {
        ModuleAffix {
            name: format!("{}_prefix", name),
            style: Style::default(),
            value: "via ".to_string(),
        }
    }

    pub fn default_suffix(name: String) -> ModuleAffix {
        ModuleAffix {
            name: format!("{}_suffix", name),
            style: Style::default(),
            value: " ".to_string(),
        }
    }

    /// Sets the style of the module.
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut ModuleAffix
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the value of the module.
    pub fn set_value<T>(&mut self, value: T) -> &mut ModuleAffix
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    /// Generates the colored ANSIString output.
    pub fn ansi_string(&self) -> ANSIString {
        self.style.paint(&self.value)
    }
}

impl fmt::Display for ModuleAffix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ansi_string())
    }
}
