use gix::bstr::{BStr, ByteSlice};
use gix::diff::blob::ResourceKind;
use gix::diff::blob::pipeline::WorktreeRoots;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

use super::Context;
use crate::configs::git_status::GitStatusConfig;
use crate::{
    config::ModuleConfig, configs::git_metrics::GitMetricsConfig, formatter::StringFormatter,
    formatter::string_formatter::StringFormatterError, module::Module,
};

/// Creates a module with the current added/deleted lines in the git repository at the
/// current directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_metrics");
    let config: GitMetricsConfig = GitMetricsConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    }

    let repo = context.get_repo().ok()?;
    let gix_repo = repo.open();
    if gix_repo.is_bare() {
        return None;
    }
    let status_module = context.new_module("git_status");
    let status_config = GitStatusConfig::try_load(status_module.config);
    // TODO: remove this special case once `gitoxide` can handle sparse indices for tree-index comparisons.
    let stats = if repo.fs_monitor_value_is_true
        || status_config.use_git_executable
        || gix_repo.index_or_empty().ok()?.is_sparse()
    {
        let mut git_args = vec!["diff", "--shortstat"];
        if config.ignore_submodules {
            git_args.push("--ignore-submodules");
        }

        let diff = repo.exec_git(context, &git_args)?.stdout;

        GitDiff::parse(&diff)
    } else {
        #[derive(Default)]
        struct Diff {
            added: usize,
            deleted: usize,
        }
        impl Diff {
            fn add(&mut self, c: Option<gix::diff::blob::sink::Counter<()>>) {
                let Some(c) = c else { return };
                self.added += c.insertions as usize;
                self.deleted += c.removals as usize;
            }
        }
        let status = super::git_status::get_static_repo_status(context, repo, &status_config)?;
        let gix_repo = gix_repo.with_object_memory();
        gix_repo.write_blob([]).ok()?; /* create empty blob */
        let tree_index_cache = prevent_external_diff(
            gix_repo
                .diff_resource_cache(
                    gix::diff::blob::pipeline::Mode::ToGit,
                    WorktreeRoots::default(),
                )
                .ok()?,
        );
        let index_worktree_cache = prevent_external_diff(
            gix_repo
                .diff_resource_cache(
                    gix::diff::blob::pipeline::Mode::ToGit,
                    WorktreeRoots {
                        old_root: None,
                        new_root: gix_repo.workdir().map(ToOwned::to_owned),
                    },
                )
                .ok()?,
        );
        let diff = status
            .changes
            .par_iter()
            .map_init(
                {
                    let repo = gix_repo.into_sync();
                    move || {
                        let repo = repo.to_thread_local();
                        (repo, tree_index_cache.clone(), index_worktree_cache.clone())
                    }
                },
                |(repo, tree_index_cache, index_worktree_cache), change| {
                    use gix::status;
                    let mut diff = Diff::default();
                    match change {
                        status::Item::TreeIndex(change) => {
                            use gix::diff::index::Change;
                            match change {
                                Change::Addition {
                                    entry_mode,
                                    location,
                                    id,
                                    ..
                                } => {
                                    diff.added += count_lines(
                                        location,
                                        id.as_ref().into(),
                                        *entry_mode,
                                        tree_index_cache,
                                        repo,
                                    );
                                }
                                Change::Deletion {
                                    entry_mode,
                                    location,
                                    id,
                                    ..
                                } => {
                                    diff.deleted += count_lines(
                                        location,
                                        id.as_ref().into(),
                                        *entry_mode,
                                        tree_index_cache,
                                        repo,
                                    );
                                }
                                Change::Modification {
                                    location,
                                    previous_entry_mode,
                                    previous_id,
                                    entry_mode,
                                    id,
                                    ..
                                } => {
                                    let location = location.as_ref();
                                    diff.add(diff_two_opt(
                                        location,
                                        previous_id.as_ref().to_owned(),
                                        *previous_entry_mode,
                                        location,
                                        id.as_ref().to_owned(),
                                        *entry_mode,
                                        tree_index_cache,
                                        repo,
                                    ));
                                }
                                Change::Rewrite {
                                    source_location,
                                    source_entry_mode,
                                    source_id,
                                    location,
                                    entry_mode,
                                    id,
                                    copy,
                                    ..
                                } => {
                                    if *copy {
                                        diff.added += count_lines(
                                            location,
                                            id.as_ref().into(),
                                            *entry_mode,
                                            tree_index_cache,
                                            repo,
                                        );
                                    } else {
                                        diff.add(diff_two_opt(
                                            source_location.as_ref(),
                                            source_id.as_ref().to_owned(),
                                            *source_entry_mode,
                                            location,
                                            id.as_ref().to_owned(),
                                            *entry_mode,
                                            tree_index_cache,
                                            repo,
                                        ));
                                    }
                                }
                            }
                        }
                        status::Item::IndexWorktree(change) => {
                            use gix::status::index_worktree::Item;
                            use gix::status::plumbing::index_as_worktree::{Change, EntryStatus};
                            match change {
                                Item::Modification {
                                    rela_path,
                                    entry,
                                    status: EntryStatus::Change(Change::Removed),
                                    ..
                                } => {
                                    diff.deleted += count_lines(
                                        rela_path.as_bstr(),
                                        entry.id,
                                        entry.mode,
                                        tree_index_cache,
                                        repo,
                                    );
                                }
                                Item::Modification {
                                    rela_path,
                                    entry,
                                    status:
                                        EntryStatus::Change(Change::Modification {
                                            content_change: Some(()),
                                            ..
                                        }),
                                    ..
                                } => {
                                    let location = rela_path.as_bstr();
                                    diff.add(diff_two_opt(
                                        location,
                                        entry.id,
                                        entry.mode,
                                        location,
                                        repo.object_hash().null(),
                                        entry.mode,
                                        index_worktree_cache,
                                        repo,
                                    ));
                                }
                                Item::Modification {
                                    rela_path,
                                    entry,
                                    status: EntryStatus::IntentToAdd,
                                    ..
                                } => {
                                    diff.added += count_lines(
                                        rela_path.as_bstr(),
                                        repo.object_hash().null(),
                                        entry.mode,
                                        index_worktree_cache,
                                        repo,
                                    );
                                }
                                Item::Rewrite { .. } => {
                                    unreachable!("not activated")
                                }
                                _ => {}
                            }
                        }
                    }
                    diff
                },
            )
            .reduce(Diff::default, |a, b| Diff {
                added: a.added + b.added,
                deleted: a.deleted + b.deleted,
            });

        GitDiff {
            added: diff.added.to_string(),
            deleted: diff.deleted.to_string(),
        }
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "added_style" => Some(Ok(config.added_style)),
                "deleted_style" => Some(Ok(config.deleted_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "added" => GitDiff::get_variable(config.only_nonzero_diffs, &stats.added),
                "deleted" => GitDiff::get_variable(config.only_nonzero_diffs, &stats.deleted),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_metrics`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn prevent_external_diff(mut cache: gix::diff::blob::Platform) -> gix::diff::blob::Platform {
    cache.options.skip_internal_diff_if_external_is_configured = false;
    cache
}

#[allow(clippy::too_many_arguments)]
fn diff_two_opt(
    lhs_location: &BStr,
    lhs_id: gix::ObjectId,
    lhs_kind: gix::index::entry::Mode,
    rhs_location: &BStr,
    rhs_id: gix::ObjectId,
    rhs_kind: gix::index::entry::Mode,
    cache: &mut gix::diff::blob::Platform,
    find: &impl gix::objs::FindObjectOrHeader,
) -> Option<gix::diff::blob::sink::Counter<()>> {
    cache
        .set_resource(
            lhs_id,
            lhs_kind.to_tree_entry_mode()?.kind(),
            lhs_location,
            ResourceKind::OldOrSource,
            find,
        )
        .ok()?;
    cache
        .set_resource(
            rhs_id,
            rhs_kind.to_tree_entry_mode()?.kind(),
            rhs_location,
            ResourceKind::NewOrDestination,
            find,
        )
        .ok()?;
    count_diff_lines(cache.prepare_diff().ok()?)
}

fn count_lines(
    location: &BStr,
    id: gix::ObjectId,
    kind: gix::index::entry::Mode,
    cache: &mut gix::diff::blob::Platform,
    find: &impl gix::objs::FindObjectOrHeader,
) -> usize {
    diff_two_opt(
        location,
        id.kind().null(),
        kind,
        location,
        id,
        kind,
        cache,
        find,
    )
    .map_or(0, |diff| diff.insertions as usize)
}

fn count_diff_lines(
    prep: gix::diff::blob::platform::prepare_diff::Outcome<'_>,
) -> Option<gix::diff::blob::sink::Counter<()>> {
    use gix::diff::blob::platform::prepare_diff::Operation;
    match prep.operation {
        Operation::InternalDiff { algorithm } => {
            let tokens = prep.interned_input();
            let counter = gix::diff::blob::diff(
                algorithm,
                &tokens,
                gix::diff::blob::sink::Counter::default(),
            );
            Some(counter)
        }
        Operation::ExternalCommand { .. } => {
            unreachable!("we disabled that")
        }
        Operation::SourceOrDestinationIsBinary => None,
    }
}

/// Represents the parsed output from a git diff.
#[derive(Default)]
struct GitDiff {
    added: String,
    deleted: String,
}

impl GitDiff {
    /// Returns the first capture group given a regular expression and a string.
    /// If it fails to get the capture group it will return "0".
    fn get_matched_str<'a>(diff: &'a str, re: &Regex) -> &'a str {
        match re.captures(diff) {
            Some(caps) => caps.get(1).unwrap().as_str(),
            _ => "0",
        }
    }

    /// Parses the result of 'git diff --shortstat' as a `GitDiff` struct.
    pub fn parse(diff: &str) -> Self {
        let added_re = Regex::new(r"(\d+) \w+\(\+\)").unwrap();
        let deleted_re = Regex::new(r"(\d+) \w+\(\-\)").unwrap();

        Self {
            added: Self::get_matched_str(diff, &added_re).to_owned(),
            deleted: Self::get_matched_str(diff, &deleted_re).to_owned(),
        }
    }

    pub fn get_variable(
        only_nonzero_diffs: bool,
        changed: &str,
    ) -> Option<Result<&str, StringFormatterError>> {
        if only_nonzero_diffs {
            match changed {
                "0" => None,
                _ => Some(Ok(changed)),
            }
        } else {
            Some(Ok(changed))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{create_command, write_file};
    use std::ffi::OsStr;
    use std::fs::OpenOptions;
    use std::io::{self, Error, ErrorKind, Write};
    use std::path::Path;
    use std::process::Stdio;

    use crate::modules::git_status::tests::make_sparse;
    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use nu_ansi_term::Color;

    #[test]
    fn shows_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        let path = repo_dir.path();

        let actual = render_metrics(path);

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_added_lines() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let the_file = path.join("the_file");
        let mut the_file = OpenOptions::new().append(true).open(the_file)?;
        writeln!(the_file, "Added line")?;
        the_file.sync_all()?;

        let actual = render_metrics(path);

        let expected = Some(format!("{} ", Color::Green.bold().paint("+1"),));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_addition() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        std::fs::write(path.join("new-file"), "new line")?;
        run_git_cmd(["add", "new-file"], Some(path), true)?;

        let actual = render_metrics(path);

        let expected = Some(format!("{} ", Color::Green.bold().paint("+1"),));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_rename_modification() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let the_file = path.join("the_file");
        let mut the_file = OpenOptions::new().append(true).open(the_file)?;
        writeln!(the_file, "Added line")?;
        the_file.sync_all()?;
        run_git_cmd(["add", "the_file"], Some(path), true)?;
        run_git_cmd(["mv", "the_file", "that_file"], Some(path), true)?;

        let actual = render_metrics(path);

        let expected = Some(format!("{} ", Color::Green.bold().paint("+1"),));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_addition_intended() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        std::fs::write(path.join("new-file"), "new line")?;
        run_git_cmd(["add", "-N", "new-file"], Some(path), true)?;

        let actual = render_metrics(path);

        let expected = Some(format!("{} ", Color::Green.bold().paint("+1"),));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_modification() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        std::fs::write(path.join("the_file"), "modify all")?;
        run_git_cmd(["add", "the_file"], Some(path), true)?;

        let actual = render_metrics(path);

        let expected = Some(format!(
            "{} {} ",
            Color::Green.bold().paint("+1"),
            Color::Red.bold().paint("-3")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_lines() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let file_path = path.join("the_file");
        write_file(file_path, "First Line\nSecond Line\n")?;

        let actual = render_metrics(path);

        let expected = Some(format!("{} ", Color::Red.bold().paint("-1")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_lines_of_entire_file() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        std::fs::remove_file(path.join("the_file"))?;

        let actual = render_metrics(path);

        let expected = Some(format!("{} ", Color::Red.bold().paint("-3")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_staged_deletion() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        run_git_cmd(["rm", "the_file"], Some(path), true)?;

        let actual = render_metrics(path);

        let expected = Some(format!("{} ", Color::Red.bold().paint("-3")));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_all_changes() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let file_path = path.join("the_file");
        write_file(file_path, "\nSecond Line\n\nModified\nAdded\n")?;

        let actual = render_metrics(path);

        let expected = Some(format!(
            "{} {} ",
            Color::Green.bold().paint("+4"),
            Color::Red.bold().paint("-2")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_nothing_if_no_changes() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let actual = render_metrics(path);

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_nothing_on_untracked() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();
        std::fs::write(path.join("untracked"), "a line")?;

        let actual = render_metrics(path);

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_nothing_if_no_changes_sparse() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        make_sparse(path)?;
        let actual = render_metrics(path);

        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_all_if_only_nonzero_diffs_is_false() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let the_file = path.join("the_file");
        let mut the_file = OpenOptions::new().append(true).open(the_file)?;
        writeln!(the_file, "Added line")?;
        the_file.sync_all()?;

        let actual = ModuleRenderer::new("git_metrics")
            .config(toml::toml! {
                [git_metrics]
                disabled = false
                only_nonzero_diffs = true
            })
            .path(path)
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("+1"),));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn doesnt_generate_git_metrics_for_bare_repo() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GitBare)?;

        let actual = render_metrics(repo_dir.path());
        assert_eq!(None, actual);

        repo_dir.close()
    }

    #[test]
    fn shows_all_changes_with_ignored_submodules() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let file_path = path.join("the_file");
        write_file(file_path, "\nSecond Line\n\nModified\nAdded\n")?;

        let actual = ModuleRenderer::new("git_metrics")
            .config(toml::toml! {
                    [git_metrics]
                    disabled = false
                    ignore_submodules = true
            })
            .path(path)
            .collect();

        let expected = Some(format!(
            "{} {} ",
            Color::Green.bold().paint("+4"),
            Color::Red.bold().paint("-2")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn works_if_git_executable_is_used() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let file_path = path.join("the_file");
        write_file(file_path, "\nSecond Line\n\nModified\nAdded\n")?;

        let actual = ModuleRenderer::new("git_metrics")
            .config(toml::toml! {
                [git_status]
                use_git_executable = true
                [git_metrics]
                disabled = false
            })
            .path(path)
            .collect();

        let expected = Some(format!(
            "{} {} ",
            Color::Green.bold().paint("+4"),
            Color::Red.bold().paint("-2")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    fn render_metrics(path: &Path) -> Option<String> {
        ModuleRenderer::new("git_metrics")
            .config(toml::toml! {
                [git_metrics]
                disabled = false
            })
            .path(path)
            .collect()
    }

    fn run_git_cmd<A, S>(args: A, dir: Option<&Path>, should_succeed: bool) -> io::Result<()>
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = create_command("git")?;
        command
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());

        if let Some(dir) = dir {
            command.current_dir(dir);
        }

        let status = command.status()?;

        if should_succeed && !status.success() {
            Err(Error::from(ErrorKind::Other))
        } else {
            Ok(())
        }
    }

    fn create_repo_with_commit() -> io::Result<tempfile::TempDir> {
        let repo_dir = tempfile::tempdir()?;
        let path = repo_dir.path();
        let file = repo_dir.path().join("the_file");

        // Initialize a new git repo
        run_git_cmd(
            [
                "init",
                "--quiet",
                path.to_str().expect("Path was not UTF-8"),
            ],
            None,
            true,
        )?;

        // Set local author info
        run_git_cmd(
            ["config", "--local", "user.email", "starship@example.com"],
            Some(path),
            true,
        )?;
        run_git_cmd(
            ["config", "--local", "user.name", "starship"],
            Some(path),
            true,
        )?;

        // Ensure on the expected branch.
        // If build environment has `init.defaultBranch` global set
        // it will default to an unknown branch, so need to make & change branch
        run_git_cmd(
            ["checkout", "-b", "master"],
            Some(path),
            // command expected to fail if already on the expected branch
            false,
        )?;

        // Write a file on master and commit it
        write_file(file, "First Line\nSecond Line\nThird Line\n")?;
        run_git_cmd(["add", "the_file"], Some(path), true)?;
        run_git_cmd(
            ["commit", "--message", "Commit A", "--no-gpg-sign"],
            Some(path),
            true,
        )?;

        Ok(repo_dir)
    }
}
