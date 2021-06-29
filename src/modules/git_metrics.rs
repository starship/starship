use regex::Regex;

use crate::{
    config::RootModuleConfig, configs::git_metrics::GitMetricsConfig, formatter::StringFormatter,
    module::Module,
};

use super::Context;

/// Creates a module with the current added/modified/deleted lines in the git repository at the
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
    let repo_root = repo.root.as_ref()?;

    let diff = context
        .exec_cmd(
            "git",
            &[
                "-C",
                &repo_root.to_string_lossy(),
                "--no-optional-locks",
                "diff",
                "--word-diff",
                "--unified=0",
            ],
        )?
        .stdout;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "a_style" => Some(Ok(config.a_style)),
                "m_style" => Some(Ok(config.m_style)),
                "d_style" => Some(Ok(config.d_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "added" => Some(Ok(get_added_lines(&diff)?)),
                "modified" => Some(Ok(get_modified_lines(&diff)?)),
                "deleted" => Some(Ok(get_deleted_lines(&diff)?)),
                _ => None,
            })
            .parse(None)
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

fn get_added_lines(diff: &str) -> Option<String> {
    let re = Regex::new(r"^\{\+.*\+\}$").unwrap();

    Some(
        diff.lines()
            .filter(|line| re.is_match(line))
            .count()
            .to_string(),
    )
}
fn get_modified_lines(diff: &str) -> Option<String> {
    let re = Regex::new(r"^.+(\[-|\{\+).*$").unwrap();

    Some(
        diff.lines()
            .filter(|line| re.is_match(line))
            .count()
            .to_string(),
    )
}
fn get_deleted_lines(diff: &str) -> Option<String> {
    let re = Regex::new(r"^\[-.*-\]$").unwrap();

    Some(
        diff.lines()
            .filter(|line| re.is_match(line))
            .count()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::fs::OpenOptions;
    use std::io::{self, Error, ErrorKind, Write};
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    use ansi_term::Color;

    use crate::test::ModuleRenderer;

    #[test]
    fn shows_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_metrics")
            .path(repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_added_lines() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let the_file = path.join("the_file");
        let mut the_file = OpenOptions::new().append(true).open(&the_file)?;
        writeln!(the_file, "\nAdded line")?;
        the_file.sync_all()?;

        let actual = ModuleRenderer::new("git_metrics").path(path).collect();

        let expected = Some(format!(
            "{} {} {} ",
            Color::Green.bold().paint("+1"),
            Color::Yellow.bold().paint("~0"),
            Color::Red.bold().paint("-0")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_modified_lines() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let the_file = path.join("the_file");
        write_file(the_file, "Modified line\nSecond Line\nThird Line")?;

        let actual = ModuleRenderer::new("git_metrics").path(path).collect();

        let expected = Some(format!(
            "{} {} {} ",
            Color::Green.bold().paint("+0"),
            Color::Yellow.bold().paint("~1"),
            Color::Red.bold().paint("-0")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_deleted_lines() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let file_path = path.join("the_file");
        write_file(file_path, "First Line\nSecond Line")?;

        let actual = ModuleRenderer::new("git_metrics").path(path).collect();

        let expected = Some(format!(
            "{} {} {} ",
            Color::Green.bold().paint("+0"),
            Color::Yellow.bold().paint("~0"),
            Color::Red.bold().paint("-1")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn shows_all_changes() -> io::Result<()> {
        let repo_dir = create_repo_with_commit()?;
        let path = repo_dir.path();

        let file_path = path.join("the_file");
        write_file(file_path, "\nSecond Line\n\nModified\nAdded")?;

        let actual = ModuleRenderer::new("git_metrics").path(path).collect();

        let expected = Some(format!(
            "{} {} {} ",
            Color::Green.bold().paint("+1"),
            Color::Yellow.bold().paint("~1"),
            Color::Red.bold().paint("-1")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    fn run_git_cmd<A, S>(args: A, dir: Option<&Path>, should_succeed: bool) -> io::Result<()>
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new("git");
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

    fn write_file(file: PathBuf, text: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file)?;
        writeln!(file, "{}", text)?;
        file.sync_all()
    }

    fn create_repo_with_commit() -> io::Result<tempfile::TempDir> {
        let repo_dir = tempfile::tempdir()?;
        let path = repo_dir.path();
        let file = repo_dir.path().join("the_file");

        // Initialize a new git repo
        run_git_cmd(
            &[
                "init",
                "--quiet",
                path.to_str().expect("Path was not UTF-8"),
            ],
            None,
            true,
        )?;

        // Set local author info
        run_git_cmd(
            &["config", "--local", "user.email", "starship@example.com"],
            Some(path),
            true,
        )?;
        run_git_cmd(
            &["config", "--local", "user.name", "starship"],
            Some(path),
            true,
        )?;

        // Ensure on the expected branch.
        // If build environment has `init.defaultBranch` global set
        // it will default to an unknown branch, so need to make & change branch
        run_git_cmd(
            &["checkout", "-b", "master"],
            Some(path),
            // command expected to fail if already on the expected branch
            false,
        )?;

        // Write a file on master and commit it
        write_file(file, "First Line\nSecond Line\nThird Line")?;
        run_git_cmd(&["add", "the_file"], Some(path), true)?;
        run_git_cmd(
            &["commit", "--message", "Commit A", "--no-gpg-sign"],
            Some(path),
            true,
        )?;

        Ok(repo_dir)
    }
}
