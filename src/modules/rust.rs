use std::ffi::OsStr;
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

    let mut module = context.new_module("rust");
    let config = RustConfig::try_load(module.config);

    let (version, toolchain) = if let Some(toolchain_override) = toolchain_override {
        match execute_rustup_run_rustc_version(&toolchain_override, config.toolchain) {
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
                execute_rustc_version_verbose(
                    rustup_settings.default_toolchain(),
                    config.toolchain,
                )?
            }
            RustupRunRustcVersionOutcome::Err => return None,
        }
    } else {
        execute_rustc_version_verbose(rustup_settings.default_toolchain(), config.toolchain)?
    };

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

fn execute_rustup_run_rustc_version(
    toolchain: &str,
    core_only: bool,
) -> RustupRunRustcVersionOutcome {
    Command::new("rustup")
        .args(&["run", toolchain, "rustc", "--version"])
        .output()
        .map(|o| extract_toolchain_from_rustup_run_rustc_version(o, core_only))
        .unwrap_or(RustupRunRustcVersionOutcome::RustupNotWorking)
}

fn extract_toolchain_from_rustup_run_rustc_version(
    output: Output,
    core_only: bool,
) -> RustupRunRustcVersionOutcome {
    if output.status.success() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            return RustupRunRustcVersionOutcome::Ok(format_rustc_version(&stdout, core_only));
        }
    } else if let Ok(stderr) = String::from_utf8(output.stderr) {
        if stderr.starts_with("error: toolchain '") && stderr.ends_with("' is not installed\n") {
            return RustupRunRustcVersionOutcome::ToolchainNotInstalled;
        }
    }
    RustupRunRustcVersionOutcome::Err
}

fn format_rustc_version(short_version_string: &str, core_only: bool) -> String {
    let release = short_version_string
        .split_whitespace()
        .nth(1)
        .unwrap_or(short_version_string);
    format_semver(release, core_only)
}

fn format_toolchain(toolchain: &str, default_host_triple: Option<&str>) -> String {
    default_host_triple
        .map(|triple| toolchain.trim_end_matches(&format!("-{}", triple)))
        .unwrap_or(toolchain)
        .to_owned()
}

fn execute_rustc_version_verbose(
    toolchain: Option<&str>,
    core_only: bool,
) -> Option<(String, String)> {
    let Output { status, stdout, .. } = Command::new("rustc").arg("-Vv").output().ok()?;
    if !status.success() {
        return None;
    }
    format_rustc_version_verbose(str::from_utf8(&stdout).ok()?, toolchain, core_only)
}

fn format_rustc_version_verbose(
    stdout: &str,
    toolchain: Option<&str>,
    core_only: bool,
) -> Option<(String, String)> {
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
    let version = format_semver(release, core_only);
    let toolchain = toolchain
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("?-{}", host));
    Some((version, toolchain))
}

