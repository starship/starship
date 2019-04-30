use ansi_term::{Style, ANSIString, ANSIStrings};
use std::fmt;

/// A segment is a single configurable element in a module. This will usually
/// contain a data point to provide context for the prompt's user
/// (e.g. The version that software is running).
pub struct Segment {
    /// The segment's name, to be used in configuration and logging.
    name: String,

    /// The segment's style. If None, will inherit the style of the module containing it.
    style: Option<Style>,

    /// The prefix used to preceed the contents of a segment.
    prefix: Option<SegmentAffix>,

    /// The string value of the current segment.
    value: String,

    /// The suffix used following the contents of a segment.
    suffix: Option<SegmentAffix>,
}

impl Segment {
    /// Creates a new segment with default fields.
    pub fn new(name: &str) -> Segment {
        Segment {
            name: name.to_string(),
            style: None,
            prefix: None,
            value: "".to_string(),
            suffix: None
        }
    }

    /// Sets the style of the segment.
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut Segment
    where
        T: Into<Style>,
    {
        self.style = Some(style.into());
        self
    }

    /// Sets the value of the segment.
    pub fn set_value<T>(&mut self, value: T) -> &mut Segment
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    /// Sets the segment prefix to the provided SegmentAffix.
    pub fn set_prefix(&mut self, mut prefix: SegmentAffix) {
        prefix.name = format!("{}_prefix", self.name);
        self.prefix = Some(prefix);
    }

    /// Sets the segment suffix to the provided SegmentAffix.
    pub fn set_suffix(&mut self, mut suffix: SegmentAffix) {
        suffix.name = format!("{}_suffix", self.name);
        self.suffix = Some(suffix);
    }

    // Returns the ANSIString of the segment value, not including its prefix and suffix
    fn value_ansi_string(&self) -> ANSIString {
        match self.style {
            Some(style) => style.paint(&self.value),
            None => ANSIString::from(&self.value),
        }
    } 

    /// Returns a vector of colored ANSIString elements to be later used with
    /// `ANSIStrings()` to optimize ANSI codes
    pub fn ansi_strings(&self) -> Vec<ANSIString> {
        let prefix = self.prefix.as_ref().and_then(|p| Some(p.ansi_string()));
        let suffix = self.suffix.as_ref().and_then(|s| Some(s.ansi_string()));
        let value = Some(self.value_ansi_string());

        // Remove `None` values from the vector
        vec!(
            prefix,
            value,
            suffix
        ).into_iter().filter_map(|e| e).collect::<Vec<ANSIString>>()
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi_strings = self.ansi_strings();
        write!(f, "{}", ANSIStrings(&ansi_strings))
    }
}

/// Segment affixes are to be used for the prefix or suffix of a segment.
/// By default they will inherit the styling of its segment, unless otherwise specified.
pub struct SegmentAffix {
    /// The affix's name, to be used in configuration and logging.
    name: String,

    /// The affix's style. If None, will inherit the style of the segment containing it.
    style: Option<Style>,

    /// The string value of the affix.
    value: String,
}

impl SegmentAffix {
    /// Creates a segment affix with no contents.
    pub fn new() -> SegmentAffix {
        SegmentAffix {
            name: String::new(),
            style: None,
            value: String::new()
        }
    }

    /// Generates the colored ANSIString output.
    pub fn ansi_string(&self) -> ANSIString {
        match self.style {
            Some(style) => style.paint(&self.value),
            None => ANSIString::from(&self.value),
        }
    }
}

impl fmt::Display for SegmentAffix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ansi_string())
    }
}
