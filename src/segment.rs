use ansi_term::{ANSIString, Style};
use std::fmt;

/// A segment is a single configurable element in a module. This will usually
/// contain a data point to provide context for the prompt's user
/// (e.g. The version that software is running).
#[derive(Clone)]
pub struct Segment {
    /// The segment's style. If None, will inherit the style of the module containing it.
    pub style: Option<Style>,

    /// The string value of the current segment.
    pub value: String,
}

impl Segment {
    /// Creates a new segment.
    pub fn new<T>(style: Option<Style>, value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            style,
            value: value.into(),
        }
    }

    // Returns the ANSIString of the segment value, not including its prefix and suffix
    pub fn ansi_string(&self) -> ANSIString {
        match self.style {
            Some(style) => style.paint(&self.value),
            None => ANSIString::from(&self.value),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ansi_string())
    }
}
