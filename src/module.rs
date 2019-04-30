use crate::segment::Segment;
use ansi_term::Style;
use std::fmt;
use std::string::ToString;
use ansi_term::{ANSIString, ANSIStrings};

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module {
    /// The module's name, to be used in configuration and logging.
    name: String,

    /// The styling to be inherited by all segments contained within this module.
    style: Style,

    /// The prefix used to separate the current module from the previous one.
    prefix: Option<ModuleAffix>,

    /// The collection of segments that compose this module.
    segments: Vec<Segment>,

    /// The suffix used to separate the current module from the next one.
    suffix: Option<ModuleAffix>,
}

impl Module {
    /// Creates a module with no segments.
    pub fn new(name: &String) -> Module {
        Module {
            name: name.to_string(),
            style: Style::default(),
            prefix: Some(ModuleAffix::default_prefix(name.to_string())),
            segments: Vec::new(),
            suffix: Some(ModuleAffix::default_suffix(name.to_string())),
        }
    }

    /// Returns a vector of colored ANSIString elements to be later used with
    /// `ANSIStrings()` to optimize ANSI codes
    pub fn ansi_strings(&self) -> Vec<ANSIString> {
        let mut ansi_strings = self.segments
            .iter()
            .map(|s| s.ansi_strings())
            .flat_map(|s| s.into_iter())
            .collect::<Vec<ANSIString>>();

        if let Some(prefix) = &self.prefix {
            &ansi_strings.insert(0, prefix.ansi_string());
        }

        if let Some(suffix) = &self.suffix {
            &ansi_strings.push(suffix.ansi_string());
        }

        ansi_strings
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi_strings = self.ansi_strings();
        write!(f, "{}", ANSIStrings(&ansi_strings))
    }
}

/// Module affixes are to be used for the prefix or suffix of a module.
struct ModuleAffix {
    /// The affix's name, to be used in configuration and logging.
    name: String,

    /// The affix's style.
    style: Style,

    /// The string value of the affix.
    value: String,
}

impl ModuleAffix {
    /// Creates a module affix with no contents.
    fn new() -> ModuleAffix {
        ModuleAffix {
            name: String::new(),
            style: Style::default(),
            value: String::new()
        }
    }

    pub fn default_prefix(name: String) -> ModuleAffix {
        ModuleAffix {
            name: format!("{}_prefix", name),
            style: Style::default(),
            value: "in ".to_string()
        }
    }

    pub fn default_suffix(name: String) -> ModuleAffix {
        ModuleAffix {
            name: format!("{}_suffix", name),
            style: Style::default(),
            value: " ".to_string()
        }
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
