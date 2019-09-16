use ansi_term::{ANSIString, Style};
use std::fmt;

/// A segment is a single configurable element in a module. This will usually
/// contain a data point to provide context for the prompt's user
/// (e.g. The version that software is running).
pub struct Segment {
    /// The segment's name, to be used in configuration and logging.
    _name: String,

    /// The segment's style. If None, will inherit the style of the module containing it.
    style: Option<Style>,

    /// The string value of the current segment.
    value: String,
}

impl Segment {
    /// Creates a new segment with default fields.
    pub fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            style: None,
            value: "".to_string(),
        }
    }

    /// Sets the style of the segment.
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut Self
    where
        T: Into<Style>,
    {
        self.style = Some(style.into());
        self
    }

    /// Sets the value of the segment.
    pub fn set_value<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    // Returns the ANSIString of the segment value, not including its prefix and suffix
    pub fn ansi_string(&self) -> ANSIString {
        match self.style {
            Some(style) => style.paint(&self.value),
            None => ANSIString::from(&self.value),
        }
    }

    /// Determines if the segment contains a value.
    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ansi_string())
    }
}