fn format_semver(semver: &str, core_only: bool) -> String {
    format!(
        "v{}",
        if core_only {
            semver.find('-').map(|i| &semver[..i]).unwrap_or(semver)
        } else {
            semver
        },
    )
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
        self.default_host_triple.as_deref()
    }

    fn default_toolchain(&self) -> Option<&str> {
        self.default_toolchain.as_deref()
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
                "/home/user/src/starship" = "1.40.0-x86_64-unknown-linux-gnu"
            }),
            Some(RustupSettings {
                default_host_triple: Some("x86_64-unknown-linux-gnu".to_owned()),
                default_toolchain: Some("stable".to_owned()),
                overrides: vec![(
                    "/home/user/src/starship".into(),
                    "1.40.0-x86_64-unknown-linux-gnu".to_owned(),
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

        macro_rules! test {
            () => {};
            (($output:ident, $core_only:expr) => $expected:expr $(,$($rest:tt)*)?) => {
                assert_eq!(
                    extract_toolchain_from_rustup_run_rustc_version($output.clone(), $core_only),
                    $expected,
                );
                test!($($($rest)*)?);
            };
        }

        static STABLE: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"rustc 1.40.0 (73528e339 2019-12-16)\n"[..].to_owned(),
            stderr: b"whatever"[..].to_owned(),
        });

        static BETA: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"rustc 1.41.0-beta.1 (eb3f7c2d3 2019-12-17)\n"[..].to_owned(),
            stderr: b"whatever"[..].to_owned(),
        });

        static NIGHTLY: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"rustc 1.42.0-nightly (da3629b05 2019-12-29)\n"[..].to_owned(),
            stderr: b"whatever"[..].to_owned(),
        });

        static TOOLCHAIN_NOT_INSTALLED: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: b"whatever"[..].to_owned(),
            stderr: b"error: toolchain 'channel-triple' is not installed\n"[..].to_owned(),
        });

        static INVALID_STDOUT: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"\xc3\x28"[..].to_owned(),
            stderr: b"whatever"[..].to_owned(),
        });

        static INVALID_STDERR: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: b"whatever"[..].to_owned(),
            stderr: b"\xc3\x28"[..].to_owned(),
        });

        static UNEXPECTED_FORMAT_OF_ERROR: Lazy<Output> = Lazy::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: b"whatever"[..].to_owned(),
            stderr: b"error:"[..].to_owned(),
        });

        test!(
            (STABLE, false) => RustupRunRustcVersionOutcome::Ok("v1.40.0".to_owned()),
            (STABLE, true) => RustupRunRustcVersionOutcome::Ok("v1.40.0".to_owned()),
            (BETA, false) => RustupRunRustcVersionOutcome::Ok("v1.41.0-beta.1".to_owned()),
            (BETA, true) => RustupRunRustcVersionOutcome::Ok("v1.41.0".to_owned()),
            (NIGHTLY, false) => RustupRunRustcVersionOutcome::Ok("v1.42.0-nightly".to_owned()),
            (NIGHTLY, true) => RustupRunRustcVersionOutcome::Ok("v1.42.0".to_owned()),
            (TOOLCHAIN_NOT_INSTALLED, false) => RustupRunRustcVersionOutcome::ToolchainNotInstalled,
            (TOOLCHAIN_NOT_INSTALLED, true) => RustupRunRustcVersionOutcome::ToolchainNotInstalled,
            (INVALID_STDOUT, false) => RustupRunRustcVersionOutcome::Err,
            (INVALID_STDOUT, true) => RustupRunRustcVersionOutcome::Err,
            (INVALID_STDERR, false) => RustupRunRustcVersionOutcome::Err,
            (INVALID_STDERR, true) => RustupRunRustcVersionOutcome::Err,
            (UNEXPECTED_FORMAT_OF_ERROR, false) => RustupRunRustcVersionOutcome::Err,
            (UNEXPECTED_FORMAT_OF_ERROR, true) => RustupRunRustcVersionOutcome::Err,
        );
    }

    #[test]
    fn test_format_rustc_version() {
        macro_rules! test {
            () => {};
            (($input:expr, $core_only:expr) => $expected:expr $(,$($rest:tt)*)?) => {
                assert_eq!(format_rustc_version($input, $core_only), $expected);
                test!($($($rest)*)?);
            };
        }

        static STABLE: &str = "rustc 1.40.0 (73528e339 2019-12-16)";
        static BETA: &str = "rustc 1.41.0-beta.1 (eb3f7c2d3 2019-12-17)";
        static NIGHTLY: &str = "rustc 1.42.0-nightly (da3629b05 2019-12-29)";
        static STABLE_WITHOUT_HASH: &str = "rustc 1.40.0";

        test!(
            (NIGHTLY, false) => "v1.42.0-nightly",
            (NIGHTLY, true) => "v1.42.0",
            (BETA, false) => "v1.41.0-beta.1",
            (BETA, true) => "v1.41.0",
            (STABLE, false) => "v1.40.0",
            (STABLE, true) => "v1.40.0",
            (STABLE_WITHOUT_HASH, false) => "v1.40.0",
            (STABLE_WITHOUT_HASH, true) => "v1.40.0",
        );
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
        macro_rules! test {
            () => {};
            (($input:expr, $toolchain:expr, $core_only:expr) => $expected:expr $(,$($rest:tt)*)?) => {
                assert_eq!(
                    format_rustc_version_verbose($input, $toolchain, $core_only)
                        .as_ref()
                        .map(|(s1, s2)| (&**s1, &**s2)),
                    $expected,
                );
                test!($($($rest)*)?);
            };
        }

        static STABLE: &str = r#"rustc 1.40.0 (73528e339 2019-12-16)
binary: rustc
commit-hash: 73528e339aae0f17a15ffa49a8ac608f50c6cf14
commit-date: 2019-12-16
host: x86_64-unknown-linux-gnu
release: 1.40.0
LLVM version: 9.0
"#;

        static BETA: &str = r#"rustc 1.41.0-beta.1 (eb3f7c2d3 2019-12-17)
binary: rustc
commit-hash: eb3f7c2d3aec576f47eba854cfbd3c1187b8a2a0
commit-date: 2019-12-17
host: x86_64-unknown-linux-gnu
release: 1.41.0-beta.1
LLVM version: 9.0
"#;

        static NIGHTLY: &str = r#"rustc 1.42.0-nightly (da3629b05 2019-12-29)
binary: rustc
commit-hash: da3629b05f8f1b425a738bfe9fe9aedd47c5417a
commit-date: 2019-12-29
host: x86_64-unknown-linux-gnu
release: 1.42.0-nightly
LLVM version: 9.0
"#;

        test!(
            (STABLE, None, false) => Some(("v1.40.0", "?-x86_64-unknown-linux-gnu")),
            (STABLE, None, true) => Some(("v1.40.0", "?-x86_64-unknown-linux-gnu")),
            (STABLE, Some("stable"), false) => Some(("v1.40.0", "stable")),
            (STABLE, Some("stable"), true) => Some(("v1.40.0", "stable")),
            (BETA, None, false) => Some(("v1.41.0-beta.1", "?-x86_64-unknown-linux-gnu")),
            (BETA, None, true) => Some(("v1.41.0", "?-x86_64-unknown-linux-gnu")),
            (BETA, Some("beta"), false) => Some(("v1.41.0-beta.1", "beta")),
            (BETA, Some("beta"), true) => Some(("v1.41.0", "beta")),
            (NIGHTLY, None, false) => Some(("v1.42.0-nightly", "?-x86_64-unknown-linux-gnu")),
            (NIGHTLY, None, true) => Some(("v1.42.0", "?-x86_64-unknown-linux-gnu")),
            (NIGHTLY, Some("nightly"), false) => Some(("v1.42.0-nightly", "nightly")),
            (NIGHTLY, Some("nightly"), true) => Some(("v1.42.0", "nightly")),
            ("", None, false) => None,
            ("", None, true) => None,
            ("", Some("stable"), false) => None,
            ("", Some("stable"), true) => None,
        );
    }
}
