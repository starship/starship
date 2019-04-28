use crate::segment::Segment;
use ansi_term::Style;
use std::fmt;
use std::string::ToString;

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module {
    /// The module's name, to be used in configuration and logging.
    name: String,

    /// The styling to be inherited by all segments contained within this module.
    style: Style,

    /// The string used to separate the current module from the previous one.
    prefix: Option<String>,

    /// The collection of segments that compose this module.
    segments: Vec<Segment>,

    /// The string used to separate the current module from the next one.
    suffix: Option<String>,
}

impl Module {
    /// Creates a module with no segments.
    pub fn new(name: String) -> Module {
        Module {
            name,
            style: Style::default(),
            prefix: Some(" via".to_string()),
            segments: Vec::new(),
            suffix: Some(" ".to_string()),
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Concatenate the string representations of all segments
        let prefix = self.prefix.unwrap_or("".to_string());
        let suffix = self.suffix.unwrap_or("".to_string());
        let segments = self
            .segments
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("");

        let painted_output = self.style.paint("{}{}{}", prefix, segments, suffix);

        write!(f, "{}{}{}", painted_output)
    }
}
