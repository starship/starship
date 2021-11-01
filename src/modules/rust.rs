use std::fs;
use std::path::Path;
use std::process::Output;

use serde::Deserialize;

use super::{Context, Module, RootModuleConfig};

use crate::configs::rust::RustConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::create_command;

/// Creates a module with the current Rust version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("rust");
    let config = RustConfig::try_load(module.config);

    let is_rs_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_rs_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                // This may result in multiple calls to `get_module_version` when a user have
                // multiple `$version` variables defined in `format`.
                "version" => get_module_version(context, &config).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `rust`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_module_version(context: &Context, config: &RustConfig) -> Option<String> {
    // `$CARGO_HOME/bin/rustc(.exe) --version` may attempt installing a rustup toolchain.
    // https://github.com/starship/starship/issues/417
    //
    // To display appropriate versions preventing `rustc` from downloading toolchains, we have to
    // check
    // 1. `$RUSTUP_TOOLCHAIN`
    // 2. `rustup override list`
    // 3. `rust-toolchain` or `rust-toolchain.toml` in `.` or parent directories
    // as `rustup` does.
    // https://github.com/rust-lang/rustup.rs/tree/eb694fcada7becc5d9d160bf7c623abe84f8971d#override-precedence
    //
    // Probably we have no other way to know whether any toolchain override is specified for the
    // current directory. The following commands also cause toolchain installations.
    // - `rustup show`
    // - `rustup show active-toolchain`
    // - `rustup which`
    if let Some(toolchain) = env_rustup_toolchain(context)
        .or_else(|| execute_rustup_override_list(&context.current_dir))
        .or_else(|| find_rust_toolchain_file(context))
    {
        match execute_rustup_run_rustc_version(&toolchain) {
            RustupRunRustcVersionOutcome::RustcVersion(rustc_version) => {
                format_rustc_version(&rustc_version, config.version_format)
            }
            RustupRunRustcVersionOutcome::ToolchainName(toolchain) => Some(toolchain),
            RustupRunRustcVersionOutcome::RustupNotWorking => {
                // If `rustup` is not in `$PATH` or cannot be executed for other reasons, we can
                // safely execute `rustc --version`.
                format_rustc_version(&execute_rustc_version()?, config.version_format)
            }
            RustupRunRustcVersionOutcome::Err => None,
        }
    } else {
        format_rustc_version(&execute_rustc_version()?, config.version_format)
    }
}

fn env_rustup_toolchain(context: &Context) -> Option<String> {
    let val = context.get_env("RUSTUP_TOOLCHAIN")?;
    Some(val.trim().to_owned())
}

fn execute_rustup_override_list(cwd: &Path) -> Option<String> {
    let Output { stdout, .. } = create_command("rustup")
        .ok()?
        .args(&["override", "list"])
        .output()
        .ok()?;
    let stdout = String::from_utf8(stdout).ok()?;
    extract_toolchain_from_rustup_override_list(&stdout, cwd)
}

fn extract_toolchain_from_rustup_override_list(stdout: &str, cwd: &Path) -> Option<String> {
    if stdout == "no overrides\n" {
        return None;
    }
    stdout
        .lines()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            let dir = words.next()?;
            let toolchain = words.next()?;
            Some((dir, toolchain))
        })
        .find(|(dir, _)| cwd.starts_with(dir))
        .map(|(_, toolchain)| toolchain.to_owned())
}

