use ansi_term::Style;

#[derive(Clone)]
pub struct Segment {
    name: Option<String>,
    style: Style,
    value: String,
    prefix: OptionalSegment,
    suffix: OptionalSegment,
}

impl Segment {
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

    pub fn set_style<'a, T>(&'a mut self, style: T) -> &'a mut Segment
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    pub fn set_value<'a, T>(&'a mut self, value: T) -> &'a mut Segment
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    pub fn set_prefix<'a>(&'a mut self, prefix: Segment) -> &'a mut Segment {
        self.prefix = Some(Box::new(prefix));
        self
    }

    pub fn set_suffix<'a>(&'a mut self, suffix: Segment) -> &'a mut Segment {
        self.suffix = Some(Box::new(suffix));
        self
    }

    /// Create a string with the formatted contents of a segment
    ///
    /// Will recursively also format the prefix and suffix of the segment being
    /// stringified.
    pub fn output<'a>(&'a self) -> String {
        let Segment {
            name: _name,
            prefix,
            value,
            style,
            suffix,
        } = self;

        let mut segment_string = String::new();

        if let Some(prefix) = prefix {
            segment_string += &prefix.output()
        }

        segment_string += &style.paint(value).to_string();

        if let Some(suffix) = suffix {
            segment_string += &suffix.output();
        }

        segment_string
    }
}

type OptionalSegment = Option<Box<Segment>>;
