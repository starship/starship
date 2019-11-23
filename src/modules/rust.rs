use std::ffi::OsStr;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{env, fs, str};

use log::warn;

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
    // 2. `$RUSTUP_HOME/settings.toml` or `~/.rustup/settings.toml`
    // 3. `rust-toolchain` in `.` or parent directories
    // as `rustup` does.
    // https://github.com/rust-lang/rustup.rs/tree/eb694fcada7becc5d9d160bf7c623abe84f8971d#override-precedence
    //
    // Probably we have no other way to know whether any toolchain override is specified for the
    // current directory. The following commands also cause toolchain installations.
    // - `rustup show`
    // - `rustup show active-toolchain`
    // - `rustup which`

    // `rustup` succeeds if it cannot find `settings.toml`.
    let rustup_settings = RustupSettings::load(&context.current_dir).unwrap_or_default();
    let toolchain_override = env_rustup_toolchain()
        .or_else(|| rustup_settings.lookup_override(&context.current_dir))
        .or_else(|| find_rust_toolchain_file(&context));

    let (version, toolchain) = if let Some(toolchain_override) = toolchain_override {
        match execute_rustup_run_rustc_version(&toolchain_override) {
            RustupRunRustcVersionOutcome::Ok(module_version) => (
                module_version,
                format_toolchain(&toolchain_override, rustup_settings.default_host_triple()),
            ),
            RustupRunRustcVersionOutcome::ToolchainNotInstalled => (
                "v?.?.?".to_owned(),
                format_toolchain(&toolchain_override, rustup_settings.default_host_triple()),
            ),
            RustupRunRustcVersionOutcome::RustupNotWorking => {
                // If `rustup` is not in `$PATH` or cannot be executed for other reasons, we can
                // safely execute `rustc --version`.
                execute_rustc_version_verbose(rustup_settings.default_toolchain())?
            }
            RustupRunRustcVersionOutcome::Err => return None,
        }
    } else {
        execute_rustc_version_verbose(rustup_settings.default_toolchain())?
    };

    let mut module = context.new_module("rust");
    let config = RustConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment(
        "version",
        &config.version.with_value(&if config.toolchain {
            format!("{} ({})", version, toolchain)
        } else {
            version
        }),
    );

    Some(module)
}

fn env_rustup_toolchain() -> Option<String> {
    // https://github.com/rust-lang/rustup.rs/blob/master/src/config.rs#L70-L72
    let val = env::var("RUSTUP_TOOLCHAIN").ok()?;
    if val.is_empty() {
        None
    } else {
        Some(val)
    }
}

fn find_rust_toolchain_file(context: &Context) -> Option<String> {
    // Look for 'rust-toolchain' as rustup does.
    // https://github.com/rust-lang/rustup.rs/blob/d84e6e50126bccd84649e42482fc35a11d019401/src/config.rs#L320-L358

    fn read_first_line(path: &Path) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let line = content.lines().next()?;
        Some(line.trim().to_owned())
    }

    if let Some(path) = context
        .get_dir_files()
        .ok()?
        .iter()
        .find(|p| p.file_name() == Some(OsStr::new("rust-toolchain")))
    {
        if let Some(toolchain) = read_first_line(path) {
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
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            return RustupRunRustcVersionOutcome::Ok(format_rustc_version(stdout));
        }
    } else if let Ok(stderr) = String::from_utf8(output.stderr) {
        if stderr.starts_with("error: toolchain '") && stderr.ends_with("' is not installed\n") {
            return RustupRunRustcVersionOutcome::ToolchainNotInstalled;
        }
    }
    RustupRunRustcVersionOutcome::Err
}

fn format_rustc_version(mut rustc_stdout: String) -> String {
    let offset = &rustc_stdout.find('(').unwrap_or_else(|| rustc_stdout.len());
    let formatted_version: String = rustc_stdout.drain(..offset).collect();

    format!("v{}", formatted_version.replace("rustc", "").trim())
}

fn format_toolchain(toolchain: &str, default_host_triple: Option<&str>) -> String {
    default_host_triple
        .map(|triple| toolchain.trim_end_matches(&format!("-{}", triple)))
        .unwrap_or(toolchain)
        .to_owned()
}

