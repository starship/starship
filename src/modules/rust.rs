use crate::utils::CommandOutput;
use std::fs;
use std::path::Path;
use std::process::ExitStatus;

use serde::Deserialize;

use super::{Context, Module, RootModuleConfig};

use crate::configs::rust::RustConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Rust version
pub async fn module<'a>(context: &'a Context<'a>) -> Option<Module<'a>> {
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

    let parsed = match StringFormatter::new(config.format) {
        Ok(formatter) => formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .async_map(|variable| async move {
                match variable.as_str() {
                    // This may result in multiple calls to `get_module_version` when a user have
                    // multiple `$version` variables defined in `format`.
                    "version" => get_module_version(context).await.map(Ok),
                    _ => None,
                }
            })
            .await
            .parse(None),
        Err(e) => Err(e),
    };

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `rust`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

async fn get_toolchain(context: &Context<'_>) -> Option<String> {
    // `$CARGO_HOME/bin/rustc(.exe) --version` may attempt installing a rustup toolchain.
    // https://github.com/starship/starship/issues/417
    //
    // To display appropriate versions preventing `rustc` from downloading toolchains, we have to
    // check
    // 1. `$RUSTUP_TOOLCHAIN`
    // 2. `rustup override list`
    // 3. `rust-toolchain` in `.` or parent directories
    // as `rustup` does.
    // https://github.com/rust-lang/rustup.rs/tree/eb694fcada7becc5d9d160bf7c623abe84f8971d#override-precedence
    //
    // Probably we have no other way to know whether any toolchain override is specified for the
    // current directory. The following commands also cause toolchain installations.
    // - `rustup show`
    // - `rustup show active-toolchain`
    // - `rustup which`
    if let Some(tool) = env_rustup_toolchain(context) {
        Some(tool)
    } else if let Some(tool) = execute_rustup_override_list(context).await {
        Some(tool)
    } else if let Some(tool) = find_rust_toolchain_file(&context) {
        Some(tool)
    } else {
        None
    }
}

async fn get_module_version(context: &Context<'_>) -> Option<String> {
    let module_version = if let Some(toolchain) = get_toolchain(context).await {
        match execute_rustup_run_rustc_version(context, &toolchain).await {
            RustupRunRustcVersionOutcome::RustcVersion(stdout) => format_rustc_version(stdout),
            RustupRunRustcVersionOutcome::ToolchainName(toolchain) => toolchain,
            RustupRunRustcVersionOutcome::RustupNotWorking => {
                // If `rustup` is not in `$PATH` or cannot be executed for other reasons, we can
                // safely execute `rustc --version`.
                format_rustc_version(execute_rustc_version(context).await?)
            }
            RustupRunRustcVersionOutcome::Err => return None,
        }
    } else {
        format_rustc_version(execute_rustc_version(context).await?)
    };

    Some(module_version)
}

fn env_rustup_toolchain(context: &Context) -> Option<String> {
    let val = context.get_env("RUSTUP_TOOLCHAIN")?;
    Some(val.trim().to_owned())
}

async fn execute_rustup_override_list(context: &Context<'_>) -> Option<String> {
    let stdout = context
        .exec_cmd("rustup", &["override", "list"])
        .await?
        .stdout;
    extract_toolchain_from_rustup_override_list(&stdout, &context.current_dir)
}

fn extract_toolchain_from_rustup_override_list(stdout: &str, cwd: &Path) -> Option<String> {
    if stdout == "no overrides\n" {
        return None;
    }
    stdout
        .lines()
        .flat_map(|line| {
            let mut words = line.split_whitespace();
            let dir = words.next()?;
            let toolchain = words.next()?;
            Some((dir, toolchain))
        })
        .find(|(dir, _)| cwd.starts_with(dir))
        .map(|(_, toolchain)| toolchain.to_owned())
}

fn find_rust_toolchain_file(context: &Context) -> Option<String> {
    // Look for 'rust-toolchain' as rustup does.
    // https://github.com/rust-lang/rustup/blob/89912c4cf51645b9c152ab7380fd07574fec43a3/src/config.rs#L546-L616

    #[derive(Deserialize)]
    struct OverrideFile {
        toolchain: ToolchainSection,
    }

    #[derive(Deserialize)]
    struct ToolchainSection {
        channel: Option<String>,
    }

    fn read_channel(path: &Path) -> Option<String> {
        let contents = fs::read_to_string(path).ok()?;

        match contents.lines().count() {
            0 => None,
            1 => Some(contents),
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

    if let Ok(true) = context
        .dir_contents()
        .map(|dir| dir.has_file("rust-toolchain"))
    {
        if let Some(toolchain) = read_channel(Path::new("rust-toolchain")) {
            return Some(toolchain);
        }
    }

    let mut dir = &*context.current_dir;
    loop {
        if let Some(toolchain) = read_channel(&dir.join("rust-toolchain")) {
            return Some(toolchain);
        }
        dir = dir.parent()?;
    }
}

async fn execute_rustup_run_rustc_version(
    context: &Context<'_>,
    toolchain: &str,
) -> RustupRunRustcVersionOutcome {
    context
        .exec_cmd_status("rustup", &["run", toolchain, "rustc", "--version"])
        .await
        .map(extract_toolchain_from_rustup_run_rustc_version)
        .unwrap_or(RustupRunRustcVersionOutcome::RustupNotWorking)
}

fn extract_toolchain_from_rustup_run_rustc_version(
    (status, output): (ExitStatus, CommandOutput),
) -> RustupRunRustcVersionOutcome {
    if status.success() {
        return RustupRunRustcVersionOutcome::RustcVersion(output.stdout);
    } else {
        let stderr = output.stderr;
        if stderr.starts_with("error: toolchain '") && stderr.ends_with("' is not installed\n") {
            let stderr = stderr
                ["error: toolchain '".len()..stderr.len() - "' is not installed\n".len()]
                .to_owned();
            return RustupRunRustcVersionOutcome::ToolchainName(stderr);
        }
    }
    RustupRunRustcVersionOutcome::Err
}

async fn execute_rustc_version(context: &Context<'_>) -> Option<String> {
    context
        .exec_cmd("rustc", &["--version"])
        .await
        .map(|output| output.stdout)
}

fn format_rustc_version(mut rustc_stdout: String) -> String {
    let offset = &rustc_stdout.find('(').unwrap_or_else(|| rustc_stdout.len());
    let formatted_version: String = rustc_stdout.drain(..offset).collect();

    format!("v{}", formatted_version.replace("rustc", "").trim())
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
    use std::io;

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

        let rustc_version = (
            ExitStatus::from_raw(0),
            CommandOutput {
                stdout: "rustc 1.34.0\n".into(),
                stderr: "".into(),
            },
        );
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(rustc_version),
            RustupRunRustcVersionOutcome::RustcVersion("rustc 1.34.0\n".to_owned()),
        );

        let toolchain_name = (
            ExitStatus::from_raw(1),
            CommandOutput {
                stdout: "".into(),
                stderr: "error: toolchain 'channel-triple' is not installed\n".into(),
            },
        );
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(toolchain_name),
            RustupRunRustcVersionOutcome::ToolchainName("channel-triple".to_owned()),
        );

        let unexpected_format_of_error = (
            ExitStatus::from_raw(1),
            CommandOutput {
                stdout: "".into(),
                stderr: "error:".into(),
            },
        );
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(unexpected_format_of_error),
            RustupRunRustcVersionOutcome::Err,
        );
    }

    #[test]
    fn test_format_rustc_version() {
        let nightly_input = String::from("rustc 1.34.0-nightly (b139669f3 2019-04-10)");
        assert_eq!(format_rustc_version(nightly_input), "v1.34.0-nightly");

        let beta_input = String::from("rustc 1.34.0-beta.1 (2bc1d406d 2019-04-10)");
        assert_eq!(format_rustc_version(beta_input), "v1.34.0-beta.1");

        let stable_input = String::from("rustc 1.34.0 (91856ed52 2019-04-10)");
        assert_eq!(format_rustc_version(stable_input), "v1.34.0");

        let version_without_hash = String::from("rustc 1.34.0");
        assert_eq!(format_rustc_version(version_without_hash), "v1.34.0");
    }

    #[test]
    fn test_find_rust_toolchain_file() -> io::Result<()> {
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
        dir.close()
    }
}