fn find_rust_toolchain_file(context: &Context) -> Option<String> {
    // Look for 'rust-toolchain' or 'rust-toolchain.toml' as rustup does.
    // for more information:
    // https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
    // for the implementation in 'rustup':
    // https://github.com/rust-lang/rustup/blob/a45e4cd21748b04472fce51ba29999ee4b62bdec/src/config.rs#L631

    #[derive(Deserialize)]
    struct OverrideFile {
        toolchain: ToolchainSection,
    }

    #[derive(Deserialize)]
    struct ToolchainSection {
        channel: Option<String>,
    }

    fn read_channel(path: &Path, only_toml: bool) -> Option<String> {
        let contents = fs::read_to_string(path).ok()?;

        match contents.lines().count() {
            0 => None,
            1 if !only_toml => Some(contents),
            _ => {
                toml::from_str::<OverrideFile>(&contents)
                    .ok()?
                    .toolchain
                    .channel
            }
        }
        .filter(|c| !c.trim().is_empty())
        .map(|c| c.trim().to_owned())
    }

    if context
        .dir_contents()
        .map_or(false, |dir| dir.has_file("rust-toolchain"))
    {
        if let Some(toolchain) = read_channel(Path::new("rust-toolchain"), false) {
            return Some(toolchain);
        }
    }

    if context
        .dir_contents()
        .map_or(false, |dir| dir.has_file("rust-toolchain.toml"))
    {
        if let Some(toolchain) = read_channel(Path::new("rust-toolchain.toml"), true) {
            return Some(toolchain);
        }
    }

    let mut dir = &*context.current_dir;
    loop {
        if let Some(toolchain) = read_channel(&dir.join("rust-toolchain"), false) {
            return Some(toolchain);
        }
        if let Some(toolchain) = read_channel(&dir.join("rust-toolchain.toml"), true) {
            return Some(toolchain);
        }
        dir = dir.parent()?;
    }
}

fn execute_rustup_run_rustc_version(toolchain: &str) -> RustupRunRustcVersionOutcome {
    create_command("rustup")
        .and_then(|mut cmd| cmd.args(&["run", toolchain, "rustc", "--version"]).output())
        .map(extract_toolchain_from_rustup_run_rustc_version)
        .unwrap_or(RustupRunRustcVersionOutcome::RustupNotWorking)
}

fn extract_toolchain_from_rustup_run_rustc_version(output: Output) -> RustupRunRustcVersionOutcome {
    if output.status.success() {
        if let Ok(output) = String::from_utf8(output.stdout) {
            return RustupRunRustcVersionOutcome::RustcVersion(output);
        }
    } else if let Ok(stderr) = String::from_utf8(output.stderr) {
        if stderr.starts_with("error: toolchain '") && stderr.ends_with("' is not installed\n") {
            let stderr = stderr
                ["error: toolchain '".len()..stderr.len() - "' is not installed\n".len()]
                .to_owned();
            return RustupRunRustcVersionOutcome::ToolchainName(stderr);
        }
    }
    RustupRunRustcVersionOutcome::Err
}

fn execute_rustc_version() -> Option<String> {
    match create_command("rustc").ok()?.arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_rustc_version(rustc_version: &str, version_format: &str) -> Option<String> {
    let version = rustc_version
        // split into ["rustc", "1.34.0", ...]
        .split_whitespace()
        // get down to "1.34.0"
        .nth(1)?;

    match VersionFormatter::format_version(version, version_format) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            log::warn!("Error formatting `rust` version:\n{}", error);
            Some(format!("v{}", version))
        }
    }
}

#[derive(Debug, PartialEq)]
enum RustupRunRustcVersionOutcome {
    RustcVersion(String),
    ToolchainName(String),
    RustupNotWorking,
    Err,
}

#[cfg(test)]
mod tests {
    use crate::context::Shell;
    use once_cell::sync::Lazy;
    use std::io;
    use std::process::{ExitStatus, Output};

    use super::*;

    #[test]
    fn test_extract_toolchain_from_rustup_override_list() {
        static NO_OVERRIDES_INPUT: &str = "no overrides\n";
        static NO_OVERRIDES_CWD: &str = "";
        assert_eq!(
            extract_toolchain_from_rustup_override_list(
                NO_OVERRIDES_INPUT,
                NO_OVERRIDES_CWD.as_ref(),
            ),
            None,
        );

        static OVERRIDES_INPUT: &str =
            "/home/user/src/a                                beta-x86_64-unknown-linux-gnu\n\
             /home/user/src/b                                nightly-x86_64-unknown-linux-gnu\n";
        static OVERRIDES_CWD_A: &str = "/home/user/src/a/src";
        static OVERRIDES_CWD_B: &str = "/home/user/src/b/tests";
        static OVERRIDES_CWD_C: &str = "/home/user/src/c/examples";
        assert_eq!(
            extract_toolchain_from_rustup_override_list(OVERRIDES_INPUT, OVERRIDES_CWD_A.as_ref()),
            Some("beta-x86_64-unknown-linux-gnu".to_owned()),
        );
        assert_eq!(
            extract_toolchain_from_rustup_override_list(OVERRIDES_INPUT, OVERRIDES_CWD_B.as_ref()),
            Some("nightly-x86_64-unknown-linux-gnu".to_owned()),
        );
        assert_eq!(
            extract_toolchain_from_rustup_override_list(OVERRIDES_INPUT, OVERRIDES_CWD_C.as_ref()),
            None,
        );
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn test_extract_toolchain_from_rustup_run_rustc_version() {
        #[cfg(unix)]
        use std::os::unix::process::ExitStatusExt as _;
        #[cfg(windows)]
        use std::os::windows::process::ExitStatusExt as _;

        static RUSTC_VERSION: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"rustc 1.34.0\n"[..].to_owned(),
            stderr: vec![],
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(RUSTC_VERSION.clone()),
            RustupRunRustcVersionOutcome::RustcVersion("rustc 1.34.0\n".to_owned()),
        );

        static TOOLCHAIN_NAME: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"error: toolchain 'channel-triple' is not installed\n"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(TOOLCHAIN_NAME.clone()),
            RustupRunRustcVersionOutcome::ToolchainName("channel-triple".to_owned()),
        );