fn execute_rustc_version_verbose(toolchain: Option<&str>) -> Option<(String, String)> {
    let Output { status, stdout, .. } = Command::new("rustc").arg("-Vv").output().ok()?;
    if !status.success() {
        return None;
    }
    format_rustc_version_verbose(str::from_utf8(&stdout).ok()?, toolchain)
}

fn format_rustc_version_verbose(stdout: &str, toolchain: Option<&str>) -> Option<(String, String)> {
    let (mut release, mut host) = (None, None);
    for line in stdout.lines() {
        if line.starts_with("release: ") {
            release = Some(line.trim_start_matches("release: "));
        }
        if line.starts_with("host: ") {
            host = Some(line.trim_start_matches("host: "));
        }
    }
    let (release, host) = (release?, host?);
    let version = format!("v{}", release);
    let toolchain = toolchain
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("?-{}", host));
    Some((version, toolchain))
}

#[derive(Debug, PartialEq)]
enum RustupRunRustcVersionOutcome {
    Ok(String),
    ToolchainNotInstalled,
    RustupNotWorking,
    Err,
}

// https://github.com/rust-lang/rustup.rs/blob/13979c9685bfc4f5baa578ad13c1cd1999419dd1/src/settings.rs#L65-L72
#[derive(Default, Debug, PartialEq)]
struct RustupSettings {
    default_host_triple: Option<String>,
    default_toolchain: Option<String>,
    overrides: Vec<(PathBuf, String)>,
}

impl RustupSettings {
    fn load(cwd: &Path) -> Option<Self> {
        // `rustup` uses `home::home_dir`, which might work differently from `dirs::home_dir` on
        // Windows.

        // https://github.com/rust-lang/rustup.rs/blob/13979c9685bfc4f5baa578ad13c1cd1999419dd1/src/config.rs#L51-L55
        // https://docs.rs/home/0.5.1/src/home/lib.rs.html#169-183
        let path = env::var_os("RUSTUP_HOME")
            .map(|rustup_home| {
                if Path::new(&rustup_home).is_absolute() {
                    rustup_home.into()
                } else {
                    cwd.join(rustup_home)
                }
            })
            .or_else(|| dirs::home_dir().map(|d| d.join(".rustup")))?
            .join("settings.toml");
        let value = toml::from_str::<toml::Value>(&fs::read_to_string(path).ok()?).ok()?;
        Self::from_toml_value(&value)
    }

