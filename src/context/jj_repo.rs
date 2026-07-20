use std::path::PathBuf;
use std::sync::OnceLock;

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

    /// Bookmarks to consider for the current change, if any
    pub bookmarks: Option<Vec<Bookmark>>,

    /// Total lines added in this change
    pub lines_added: u32,
    /// Total lines removed in this change
    pub lines_removed: u32,

    /// CONFLICTED | STATUS_ADDED | STATUS_COPIED | STATUS_DELETED | STATUS_MODIFIED | STATUS_RENAMED
    flags: u32,
}

#[derive(Debug, PartialEq)]
pub struct Bookmark(pub Box<str>);

impl JJRepo {
    /// Discover if we're in a JJ repo by looking for a `.jj` directory in the current directory
    /// and its parents.
    ///
    /// While this is technically more error prone than `jj workspace root`, it also much faster
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
                    // Find conflicts in parents and select only one of them: we want to print
                    // `conflict_before|` once if there is a conflict and it's not in '@'
                    "@ | latest(heads(conflicts()::@ & conflicts()), 1)".as_ref(),
                    "--no-patch".as_ref(),
                    // Important to ensure `conflict_before|` is always printed first if a conflict
                    // is present before '@' (and not in '@')
                    "--reversed".as_ref(),
                    "--template".as_ref(),
                    r#"if(
                        current_working_copy,
                        join(
                            "\n",
                            conflict,
                            description.len() > 0,
                            hidden,
                            immutable,
                            change_id,
                            change_id.shortest().prefix().len(),
                            commit_id,
                            commit_id.shortest().prefix().len(),
                            bookmarks,
                            parents.map(|p| p.bookmarks()),
                            diff.stat().total_added(),
                            diff.stat().total_removed(),
                            self
                                .diff(".")
                                .files()
                                .filter(|file| !conflict || conflicted_files.all(|c| c.path().absolute() != file.path().absolute()))
                                .map(|file| file.status_char())
                                .join(""),
                            "",
                        ),
                        "conflict_before|",
                    )"#.as_ref(),
                ])?;

                let mut lines = res.stdout.lines();

                // If the conflict is _before_ '@', the output is
                //
                //     conflict_before|false
                //     <...>
                //
                // If the conflict is _at_ '@', the output is
                //
                //     true
                //     <...>
                let first_line = lines.next()?;
                let has_conflict = first_line == "conflict_before|false" || read_boolean(first_line)?;

                let description = read_boolean(lines.next()?)?;
                let hidden = read_boolean(lines.next()?)?;
                let immutable = read_boolean(lines.next()?)?;

                // A full JJ change ID is always `[k-z]{32}`,
                // so we do a quick sanity check on it just to confirm it looks ok
                let change = lines.next()?;
                if change.len() != 32 || !change.is_ascii() {
                    return None;
                }

                Some(CurrentChange {
                    change: Box::from(change),
                    change_shortest: lines.next()?.parse().ok()?,
                    commit: Box::from(lines.next().filter(|s| s.is_ascii())?),
                    commit_shortest: lines.next()?.parse().ok()?,
                    bookmarks: parse_bookmark_lines(lines.next()?, lines.next()?),
                    lines_added: lines.next()?.parse().ok()?,
                    lines_removed: lines.next()?.parse().ok()?,
                    flags: {
                        let mut flags = 0;

                        if has_conflict {
                            flags |= CurrentChange::CONFLICTED;
                        }
                        if description {
                            flags |= CurrentChange::DESCRIPTION;
                        }
                        if hidden {
                            flags |= CurrentChange::HIDDEN;
                        }
                        if immutable {
                            flags |= CurrentChange::IMMUTABLE;
                        }

                        if let Some(statuses) = lines.next() {
                            if statuses.contains('A') {
                                flags |= CurrentChange::STATUS_ADDED;
                            }
                            if statuses.contains('C') {
                                flags |= CurrentChange::STATUS_COPIED;
                            }
                            if statuses.contains('D') {
                                flags |= CurrentChange::STATUS_DELETED;
                            }
                            if statuses.contains('M') {
                                flags |= CurrentChange::STATUS_MODIFIED;
                            }
                            if statuses.contains('R') {
                                flags |= CurrentChange::STATUS_RENAMED;
                            }
                        }

                        flags
                    },
                })
            })
            .as_ref()
    }
}

impl CurrentChange {
    const CONFLICTED: u32 = 1 << 0;
    const DESCRIPTION: u32 = 1 << 1;
    const HIDDEN: u32 = 1 << 2;
    const IMMUTABLE: u32 = 1 << 3;

    const STATUS_ADDED: u32 = 1 << 4;
    const STATUS_COPIED: u32 = 1 << 5;
    const STATUS_DELETED: u32 = 1 << 6;
    const STATUS_MODIFIED: u32 = 1 << 7;
    const STATUS_RENAMED: u32 = 1 << 8;

    /// True if any mutable change up to the current one is conflicted
    pub fn conflicted(&self) -> bool {
        self.flags & Self::CONFLICTED == Self::CONFLICTED
    }

    /// True if the current change has a non-empty description
    pub fn description(&self) -> bool {
        self.flags & Self::DESCRIPTION == Self::DESCRIPTION
    }