        static INVALID_STDOUT: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"\xc3\x28"[..].to_owned(),
            stderr: vec![],
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(INVALID_STDOUT.clone()),
            RustupRunRustcVersionOutcome::Err,
        );

        static INVALID_STDERR: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"\xc3\x28"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(INVALID_STDERR.clone()),
            RustupRunRustcVersionOutcome::Err,
        );

        static UNEXPECTED_FORMAT_OF_ERROR: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"error:"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(UNEXPECTED_FORMAT_OF_ERROR.clone()),
            RustupRunRustcVersionOutcome::Err,
        );
    }

    #[test]
    fn test_format_rustc_version() {
        let config = RustConfig::default();
        let rustc_stable = "rustc 1.34.0 (91856ed52 2019-04-10)";
        let rustc_beta = "rustc 1.34.0-beta.1 (2bc1d406d 2019-04-10)";
        let rustc_nightly = "rustc 1.34.0-nightly (b139669f3 2019-04-10)";
        assert_eq!(
            format_rustc_version(rustc_nightly, config.version_format),
            Some("v1.34.0-nightly".to_string())
        );
        assert_eq!(
            format_rustc_version(rustc_beta, config.version_format),
            Some("v1.34.0-beta.1".to_string())
        );
        assert_eq!(
            format_rustc_version(rustc_stable, config.version_format),
            Some("v1.34.0".to_string())
        );
        assert_eq!(
            format_rustc_version("rustc 1.34.0", config.version_format),
            Some("v1.34.0".to_string())
        );
    }

    #[test]
    fn test_find_rust_toolchain_file() -> io::Result<()> {
        // `rust-toolchain` with toolchain in one line
        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("rust-toolchain"), "1.34.0")?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            dir.path().into(),
            dir.path().into(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain` in toml format
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("rust-toolchain"),
            "[toolchain]\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            dir.path().into(),
            dir.path().into(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain` in toml format with new lines
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("rust-toolchain"),
            "\n\n[toolchain]\n\n\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            dir.path().into(),
            dir.path().into(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain` in parent directory.
        let dir = tempfile::tempdir()?;
        let child_dir_path = dir.path().join("child");
        fs::create_dir(&child_dir_path)?;
        fs::write(
            dir.path().join("rust-toolchain"),
            "\n\n[toolchain]\n\n\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            child_dir_path.clone(),
            child_dir_path,
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain.toml` with toolchain in one line
        // This should not work!
        // See https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("rust-toolchain.toml"), "1.34.0")?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            dir.path().into(),
            dir.path().into(),
        );

        assert_eq!(find_rust_toolchain_file(&context), None);
        dir.close()?;

        // `rust-toolchain.toml` in toml format
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            dir.path().into(),
            dir.path().into(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain.toml` in toml format with new lines
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("rust-toolchain.toml"),
            "\n\n[toolchain]\n\n\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            dir.path().into(),
            dir.path().into(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain.toml` in parent directory.
        let dir = tempfile::tempdir()?;
        let child_dir_path = dir.path().join("child");
        fs::create_dir(&child_dir_path)?;
        fs::write(
            dir.path().join("rust-toolchain.toml"),
            "\n\n[toolchain]\n\n\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            child_dir_path.clone(),
            child_dir_path,
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()
    }
}
