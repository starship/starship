use crate::config::Config;
use crate::segment::Segment;
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};
use std::fmt;

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module<'a> {
    /// The module's configuration map if available
    config: Option<&'a toml::value::Table>,

    /// The module's name, to be used in configuration and logging.
    name: String,

    /// The styling to be inherited by all segments contained within this module.
    style: Style,

    /// The prefix used to separate the current module from the previous one.
    prefix: Affix,

    /// The collection of segments that compose this module.
    segments: Vec<Segment>,

    /// The suffix used to separate the current module from the next one.
    suffix: Affix,
}

impl<'a> Module<'a> {
    /// Creates a module with no segments.
    pub fn new(name: &str, config: Option<&'a toml::value::Table>) -> Module<'a> {
        Module {
            config,
            name: name.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(name),
            segments: Vec::new(),
            suffix: Affix::default_suffix(name),
        }
    }

    /// Get a reference to a newly created segment in the module
    pub fn new_segment(&mut self, name: &str, value: &str) -> &mut Segment {
        let mut segment = Segment::new(name);
        segment.set_style(self.style);
        // Use the provided value unless overwritten by config
        segment.set_value(self.config_value_str(name).unwrap_or(value));
        self.segments.push(segment);

        self.segments.last_mut().unwrap()
    }

    /// Whether a module has any segments
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get the module's prefix
    pub fn get_prefix(&mut self) -> &mut Affix {
        &mut self.prefix
    }

    /// Get the module's suffix
    pub fn get_suffix(&mut self) -> &mut Affix {
        &mut self.suffix
    }

    /// Sets the style of the segment.
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut Module<'a>
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Returns a vector of colored ANSIString elements to be later used with
    /// `ANSIStrings()` to optimize ANSI codes
    pub fn ansi_strings(&self) -> Vec<ANSIString> {
        let shell = std::env::var("STARSHIP_SHELL").unwrap_or_default();
        let ansi_strings = self
            .segments
            .iter()
            .map(Segment::ansi_string)
            .collect::<Vec<ANSIString>>();

        let mut ansi_strings = match shell.as_str() {
            "bash" => ansi_strings_modified(ansi_strings, shell),
            "zsh" => ansi_strings_modified(ansi_strings, shell),
            _ => ansi_strings,
        };

        ansi_strings.insert(0, self.prefix.ansi_string());
        ansi_strings.push(self.suffix.ansi_string());

        ansi_strings
    }

    pub fn to_string_without_prefix(&self) -> String {
        ANSIStrings(&self.ansi_strings()[1..]).to_string()
    }

    /// Get a module's config value as a string
    pub fn config_value_str(&self, key: &str) -> Option<&str> {
        self.config.and_then(|config| config.get_as_str(key))
    }

    /// Get a module's config value as an int
    pub fn config_value_i64(&self, key: &str) -> Option<i64> {
        self.config.and_then(|config| config.get_as_i64(key))
    }

    /// Get a module's config value as a bool
    pub fn config_value_bool(&self, key: &str) -> Option<bool> {
        self.config.and_then(|config| config.get_as_bool(key))
    }
}

impl<'a> fmt::Display for Module<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi_strings = self.ansi_strings();
        write!(f, "{}", ANSIStrings(&ansi_strings))
    }
}

/// Many shells cannot deal with raw unprintable characters (like ANSI escape sequences) and
/// miscompute the cursor position as a result, leading to strange visual bugs. Here, we wrap these
/// characters in shell-specific escape codes to indicate to the shell that they are zero-length.
fn ansi_strings_modified(ansi_strings: Vec<ANSIString>, shell: String) -> Vec<ANSIString> {
    const ESCAPE_BEGIN: char = '\u{1b}';
    const MAYBE_ESCAPE_END: char = 'm';
    ansi_strings
        .iter()
        .map(|ansi| {
            let mut escaped = false;
            let final_string: String = ansi
                .to_string()
                .chars()
                .map(|x| match x {
                    ESCAPE_BEGIN => {
                        escaped = true;
                        match shell.as_str() {
                            "bash" => String::from("\u{5c}\u{5b}\u{1b}"), // => \[ESC
                            "zsh" => String::from("\u{25}\u{7b}\u{1b}"),  // => %{ESC
                            _ => x.to_string(),
                        }
                    }
                    MAYBE_ESCAPE_END => {
                        if escaped {
                            escaped = false;
                            match shell.as_str() {
                                "bash" => String::from("m\u{5c}\u{5d}"), // => m\]
                                "zsh" => String::from("m\u{25}\u{7d}"),  // => m%}
                                _ => x.to_string(),
                            }
                        } else {
                            x.to_string()
                        }
                    }
                    _ => x.to_string(),
                })
                .collect();
            ANSIString::from(final_string)
        })
        .collect::<Vec<ANSIString>>()
}

/// Module affixes are to be used for the prefix or suffix of a module.
pub struct Affix {
    /// The affix's name, to be used in configuration and logging.
    name: String,

    /// The affix's style.
    style: Style,

    /// The string value of the affix.
    value: String,
}

impl Affix {
    pub fn default_prefix(name: &str) -> Self {
        Self {
            name: format!("{}_prefix", name),
            style: Style::default(),
            value: "via ".to_string(),
        }
    }

    pub fn default_suffix(name: &str) -> Self {
        Self {
            name: format!("{}_suffix", name),
            style: Style::default(),
            value: " ".to_string(),
        }
    }

    /// Sets the style of the module.
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the value of the module.
    pub fn set_value<T>(&mut self, value: T) -> &mut Self
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

impl fmt::Display for Affix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ansi_string())
    }
}
