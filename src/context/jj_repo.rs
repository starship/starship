use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;

use super::Context;

/// A Jujutsu (JJ) repository's data.
pub struct JJRepo {
    /// Repository root, passed as `--repository` to the JJ CLI
    root: PathBuf,
    current_change: OnceLock<Option<CurrentChange>>,
}

#[derive(Debug)]
pub struct CurrentChange {
    /// JJ change ID
    ///
    /// Checked to be ASCII at construction
    pub change: Box<str>,
    /// Size of the shortest unique prefix for the change ID
    pub change_shortest: u8,
    /// Underlying VCS commit ID (most often Git)
    ///
    /// Checked to be ASCII at construction
    pub commit: Box<str>,
    /// Size of the shortest unique prefix for the commit ID
    pub commit_shortest: u8,

    /// `Some(_)` only if the change is [divergent][1]
    ///
    /// [1]: https://docs.jj-vcs.dev/latest/glossary/#divergent-change
    pub change_offset: Option<u16>,

    /// Bookmarks to consider for the current change, if any
    pub bookmarks: Box<[Bookmark]>,

    /// Total lines added in this change
    pub lines_added: u32,
    /// Total lines removed in this change
    pub lines_removed: u32,

    pub status: Status,
}

#[derive(Debug, Default)]
pub struct Status {
    /// See `impl CurrentChange` below
    flags: u8,

    /// Count of added files
    pub added: usize,
    /// Count of copied files
    pub copied: usize,
    /// Count of deleted files
    pub deleted: usize,
    /// Count of modified files
    pub modified: usize,
    /// Count of renamed files
    pub renamed: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bookmark(pub Box<str>);

impl JJRepo {
    /// Discover if we're in a JJ repo by looking for a `.jj` directory in the current directory
    /// and its parents.
    ///
    /// While this is technically more error prone than `jj workspace root`, it is also much faster
    /// and subsequent JJ commands will fail fast if it's not actually a JJ repo.
    pub fn discover(context: &Context) -> Option<Self> {
        context
            .begin_ancestor_scan()
            .set_folders(&[".jj"])
            .scan()
            .map(Self::with_root)
    }

    pub fn with_root(root: PathBuf) -> Self {
        // NOTE: we don't compute anything by default, gating everything behind `OnceLock`s
        //       to avoid running `jj` commands for nothing if the information is never asked for.
        Self {
            root,
            current_change: OnceLock::new(),
        }
    }

