use std::path::Path;
use std::process::{Command, Output};
use std::{env, fs};

use super::{Context, Module, RootModuleConfig};

use crate::configs::rust::RustConfig;

/// Creates a module with the current Rust version
///
/// Will display the Rust version if any of the following criteria are met:
///     - Current directory contains a file with a `.rs` extension
///     - Current directory contains a `Cargo.toml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_rs_project = context
        .try_begin_scan()?
        .set_files(&["Cargo.toml"])
        .set_extensions(&["rs"])
        .is_match();

    if !is_rs_project {
        return None;
    }

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
    let module_version = if let Some(toolchain) = env_rustup_toolchain()
        .or_else(|| execute_rustup_override_list(&context.current_dir))
        .or_else(|| find_rust_toolchain_file(&context))
    {
        match execute_rustup_run_rustc_version(&toolchain) {
            RustupRunRustcVersionOutcome::RustcVersion(stdout) => format_rustc_version(stdout),
            RustupRunRustcVersionOutcome::ToolchainName(toolchain) => toolchain,
            RustupRunRustcVersionOutcome::RustupNotWorking => {
                // If `rustup` is not in `$PATH` or cannot be executed for other reasons, we can
                // safely execute `rustc --version`.
                format_rustc_version(execute_rustc_version()?)
            }
            RustupRunRustcVersionOutcome::Err => return None,
        }
    } else {
        format_rustc_version(execute_rustc_version()?)
    };

    let mut module = context.new_module("rust");
    let config = RustConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &config.version.with_value(&module_version));

    Some(module)
}

fn env_rustup_toolchain() -> Option<String> {
    let val = env::var("RUSTUP_TOOLCHAIN").ok()?;
    Some(val.trim().to_owned())
}

fn execute_rustup_override_list(cwd: &Path) -> Option<String> {
    let Output { stdout, .. } = Command::new("rustup")
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
    // https://github.com/rust-lang/rustup.rs/blob/d84e6e50126bccd84649e42482fc35a11d019401/src/config.rs#L320-L358

    fn read_first_line(path: &Path) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let line = content.lines().next()?;
        Some(line.trim().to_owned())
    }

    if let Ok(true) = context.dir_contents().map(|dir| dir.has_file("rust-toolchain")) {
        if let Some(toolchain) = read_first_line(Path::new("rust-toolchain")) {
            return Some(toolchain);
        }
    }

    let mut dir = &*context.current_dir;
    loop {
        if let Some(toolchain) = read_first_line(&dir.join("rust-toolchain")) {
            return Some(toolchain);
        }
        dir = dir.parent()?;
    }
}

fn execute_rustup_run_rustc_version(toolchain: &str) -> RustupRunRustcVersionOutcome {
    Command::new("rustup")
        .args(&["run", toolchain, "rustc", "--version"])
        .output()
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
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
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
    use once_cell::sync::Lazy;
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
        let nightly_input = String::from("rustc 1.34.0-nightly (b139669f3 2019-04-10)");
        assert_eq!(format_rustc_version(nightly_input), "v1.34.0-nightly");

        let beta_input = String::from("rustc 1.34.0-beta.1 (2bc1d406d 2019-04-10)");
        assert_eq!(format_rustc_version(beta_input), "v1.34.0-beta.1");

        let stable_input = String::from("rustc 1.34.0 (91856ed52 2019-04-10)");
        assert_eq!(format_rustc_version(stable_input), "v1.34.0");

        let version_without_hash = String::from("rustc 1.34.0");
        assert_eq!(format_rustc_version(version_without_hash), "v1.34.0");
    }
}
