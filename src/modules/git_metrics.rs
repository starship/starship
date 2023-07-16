use regex::Regex;
use std::ffi::OsStr;

use crate::{
    config::ModuleConfig, configs::git_metrics::GitMetricsConfig,
    formatter::string_formatter::StringFormatterError, formatter::StringFormatter, module::Module,
};

use super::Context;

/// Creates a module with the current added/deleted lines in the git repository at the
/// current directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_metrics");
    let config: GitMetricsConfig = GitMetricsConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    let repo = context.get_repo().ok()?;
    let repo_root = repo.workdir.as_ref()?;

    let mut args = vec![
        OsStr::new("--git-dir"),
        repo.path.as_os_str(),
        OsStr::new("--work-tree"),
        repo_root.as_os_str(),
        OsStr::new("--no-optional-locks"),
        OsStr::new("diff"),
        OsStr::new("--shortstat"),
    ];

    if config.ignore_submodules {
        args.push(OsStr::new("--ignore-submodules"));
    }

    if config.include_staged {
        args.push(OsStr::new("HEAD"))
    }

    let diff = context.exec_cmd("git", &args)?.stdout;

    let stats = GitDiff::parse(&diff);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "added_style" => Some(Ok(config.added_style)),
                "deleted_style" => Some(Ok(config.deleted_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "added" => GitDiff::get_variable(config.only_nonzero_diffs, stats.added),
                "deleted" => GitDiff::get_variable(config.only_nonzero_diffs, stats.deleted),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_metrics`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

/// Represents the parsed output from a git diff.
struct GitDiff<'a> {
    added: &'a str,
    deleted: &'a str,
}

impl<'a> GitDiff<'a> {
    /// Returns the first capture group given a regular expression and a string.
    /// If it fails to get the capture group it will return "0".
    fn get_matched_str(diff: &'a str, re: &Regex) -> &'a str {
        match re.captures(diff) {
            Some(caps) => caps.get(1).unwrap().as_str(),
            _ => "0",
        }
    }

    /// Parses the result of 'git diff --shortstat' as a `GitDiff` struct.
    pub fn parse(diff: &'a str) -> Self {
        let added_re = Regex::new(r"(\d+) \w+\(\+\)").unwrap();
        let deleted_re = Regex::new(r"(\d+) \w+\(\-\)").unwrap();

        Self {
            added: GitDiff::get_matched_str(diff, &added_re),
            deleted: GitDiff::get_matched_str(diff, &deleted_re),
        }
    }

    pub fn get_variable(
        only_nonzero_diffs: bool,
        changed: &str,
    ) -> Option<Result<&str, StringFormatterError>> {
        match only_nonzero_diffs {
            true => match changed {
                "0" => None,
                _ => Some(Ok(changed)),
            },
            false => Some(Ok(changed)),
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

    use nu_ansi_term::Color;

    use crate::test::ModuleRenderer;

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
                only_nonzero_diffs = false
            })
            .path(path)
            .collect();

        let expected = Some(format!(
            "{} {} ",
            Color::Green.bold().paint("+1"),
            Color::Red.bold().paint("-0")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_all_changes_with_include_staged() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let file_path = path.join("the_file");
        write_file(file_path, "\nSecond Line\n\nModified\nAdded\n")?;

        run_git_cmd(["add", path.join("the_file").to_str().expect("Path was not UTF-8")],
                    Some(path),
                    true)?;

        let actual = ModuleRenderer::new("git_metrics")
            .config(toml::toml! {
                    [git_metrics]
                    disabled = false
                    ignore_submodules = true
                    include_staged = true
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