    fn from_toml_value(value: &toml::Value) -> Option<Self> {
        let version = value.get("version")?.as_str()?;
        // The version has not changed since 2015.
        if version != "12" {
            warn!("the version is {:?}. expected \"12\"", version);
        }
        Some(Self {
            default_host_triple: value
                .get("default_host_triple")
                .and_then(toml::Value::as_str)
                .map(ToOwned::to_owned),
            default_toolchain: value
                .get("default_toolchain")
                .and_then(toml::Value::as_str)
                .map(ToOwned::to_owned),
            overrides: value
                .get("overrides")
                .and_then(toml::Value::as_table)
                .map(|table| {
                    table
                        .into_iter()
                        .flat_map(|(k, v)| v.as_str().map(|v| (k.into(), v.to_owned())))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }

    fn default_host_triple(&self) -> Option<&str> {
        self.default_host_triple.as_ref().map(Deref::deref)
    }

    fn default_toolchain(&self) -> Option<&str> {
        self.default_toolchain.as_ref().map(Deref::deref)
    }

    fn lookup_override(&self, cwd: &Path) -> Option<String> {
        self.overrides
            .iter()
            .find(|(dir, _)| cwd.starts_with(dir))
            .map(|(_, name)| name.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use std::process::{ExitStatus, Output};
    use toml::toml;

    use super::*;

    #[test]
    fn test_rustup_settings_from_toml_value() {
        assert_eq!(
            RustupSettings::from_toml_value(&toml! {
                default_host_triple = "x86_64-unknown-linux-gnu"
                default_toolchain = "stable"
                version = "12"

                [overrides]
                "/home/user/src/starship" = "1.39.0-x86_64-unknown-linux-gnu"
            }),
            Some(RustupSettings {
                default_host_triple: Some("x86_64-unknown-linux-gnu".to_owned()),
                default_toolchain: Some("stable".to_owned()),
                overrides: vec![(
                    "/home/user/src/starship".into(),
                    "1.39.0-x86_64-unknown-linux-gnu".to_owned(),
                )]
            }),
        );

        assert_eq!(
            RustupSettings::from_toml_value(&toml! {
                default_host_triple = "x86_64-unknown-linux-gnu"
                default_toolchain = "stable"

                [overrides]
                "/home/user/src/starship" = "1.39.0-x86_64-unknown-linux-gnu"
            }),
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

        static OK: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"rustc 1.34.0\n"[..].to_owned(),
            stderr: b"whatever"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(OK.clone()),
            RustupRunRustcVersionOutcome::Ok("v1.34.0".to_owned()),
        );

        static TOOLCHAIN_NOT_INSTALLED: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: b"whatever"[..].to_owned(),
            stderr: b"error: toolchain 'channel-triple' is not installed\n"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(TOOLCHAIN_NOT_INSTALLED.clone()),
            RustupRunRustcVersionOutcome::ToolchainNotInstalled,
        );

        static INVALID_STDOUT: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"\xc3\x28"[..].to_owned(),
            stderr: b"whatever"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(INVALID_STDOUT.clone()),
            RustupRunRustcVersionOutcome::Err,
        );

        static INVALID_STDERR: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: b"whatever"[..].to_owned(),
            stderr: b"\xc3\x28"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(INVALID_STDERR.clone()),
            RustupRunRustcVersionOutcome::Err,
        );

        static UNEXPECTED_FORMAT_OF_ERROR: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: b"whatever"[..].to_owned(),
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

    #[test]
    fn test_format_toolchain() {
        assert_eq!(format_toolchain("stable", None), "stable");
        assert_eq!(
            format_toolchain("stable", Some("x86_64-unknown-linux")),
            "stable",
        );
        assert_eq!(
            format_toolchain("stable-x86_64-unknown-linux", None),
            "stable-x86_64-unknown-linux",
        );
        assert_eq!(
            format_toolchain("stable-x86_64-unknown-linux", Some("x86_64-unknown-linux")),
            "stable",
        );
    }

    #[test]
    fn test_format_rustc_version_verbose() {
        static STABLE: &str = r#"rustc 1.39.0 (4560ea788 2019-11-04)
binary: rustc
commit-hash: 4560ea788cb760f0a34127156c78e2552949f734
commit-date: 2019-11-04
host: x86_64-unknown-linux-gnu
release: 1.39.0
LLVM version: 9.0
"#;

        static BETA: &str = r#"rustc 1.40.0-beta.1 (76b40532a 2019-11-05)
binary: rustc
commit-hash: 76b40532a01d604385d3167710429d40b59905dd
commit-date: 2019-11-05
host: x86_64-unknown-linux-gnu
release: 1.40.0-beta.1
LLVM version: 9.0
"#;

        static NIGHTLY: &str = r#"rustc 1.40.0-nightly (1423bec54 2019-11-05)
binary: rustc
commit-hash: 1423bec54cf2db283b614e527cfd602b481485d1
commit-date: 2019-11-05
host: x86_64-unknown-linux-gnu
release: 1.40.0-nightly
LLVM version: 9.0
"#;

        assert_eq!(
            format_rustc_version_verbose(STABLE, None),
            Some((
                "v1.39.0".to_owned(),
                "?-x86_64-unknown-linux-gnu".to_owned(),
            )),
        );
        assert_eq!(
            format_rustc_version_verbose(STABLE, Some("stable")),
            Some(("v1.39.0".to_owned(), "stable".to_owned())),
        );

        assert_eq!(
            format_rustc_version_verbose(BETA, None),
            Some((
                "v1.40.0-beta.1".to_owned(),
                "?-x86_64-unknown-linux-gnu".to_owned(),
            )),
        );
        assert_eq!(
            format_rustc_version_verbose(BETA, Some("beta")),
            Some(("v1.40.0-beta.1".to_owned(), "beta".to_owned())),
        );

        assert_eq!(
            format_rustc_version_verbose(NIGHTLY, None),
            Some((
                "v1.40.0-nightly".to_owned(),
                "?-x86_64-unknown-linux-gnu".to_owned(),
            )),
        );
        assert_eq!(
            format_rustc_version_verbose(NIGHTLY, Some("nightly")),
            Some(("v1.40.0-nightly".to_owned(), "nightly".to_owned())),
        );
    }
}