    /// Root directory of the Jujutsu repository
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Information about the current change's state.
    pub fn current_change(&self, context: &Context) -> Option<&CurrentChange> {
        self.current_change
            .get_or_init(|| {
                let res = context.exec_cmd("jj", &[
                    "--color".as_ref(),
                    "never".as_ref(),
                    // Will search for diffs and such normally but not update the underlying repository
                    "--no-integrate-operation".as_ref(),
                    "--no-pager".as_ref(),
                    "--quiet".as_ref(),
                    "--repository".as_ref(),
                    self.root.as_os_str(),
                    "show".as_ref(),
                    // Finds:
                    //
                    // 1. current working copy
                    // 2. closest conflict amongst self & mutable parents (if any)
                    // 3. closest bookmark
                    //
                    // Note they can all resolve to the same actual change or to two changes only,
                    // that's perfectly fine
                    //
                    // Parent information can:
                    //
                    // - Be missing if `@` has both a conflict and bookmarks
                    // - Come from one parent if it has both a conflict and bookmarks
                    // - Come from two parents if one has a conflict and the other bookmarks
                    //
                    // In any case, there will never be two parents with bookmarks or two parents
                    // with a conflict thanks to this revset
                    "@ | latest(heads(mutable() & ::@ & conflicts()), 1) | latest(bookmarks() & ::@, 1)".as_ref(),
                    "--no-patch".as_ref(),
                    "--template".as_ref(),
                    r#"if(
                        current_working_copy,
                        join(
                            "\n",
                            concat("current.has_description = ", description.len() > 0),
                            concat("current.hidden = ", hidden),
                            concat("current.immutable = ", immutable),
                            concat("current.divergent = ", divergent),
                            concat("current.change_offset = ", change_offset),
                            concat("current.change_id = \"", change_id, "\""),
                            concat("current.change_id_prefix_len = ", change_id.shortest().prefix().len()),
                            concat("current.commit_id = \"", commit_id, "\""),
                            concat("current.commit_id_prefix_len = ", commit_id.shortest().prefix().len()),
                            concat("current.lines_added = ", diff.stat().total_added()),
                            concat("current.lines_removed = ", diff.stat().total_removed()),
                            concat("current.conflicted = ", conflict),
                            concat("current.bookmarks = [", bookmarks.map(|bm| concat("\"", bm, "\"")).join(","), "]"),
                            concat(
                                "current.statuses = \"",
                                self
                                    .diff(".")
                                    .files()
                                    .map(|file| file.status_char())
                                    .join(""),
                                "\"",
                            ),
                            "",
                        ),
                        concat(
                            if(
                                conflict,
                                "parents.conflicted = true\n",
                            ),
                            if(
                                bookmarks.len() > 0,
                                concat("parents.bookmarks = [", bookmarks.map(|bm| concat("\"", bm, "\"")).join(","), "]\n"),
                            ),
                        ),
                    )"#.as_ref(),
                ])?;

                #[derive(Deserialize)]
                struct RawOutput<'o> {
                    #[serde(borrow)]
                    current: RawCurrentChange<'o>,
                    #[serde(default, borrow)]
                    parents: RawParents<'o>,
                }

                #[derive(Deserialize)]
                struct RawCurrentChange<'o> {
                    has_description: bool,
                    hidden: bool,
                    immutable: bool,
                    divergent: bool,
                    change_offset: u16,
                    change_id: &'o str,
                    change_id_prefix_len: u8,
                    commit_id: &'o str,
                    commit_id_prefix_len: u8,
                    lines_added: u32,
                    lines_removed: u32,
                    statuses: &'o str,
                    #[serde(default)]
                    conflicted: bool,
                    #[serde(default, borrow)]
                    bookmarks: Box<[&'o str]>,
                }

                #[derive(Default, Deserialize)]
                struct RawParents<'o> {
                    #[serde(default)]
                    conflicted: bool,
                    #[serde(default, borrow)]
                    bookmarks: Box<[&'o str]>,
                }

                let RawOutput { current, parents } = toml::from_str(&res.stdout).ok()?;

                if current.change_id.len() != 32 || !current.change_id.is_ascii() {
                    return None;
                }

                Some(
                    CurrentChange {
                        change: Box::from(current.change_id),
                        change_shortest: current.change_id_prefix_len,

                        commit: Box::from(current.commit_id),
                        commit_shortest: current.commit_id_prefix_len,

                        change_offset: current.divergent.then_some(current.change_offset),

                        lines_added: current.lines_added,
                        lines_removed: current.lines_removed,

                        bookmarks: current.bookmarks.iter().chain(parents.bookmarks.iter()).map(|&s| Bookmark(Box::from(s))).collect(),

                        status: {
                            let mut status = Status::default();

                            if current.conflicted || parents.conflicted {
                                status.flags |= CurrentChange::CONFLICTED;
                            }
                            if current.has_description {
                                status.flags |= CurrentChange::DESCRIPTION;
                            }
                            if current.hidden {
                                status.flags |= CurrentChange::HIDDEN;
                            }
                            if current.immutable {
                                status.flags |= CurrentChange::IMMUTABLE;
                            }

                            // JJ documents the characters it will return, those that interest us are
                            // all single bytes so we don't need to do the u8 -> char conversion
                            for byte in current.statuses.bytes() {
                                status.added += usize::from(byte == b'A');
                                status.copied += usize::from(byte == b'C');
                                status.deleted += usize::from(byte == b'D');
                                status.modified += usize::from(byte == b'M');
                                status.renamed += usize::from(byte == b'R');
                            }

                            status
                        }
                    }
                )
            })
            .as_ref()
    }
}

