pub mod painted;

use crate::segment;
use crate::segment::Segment;
use nu_ansi_term::AnsiString;
use painted::{Painted, TerminalWidth};
use std::fmt;
use std::time::Duration;

// List of all modules
// Default ordering is handled in configs/starship_root.rs
pub const ALL_MODULES: &[&str] = crate::modules::registry::ALL_MODULES;

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module<'a> {
    /// The module's configuration map if available
    pub config: Option<&'a toml::Value>,

    /// The module's name, to be used in configuration and logging.
    name: String,

    /// The module's description
    description: String,

    /// The collection of segments that compose this module.
    pub segments: Vec<Segment>,

    /// the time it took to compute this module
    pub duration: Duration,
}

impl<'a> Module<'a> {
    /// Creates a module with no segments.
    pub fn new(
        name: impl Into<String>,
        desc: impl Into<String>,
        config: Option<&'a toml::Value>,
    ) -> Self {
        Self {
            config,
            name: name.into(),
            description: desc.into(),
            segments: Vec::new(),
            duration: Duration::default(),
        }
    }

    /// Set segments in module
    pub fn set_segments(&mut self, segments: Vec<Segment>) {
        self.segments = segments;
    }

    /// Get module's name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Get module's description
    pub fn get_description(&self) -> &String {
        &self.description
    }

    /// Whether a module has non-empty segments
    pub fn is_empty(&self) -> bool {
        self.segments
            .iter()
            // no trim: if we add spaces/linebreaks it's not "empty" as we change the final output
            .all(|segment| segment.value().is_empty())
    }

    /// Get values of the module's segments
    pub fn get_segments(&self) -> Vec<&str> {
        self.segments.iter().map(segment::Segment::value).collect()
    }

    /// Resolves this module's segments into their final painted form, with
    /// fills left at their natural width.
    pub fn paint(&self) -> Painted {
        self.paint_for_width(None)
    }

    /// Resolves this module's segments into their final painted form, stretching
    /// fills to take up the width left over on each line.
    pub fn paint_for_width(&self, available_width: Option<TerminalWidth>) -> Painted {
        Painted::paint(&self.segments, available_width)
    }

    pub fn ansi_strings(&self) -> Vec<AnsiString<'static>> {
        self.paint()
            .runs()
            .iter()
            .map(|run| run.style().as_ansi_style().paint(run.text().to_owned()))
            .collect()
    }
}

impl fmt::Display for Module<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.paint())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_modules_is_in_alphabetical_order() {
        let mut sorted_modules: Vec<&str> = ALL_MODULES.to_vec();
        sorted_modules.sort_unstable();
        assert_eq!(sorted_modules.as_slice(), ALL_MODULES);
    }

    #[test]
    fn test_module_is_empty_with_no_segments() {
        let name = "unit_test";
        let desc = "This is a unit test";
        let module = Module {
            config: None,
            name: name.to_string(),
            description: desc.to_string(),
            segments: Vec::new(),
            duration: Duration::default(),
        };

        assert!(module.is_empty());
    }

    #[test]
    fn test_module_is_empty_with_all_empty_segments() {
        let name = "unit_test";
        let desc = "This is a unit test";
        let module = Module {
            config: None,
            name: name.to_string(),
            description: desc.to_string(),
            segments: Segment::from_text(None, ""),
            duration: Duration::default(),
        };

        assert!(module.is_empty());
    }

    #[test]
    fn test_module_is_not_empty_with_linebreak_only() {
        let name = "unit_test";
        let desc = "This is a unit test";
        let module = Module {
            config: None,
            name: name.to_string(),
            description: desc.to_string(),
            segments: Segment::from_text(None, "\n"),
            duration: Duration::default(),
        };

        assert!(!module.is_empty());
    }

    #[test]
    fn test_module_is_not_empty_with_space_only() {
        let name = "unit_test";
        let desc = "This is a unit test";
        let module = Module {
            config: None,
            name: name.to_string(),
            description: desc.to_string(),
            segments: Segment::from_text(None, " "),
            duration: Duration::default(),
        };

        assert!(!module.is_empty());
    }
}