    /// True if the current change is hidden
    pub fn hidden(&self) -> bool {
        self.flags & Self::HIDDEN == Self::HIDDEN
    }

    /// True if the current change is immutable
    pub fn immutable(&self) -> bool {
        self.flags & Self::IMMUTABLE == Self::IMMUTABLE
    }

    /// True if the current change includes at least one added file
    pub fn status_added(&self) -> bool {
        self.flags & Self::STATUS_ADDED == Self::STATUS_ADDED
    }

    /// True if the current change includes at least one copied file
    pub fn status_copied(&self) -> bool {
        self.flags & Self::STATUS_COPIED == Self::STATUS_COPIED
    }

    /// True if the current change includes at least one deleted file
    pub fn status_deleted(&self) -> bool {
        self.flags & Self::STATUS_DELETED == Self::STATUS_DELETED
    }

    /// True if the current change includes at least one modified file
    pub fn status_modified(&self) -> bool {
        self.flags & Self::STATUS_MODIFIED == Self::STATUS_MODIFIED
    }

    /// True if the current change includes at least one renamed file
    pub fn status_renamed(&self) -> bool {
        self.flags & Self::STATUS_RENAMED == Self::STATUS_RENAMED
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

/// Read a single boolean
fn read_boolean(line: &str) -> Option<bool> {
    match line {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

/// Select the bookmarks to save (current > parents)
fn parse_bookmark_lines(current: &str, parents: &str) -> Option<Vec<Bookmark>> {
    let line = if !current.is_empty() {
        current
    } else if !parents.is_empty() {
        parents
    } else {
        return None;
    };

    Some(line.split(' ').map(|b| Bookmark(Box::from(b))).collect())
}

#[cfg(test)]
impl JJRepo {
    pub const BASE: &str = "/jj/base";
    pub const EMPTY_OUTPUT: &str = "/jj/empty-output";
    pub const INVALID_OUTPUT: &str = "/jj/invalid-output";

    pub const BOOKMARK_NO_CURRENT: &str = "/jj/bookmarks/no-current";

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
    const CONFLICT: usize = 0;
    #[allow(unused)]
    const DESC: usize = 1;
    #[allow(unused)]
    const HIDDEN: usize = 2;
    #[allow(unused)]
    const IMMUTABLE: usize = 3;
    #[allow(unused)]
    const CHANGE: usize = 4;
    #[allow(unused)]
    const CHANGE_SHORT_LENGTH: usize = 5;
    #[allow(unused)]
    const COMMIT: usize = 6;
    #[allow(unused)]
    const COMMIT_SHORT_LENGTH: usize = 7;
    #[allow(unused)]
    const BOOKMARKS_CUR: usize = 8;
    #[allow(unused)]
    const BOOKMARKS_PREV: usize = 9;
    #[allow(unused)]
    const LINES_A: usize = 10;
    #[allow(unused)]
    const LINES_D: usize = 11;
    #[allow(unused)]
    const FILES: usize = 12;

    /// Generate output for JJ while allowing easy replacement of lines to test various valid
    /// possibilities
    fn output<const N: usize>(mods: [(usize, &str); N]) -> Option<CommandOutput> {
        let mut stdout = [
            // conflict
            "conflict_before|false",
            // description
            "false",
            // hidden
            "false",
            // immutable
            "false",
            // change id
            "pvtxwmvtttmrkkoqkutlystlnozssmnk",
            // change id shortest prefix length
            "3",
            // commit id
            "30363e463b3a5c87ad352b2d342f7408e3c2dda8",
            // commit id shortest prefix length
            "4",
            // @ bookmarks
            "cur_local cur_tracked* cur_modified@upstream* cur_untracked@origin",
            // @- bookmarks
            "par_local par_tracked* par_modified@upstream* par_untracked@origin",
            // lines added
            "100",
            // lines deleted
            "90",
            // files status in current directory
            "ACDMR",
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
            JJRepo::BOOKMARK_NO_CURRENT,
            || output([
                (BOOKMARKS_CUR, ""),
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
    use super::{Bookmark, parse_bookmark_lines, read_boolean};

    fn b(s: &str) -> Bookmark {
        Bookmark(s.into())
    }

    #[test]
    fn test_read_boolean() {
        assert_eq!(read_boolean("true"), Some(true));
        assert_eq!(read_boolean("false"), Some(false));
        assert_eq!(read_boolean("falsefalse"), None);
        assert_eq!(read_boolean("trueabcdef"), None);
        assert_eq!(read_boolean(""), None);
    }

    #[test]
    fn test_parse_bookmark_lines() {
        assert_eq!(parse_bookmark_lines("", ""), None,);

        assert_eq!(
            parse_bookmark_lines("b1 b2* b3@upstream b4@origin*", "ignored"),
            Some(vec![b("b1"), b("b2*"), b("b3@upstream"), b("b4@origin*")]),
        );

        assert_eq!(
            parse_bookmark_lines("", "p1 p2* p3@upstream p4@origin*"),
            Some(vec![b("p1"), b("p2*"), b("p3@upstream"), b("p4@origin*")]),
        );
    }

    #[test]
    fn test_bookmark_methods() {
        #[track_caller]
        fn check_bookmark(orig: &str, name: &str, remote: Option<&str>, diverged: bool) {
            let bm = b(orig);

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
