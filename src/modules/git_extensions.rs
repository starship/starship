use super::{Context, Module, ModuleConfig};
use crate::configs::git_extensions::GitExtensionsConfig;
use crate::context::Repo;
use crate::formatter::StringFormatter;

/// Creates a module with any git extensions active in the repo at the current directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_extensions");
    let config: GitExtensionsConfig = GitExtensionsConfig::try_load(module.config);

    let repo = context.get_repo().ok()?;

    let exts = config.extensions;
    let mut active_exts = "active exts detected: ";

    // iterate over exts, running functions and appending (with trailing ', ') to active_exts

    // strip trailing ', '


    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "active_exts" => Some(Ok(active_exts)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_extensions`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::ffi::OsStr;
    use std::fs::OpenOptions;
    use std::io::{self, Error, ErrorKind, Write};
    use std::path::Path;
    use std::process::Stdio;

    use crate::test::ModuleRenderer;
    use crate::utils::create_command;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_extensions")
            .config(toml::toml! {
                [git_extensions]
                disabled=false
            })
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn show_nothing_on_repo_without_extensions() -> io::Result<()> {
        let repo_dir = create_repo()?;
        // let path = repo_dir.path();

        let actual = ModuleRenderer::new("git_extensions")
            .config(toml::toml! {
                [git_extensions]
                disabled=false
            })
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn show_lfs() -> io::Result<()> {
        let repo_dir = add_lfs_to_repo(create_repo()?)?;
        // let path = repo_dir.path();

        let actual = ModuleRenderer::new("git_extensions")
            .config(toml::toml! {
                [git_extensions]
                disabled=false
            })
            .path(repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Fixed(149).bold().paint("git_exts: lemon curry")
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    fn add_lfs_to_repo(repo_dir: tempfile::TempDir) -> io::Result<tempfile::TempDir> {
        run_git_cmd(
            &[
                "config",
                "lfs.repositoryformatversion",
                "0",
            ],
            Some(repo_dir.path()),
            true
        )?;

        Ok(repo_dir)
    }

    // the following largely copied from tests in src/modules/git_state.rs - needs a refactor

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

    fn create_repo() -> io::Result<tempfile::TempDir> {
        let repo_dir = tempfile::tempdir()?;
        let path = repo_dir.path();
        let file_in_repo = repo_dir.path().join("some_file");

        let write_file = |text: &str| {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_in_repo)?;
            write!(file, "{}", text)
        };

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
        write_file("Version A")?;
        run_git_cmd(&["add", "some_file"], Some(path), true)?;
        run_git_cmd(
            &["commit", "--message", "Commit A", "--no-gpg-sign"],
            Some(path),
            true,
        )?;

        // this stuff copied from src/modules/git_state.rs not needed -
        // these tests aren't about conflicts
        // // Switch to another branch, and commit a change to the file
        // run_git_cmd(&["checkout", "-b", "other-branch"], Some(path), true)?;
        // write_file("Version B")?;
        // run_git_cmd(
        //     &["commit", "--all", "--message", "Commit B", "--no-gpg-sign"],
        //     Some(path),
        //     true,
        // )?;
        // 
        // // Switch back to master, and commit a third change to the file
        // run_git_cmd(&["checkout", "master"], Some(path), true)?;
        // write_file("Version C")?;
        // run_git_cmd(
        //     &["commit", "--all", "--message", "Commit C", "--no-gpg-sign"],
        //     Some(path),
        //     true,
        // )?;

        Ok(repo_dir)
    }

}
