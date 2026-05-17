use crate::config::StyleRefs;
use crate::segment;
use crate::segment::{FillSegment, Segment};
use nu_ansi_term::{AnsiString, AnsiStrings, Style as AnsiStyle};
use std::fmt;
use std::iter::Peekable;
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
    "claude_context",
    "claude_cost",
    "claude_model",
    "cmake",
    "cmd_duration",
    "cobol",
    "conda",
    "container",
    "cpp",
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
    "fortran",
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
    "hg_state",
    "hostname",
    "java",
    "jobs",
    "julia",
    "kotlin",
    "kubernetes",
    "line_break",
    "localip",
    "lua",
    "maven",
    "memory_usage",
    "meson",
    "mise",
    "mojo",
    "nats",
    "netns",
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
    "pixi",
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
    "vcs",
    "vcsh",
    "vlang",
    "xmake",
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

    /// Returns a vector of colored `AnsiString` elements to be later used with
    /// `AnsiStrings()` to optimize ANSI codes
    pub fn ansi_strings(&self) -> Vec<AnsiString<'_>> {
        self.ansi_strings_for_width(None)
    }

    pub fn ansi_strings_for_width(&self, width: Option<usize>) -> Vec<AnsiString<'_>> {
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

fn ansi_line<'a, I>(segments: &mut Peekable<I>, term_width: Option<usize>) -> Vec<AnsiString<'a>>
where
    I: Iterator<Item = &'a Segment>,
{
    let mut used = 0usize;
    let mut current: Vec<AnsiString> = Vec::new();
    let mut chunks: Vec<(Vec<AnsiString>, &FillSegment)> = Vec::new();
    let mut prev_style: Option<AnsiStyle> = None;

    while let Some(segment) = segments.next() {
        match segment {
            Segment::Fill(fs) => {
                chunks.push((current, fs));
                current = Vec::new();
                prev_style = None;
            }
            _ => {
                used += segment.width_graphemes();
                let next_style = segments.peek().and_then(|s| s.style());

                let style_refs = StyleRefs::new(prev_style, next_style);
                let current_segment_string = segment.ansi_string(Some(style_refs));

                prev_style = Some(*current_segment_string.style_ref());
                current.push(current_segment_string);
            }
        }

        if matches!(segment, Segment::LineTerm) {
            break;
        }
    }

    if chunks.is_empty() {
        return current;
    }

    let (fill_size, mut fill_remainder) = term_width
        .and_then(|tw| if tw > used { Some(tw - used) } else { None })
        .map(|remaining| (Some(remaining / chunks.len()), remaining % chunks.len()))
        .unwrap_or((None, 0));
    let mut fill_slack = 0usize;

    let mut chunk_iter = chunks.iter_mut().peekable();
    let mut output = Vec::new();

    while let Some((strs, fill)) = chunk_iter.next() {
        if fill_remainder > 0 {
            fill_remainder -= 1;
            fill_slack += 1;
        }

        let wanted_len = fill_size.map(|s| s + fill_slack);
        prev_style = strs.last().map(|s| *s.style_ref());
        let next_style = chunk_iter
            .peek()
            .and_then(|(s, _)| s.first().map(|s| *s.style_ref()))
            // For the last fill there is no next chunk; the trailing text is in `current`.
            .or_else(|| current.first().map(|s| *s.style_ref()));
        let style_refs = StyleRefs::new(prev_style, next_style);

        let (fill_string, actual_len) = fill.ansi_string_with_width(wanted_len, Some(style_refs));
        output.extend_from_slice(strs);
        output.extend_from_slice(&fill_string);

        fill_slack = wanted_len.map_or(0, |w| w.saturating_sub(actual_len));
    }
    // `current` stores the non-fill tail after the last fill segment.
    output.extend(current);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::parse_style_string;
    use crate::print::UnicodeWidthGraphemes;
    use nu_ansi_term::Color;

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

    #[test]
    fn test_fill_segment_resolves_next_style_from_trailing_text() {
        let mut module = Module::new("unit_test", "This is a unit test", None);
        module.set_segments(vec![
            Segment::from_text(parse_style_string("fg:red", None), "A")
                .into_iter()
                .next()
                .unwrap(),
            Segment::fill(parse_style_string("fg:next_bg", None), "."),
            Segment::from_text(parse_style_string("bg:blue", None), "B")
                .into_iter()
                .next()
                .unwrap(),
        ]);

        // width=6, "A"(1)+"B"(1)=2 used, fill gets 4 dots.
        // fill uses fg:next_bg; next segment has bg:blue → fill renders in blue fg.
        let rendered = module.ansi_strings_for_width(Some(6));
        let combined = AnsiStrings(&rendered).to_string();
        let expected = AnsiStrings(&[
            Color::Red.paint("A"),
            Color::Blue.paint("...."),
            nu_ansi_term::Style::new().on(Color::Blue).paint("B"),
        ])
        .to_string();
        assert_eq!(combined, expected);
    }

    #[test]
    fn test_fill_remainder_is_distributed_to_leading_fills() {
        let mut module = Module::new("unit_test", "This is a unit test", None);
        module.set_segments(vec![
            Segment::from_text(None, "L").into_iter().next().unwrap(),
            Segment::fill(None, "."),
            Segment::from_text(None, "M").into_iter().next().unwrap(),
            Segment::fill(None, "."),
            Segment::from_text(None, "R").into_iter().next().unwrap(),
        ]);

        // width=12, used=3, remaining=9, two fills → base_size=4, remainder=1.
        // fill1 gets the extra cell first: 5; fill2 gets 4.
        let rendered = module.ansi_strings_for_width(Some(12));
        let combined = AnsiStrings(&rendered).to_string();
        assert_eq!(combined, "L.....M....R");
        assert_eq!(combined.width_graphemes(), 12);
    }

    #[test]
    fn test_fill_slack_is_carried_to_following_fill_segments() {
        let mut module = Module::new("unit_test", "This is a unit test", None);
        module.set_segments(vec![
            Segment::from_text(None, "L").into_iter().next().unwrap(),
            Segment::fill(None, "🟦"),
            Segment::from_text(None, "M").into_iter().next().unwrap(),
            Segment::fill(None, "🟦"),
            Segment::from_text(None, "R").into_iter().next().unwrap(),
        ]);

        // width=10, "L"+"M"+"R"=3 used, remaining=7, two fills → fill_size=3, fill_remainder=1.
        // fill1 gets the extra remainder cell: wants 4 → 🟦🟦 (4).
        // fill2 gets base only: wants 3 → 🟦 (2 used, 1 wasted at end).
        // Result: "L🟦🟦M🟦R" with total width 9.
        let rendered = module.ansi_strings_for_width(Some(10));
        let combined = AnsiStrings(&rendered).to_string();
        assert_eq!(combined, "L🟦🟦M🟦R");
        assert_eq!(combined.width_graphemes(), 9);
    }
}
