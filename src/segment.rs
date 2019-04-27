use ansi_term::Style;
use std::fmt;

pub struct Segment {

    /// The string's name, to be used in configuration and logging.
    name: String,

    /// The segment's style. If None, will inherit the style of the module containing it.
    style: Style,

    /// The string value of the current segment.
    value: String
}

impl Segment {
    /// Creates a new segment with default fields
    pub fn new<T>(name: String) -> Segment {
        Segment {
            name: name,
            style: Style::default(),
            value: "".to_string(),
        }
    }

    /// Sets the style of the segment
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut Segment
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the value of the segment
    pub fn set_value<T>(&mut self, value: T) -> &mut Segment
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.style.paint(self.value))
    }
}
