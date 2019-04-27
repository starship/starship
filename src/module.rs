use ansi_term::Style;
use crate::segment::Segment;
use std::fmt;
use std::string::ToString;

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module {

    /// The module's name, to be used in configuration and logging.
    name: String,

    /// The string used to separate the current module from the previous one.
    prefix: Option<String>,

    /// The string used to separate the current module from the next one. 
    suffix: Option<String>,

    /// The styling to be inherited by all segments contained within this module.
    style: Style,
    
    /// The collection of segments that compose this module.
    segments: Vec<Segment>
}

impl Module {
    /// Creates a module with no segments.
    pub fn new(name: String) -> Module {
        Module {
            name,
            prefix: Some(" via".to_string()),
            suffix: Some(" ".to_string()),
            style: Style::default(),
            segments: Vec::new()
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self.segments.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("");
        
        write!(f, "{}", string)
    }
}
