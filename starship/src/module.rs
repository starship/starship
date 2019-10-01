use crate::config::{ModuleConfig, SegmentConfig};
use crate::segment::Segment;
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};
use std::fmt;

// List of all modules
pub const ALL_MODULES: &[&str] = &[
    "aws",
    #[cfg(feature = "battery")]
    "battery",
    "character",
    "cmd_duration",
    "directory",
    "env_var",
    "git_branch",
    "git_state",
    "git_status",
    "golang",
    "hostname",
    "java",
    "jobs",
    "kubernetes",
    "line_break",
    "memory_usage",
    "nix_shell",
    "nodejs",
    "package",
    "python",
    "ruby",
    "rust",
    "time",
    "username",
];

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module<'a> {
    /// The module's configuration map if available
    pub config: Option<&'a toml::Value>,

    /// The module's name, to be used in configuration and logging.
    _name: String,

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
    pub fn new(name: &str, config: Option<&'a toml::Value>) -> Module<'a> {
        Module {
            config,
            _name: name.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(name),
            segments: Vec::new(),
            suffix: Affix::default_suffix(name),
        }
    }

    /// Get a reference to a newly created segment in the module
    #[deprecated(
        since = "0.20.0",
        note = "please use `module.create_segment()` instead"
    )]
    pub fn new_segment(&mut self, name: &str, value: &str) -> &mut Segment {
        let mut segment = Segment::new(name);
        let segment_config_mock = SegmentConfig { value, style: None };

        if let Some(module_config) = self.config {
            let segment_config = segment_config_mock.load_config(&module_config);
            segment.set_style(segment_config.style.unwrap_or(self.style));
            segment.set_value(segment_config.value);
        } else {
            segment.set_style(segment_config_mock.style.unwrap_or(self.style));
            segment.set_value(segment_config_mock.value);
        }

        self.segments.push(segment);
        self.segments.last_mut().unwrap()
    }

    /// Get a reference to a newly created segment in the module
    pub fn create_segment(&mut self, name: &str, segment_config: &SegmentConfig) -> &mut Segment {
        let mut segment = Segment::new(name);
        segment.set_style(segment_config.style.unwrap_or(self.style));
        segment.set_value(segment_config.value);
        self.segments.push(segment);

        self.segments.last_mut().unwrap()
    }

    /// Should config exists, get a reference to a newly created segment in the module
    #[deprecated(
        since = "0.20.0",
        note = "please use `module.create_segment()` instead"
    )]
    pub fn new_segment_if_config_exists(&mut self, name: &str) -> Option<&mut Segment> {
        // Use the provided value unless overwritten by config
        if let Some(value) = self.config_value_str(name) {
            let mut segment = Segment::new(name);
            segment.set_style(self.style);
            segment.set_value(value);
            self.segments.push(segment);
            Some(self.segments.last_mut().unwrap())
        } else {
            None
        }
    }

    /// Whether a module has non-empty segments
    pub fn is_empty(&self) -> bool {
        self.segments.iter().all(|segment| segment.is_empty())
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
    #[deprecated(
        since = "0.20.0",
        note = "please use <RootModuleConfig>::try_load(module.config) instead"
    )]
    pub fn config_value_str(&self, key: &str) -> Option<&str> {
        <&str>::from_config(self.config?.as_table()?.get(key)?)
    }

    /// Get a module's config value as an int
    #[deprecated(
        since = "0.20.0",
        note = "please use <RootModuleConfig>::try_load(module.config) instead"
    )]
    pub fn config_value_i64(&self, key: &str) -> Option<i64> {
        <i64>::from_config(self.config?.as_table()?.get(key)?)
    }

    /// Get a module's config value as a bool
    #[deprecated(
        since = "0.20.0",
        note = "please use <RootModuleConfig>::try_load(module.config) instead"
    )]
    pub fn config_value_bool(&self, key: &str) -> Option<bool> {
        <bool>::from_config(self.config?.as_table()?.get(key)?)
    }

    /// Get a module's config value as a style
    #[deprecated(
        since = "0.20.0",
        note = "please use <RootModuleConfig>::try_load(module.config) instead"
    )]
    pub fn config_value_style(&self, key: &str) -> Option<Style> {
        <Style>::from_config(self.config?.as_table()?.get(key)?)
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
    _name: String,

    /// The affix's style.
    style: Style,

    /// The string value of the affix.
    value: String,
}

impl Affix {
    pub fn default_prefix(name: &str) -> Self {
        Self {
            _name: format!("{}_prefix", name),
            style: Style::default(),
            value: "via ".to_string(),
        }
    }

    pub fn default_suffix(name: &str) -> Self {
        Self {
            _name: format!("{}_suffix", name),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_is_empty_with_no_segments() {
        let name = "unit_test";
        let module = Module {
            config: None,
            _name: name.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(name),
            segments: Vec::new(),
            suffix: Affix::default_suffix(name),
        };

        assert!(module.is_empty());
    }

    #[test]
    fn test_module_is_empty_with_all_empty_segments() {
        let name = "unit_test";
        let module = Module {
            config: None,
            _name: name.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(name),
            segments: vec![Segment::new("test_segment")],
            suffix: Affix::default_suffix(name),
        };

        assert!(module.is_empty());
    }
}
