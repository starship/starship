use crate::segment::Segment;
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};
use std::fmt;
use std::string::ToString;

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module {
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

impl Module {
    /// Creates a module with no segments.
    pub fn new(name: &str) -> Module {
        Module {
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
        segment.set_value(value.into());
        self.segments.push(segment);

        self.segments.last_mut().unwrap()
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
    pub fn set_style<T>(&mut self, style: T) -> &mut Module
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
            .map(|s| s.ansi_strings())
            .flat_map(|s| s.into_iter())
            .collect::<Vec<ANSIString>>();

        ansi_strings.insert(0, self.prefix.ansi_string());
        ansi_strings.push(self.suffix.ansi_string());

        ansi_strings
    }

    pub fn to_string_without_prefix(&self) -> String {
        ANSIStrings(&self.ansi_strings()[1..]).to_string()
    }
}

impl fmt::Display for Module {
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
