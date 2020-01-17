use crate::config::SegmentConfig;
use crate::context::Shell;
use crate::segment::Segment;
use crate::utils::wrap_colorseq_for_shell;
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};
use std::fmt;

// List of all modules
// Keep these ordered alphabetically.
// Default ordering is handled in configs/mod.rs
pub const ALL_MODULES: &[&str] = &[
    "aws",
    #[cfg(feature = "battery")]
    "battery",
    "character",
    "cmd_duration",
    "conda",
    "directory",
    "dotnet",
    "env_var",
    "git_branch",
    "git_commit",
    "git_state",
    "git_status",
    "golang",
    "haskell",
    "hg_branch",
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
    "php",
    "terraform",
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

    /// The module's description
    description: String,

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
    pub fn new(name: &str, desc: &str, config: Option<&'a toml::Value>) -> Module<'a> {
        Module {
            config,
            _name: name.to_string(),
            description: desc.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(name),
            segments: Vec::new(),
            suffix: Affix::default_suffix(name),
        }
    }

    /// Get a reference to a newly created segment in the module
    pub fn create_segment(&mut self, name: &str, segment_config: &SegmentConfig) -> &mut Segment {
        let mut segment = Segment::new(name);
        segment.set_style(segment_config.style.unwrap_or(self.style));
        segment.set_value(segment_config.value);
        self.segments.push(segment);

        self.segments.last_mut().unwrap()
    }

    /// Get module's name
    pub fn get_name(&self) -> &String {
        &self._name
    }

    /// Get module's description
    pub fn get_description(&self) -> &String {
        &self.description
    }

    /// Whether a module has non-empty segments
    pub fn is_empty(&self) -> bool {
        self.segments.iter().all(|segment| segment.is_empty())
    }

    pub fn get_segments(&self) -> Vec<&str> {
        self.segments.iter().map(Segment::get_value).collect()
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
        self.ansi_strings_for_shell(Shell::Unknown)
    }

    pub fn ansi_strings_for_shell(&self, shell: Shell) -> Vec<ANSIString> {
        let mut ansi_strings = self
            .segments
            .iter()
            .map(Segment::ansi_string)
            .collect::<Vec<ANSIString>>();

        ansi_strings.insert(0, self.prefix.ansi_string());
        ansi_strings.push(self.suffix.ansi_string());

        ansi_strings = match shell {
            Shell::Bash => ansi_strings_modified(ansi_strings, shell),
            Shell::Zsh => ansi_strings_modified(ansi_strings, shell),
            _ => ansi_strings,
        };

        ansi_strings
    }

    pub fn to_string_without_prefix(&self, shell: Shell) -> String {
        ANSIStrings(&self.ansi_strings_for_shell(shell)[1..]).to_string()
    }
}

impl<'a> fmt::Display for Module<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi_strings = self.ansi_strings();
        write!(f, "{}", ANSIStrings(&ansi_strings))
    }
}

fn ansi_strings_modified(ansi_strings: Vec<ANSIString>, shell: String) -> Vec<ANSIString> {
    ansi_strings
        .into_iter()
        .map(|ansi| {
            let wrapped = wrap_colorseq_for_shell(ansi.to_string(), &shell);
            ANSIString::from(wrapped)
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
        let desc = "This is a unit test";
        let module = Module {
            config: None,
            _name: name.to_string(),
            description: desc.to_string(),
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
        let desc = "This is a unit test";
        let module = Module {
            config: None,
            _name: name.to_string(),
            description: desc.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(name),
            segments: vec![Segment::new("test_segment")],
            suffix: Affix::default_suffix(name),
        };

        assert!(module.is_empty());
    }
}
