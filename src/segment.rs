use ansi_term::Style;

#[derive(Clone)]
pub struct Segment {
    name: Option<String>,
    style: Style,
    value: String,
    prefix: BoxedSegment,
    suffix: BoxedSegment,
}

impl Segment {
    /// Creates a new segment with default fields
    pub fn new<T>(name: T) -> Segment
    where
        T: Into<String>,
        T: Copy,
    {
        let default_prefix = Some(Box::new(Segment {
            name: Some(format!("{} {}", name.into(), "prefix")),
            style: Style::default(),
            value: String::from("via "),
            prefix: None,
            suffix: None,
        }));

        let default_suffix = Some(Box::new(Segment {
            name: Some(format!("{} {}", name.into(), "suffix")),
            style: Style::default(),
            value: String::from(" "),
            prefix: None,
            suffix: None,
        }));

        Segment {
            name: Some(name.into()),
            style: Style::default(),
            value: String::from(""),
            prefix: default_prefix,
            suffix: default_suffix,
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

    /// Sets the prefix of the segment
    pub fn set_prefix(&mut self, prefix: BoxedSegment) -> &mut Segment {
        self.prefix = prefix;
        self
    }

    /// Sets the suffix of the segment
    pub fn set_suffix(&mut self, suffix: BoxedSegment) -> &mut Segment {
        self.suffix = suffix;
        self
    }

    /// Create a string with the formatted contents of a segment
    ///
    /// Will recursively also format the prefix and suffix of the segment being
    /// stringified. Skips the prefix of the first segment.
    pub fn output(&self, index: usize) -> String {
        let Segment {
            name: _name,
            prefix,
            value,
            style,
            suffix,
        } = self;

        let mut segment_string = String::new();

        // Skip the prefix for the first segment
        if index != 0 {
            if let Some(prefix) = prefix {
                segment_string += &prefix.output(index)
            }
        }

        segment_string += &style.paint(value).to_string();

        if let Some(suffix) = suffix {
            segment_string += &suffix.output(index);
        }

        segment_string
    }
}

type BoxedSegment = Option<Box<Segment>>;
