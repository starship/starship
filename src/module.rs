use crate::context::Shell;
use crate::segment::Segment;
use crate::utils::wrap_colorseq_for_shell;
use ansi_term::{ANSIString, ANSIStrings};
use std::fmt;
use std::time::Duration;

// List of all modules
// Keep these ordered alphabetically.
// Default ordering is handled in configs/starship_root.rs
pub const ALL_MODULES: &[&str] = &[
    "aws",
    #[cfg(feature = "battery")]
    "battery",
    "character",
    "cmake",
    "cmd_duration",
    "conda",
    "dart",
    "deno",
    "directory",
    "docker_context",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "env_var",
    "gcloud",
    "git_branch",
    "git_commit",
    "git_state",
    "git_status",
    "golang",
    "helm",
    "hg_branch",
    "hostname",
    "java",
    #[cfg(target_os = "linux")]
    "linux_netns",
    "scala",
    "jobs",
    "julia",
    "kotlin",
    "kubernetes",
    "line_break",
    "lua",
    "memory_usage",
    "nim",
    "nix_shell",
    "nodejs",
    "ocaml",
    "openstack",
    "package",
    "perl",
    "purescript",
    "python",
    "rlang",
    "red",
    "ruby",
    "crystal",
    "rust",
    "php",
    "swift",
    "terraform",
    "shell",
    "shlvl",
    "singularity",
    "status",
    "time",
    "username",
    "vcsh",
    "vagrant",
    "vlang",
    "zig",
];

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
    pub fn new(name: &str, desc: &str, config: Option<&'a toml::Value>) -> Module<'a> {
        Module {
            config,
            name: name.to_string(),
            description: desc.to_string(),
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
            .all(|segment| segment.value.is_empty())
    }

    /// Get values of the module's segments
    pub fn get_segments(&self) -> Vec<&str> {
        self.segments
            .iter()
            .map(|segment| segment.value.as_str())
            .collect()
    }

    /// Returns a vector of colored ANSIString elements to be later used with
    /// `ANSIStrings()` to optimize ANSI codes
    pub fn ansi_strings(&self) -> Vec<ANSIString> {
        self.ansi_strings_for_shell(Shell::Unknown)
    }

    pub fn ansi_strings_for_shell(&self, shell: Shell) -> Vec<ANSIString> {
        let ansi_strings = self
            .segments
            .iter()
            .map(Segment::ansi_string)
            .collect::<Vec<ANSIString>>();

        match shell {
            Shell::Bash => ansi_strings_modified(ansi_strings, shell),
            Shell::Zsh => ansi_strings_modified(ansi_strings, shell),
            Shell::Tcsh => ansi_strings_modified(ansi_strings, shell),
            _ => ansi_strings,
        }
    }
}

impl<'a> fmt::Display for Module<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi_strings = self.ansi_strings();
        write!(f, "{}", ANSIStrings(&ansi_strings))
    }
}

fn ansi_strings_modified(ansi_strings: Vec<ANSIString>, shell: Shell) -> Vec<ANSIString> {
    ansi_strings
        .into_iter()
        .map(|ansi| {
            let wrapped = wrap_colorseq_for_shell(ansi.to_string(), shell);
            ANSIString::from(wrapped)
        })
        .collect::<Vec<ANSIString>>()
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
            segments: vec![Segment::new(None, "")],
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
            segments: vec![Segment::new(None, "\n")],
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
            segments: vec![Segment::new(None, " ")],
            duration: Duration::default(),
        };

        assert!(!module.is_empty());
    }
}