impl CurrentChange {
    const CONFLICTED: u8 = 1 << 0;
    const DESCRIPTION: u8 = 1 << 1;
    const HIDDEN: u8 = 1 << 2;
    const IMMUTABLE: u8 = 1 << 3;

    /// True if any mutable change up to the current one is conflicted
    pub fn conflicted(&self) -> bool {
        self.status.flags & Self::CONFLICTED == Self::CONFLICTED
    }

    /// True if the current change has a non-empty description
    pub fn description(&self) -> bool {
        self.status.flags & Self::DESCRIPTION == Self::DESCRIPTION
    }

    /// True if the current change is hidden
    pub fn hidden(&self) -> bool {
        self.status.flags & Self::HIDDEN == Self::HIDDEN
    }

    /// True if the current change is immutable
    pub fn immutable(&self) -> bool {
        self.status.flags & Self::IMMUTABLE == Self::IMMUTABLE
    }
}

impl Bookmark {
    /// False when the bookmark is up-to-date, meaning it's either:
    /// - Local, so logically up-to-date with itself
    /// - Remote, in which case the comparison is made with the last fetched state
    pub fn diverged(&self) -> bool {
        self.0.ends_with('*')
    }

    /// Name of the bookmark
    pub fn name(&self) -> &str {
        let no_star = self.0.trim_end_matches('*');
        no_star.split_once('@').map_or(no_star, |raw| raw.0)
    }

    /// Remote of the bookmark, if any
    pub fn remote(&self) -> Option<&str> {
        let no_star = self.0.trim_end_matches('*');
        no_star.split_once('@').map(|raw| raw.1)
    }
}

#[cfg(test)]
impl JJRepo {
    pub const BASE: &str = "/jj/base";
    pub const EMPTY_OUTPUT: &str = "/jj/empty-output";
    pub const INVALID_OUTPUT: &str = "/jj/invalid-output";

    pub const BOOKMARKS_NONE: &str = "/jj/bookmarks/no-current-no-parent";
    pub const BOOKMARKS_IN_PARENT: &str = "/jj/bookmarks/in-parent";

    pub const CHANGE_DIVERGENT: &str = "/jj/change/divergent";
    pub const CHANGE_NOT_ASCII: &str = "/jj/change/not-ascii";

    pub const METRIC_ADDED: &str = "/jj/metrics-added";
    pub const METRIC_DELETED: &str = "/jj/metrics-deleted";
    pub const METRIC_ZERO: &str = "/jj/metrics-zero";

    pub const STATUS_CONFLICTED_NONE: &str = "/jj/status/conflicted-none";
    pub const STATUS_CONFLICTED_CURRENT: &str = "/jj/status/conflicted-current";
    pub const STATUS_CONFLICTED_PARENT: &str = "/jj/status/conflicted-parent";

    pub const STATUS_DESCRIPTION: &str = "/jj/status/description";
    pub const STATUS_HIDDEN: &str = "/jj/status/hidden";
    pub const STATUS_IMMUTABLE: &str = "/jj/status/immutable";

    pub const STATUS_ADDED: &str = "/jj/status/added";
    pub const STATUS_COPIED: &str = "/jj/status/copied";
    pub const STATUS_DELETED: &str = "/jj/status/deleted";
    pub const STATUS_MODIFIED: &str = "/jj/status/modified";
    pub const STATUS_RENAMED: &str = "/jj/status/renamed";
    pub const STATUS_NO_CHANGES: &str = "/jj/status/no-changes";

    pub const NONE: &str = "/jj/no-repo";
}

