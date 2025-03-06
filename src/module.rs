use crate::segment;
use crate::segment::{FillSegment, Segment};
use nu_ansi_term::{AnsiString, AnsiStrings, Style as AnsiStyle};
use std::fmt;
use std::time::Duration;

// List of all modules
// Default ordering is handled in configs/starship_root.rs
pub const ALL_MODULES: &[&str] = &[
    "aws",
    "azure",
    #[cfg(feature = "battery")]
    "battery",
    "buf",
    "bun",
    "c",
    "character",
    "cmake",
    "cmd_duration",
    "cobol",
    "conda",
    "container",
    "crystal",
    "daml",
    "dart",
    "deno",
    "directory",
    "direnv",
    "docker_context",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "fennel",
    "fill",
    "fossil_branch",
    "fossil_metrics",
    "gcloud",
    "git_branch",
    "git_commit",
    "git_metrics",
    "git_state",
    "git_status",
    "gleam",
    "golang",
    "gradle",
    "guix_shell",
    "haskell",
    "haxe",
    "helm",
    "hg_branch",
    "hostname",
    "java",
    "jobs",
    "julia",
    "kotlin",
    "kubernetes",
    "line_break",
    "localip",
    "lua",
    "memory_usage",
    "meson",
    "mojo",
    "nats",
    "nim",
    "nix_shell",
    "nodejs",
    "ocaml",
    "odin",
    "opa",
    "openstack",
    "os",
    "package",
    "perl",
    "php",
    "pijul_channel",
    "pulumi",
    "purescript",
    "python",
    "quarto",
    "raku",
    "red",
    "rlang",
    "ruby",
    "rust",
    "scala",
    "shell",
    "shlvl",
    "singularity",
    "solidity",
    "spack",
    "status",
    "sudo",
    "swift",
    "terraform",
    "time",
    "typst",
    "username",
    "vagrant",
    "vcsh",
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
    pub fn new(name: &str, desc: &str, config: Option<&'a toml::Value>) -> Self {
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
            .all(|segment| segment.value().is_empty())
    }

    /// Get values of the module's segments
    pub fn get_segments(&self) -> Vec<&str> {
        self.segments.iter().map(segment::Segment::value).collect()
    }

    /// Returns a vector of colored `AnsiString` elements to be later used with
    /// `AnsiStrings()` to optimize ANSI codes
    pub fn ansi_strings(&self) -> Vec<AnsiString> {
        self.ansi_strings_for_width(None)
    }

    pub fn ansi_strings_for_width(&self, width: Option<usize>) -> Vec<AnsiString> {
        let mut iter = self.segments.iter().peekable();
        let mut ansi_strings: Vec<AnsiString> = Vec::new();
        while iter.peek().is_some() {
            ansi_strings.extend(ansi_line(&mut iter, width));
        }
        ansi_strings
    }
}

impl fmt::Display for Module<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi_strings = self.ansi_strings();
        write!(f, "{}", AnsiStrings(&ansi_strings))
    }
}

fn ansi_line<'a, I>(segments: &mut I, term_width: Option<usize>) -> Vec<AnsiString<'a>>
where
    I: Iterator<Item = &'a Segment>,
{
    let mut used = 0usize;
    let mut current: Vec<AnsiString> = Vec::new();
    let mut chunks: Vec<(Vec<AnsiString>, &FillSegment)> = Vec::new();
    let mut prev_style: Option<AnsiStyle> = None;

    for segment in segments {
        match segment {
            Segment::Fill(fs) => {
                chunks.push((current, fs));
                current = Vec::new();
                prev_style = None;
            }
            _ => {
                used += segment.width_graphemes();
                let current_segment_string = segment.ansi_string(prev_style.as_ref());

                prev_style = Some(*current_segment_string.style_ref());
                current.push(current_segment_string);
            }
        }

        if matches!(segment, Segment::LineTerm) {
            break;
        }
    }

    if chunks.is_empty() {
        current
    } else {
        let fill_size = term_width
            .and_then(|tw| if tw > used { Some(tw - used) } else { None })
            .map(|remaining| remaining / chunks.len());
        chunks
            .into_iter()
            .flat_map(|(strs, fill)| {
                let fill_string = fill.ansi_string(
                    fill_size,
                    strs.last().map(nu_ansi_term::AnsiGenericString::style_ref),
                );
                strs.into_iter().chain(std::iter::once(fill_string))
            })
            .chain(current)
            .collect::<Vec<AnsiString>>()
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