/// Helper function to generate mock command outputs for JJ module tests
#[cfg(test)]
pub fn mock_jj_cmd(s: &str) -> Option<crate::utils::CommandOutput> {
    use crate::utils::CommandOutput;

    // Constants to avoid magic numbers in `let outputs` and allow changing the indexes
    // without having to rewrite the whole array every time.
    // We allow unused for all of them to avoid having to add/remove/rename in commits all over.
    #[allow(unused)]
    const HAS_DESCRIPTION: usize = 0;
    #[allow(unused)]
    const HIDDEN: usize = 1;
    #[allow(unused)]
    const IMMUTABLE: usize = 2;
    #[allow(unused)]
    const DIVERGENT: usize = 3;
    #[allow(unused)]
    const CHANGE_OFFSET: usize = 4;

    #[allow(unused)]
    const CHANGE: usize = 5;
    #[allow(unused)]
    const CHANGE_SHORT_LENGTH: usize = 6;
    #[allow(unused)]
    const COMMIT: usize = 7;
    #[allow(unused)]
    const COMMIT_SHORT_LENGTH: usize = 8;

    #[allow(unused)]
    const LINES_ADDED: usize = 9;
    #[allow(unused)]
    const LINES_REMOVED: usize = 10;

    #[allow(unused)]
    const STATUSES: usize = 11;

    #[allow(unused)]
    const CURRENT_CONFLICTED: usize = 12;
    #[allow(unused)]
    const PARENT_CONFLICTED: usize = 14;

    #[allow(unused)]
    const CURRENT_BOOKMARKS: usize = 13;
    #[allow(unused)]
    const PARENT_BOOKMARKS: usize = 15;

    /// Generate output for JJ while allowing easy replacement of lines to test various valid
    /// possibilities
    fn output<const N: usize>(mods: [(usize, &str); N]) -> Option<CommandOutput> {
        let mut stdout = [
            // -- Current --
            // -- 0
            "current.has_description = false",
            "current.hidden = false",
            "current.immutable = false",
            "current.divergent = false",
            "current.change_offset = 0",
            // -- 5
            "current.change_id = \"pvtxwmvtttmrkkoqkutlystlnozssmnk\"",
            "current.change_id_prefix_len = 3",
            "current.commit_id = \"30363e463b3a5c87ad352b2d342f7408e3c2dda8\"",
            "current.commit_id_prefix_len = 4",
            "current.lines_added = 100",
            // -- 10
            "current.lines_removed = 90",
            "current.statuses = \"ACDMR\"",
            "current.conflicted = false",
            r#"current.bookmarks = ["cur_local", "cur_tracked*", "cur_modified@upstream*", "cur_untracked@origin"]"#,
            // -- Parents, both values are optional --
            "parents.conflicted = true",
            // -- 15
            // r#"parents.bookmarks = ["par_local", "par_tracked*", "par_modified@upstream*", "par_untracked@origin"]"#,
            "",
        ];

        for (index, replacement) in mods {
            stdout[index] = replacement;
        }

        Some(CommandOutput {
            stdout: stdout.as_ref().join("\n"),
            stderr: String::new(),
        })
    }

    if !s.contains("show @") {
        panic!("Found non-mocked JJ command: {s}");
    }

    // Voluntarily unformatted to allow easy modifications that don't conflict all the time
    // and to make each outputs formatted the same for easier reading
    #[rustfmt::skip]
    #[expect(clippy::type_complexity)]
    let outputs: [(_, fn() -> Option<CommandOutput>); _] = [
        (
            JJRepo::BASE,
            || output([])
        ),
        // Repos testing jj_bookmark rendering
        (
            JJRepo::BOOKMARKS_NONE,
            || output([
                (CURRENT_BOOKMARKS, "current.bookmarks = []"),
            ]),
        ),
        (
            JJRepo::BOOKMARKS_IN_PARENT,
            || output([
                (CURRENT_BOOKMARKS, "current.bookmarks = []"),
                (PARENT_BOOKMARKS, r#"parents.bookmarks = ["par_local", "par_tracked*", "par_modified@upstream*", "par_untracked@origin"]"#),
            ]),
        ),
        // Repos testing jj_change rendering
        (
            JJRepo::CHANGE_DIVERGENT,
            || output([
                (DIVERGENT, "current.divergent = true"),
                (CHANGE_OFFSET, "current.change_offset = 2"),
            ]),
        ),
        (
            JJRepo::CHANGE_NOT_ASCII,
            || output([
                (CHANGE, "current.change_id = \"Étxwmvtttmrkkoqkutlystlnozssmnk\""),
            ]),
        ),
        // Repos testing jj_metrics rendering
        (
            JJRepo::METRIC_ADDED,
            || output([
                (LINES_REMOVED, "current.lines_removed = 0"),
            ]),
        ),
        (
            JJRepo::METRIC_DELETED,
            || output([
                (LINES_ADDED, "current.lines_added = 0")
            ]),
        ),
        (
            JJRepo::METRIC_ZERO,
            || output([
                (LINES_ADDED, "current.lines_added = 0"),
                (LINES_REMOVED, "current.lines_removed = 0"),
            ]),
        ),
        // Repos testing jj_status rendering
        (
            JJRepo::STATUS_CONFLICTED_NONE,
            || output([
                (CURRENT_CONFLICTED, "current.conflicted = false"),
                (PARENT_CONFLICTED, ""),
            ]),
        ),
        (
            JJRepo::STATUS_CONFLICTED_CURRENT,
            || output([
                (CURRENT_CONFLICTED, "current.conflicted = true"),
                (PARENT_CONFLICTED, ""),
            ]),
        ),
        (
            JJRepo::STATUS_CONFLICTED_PARENT,
            || output([
                (CURRENT_CONFLICTED, "current.conflicted = false"),
                (PARENT_CONFLICTED, "parents.conflicted = true"),
            ]),
        ),
        (
            JJRepo::STATUS_DESCRIPTION,
            || output([
                (HAS_DESCRIPTION, "current.has_description = true"),
            ]),
        ),
        (
            JJRepo::STATUS_HIDDEN,
            || output([
                (HIDDEN, "current.hidden = true"),
            ]),
        ),
        (
            JJRepo::STATUS_IMMUTABLE,
            || output([
                (IMMUTABLE, "current.immutable = true"),
            ]),
        ),
        (
            JJRepo::STATUS_ADDED,
            || output([
                (STATUSES, "current.statuses = \"AA\""),
            ]),
        ),
        (
            JJRepo::STATUS_COPIED,
            || output([
                (STATUSES, "current.statuses = \"CCC\""),
            ]),
        ),
        (
            JJRepo::STATUS_DELETED,
            || output([
                (STATUSES, "current.statuses = \"DDDD\""),
            ]),
        ),
        (
            JJRepo::STATUS_MODIFIED,
            || output([
                (STATUSES, "current.statuses = \"MMMMM\""),
            ]),
        ),
        (
            JJRepo::STATUS_RENAMED,
            || output([
                (STATUSES, "current.statuses = \"RRRRRR\""),
            ]),
        ),
        (
            JJRepo::STATUS_NO_CHANGES,
            || output([
                (STATUSES, "current.statuses = \"\""),
            ]),
        ),
        // Used to test the parsing will correctly fail on empty stdout
        (
            JJRepo::EMPTY_OUTPUT,
            || {
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::new(),
                })
            },
        ),
        // Used to test the parsing will correctly fail on invalid stdout
        (
            JJRepo::INVALID_OUTPUT,
            || {
                Some(CommandOutput {
                    stdout: String::from("invalid output"),
                    stderr: String::new(),
                })
            },
        ),
    ];

    for (key, closure) in outputs {
        if s.contains(key) {
            return (closure)();
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::Bookmark;

    #[test]
    fn test_bookmark_methods() {
        #[track_caller]
        fn check_bookmark(orig: &str, name: &str, remote: Option<&str>, diverged: bool) {
            let bm = Bookmark(orig.into());

            assert_eq!(bm.name(), name);
            assert_eq!(bm.remote(), remote);
            assert_eq!(bm.diverged(), diverged);
        }

        check_bookmark("name", "name", None, false);
        check_bookmark("name*", "name", None, true);
        check_bookmark("name@remote", "name", Some("remote"), false);
        check_bookmark("name@remote*", "name", Some("remote"), true);
    }
}
