use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::OnceLock;
use std::time::Duration;

use guess_host_triple::guess_host_triple;
use process_control::{ChildExt, Control};
use serde::Deserialize;

use super::{Context, Module, ModuleConfig};
use crate::configs::rust::RustConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::{create_command, read_file};

/// Lazily shares toolchain selection and version detection within one module render.
struct RustToolchainInfo {
    /// Rustup settings parsed from $HOME/.rustup/settings.toml
    rustup_settings: OnceLock<RustupSettings>,
    /// Version and label from the same compiler, shared by all format variables
    compiler_info: OnceLock<CompilerInfo>,
}

#[derive(Debug)]
struct CompilerInfo {
    version: VersionDetection,
    toolchain: Option<String>,
}

impl CompilerInfo {
    fn new(version: VersionDetection, toolchain: Option<String>) -> Self {
        let toolchain = toolchain.or_else(|| match &version {
            VersionDetection::VerboseVersion { host, .. } => Some(host.clone()),
            _ => None,
        });
        Self { version, toolchain }
    }
}

impl RustToolchainInfo {
    fn new() -> Self {
        Self {
            rustup_settings: OnceLock::new(),
            compiler_info: OnceLock::new(),
        }
    }

    fn default_host_triple(&self, context: &Context) -> Option<&str> {
        match self.rustup_settings(context).default_host_triple() {
            Some(triple) => Some(triple),
            None => guess_host_triple(),
        }
    }

    fn rustup_settings(&self, context: &Context) -> &RustupSettings {
        self.rustup_settings
            .get_or_init(|| RustupSettings::load(context).unwrap_or_default())
    }

    /// Resolves the selected rustup toolchain without installing it.
    fn selected_toolchain(&self, context: &Context, rustup: Option<&Path>) -> Option<String> {
        // `$CARGO_HOME/bin/rustc(.exe) --version` may attempt installing a rustup toolchain.
        // https://github.com/starship/starship/issues/417
        //
        // To display appropriate versions preventing `rustc` from downloading toolchains, we have to
        // check
        // 1. `$RUSTUP_TOOLCHAIN`
        // 2. The override list from ~/.rustup/settings.toml (like `rustup override list`)
        // 3. `rust-toolchain` or `rust-toolchain.toml` in `.` or parent directories
        // 4. The `default_toolchain` from ~/.rustup/settings.toml (like `rustup default`)
        // 5. `rustup default` (in addition to the above, this also looks at global fallback config files)
        // as `rustup` does.
        // https://github.com/rust-lang/rustup.rs/tree/eb694fcada7becc5d9d160bf7c623abe84f8971d#override-precedence
        //
        // Probably we have no other way to know whether any toolchain override is specified for the
        // current directory. The following commands also cause toolchain installations.
        // - `rustup show`
        // - `rustup show active-toolchain`
        // - `rustup which`
        let out = env_rustup_toolchain(context)
            .or_else(|| {
                self.rustup_settings(context)
                    .lookup_override(context.current_dir.as_path())
            })
            .or_else(|| find_rust_toolchain_file(context))
            .or_else(|| {
                self.rustup_settings(context)
                    .default_toolchain()
                    .map(std::string::ToString::to_string)
            })
            .or_else(|| rustup.and_then(|rustup| execute_rustup_default(context, rustup)));

        log::debug!("Selected rustup toolchain is {out:?}");
        out
    }

    /// Detects the selected toolchain version, preferring metadata over execution.
    fn toolchain_version(&self, context: &Context) -> &VersionDetection {
        &self.compiler_info(context).version
    }

    fn compiler_info(&self, context: &Context) -> &CompilerInfo {
        self.compiler_info
            .get_or_init(|| match rustc_source(context) {
                RustcSource::Missing => CompilerInfo::new(VersionDetection::Unavailable, None),
                RustcSource::RustupProxy(rustup) => {
                    self.rustup_compiler_info(context, Some(&rustup))
                }
                RustcSource::Toolchain {
                    program,
                    directory,
                    name,
                } => {
                    let version = get_version_from_toolchain_dir(&directory)
                        .map(VersionDetection::StandardVersion)
                        .unwrap_or_else(|| match query_rustc(context, &program) {
                            RustcQuery::Version(version) => version,
                            RustcQuery::RustupProxy => VersionDetection::Unavailable,
                        });
                    CompilerInfo::new(
                        version,
                        Some(format_toolchain(&name, self.default_host_triple(context))),
                    )
                }
                RustcSource::Other(program) => match query_rustc(context, &program) {
                    RustcQuery::Version(version) => CompilerInfo::new(version, None),
                    RustcQuery::RustupProxy => self
                        .rustup_compiler_info(context, find_rustup(context, &program).as_deref()),
                },
            })
    }

    fn rustup_compiler_info(&self, context: &Context, rustup: Option<&Path>) -> CompilerInfo {
        let Some(toolchain) = self.selected_toolchain(context, rustup) else {
            log::debug!("No rustup toolchain selected; skipping proxy execution");
            return CompilerInfo::new(VersionDetection::Unavailable, None);
        };
        let host_triple = self.default_host_triple(context);
        let directory = rustup_home(context)
            .ok()
            .and_then(|home| find_toolchain_dir(&toolchain, host_triple, &home.join("toolchains")));
        let version = directory
            .as_deref()
            .and_then(get_version_from_toolchain_dir)
            .map(VersionDetection::StandardVersion)
            .unwrap_or_else(|| {
                run_rustc_from_toolchain(
                    &toolchain,
                    directory.as_deref(),
                    rustup,
                    |program, args| rust_command(context, program, args, &toolchain),
                )
            });
        CompilerInfo::new(version, Some(format_toolchain(&toolchain, host_triple)))
    }
}

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

    let toolchain_info = RustToolchainInfo::new();

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
                "version" => get_module_version(context, &config, &toolchain_info).map(Ok),
                "numver" => get_module_numeric_version(context, &toolchain_info).map(Ok),
                "toolchain" => get_toolchain_name(context, &toolchain_info).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `rust`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_module_version(
    context: &Context,
    config: &RustConfig,
    toolchain_info: &RustToolchainInfo,
) -> Option<String> {
    type Outcome = VersionDetection;

    match toolchain_info.toolchain_version(context) {
        Outcome::StandardVersion(rustc_version) => {
            format_rustc_version(rustc_version, config.version_format)
        }
        Outcome::VerboseVersion { release, .. } => {
            Some(format_rust_release(release, config.version_format))
        }
        Outcome::ToolchainNotInstalled(name) => Some(name.clone()),
        Outcome::Unavailable => None,
    }
}

fn get_module_numeric_version(
    context: &Context,
    toolchain_info: &RustToolchainInfo,
) -> Option<String> {
    type Outcome = VersionDetection;

    match toolchain_info.toolchain_version(context) {
        Outcome::StandardVersion(version) => {
            let release = version.split_whitespace().nth(1).unwrap_or(version);
            Some(format_semver(release))
        }
        Outcome::VerboseVersion { release, .. } => Some(format_semver(release)),
        Outcome::ToolchainNotInstalled(_) | Outcome::Unavailable => None,
    }
}

fn get_toolchain_name(context: &Context, toolchain_info: &RustToolchainInfo) -> Option<String> {
    toolchain_info.compiler_info(context).toolchain.clone()
}

fn rustup_home(context: &Context) -> std::io::Result<PathBuf> {
    if cfg!(test) {
        let rustup_home = context.get_env_os("RUSTUP_HOME");
        if let Some(path) = rustup_home {
            return Ok(PathBuf::from(path));
        }
    }
    home::rustup_home_with_cwd(&context.current_dir)
}

fn env_rustup_toolchain(context: &Context) -> Option<String> {
    log::trace!("Searching for rustup toolchain in environment.");
    let val = context.get_env("RUSTUP_TOOLCHAIN")?;
    Some(val.trim().to_owned())
}

fn execute_rustup_default(context: &Context, rustup: &Path) -> Option<String> {
    log::trace!("Searching for toolchain with rustup default");
    // `rustup default` output is:
    //    stable-x86_64-apple-darwin (default)
    context
        .exec_cmd(rustup, &["default"])?
        .stdout
        .split_whitespace()
        .next()
        .map(str::to_owned)
}

fn find_rust_toolchain_file(context: &Context) -> Option<String> {
    log::trace!("Searching for toolchain in toolchain file");
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
        let contents = read_file(path).ok()?;

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
        .filter(|c| {
            let valid = is_valid_toolchain_name(c);
            if !valid {
                log::warn!(
                    "Ignoring toolchain '{c}' from {path:?}: invalid or path-based toolchain names are not permitted"
                );
            }
            valid
        })
    }

    let directory = context
        .begin_ancestor_scan()
        .set_files(&["rust-toolchain", "rust-toolchain.toml"])
        .scan()?;

    read_channel(&directory.join("rust-toolchain"), false)
        .or_else(|| read_channel(&directory.join("rust-toolchain.toml"), true))
}

fn parse_toolchain_version_output(output: Output) -> VersionDetection {
    if output.status.success() {
        if let Ok(output) = String::from_utf8(output.stdout) {
            return VersionDetection::StandardVersion(output);
        }
    } else if let Ok(stderr) = String::from_utf8(output.stderr)
        && stderr.starts_with("error: toolchain '")
        && stderr.ends_with("' is not installed\n")
    {
        let stderr = stderr
            ["error: toolchain '".len()..stderr.len() - "' is not installed\n".len()]
            .to_owned();
        return VersionDetection::ToolchainNotInstalled(stderr);
    }
    VersionDetection::Unavailable
}

enum RustcQuery {
    Version(VersionDetection),
    RustupProxy,
}

fn query_rustc(context: &Context, program: &Path) -> RustcQuery {
    // Match Context's command mocks while retaining failed-command stderr and
    // child-only environment overrides in the real execution path.
    #[cfg(test)]
    if let Some(output) = context.cmd.get("rustc -Vv") {
        return RustcQuery::Version(
            output
                .as_ref()
                .and_then(|output| parse_rustc_verbose_version(&output.stdout))
                .unwrap_or(VersionDetection::Unavailable),
        );
    }

    // A wrapper or a proxy left behind during self-update may not share the
    // manager's file identity. Give it an absent custom toolchain name: rustup
    // cannot download custom toolchains, and the environment takes precedence
    // over project override files. Real compilers ignore these rustup variables.
    let probe = format!("starship-probe-{:032x}", rand::random::<u128>());
    let output = rust_command(context, program, &["-Vv"], &probe);
    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8(output.stdout)
                .ok()
                .and_then(|stdout| parse_rustc_verbose_version(&stdout))
                .unwrap_or(VersionDetection::Unavailable);
            RustcQuery::Version(version)
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains(&probe)
                && stderr.contains("toolchain")
                && (stderr.contains("not installed") || stderr.contains("invalid"))
            {
                RustcQuery::RustupProxy
            } else {
                log::debug!("Compiler at {program:?} failed to report its version: {stderr}");
                RustcQuery::Version(VersionDetection::Unavailable)
            }
        }
        Err(error) => {
            log::debug!("Failed to query compiler at {program:?}: {error}");
            RustcQuery::Version(VersionDetection::Unavailable)
        }
    }
}

fn rust_command(
    context: &Context,
    program: &Path,
    args: &[&str],
    toolchain: &str,
) -> std::io::Result<Output> {
    let mut command = create_command(program)?;
    #[cfg(test)]
    command.envs(&context.env.env);
    command
        .args(args)
        .current_dir(&context.current_dir)
        .env("RUSTUP_TOOLCHAIN", toolchain)
        .env("RUSTUP_AUTO_INSTALL", "0")
        .spawn()?
        .controlled_with_output()
        .time_limit(Duration::from_millis(context.root_config.command_timeout))
        .terminate_for_timeout()
        .wait()?
        .map(process_control::Output::into_std_lossy)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Rust version command timed out",
            )
        })
}

#[derive(Debug, PartialEq, Eq)]
enum RustcSource {
    Missing,
    RustupProxy(PathBuf),
    Toolchain {
        program: PathBuf,
        directory: PathBuf,
        name: String,
    },
    Other(PathBuf),
}

/// File identity establishes a proxy; directory names and neighboring files do
/// not. Unknown wrappers are left to the protected compiler query.
fn rustc_source(context: &Context) -> RustcSource {
    let Ok(rustc_path) = which::which_in("rustc", context.get_env_os("PATH"), &context.current_dir)
    else {
        log::trace!("No `rustc` found in PATH");
        return RustcSource::Missing;
    };
    let resolved = dunce::canonicalize(&rustc_path).unwrap_or_else(|_| rustc_path.clone());
    if let Ok(compiler) = same_file::Handle::from_path(&rustc_path)
        && let Some(rustup) = rustup_candidates(context, &rustc_path, &resolved).find(|rustup| {
            same_file::Handle::from_path(rustup).is_ok_and(|manager| manager == compiler)
        })
    {
        return RustcSource::RustupProxy(rustup);
    }

    if let Ok(home) = rustup_home(context) {
        let root = context.current_dir.join(home).join("toolchains");
        let resolved_root = dunce::canonicalize(&root).unwrap_or_else(|_| root.clone());
        for root in [&root, &resolved_root] {
            for program in [&rustc_path, &resolved] {
                let Ok(relative) = program.strip_prefix(root) else {
                    continue;
                };
                let parts = relative.components().collect::<Vec<_>>();
                if parts.len() == 3
                    && parts[1].as_os_str() == "bin"
                    && parts[2].as_os_str()
                        == format!("rustc{}", std::env::consts::EXE_SUFFIX).as_str()
                    && let Some(name) = parts[0].as_os_str().to_str()
                {
                    return RustcSource::Toolchain {
                        program: rustc_path.clone(),
                        directory: root.join(name),
                        name: name.to_owned(),
                    };
                }
            }
        }
    }
    RustcSource::Other(rustc_path)
}

fn rustup_candidates(
    context: &Context,
    compiler: &Path,
    resolved: &Path,
) -> impl Iterator<Item = PathBuf> {
    // A shared package-manager prefix may expose an unrelated rustup. Prefer
    // the manager next to the actual wrapper before checking that prefix/PATH.
    let name = format!("rustup{}", std::env::consts::EXE_SUFFIX);
    [
        resolved.with_file_name(&name),
        compiler.with_file_name(&name),
    ]
    .into_iter()
    .chain(which::which_in(
        "rustup",
        context.get_env_os("PATH"),
        &context.current_dir,
    ))
}

fn find_rustup(context: &Context, compiler: &Path) -> Option<PathBuf> {
    let resolved = dunce::canonicalize(compiler).unwrap_or_else(|_| compiler.to_path_buf());
    rustup_candidates(context, compiler, &resolved).find(|path| path.is_file())
}

/// Runs an installed toolchain's compiler, or uses `rustup run` when no
/// installed directory was resolved. Unlike `Context::exec_cmd`, this needs
/// failed-command stderr to distinguish a missing toolchain from a broken compiler.
/// The command boundary is injectable without changing the process environment.
fn run_rustc_from_toolchain(
    toolchain: &str,
    directory: Option<&Path>,
    rustup: Option<&Path>,
    run: impl FnOnce(&Path, &[&str]) -> std::io::Result<Output>,
) -> VersionDetection {
    let output = if let Some(directory) = directory {
        let rustc = directory
            .join("bin")
            .join(format!("rustc{}", std::env::consts::EXE_SUFFIX));
        run(&rustc, &["--version"])
    } else if let Some(rustup) = rustup {
        // `rustup run` does not install a missing toolchain.
        run(rustup, &["run", "--", toolchain, "rustc", "--version"])
    } else {
        log::debug!("No rustup manager found for toolchain {toolchain:?}");
        return VersionDetection::Unavailable;
    };
    match output {
        Ok(output) => parse_toolchain_version_output(output),
        Err(err) => {
            log::debug!("Failed to execute rustc/rustup for toolchain {toolchain:?}: {err}");
            VersionDetection::Unavailable
        }
    }
}

fn is_valid_toolchain_name(name: &str) -> bool {
    // A valid toolchain name is a single normal path component matching the
    // input verbatim (rejecting `.`, `..`, roots, Windows drive prefixes like
    // `C:`, and trailing `/`, `\.`, etc.).
    let mut components = Path::new(name).components();
    matches!(
        (components.next(), components.next()),
        (Some(std::path::Component::Normal(c)), None) if c == name
    )
}

/// Resolves a toolchain name to its directory under `~/.rustup/toolchains`.
///
/// The name may be fully qualified (`stable-aarch64-apple-darwin`) or a short
/// channel name (`stable`); short names are completed with the default host
/// triple. If neither exact name is installed, resolution is left to rustup.
///
/// Path-based toolchains (custom toolchain directories) return `None`: they
/// are not installed under `~/.rustup/toolchains` and their on-disk metadata
/// may not match the actual compiler, so the caller should fall through to
/// running rustc/rustup instead.
fn find_toolchain_dir(
    toolchain: &str,
    host_triple: Option<&str>,
    toolchains_dir: &Path,
) -> Option<PathBuf> {
    if !is_valid_toolchain_name(toolchain) {
        log::debug!(
            "Skipping toolchain directory lookup for {toolchain:?}: not a plain toolchain name"
        );
        return None;
    }

    let exact = toolchains_dir.join(toolchain);
    if exact.is_dir() {
        log::trace!("Resolved toolchain {toolchain:?} to {exact:?}");
        return Some(exact);
    }

    if let Some(triple) = host_triple {
        let qualified = toolchains_dir.join(format!("{toolchain}-{triple}"));
        if qualified.is_dir() {
            log::trace!("Resolved toolchain {toolchain:?} to {qualified:?}");
            return Some(qualified);
        }
    }

    log::trace!(
        "No installed toolchain directory found for {toolchain:?} under {toolchains_dir:?}"
    );
    None
}

/// Reads the Rust version from an installed toolchain's `rustc(1)` man page
/// header without spawning a subprocess. Returns `None` if the compiler or a
/// usable man page is missing so the caller uses the existing subprocess fallback.
/// Returns a string in `rustc --version` style, e.g.
/// `"rustc 1.77.0 (aeda7d245 2024-03-13)"`.
fn get_version_from_toolchain_dir(toolchain_dir: &Path) -> Option<String> {
    // Metadata can remain after the compiler component has been removed.
    if !toolchain_dir
        .join("bin")
        .join(format!("rustc{}", std::env::consts::EXE_SUFFIX))
        .is_file()
    {
        log::trace!(
            "Toolchain directory {toolchain_dir:?} has no rustc binary; skipping man page lookup"
        );
        return None;
    }

    let man_path = toolchain_dir
        .join("share")
        .join("man")
        .join("man1")
        .join("rustc.1");
    let man = read_file(&man_path).ok()?;
    let version = scan_man_page_rust_version(&man)?;
    log::trace!("Read rustc version from {man_path:?}: {version}");
    Some(format!("rustc {version}"))
}

/// Extracts the version from the `.TH` header of a toolchain's `rustc(1)` man
/// page, e.g.
/// `.TH RUSTC "1" "April 2019" "rustc 1.77.0 (aeda7d245 2024-03-13)" "User Commands"`.
///
/// Only the full verbose form (`<version> (<hash> <date>)`) is accepted; older
/// toolchains shipped an `<INSERT VERSION HERE>` placeholder or a bare version
/// without the hash — returns `None` for those so the caller falls through to
/// the subprocess fallback.
fn scan_man_page_rust_version(content: &str) -> Option<&str> {
    let line = content.lines().find(|l| l.starts_with(".TH RUSTC"))?;
    let rest = &line[line.find("\"rustc ")? + "\"rustc ".len()..];
    let version = &rest[..rest.find('"')?];
    (version.starts_with(|c: char| c.is_ascii_digit())
        && version.contains(" (")
        && version.ends_with(')'))
    .then_some(version)
}

fn format_rustc_version(rustc_version: &str, version_format: &str) -> Option<String> {
    let version = rustc_version
        // split into ["rustc", "1.34.0", ...]
        .split_whitespace()
        // get down to "1.34.0"
        .nth(1)?;

    Some(format_rust_release(version, version_format))
}

fn format_rust_release(version: &str, version_format: &str) -> String {
    match VersionFormatter::format_version(version, version_format) {
        Ok(formatted) => formatted,
        Err(error) => {
            log::warn!("Error formatting `rust` version:\n{error}");
            format!("v{version}")
        }
    }
}

fn format_toolchain(toolchain: &str, default_host_triple: Option<&str>) -> String {
    default_host_triple
        .map_or(toolchain, |triple| {
            toolchain.trim_end_matches(&format!("-{triple}"))
        })
        .to_owned()
}

fn parse_rustc_verbose_version(stdout: &str) -> Option<VersionDetection> {
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
    Some(VersionDetection::VerboseVersion {
        release: release.to_owned(),
        host: host.to_owned(),
    })
}

fn format_semver(semver: &str) -> String {
    format!("v{}", semver.find('-').map_or(semver, |i| &semver[..i]))
}

#[derive(Debug, PartialEq)]
enum VersionDetection {
    /// Standard output from a selected rustup toolchain or its metadata.
    StandardVersion(String),
    /// Verbose output from the fallback compiler; the host belongs to this compiler.
    VerboseVersion {
        release: String,
        host: String,
    },
    ToolchainNotInstalled(String),
    /// The compiler version could not be read.
    Unavailable,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct RustupSettings {
    default_host_triple: Option<String>,
    default_toolchain: Option<String>,
    overrides: HashMap<PathBuf, String>,
    version: Option<String>,
}

#[inline]
#[cfg(windows)]
fn strip_dos_path(path: PathBuf) -> PathBuf {
    // Use the display version of the path to strip \\?\
    let path = path.to_string_lossy();
    PathBuf::from(path.strip_prefix(r"\\?\").unwrap_or(&path))
}

#[inline]
#[cfg(not(windows))]
fn strip_dos_path(path: PathBuf) -> PathBuf {
    path
}

impl RustupSettings {
    fn load(context: &Context) -> Option<Self> {
        let path = rustup_home(context).ok()?.join("settings.toml");
        Self::from_toml_str(&read_file(path).ok()?)
    }

    fn from_toml_str(toml_str: &str) -> Option<Self> {
        let settings = toml::from_str::<Self>(toml_str).ok()?;
        if settings.version.as_deref() == Some("12") {
            Some(settings)
        } else {
            log::warn!(
                r#"Rustup settings version is {:?}, expected "12""#,
                settings.version
            );
            None
        }
    }

    fn default_host_triple(&self) -> Option<&str> {
        self.default_host_triple.as_deref()
    }

    fn default_toolchain(&self) -> Option<&str> {
        self.default_toolchain.as_deref()
    }

    fn lookup_override(&self, cwd: &Path) -> Option<String> {
        let cwd = strip_dos_path(cwd.to_owned());
        self.overrides
            .iter()
            .map(|(dir, toolchain)| (strip_dos_path(dir.clone()), toolchain))
            .filter(|(dir, _)| cwd.starts_with(dir))
            .max_by_key(|(dir, _)| dir.components().count())
            .map(|(_, name)| name.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::context::{Env, Properties, Shell, Target};
    use std::fs;
    use std::io::{self, Write};
    use std::process::{ExitStatus, Output};
    use std::sync::LazyLock;

    use super::*;

    fn write_rust_fixture(path: &Path, contents: &[u8], executable: bool) -> io::Result<()> {
        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = fs::File::create(path)?;
        file.write_all(contents)?;
        file.sync_all()?;
        drop(file);
        #[cfg(unix)]
        if executable {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
        }
        #[cfg(not(unix))]
        let _ = executable;
        Ok(())
    }

    fn install_rust_fixture(home: &Path, name: &str, version: &str) -> io::Result<PathBuf> {
        let directory = home.join("toolchains").join(name);
        write_rust_fixture(
            &directory
                .join("bin")
                .join(format!("rustc{}", std::env::consts::EXE_SUFFIX)),
            b"fixture compiler",
            true,
        )?;
        write_rust_fixture(
            &directory.join("share/man/man1/rustc.1"),
            format!(".TH RUSTC \"1\" \"July 2024\" \"rustc {version} (abcdef123 2024-07-01)\" \"User Commands\"\n").as_bytes(),
            false,
        )?;
        Ok(directory)
    }

    #[test]
    fn test_standalone_compilers_ignore_rustup_even_in_shared_directories() -> io::Result<()> {
        use crate::test::ModuleRenderer;
        use crate::utils::CommandOutput;

        for (inside_cargo, beside_rustup) in [(false, true), (true, false), (true, true)] {
            for succeeds in [true, false] {
                let dir = tempfile::tempdir()?;
                let root = dir.path().canonicalize()?;
                let home = root.join("rustup");
                let cargo = root.join("cargo");
                let bin = if inside_cargo {
                    cargo.join("bin")
                } else {
                    root.join("bin")
                };
                write_rust_fixture(&root.join("Cargo.toml"), b"", false)?;
                write_rust_fixture(
                    &home.join("settings.toml"),
                    b"version = \"12\"\ndefault_toolchain = \"stable\"\n[overrides]\n",
                    false,
                )?;
                install_rust_fixture(&home, "stable", "1.80.0")?;
                write_rust_fixture(
                    &bin.join(format!("rustc{}", std::env::consts::EXE_SUFFIX)),
                    b"independent compiler",
                    true,
                )?;
                if beside_rustup {
                    write_rust_fixture(
                        &bin.join(format!("rustup{}", std::env::consts::EXE_SUFFIX)),
                        b"independent rustup",
                        true,
                    )?;
                }
                let output = ModuleRenderer::new("rust")
                    .path(&root)
                    .env("PATH", bin.to_string_lossy())
                    .env("RUSTUP_HOME", home.to_string_lossy())
                    .env("CARGO_HOME", cargo.to_string_lossy())
                    .env("RUSTUP_TOOLCHAIN", "stable")
                    .cmd("rustc -Vv", succeeds.then(|| CommandOutput {
                        stdout: "rustc 1.90.0-nightly\nrelease: 1.90.0-nightly\nhost: x86_64-unknown-linux-gnu\n".into(),
                        stderr: String::new(),
                    }))
                    .config(toml::toml! { [rust]
                        format = "$version|$numver|$toolchain"
                    })
                    .collect();
                assert_eq!(
                    output,
                    Some(
                        if succeeds {
                            "v1.90.0-nightly|v1.90.0|x86_64-unknown-linux-gnu"
                        } else {
                            "||"
                        }
                        .into()
                    ),
                    "inside_cargo={inside_cargo}, beside_rustup={beside_rustup}, succeeds={succeeds}",
                );
                dir.close()?;
            }
        }
        Ok(())
    }

    #[test]
    fn test_direct_toolchain_metadata_beats_rustup_default() -> io::Result<()> {
        use crate::test::ModuleRenderer;

        let dir = tempfile::tempdir()?;
        let root = dir.path().canonicalize()?;
        let home = root.join("rustup");
        write_rust_fixture(&root.join("Cargo.toml"), b"", false)?;
        write_rust_fixture(
            &home.join("settings.toml"),
            b"version = \"12\"\ndefault_toolchain = \"stable\"\ndefault_host_triple = \"x86_64-unknown-linux-gnu\"\n[overrides]\n",
            false,
        )?;
        install_rust_fixture(&home, "stable", "1.80.0")?;
        let nightly =
            install_rust_fixture(&home, "nightly-x86_64-unknown-linux-gnu", "1.90.0-nightly")?;
        let output = ModuleRenderer::new("rust")
            .path(&root)
            .env("PATH", nightly.join("bin").to_string_lossy())
            .env("RUSTUP_HOME", home.to_string_lossy())
            .env("CARGO_HOME", root.join("cargo").to_string_lossy())
            .env("RUSTUP_TOOLCHAIN", "stable")
            .cmd("rustc -Vv", None)
            .config(toml::toml! { [rust]
                format = "$version|$numver|$toolchain"
            })
            .collect();
        assert_eq!(output, Some("v1.90.0-nightly|v1.90.0|nightly".into()));
        dir.close()
    }

    #[test]
    #[cfg(unix)]
    fn test_package_wrappers_are_probed_without_executing_project_toolchains() -> io::Result<()> {
        use crate::test::ModuleRenderer;

        for (kind, has_default, has_manager) in [
            ("proxy", true, true),
            ("proxy", true, false),
            ("proxy", false, true),
            ("compiler", true, true),
            ("broken", true, true),
        ] {
            let dir = tempfile::tempdir()?;
            let root = dir.path().canonicalize()?;
            let home = root.join("rustup");
            let bin = root.join("package/bin");
            write_rust_fixture(&root.join("Cargo.toml"), b"", false)?;
            write_rust_fixture(
                &root.join("rust-toolchain.toml"),
                b"[toolchain]\npath = './malicious'\n",
                false,
            )?;
            let settings = if has_default {
                "version = \"12\"\ndefault_toolchain = \"stable\"\n[overrides]\n"
            } else {
                "version = \"12\"\n[overrides]\n"
            };
            write_rust_fixture(&home.join("settings.toml"), settings.as_bytes(), false)?;
            if has_default {
                install_rust_fixture(&home, "stable", "1.80.0")?;
            }
            if has_manager {
                write_rust_fixture(&bin.join("rustup"), b"#!/bin/sh\nexit 1\n", true)?;
            }
            let response = match kind {
                "proxy" => {
                    "printf \"error: override toolchain '%s' is not installed: unavailable\\n\" \"$RUSTUP_TOOLCHAIN\" >&2\nexit 1\n"
                }
                "compiler" => {
                    "printf 'rustc 1.90.0-nightly\\nrelease: 1.90.0-nightly\\nhost: x86_64-unknown-linux-gnu\\n'\n"
                }
                _ => "printf 'compiler failed\\n' >&2\nexit 1\n",
            };
            let script = format!(
                "#!/bin/sh\ncase \"$RUSTUP_TOOLCHAIN\" in\nstarship-probe-*) ;;\n*) : > unsafe-execution; exit 1 ;;\nesac\nif [ \"$RUSTUP_AUTO_INSTALL\" != 0 ]; then : > unsafe-execution; exit 1; fi\n{response}"
            );
            write_rust_fixture(&bin.join("rustc"), script.as_bytes(), true)?;
            let output = ModuleRenderer::new("rust")
                .path(&root)
                .env("PATH", bin.to_string_lossy())
                .env("RUSTUP_HOME", home.to_string_lossy())
                .env("CARGO_HOME", root.join("cargo").to_string_lossy())
                .config(toml::toml! {
                    command_timeout = 5000
                    [rust]
                    format = "$version|$numver|$toolchain"
                })
                .collect();
            assert!(
                !root.join("unsafe-execution").exists(),
                "unprotected {kind} query"
            );
            let expected = match (kind, has_default) {
                ("proxy", true) => "v1.80.0|v1.80.0|stable",
                ("compiler", _) => "v1.90.0-nightly|v1.90.0|x86_64-unknown-linux-gnu",
                _ => "||",
            };
            assert_eq!(
                output,
                Some(expected.into()),
                "{kind}, default={has_default}, manager={has_manager}"
            );
            dir.close()?;
        }
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn test_compiler_query_respects_command_timeout() -> io::Result<()> {
        use crate::test::ModuleRenderer;
        let dir = tempfile::tempdir()?;
        let root = dir.path().canonicalize()?;
        let bin = root.join("bin");
        write_rust_fixture(&root.join("Cargo.toml"), b"", false)?;
        write_rust_fixture(
            &bin.join("rustc"),
            b"#!/bin/sh\nwhile :; do :; done\n",
            true,
        )?;
        let started = std::time::Instant::now();
        let output = ModuleRenderer::new("rust")
            .path(&root)
            .env("PATH", bin.to_string_lossy())
            .env("RUSTUP_HOME", root.join("rustup").to_string_lossy())
            .env("CARGO_HOME", root.join("cargo").to_string_lossy())
            .config(toml::toml! {
                command_timeout = 30
                [rust]
                format = "$version|$toolchain"
            })
            .collect();
        assert_eq!(output, Some("|".into()));
        assert!(started.elapsed() < Duration::from_secs(5));
        dir.close()
    }

    #[test]
    #[cfg(unix)]
    fn test_symlinked_package_wrapper_uses_its_own_rustup_manager() -> io::Result<()> {
        use crate::test::ModuleRenderer;
        use std::os::unix::fs::symlink;
        let dir = tempfile::tempdir()?;
        let root = dir.path().canonicalize()?;
        let home = root.join("rustup");
        let bin = root.join("bin");
        let package = root.join("package/bin");
        write_rust_fixture(&root.join("Cargo.toml"), b"", false)?;
        write_rust_fixture(
            &home.join("settings.toml"),
            b"version = \"12\"\n[overrides]\n",
            false,
        )?;
        install_rust_fixture(&home, "stable", "1.80.0")?;
        install_rust_fixture(&home, "nightly", "1.90.0-nightly")?;
        write_rust_fixture(&package.join("rustc"), b"#!/bin/sh\nprintf \"error: toolchain '%s' is not installed\\n\" \"$RUSTUP_TOOLCHAIN\" >&2\nexit 1\n", true)?;
        write_rust_fixture(
            &package.join("rustup"),
            b"#!/bin/sh\nprintf 'nightly (default)\\n'\n",
            true,
        )?;
        write_rust_fixture(
            &bin.join("rustup"),
            b"#!/bin/sh\nprintf 'stable (default)\\n'\n",
            true,
        )?;
        symlink(package.join("rustc"), bin.join("rustc"))?;
        let output = ModuleRenderer::new("rust")
            .path(&root)
            .env("PATH", bin.to_string_lossy())
            .env("RUSTUP_HOME", home.to_string_lossy())
            .env("CARGO_HOME", root.join("cargo").to_string_lossy())
            .config(toml::toml! {
                command_timeout = 5000
                [rust]
                format = "$version|$toolchain"
            })
            .collect();
        assert_eq!(output, Some("v1.90.0-nightly|nightly".into()));
        dir.close()
    }

    #[test]
    #[cfg(unix)]
    fn test_self_update_transition_is_detected_again_on_each_render() -> io::Result<()> {
        use crate::test::ModuleRenderer;

        let dir = tempfile::tempdir()?;
        let root = dir.path().canonicalize()?;
        let home = root.join("rustup");
        let bin = root.join("bin");
        write_rust_fixture(&root.join("Cargo.toml"), b"", false)?;
        write_rust_fixture(
            &home.join("settings.toml"),
            b"version = \"12\"\ndefault_toolchain = \"stable\"\n[overrides]\n",
            false,
        )?;
        let proxy = b"#!/bin/sh\ncase \"$RUSTUP_TOOLCHAIN\" in\nstarship-probe-*) printf \"error: toolchain '%s' is not installed\\n\" \"$RUSTUP_TOOLCHAIN\" >&2; exit 1 ;;\n*) : > unsafe-execution; exit 1 ;;\nesac\n";
        write_rust_fixture(&bin.join("rustup"), proxy, true)?;
        fs::hard_link(bin.join("rustup"), bin.join("rustc"))?;
        for (phase, version) in [(0, "1.80.0"), (1, "1.81.0"), (2, "1.81.0")] {
            install_rust_fixture(&home, "stable", version)?;
            if phase == 1 {
                // An atomic manager replacement can precede proxy relinking.
                write_rust_fixture(
                    &bin.join("rustup-new"),
                    b"#!/bin/sh\n# updated manager\nexit 1\n",
                    true,
                )?;
                fs::rename(bin.join("rustup-new"), bin.join("rustup"))?;
            } else if phase == 2 {
                fs::remove_file(bin.join("rustc"))?;
                fs::hard_link(bin.join("rustup"), bin.join("rustc"))?;
            }
            let output = ModuleRenderer::new("rust")
                .path(&root)
                .env("PATH", bin.to_string_lossy())
                .env("RUSTUP_HOME", home.to_string_lossy())
                .env("CARGO_HOME", root.join("cargo").to_string_lossy())
                .config(toml::toml! {
                    command_timeout = 5000
                    [rust]
                    format = "$version|$toolchain"
                })
                .collect();
            assert!(!root.join("unsafe-execution").exists(), "phase {phase}");
            assert_eq!(output, Some(format!("v{version}|stable")), "phase {phase}");
        }
        dir.close()
    }

    #[test]
    fn test_toolchain_selection_precedence() -> io::Result<()> {
        for (environment, directory_override, file, default, expected) in [
            (true, true, true, true, "from-environment"),
            (false, true, true, true, "from-directory"),
            (false, false, true, true, "from-file"),
            (false, false, false, true, "from-settings"),
            (false, false, false, false, "from-rustup"),
        ] {
            let dir = tempfile::tempdir()?;
            if file {
                let mut toolchain_file = fs::File::create(dir.path().join("rust-toolchain"))?;
                toolchain_file.write_all(b"from-file")?;
                toolchain_file.sync_all()?;
            }
            let mut env = Env::default();
            if environment {
                env.insert("RUSTUP_TOOLCHAIN", "from-environment".into());
            }
            let mut context = Context::new_with_shell_and_path(
                Default::default(),
                Shell::Unknown,
                Target::Main,
                dir.path().into(),
                dir.path().into(),
                env,
            );
            context.cmd.insert(
                "rustup default",
                Some(crate::utils::CommandOutput {
                    stdout: "from-rustup (default)\n".into(),
                    stderr: String::new(),
                }),
            );
            let info = RustToolchainInfo::new();
            let overrides = if directory_override {
                [(context.current_dir.clone(), "from-directory".into())]
                    .into_iter()
                    .collect()
            } else {
                HashMap::new()
            };
            info.rustup_settings
                .set(RustupSettings {
                    default_toolchain: default.then(|| "from-settings".into()),
                    overrides,
                    ..Default::default()
                })
                .unwrap();
            assert_eq!(
                info.selected_toolchain(&context, Some(Path::new("rustup")))
                    .as_deref(),
                Some(expected)
            );
            dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_compiler_spawn_failure_keeps_selected_toolchain_path() {
        let directory = Path::new("toolchains/stable-aarch64-unknown-linux-gnu");
        let outcome = run_rustc_from_toolchain("stable", Some(directory), None, |program, args| {
            assert_eq!(
                program,
                directory
                    .join("bin")
                    .join(format!("rustc{}", std::env::consts::EXE_SUFFIX))
            );
            assert_eq!(args, ["--version"]);
            Err(io::Error::from(io::ErrorKind::InvalidData))
        });
        assert_eq!(outcome, VersionDetection::Unavailable);
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn test_missing_toolchain_uses_rustup_run_without_installing() {
        #[cfg(unix)]
        use std::os::unix::process::ExitStatusExt as _;
        #[cfg(windows)]
        use std::os::windows::process::ExitStatusExt as _;
        let outcome = run_rustc_from_toolchain(
            "nightly",
            None,
            Some(Path::new("rustup")),
            |program, args| {
                assert_eq!(program, Path::new("rustup"));
                assert_eq!(args, ["run", "--", "nightly", "rustc", "--version"]);
                Ok(Output {
                    status: ExitStatus::from_raw(1),
                    stdout: vec![],
                    stderr: b"error: toolchain 'nightly' is not installed\n".to_vec(),
                })
            },
        );
        assert_eq!(
            outcome,
            VersionDetection::ToolchainNotInstalled("nightly".into())
        );
    }

    #[test]
    fn test_missing_path_compiler_does_not_query_another_rustc() -> io::Result<()> {
        use crate::test::ModuleRenderer;
        use crate::utils::CommandOutput;
        let dir = tempfile::tempdir()?;
        write_rust_fixture(&dir.path().join("Cargo.toml"), b"", false)?;
        let output = ModuleRenderer::new("rust")
            .path(dir.path())
            .env("PATH", dir.path().to_string_lossy())
            .env("RUSTUP_HOME", dir.path().join("rustup").to_string_lossy())
            .cmd(
                "rustc -Vv",
                Some(CommandOutput {
                    stdout: "release: 1.90.0-nightly\nhost: x86_64-unknown-linux-gnu\n".into(),
                    stderr: String::new(),
                }),
            )
            .config(toml::toml! { [rust]
                format = "$version|$toolchain"
            })
            .collect();
        assert_eq!(output, Some("|".into()));
        dir.close()
    }

    #[test]
    fn test_unmanaged_compiler_uses_only_verbose_output() -> io::Result<()> {
        use crate::test::ModuleRenderer;
        use crate::utils::CommandOutput;
        let dir = tempfile::tempdir()?;
        fs::File::create(dir.path().join("Cargo.toml"))?.sync_all()?;
        write_rust_fixture(
            &dir.path()
                .join(format!("rustc{}", std::env::consts::EXE_SUFFIX)),
            b"compiler",
            true,
        )?;
        let output = ModuleRenderer::new("rust")
            .path(dir.path())
            .env("PATH", dir.path().to_string_lossy())
            .env("RUSTUP_HOME", dir.path().to_string_lossy())
            .cmd("rustc --version", None)
            .cmd(
                "rustc -Vv",
                Some(CommandOutput {
                    stdout: "rustc 1.42.0-nightly\nrelease: 1.42.0-nightly\nhost: x86_64-unknown-linux-gnu\n"
                        .into(),
                    stderr: String::new(),
                }),
            )
            .config(toml::toml! { [rust]
                format = "$version|$numver|$toolchain"
            })
            .collect();
        assert_eq!(
            output,
            Some("v1.42.0-nightly|v1.42.0|x86_64-unknown-linux-gnu".into())
        );
        dir.close()
    }

    #[test]
    fn test_fallback_version_is_cached_for_all_variables() -> io::Result<()> {
        use crate::utils::CommandOutput;
        for succeeds in [true, false] {
            let dir = tempfile::tempdir()?;
            write_rust_fixture(
                &dir.path()
                    .join(format!("rustc{}", std::env::consts::EXE_SUFFIX)),
                b"compiler",
                true,
            )?;
            let mut env = Env::default();
            env.insert("PATH", dir.path().to_string_lossy().into_owned());
            let mut context = Context::new_with_shell_and_path(
                Default::default(),
                Shell::Unknown,
                Target::Main,
                dir.path().into(),
                dir.path().into(),
                env,
            );
            context.cmd.insert("rustc --version", None);
            let output = CommandOutput {
                stdout: "release: 1.42.0-nightly\nhost: x86_64-unknown-linux-gnu\n".into(),
                stderr: String::new(),
            };
            context
                .cmd
                .insert("rustc -Vv", succeeds.then(|| output.clone()));
            let info = RustToolchainInfo::new();
            info.rustup_settings
                .set(RustupSettings {
                    default_toolchain: Some("stable".into()),
                    default_host_triple: Some("aarch64-apple-darwin".into()),
                    ..Default::default()
                })
                .unwrap();
            let config = RustConfig {
                version_format: "${raw}",
                ..Default::default()
            };
            assert_eq!(
                get_module_numeric_version(&context, &info),
                succeeds.then(|| "v1.42.0".into()),
            );
            // Later variables must use the original result, caching failures as well as success.
            context
                .cmd
                .insert("rustc -Vv", (!succeeds).then_some(output));
            assert_eq!(
                get_module_version(&context, &config, &info),
                succeeds.then(|| "1.42.0-nightly".into()),
            );
            assert_eq!(
                get_toolchain_name(&context, &info),
                succeeds.then(|| "x86_64-unknown-linux-gnu".into()),
            );
            dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_unmanaged_compiler_ignores_rustup_default_toolchain() -> io::Result<()> {
        use crate::test::ModuleRenderer;
        use crate::utils::CommandOutput;
        let dir = tempfile::tempdir()?;
        fs::File::create(dir.path().join("Cargo.toml"))?.sync_all()?;
        fs::write(
            dir.path().join("settings.toml"),
            "version = \"12\"\ndefault_toolchain = \"stable\"\n[overrides]\n",
        )?;
        let rustc = dir
            .path()
            .join(format!("rustc{}", std::env::consts::EXE_SUFFIX));
        fs::File::create(&rustc)?.sync_all()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&rustc, fs::Permissions::from_mode(0o755))?;
        }
        let output = ModuleRenderer::new("rust")
            .path(dir.path())
            .env("PATH", dir.path().to_string_lossy())
            .env("CARGO_HOME", dir.path().join("cargo").to_string_lossy())
            .env("RUSTUP_HOME", dir.path().to_string_lossy())
            .cmd(
                "rustc --version",
                Some(CommandOutput {
                    stdout: "rustc 1.42.0 (b8cedc004 2020-03-09)\n".into(),
                    stderr: String::new(),
                }),
            )
            .cmd(
                "rustc -Vv",
                Some(CommandOutput {
                    stdout: "rustc 1.42.0\nrelease: 1.42.0\nhost: x86_64-unknown-linux-gnu\n"
                        .into(),
                    stderr: String::new(),
                }),
            )
            .config(toml::toml! { [rust]
                format = "$version|$numver|$toolchain"
            })
            .collect();
        assert_eq!(
            output,
            Some("v1.42.0|v1.42.0|x86_64-unknown-linux-gnu".into())
        );
        dir.close()
    }

    #[test]
    fn test_direct_toolchain_compiler_without_selected_toolchain() -> io::Result<()> {
        use crate::test::ModuleRenderer;
        use crate::utils::CommandOutput;
        let dir = tempfile::tempdir()?;
        fs::File::create(dir.path().join("Cargo.toml"))?.sync_all()?;
        let rustup_home = dir.path().canonicalize()?.join("rustup");
        let bin = rustup_home.join("toolchains").join("custom").join("bin");
        fs::create_dir_all(&bin)?;
        let rustc = bin.join(format!("rustc{}", std::env::consts::EXE_SUFFIX));
        fs::File::create(&rustc)?.sync_all()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&rustc, fs::Permissions::from_mode(0o755))?;
        }
        let output = ModuleRenderer::new("rust")
            .path(dir.path())
            .env("PATH", bin.to_string_lossy())
            .env("CARGO_HOME", dir.path().join("cargo").to_string_lossy())
            .env("RUSTUP_HOME", rustup_home.to_string_lossy())
            .cmd("rustup default", None)
            .cmd(
                "rustc -Vv",
                Some(CommandOutput {
                    stdout: "rustc 1.42.0\nrelease: 1.42.0\nhost: x86_64-unknown-linux-gnu\n"
                        .into(),
                    stderr: String::new(),
                }),
            )
            .config(toml::toml! { [rust]
                format = "$version|$numver|$toolchain"
            })
            .collect();
        assert_eq!(output, Some("v1.42.0|v1.42.0|custom".into()));
        dir.close()
    }

    #[test]
    fn test_rustup_managed_compiler_does_not_fallback_to_rustc_proxy_without_selected_toolchain()
    -> io::Result<()> {
        use crate::test::ModuleRenderer;
        use crate::utils::CommandOutput;
        for cargo_subdirectory in ["cargo", "toolchains/cargo"] {
            let dir = tempfile::tempdir()?;
            let rustup_home = dir.path().canonicalize()?;
            let cargo_home = rustup_home.join(cargo_subdirectory);
            let bin = cargo_home.join("bin");
            fs::create_dir_all(&bin)?;
            fs::File::create(dir.path().join("Cargo.toml"))?.sync_all()?;
            let mut toolchain_file = fs::File::create(dir.path().join("rust-toolchain"))?;
            toolchain_file.write_all(b"../malicious")?;
            toolchain_file.sync_all()?;
            drop(toolchain_file);
            let rustc = bin.join(format!("rustc{}", std::env::consts::EXE_SUFFIX));
            fs::File::create(&rustc)?.sync_all()?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&rustc, fs::Permissions::from_mode(0o755))?;
            }
            fs::hard_link(
                &rustc,
                bin.join(format!("rustup{}", std::env::consts::EXE_SUFFIX)),
            )?;
            let output = ModuleRenderer::new("rust")
                .path(dir.path())
                .env("PATH", bin.to_string_lossy())
                .env("CARGO_HOME", cargo_home.to_string_lossy())
                .env("RUSTUP_HOME", rustup_home.to_string_lossy())
                .cmd("rustup default", None)
                .cmd(
                    "rustc -Vv",
                    Some(CommandOutput {
                        stdout: "rustc 1.42.0\nrelease: 1.42.0\nhost: x86_64-unknown-linux-gnu\n"
                            .into(),
                        stderr: String::new(),
                    }),
                )
                .config(toml::toml! { [rust]
                    format = "$version|$numver|$toolchain"
                })
                .collect();
            assert_eq!(output, Some("||".into()), "{cargo_subdirectory}");
            dir.close()?;
        }
        Ok(())
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn test_module_keeps_toolchain_when_compiler_cannot_execute() -> io::Result<()> {
        use crate::test::ModuleRenderer;
        let dir = tempfile::tempdir()?;
        let bin = dir.path().join("bin");
        let toolchain = "stable-aarch64-unknown-linux-gnu";
        fs::File::create(dir.path().join("Cargo.toml"))?.sync_all()?;
        let compiler_bin = dir.path().join("toolchains").join(toolchain).join("bin");
        fs::create_dir_all(&bin)?;
        fs::create_dir_all(&compiler_bin)?;
        // Executable files with invalid binary contents reproduce an incompatible
        // compiler without running a host toolchain or requiring another architecture.
        for directory in [&bin, &compiler_bin] {
            let compiler = directory.join(format!("rustc{}", std::env::consts::EXE_SUFFIX));
            let mut file = fs::File::create(&compiler)?;
            file.write_all(b"\0invalid executable")?;
            file.sync_all()?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&compiler, fs::Permissions::from_mode(0o755))?;
            }
        }
        fs::hard_link(
            bin.join(format!("rustc{}", std::env::consts::EXE_SUFFIX)),
            bin.join(format!("rustup{}", std::env::consts::EXE_SUFFIX)),
        )?;
        let mut toolchain_file = fs::File::create(dir.path().join("rust-toolchain"))?;
        toolchain_file.write_all(toolchain.as_bytes())?;
        toolchain_file.sync_all()?;
        let mut settings = fs::File::create(dir.path().join("settings.toml"))?;
        settings.write_all(
            b"version = \"12\"\ndefault_host_triple = \"x86_64-unknown-linux-gnu\"\n[overrides]\n",
        )?;
        settings.sync_all()?;
        let output = ModuleRenderer::new("rust")
            .path(dir.path())
            .env("PATH", bin.to_string_lossy())
            .env("RUSTUP_HOME", dir.path().to_string_lossy())
            .config(toml::toml! { [rust]
                format = "$version|$numver|$toolchain"
            })
            .collect();
        assert_eq!(output, Some("||stable-aarch64-unknown-linux-gnu".into()));
        drop(toolchain_file);
        drop(settings);
        dir.close()
    }

    #[test]
    fn test_settings_use_context_rustup_home() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut settings = fs::File::create(dir.path().join("settings.toml"))?;
        write!(
            settings,
            "version = \"12\"\ndefault_toolchain = \"test-toolchain\"\n[overrides]\n"
        )?;
        settings.sync_all()?;
        let mut env = Env::default();
        env.insert("RUSTUP_HOME", dir.path().to_string_lossy().into_owned());
        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            env,
        );
        assert_eq!(
            RustupSettings::load(&context).unwrap().default_toolchain(),
            Some("test-toolchain")
        );
        drop(settings);
        dir.close()
    }

    #[test]
    fn test_selected_toolchain_survives_compiler_failure() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );
        let info = RustToolchainInfo::new();
        info.rustup_settings
            .set(RustupSettings {
                default_host_triple: Some("x86_64-unknown-linux-gnu".into()),
                ..Default::default()
            })
            .unwrap();
        info.compiler_info
            .set(CompilerInfo::new(
                VersionDetection::Unavailable,
                Some("stable-aarch64-unknown-linux-gnu".into()),
            ))
            .unwrap();

        assert_eq!(
            get_toolchain_name(&context, &info),
            Some("stable-aarch64-unknown-linux-gnu".into())
        );
        assert_eq!(get_module_numeric_version(&context, &info), None);
        dir.close()
    }

    #[test]
    fn test_rustup_settings_from_toml_value() {
        assert_eq!(
            RustupSettings::from_toml_str(
                r#"
default_host_triple = "x86_64-unknown-linux-gnu"
default_toolchain = "stable"
version = "12"

[overrides]
"/home/user/src/starship" = "1.40.0-x86_64-unknown-linux-gnu"
"#
            ),
            Some(RustupSettings {
                default_host_triple: Some("x86_64-unknown-linux-gnu".to_owned()),
                default_toolchain: Some("stable".to_owned()),
                overrides: vec![(
                    "/home/user/src/starship".into(),
                    "1.40.0-x86_64-unknown-linux-gnu".to_owned(),
                )]
                .into_iter()
                .collect(),
                version: Some("12".to_string())
            }),
        );

        // Invalid or missing version key causes a failure
        assert_eq!(
            RustupSettings::from_toml_str(
                r#"
                default_host_triple = "x86_64-unknown-linux-gnu"
                default_toolchain = "stable"

                [overrides]
                "/home/user/src/starship" = "1.39.0-x86_64-unknown-linux-gnu"
            "#
            ),
            None,
        );
    }

    #[test]
    fn test_override_matches_correct_directories() {
        let test_settings = RustupSettings::from_toml_str(
            r#"
default_host_triple = "x86_64-unknown-linux-gnu"
default_toolchain = "stable"
version = "12"

[overrides]
"/home/user/src/a" = "beta-x86_64-unknown-linux-gnu"
"/home/user/src/b" = "nightly-x86_64-unknown-linux-gnu"
"/home/user/src/b/d c" = "stable-x86_64-pc-windows-msvc"
"#,
        )
        .unwrap();

        static OVERRIDES_CWD_A: &str = "/home/user/src/a/src";
        static OVERRIDES_CWD_B: &str = "/home/user/src/b/tests";
        static OVERRIDES_CWD_C: &str = "/home/user/src/c/examples";
        static OVERRIDES_CWD_D: &str = "/home/user/src/b/d c/spaces";
        static OVERRIDES_CWD_E: &str = "/home/user/src/b_and_more";
        static OVERRIDES_CWD_F: &str = "/home/user/src/b";

        static BETA_TOOLCHAIN: &str = "beta-x86_64-unknown-linux-gnu";
        static NIGHTLY_TOOLCHAIN: &str = "nightly-x86_64-unknown-linux-gnu";
        static STABLE_TOOLCHAIN: &str = "stable-x86_64-pc-windows-msvc";

        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_A.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_B.as_ref()),
            Some(NIGHTLY_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_C.as_ref()),
            None
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_D.as_ref()),
            Some(STABLE_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_E.as_ref()),
            None
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_F.as_ref()),
            Some(NIGHTLY_TOOLCHAIN.to_string())
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_extract_toolchain_from_override_with_dospath() {
        let test_settings = RustupSettings::from_toml_str(
            r#"
default_host_triple = "x86_64-unknown-linux-gnu"
default_toolchain = "stable"
version = "12"

[overrides]
"C:\\src1" = "beta-x86_64-unknown-linux-gnu"
"\\\\?\\C:\\src2" = "beta-x86_64-unknown-linux-gnu"
"#,
        )
        .unwrap();
        static OVERRIDES_CWD_A: &str = r"\\?\C:\src1";
        static OVERRIDES_CWD_B: &str = r"C:\src1";
        static OVERRIDES_CWD_C: &str = r"\\?\C:\src2";
        static OVERRIDES_CWD_D: &str = r"C:\src2";

        static BETA_TOOLCHAIN: &str = "beta-x86_64-unknown-linux-gnu";

        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_A.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_B.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_C.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_D.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn test_parse_toolchain_version_output() {
        #[cfg(unix)]
        use std::os::unix::process::ExitStatusExt as _;
        #[cfg(windows)]
        use std::os::windows::process::ExitStatusExt as _;

        static RUSTC_VERSION: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"rustc 1.34.0\n"[..].to_owned(),
            stderr: vec![],
        });
        assert_eq!(
            parse_toolchain_version_output(RUSTC_VERSION.clone()),
            VersionDetection::StandardVersion("rustc 1.34.0\n".to_owned()),
        );

        static TOOLCHAIN_NAME: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"error: toolchain 'channel-triple' is not installed\n"[..].to_owned(),
        });
        assert_eq!(
            parse_toolchain_version_output(TOOLCHAIN_NAME.clone()),
            VersionDetection::ToolchainNotInstalled("channel-triple".to_owned()),
        );

        static INVALID_STDOUT: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"\xc3\x28"[..].to_owned(),
            stderr: vec![],
        });
        assert_eq!(
            parse_toolchain_version_output(INVALID_STDOUT.clone()),
            VersionDetection::Unavailable,
        );

        static INVALID_STDERR: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"\xc3\x28"[..].to_owned(),
        });
        assert_eq!(
            parse_toolchain_version_output(INVALID_STDERR.clone()),
            VersionDetection::Unavailable,
        );

        static UNEXPECTED_FORMAT_OF_ERROR: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"error:"[..].to_owned(),
        });
        assert_eq!(
            parse_toolchain_version_output(UNEXPECTED_FORMAT_OF_ERROR.clone()),
            VersionDetection::Unavailable,
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
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
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
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
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
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
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
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            child_dir_path.clone(),
            child_dir_path,
            Env::default(),
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
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
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
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
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
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
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
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            child_dir_path.clone(),
            child_dir_path,
            Env::default(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()
    }

    #[test]
    fn test_parse_rustc_verbose_version() {
        static STABLE: &str = r"rustc 1.40.0 (73528e339 2019-12-16)
binary: rustc
commit-hash: 73528e339aae0f17a15ffa49a8ac608f50c6cf14
commit-date: 2019-12-16
host: x86_64-unknown-linux-gnu
release: 1.40.0
LLVM version: 9.0
";

        static BETA: &str = r"rustc 1.41.0-beta.1 (eb3f7c2d3 2019-12-17)
binary: rustc
commit-hash: eb3f7c2d3aec576f47eba854cfbd3c1187b8a2a0
commit-date: 2019-12-17
host: x86_64-unknown-linux-gnu
release: 1.41.0-beta.1
LLVM version: 9.0
";

        static NIGHTLY: &str = r"rustc 1.42.0-nightly (da3629b05 2019-12-29)
binary: rustc
commit-hash: da3629b05f8f1b425a738bfe9fe9aedd47c5417a
commit-date: 2019-12-29
host: x86_64-unknown-linux-gnu
release: 1.42.0-nightly
LLVM version: 9.0
";

        for (output, release) in [
            (STABLE, "1.40.0"),
            (BETA, "1.41.0-beta.1"),
            (NIGHTLY, "1.42.0-nightly"),
        ] {
            assert_eq!(
                parse_rustc_verbose_version(output),
                Some(VersionDetection::VerboseVersion {
                    release: release.into(),
                    host: "x86_64-unknown-linux-gnu".into(),
                }),
            );
        }
        for output in ["", "release: 1.40.0\n", "host: x86_64-unknown-linux-gnu\n"] {
            assert_eq!(parse_rustc_verbose_version(output), None);
        }
    }

    #[test]
    fn test_find_rust_toolchain_file_rejects_paths() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let toolchain_file = dir.path().join("rust-toolchain");
        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );

        let absolute_toolchain = dir
            .path()
            .join("my-toolchain")
            .to_string_lossy()
            .into_owned();

        for invalid in [
            "../some-toolchain",
            &absolute_toolchain,
            ".",
            "..",
            "../.",
            "/",
            "/.",
            "foo/",
            "foo/.",
            #[cfg(windows)]
            "C:",
            #[cfg(windows)]
            "C:.",
        ] {
            let mut file = fs::File::create(&toolchain_file)?;
            file.write_all(invalid.as_bytes())?;
            file.sync_all()?;
            drop(file);
            assert_eq!(
                find_rust_toolchain_file(&context),
                None,
                "should reject invalid toolchain name: {invalid:?}"
            );
        }

        let mut file = fs::File::create(&toolchain_file)?;
        file.write_all(b"stable")?;
        file.sync_all()?;
        drop(file);
        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("stable".to_owned()),
            "should accept plain channel names",
        );

        dir.close()
    }

    #[test]
    fn test_find_toolchain_dir_does_not_select_a_different_toolchain() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir(dir.path().join("nightly-2025-01-01-aarch64-apple-darwin"))?;
        fs::create_dir(dir.path().join("nightly-x86_64-unknown-linux-gnu"))?;
        assert_eq!(
            find_toolchain_dir("nightly", Some("aarch64-apple-darwin"), dir.path()),
            None,
            "should not select an archived nightly or a toolchain for another host",
        );
        assert_eq!(
            find_toolchain_dir("nightly", None, dir.path()),
            None,
            "should defer to rustup when the host is unknown"
        );
        dir.close()
    }

    #[test]
    fn test_find_toolchain_dir_resolves_exact_names() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let installed = dir.path().join("nightly-aarch64-apple-darwin");
        fs::create_dir(&installed)?;
        assert_eq!(
            find_toolchain_dir("nightly", Some("aarch64-apple-darwin"), dir.path()),
            Some(installed.clone()),
            "should complete short channel names with the default host triple",
        );
        assert_eq!(
            find_toolchain_dir("nightly-aarch64-apple-darwin", None, dir.path()),
            Some(installed),
            "should resolve fully qualified toolchain names exactly",
        );
        dir.close()
    }

    #[test]
    fn test_toolchain_metadata_requires_installed_rustc() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rustlib = dir.path().join("lib/rustlib");
        fs::create_dir_all(&rustlib)?;
        let mut manifest = fs::File::create(rustlib.join("multirust-channel-manifest.toml"))?;
        manifest.write_all(b"[pkg.rust]\nversion = \"1.99.0 (abcdef123 2026-01-01)\"\n")?;
        manifest.sync_all()?;
        assert_eq!(
            get_version_from_toolchain_dir(dir.path()),
            None,
            "should ignore a channel manifest when rustc is missing"
        );

        let bin = dir.path().join("bin");
        fs::create_dir(&bin)?;
        fs::File::create(bin.join(format!("rustc{}", std::env::consts::EXE_SUFFIX)))?.sync_all()?;
        assert_eq!(
            get_version_from_toolchain_dir(dir.path()),
            None,
            "should fall back to a subprocess when only the channel manifest is available",
        );

        let man_dir = dir.path().join("share/man/man1");
        fs::create_dir_all(&man_dir)?;
        let mut man = fs::File::create(man_dir.join("rustc.1"))?;
        man.write_all(b".TH RUSTC \"1\" \"April 2019\" \"rustc 1.98.0 (abcdef123 2026-01-01)\" \"User Commands\"\n")?;
        man.sync_all()?;
        assert_eq!(
            get_version_from_toolchain_dir(dir.path()),
            Some("rustc 1.98.0 (abcdef123 2026-01-01)".to_owned()),
            "should read the man page version when rustc is installed",
        );
        fs::remove_file(bin.join(format!("rustc{}", std::env::consts::EXE_SUFFIX)))?;
        assert_eq!(
            get_version_from_toolchain_dir(dir.path()),
            None,
            "should ignore remaining metadata after rustc is removed"
        );
        drop(man);
        drop(manifest);
        dir.close()
    }

    #[test]
    fn test_rustc_detection_uses_context_path() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut env = Env::default();
        env.insert("PATH", dir.path().to_string_lossy().into_owned());
        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            env,
        );
        assert!(
            matches!(rustc_source(&context), RustcSource::Missing),
            "should use the mocked PATH where rustc is absent"
        );
        let rustc = dir
            .path()
            .join(format!("rustc{}", std::env::consts::EXE_SUFFIX));
        fs::File::create(&rustc)?.sync_all()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&rustc, fs::Permissions::from_mode(0o755))?;
        }
        assert!(
            matches!(rustc_source(&context), RustcSource::Other(_)),
            "should detect an unmanaged rustc in the mocked PATH"
        );
        let rustup = dir
            .path()
            .join(format!("rustup{}", std::env::consts::EXE_SUFFIX));
        fs::File::create(&rustup)?.sync_all()?;
        assert!(
            matches!(rustc_source(&context), RustcSource::Other(_)),
            "an unrelated neighboring rustup must not replace the compiler"
        );
        fs::remove_file(&rustup)?;
        fs::hard_link(&rustc, &rustup)?;
        assert!(
            matches!(rustc_source(&context), RustcSource::RustupProxy(_)),
            "should detect a rustup proxy in the mocked PATH"
        );
        dir.close()
    }

    #[test]
    #[cfg(unix)]
    fn test_rustc_detection_resolves_package_manager_symlinks() -> io::Result<()> {
        use std::os::unix::fs::{PermissionsExt, symlink};

        for (binary_name, expected_proxy) in [("rustup", true), ("rustc", false)] {
            let dir = tempfile::tempdir()?;
            let bin = dir.path().join("bin");
            let package_bin = dir.path().join("package/bin");
            fs::create_dir_all(&bin)?;
            fs::create_dir_all(&package_bin)?;
            let binary = package_bin.join(binary_name);
            fs::File::create(&binary)?.sync_all()?;
            fs::set_permissions(&binary, fs::Permissions::from_mode(0o755))?;
            symlink(&binary, bin.join("rustc"))?;
            // Both tools may be exposed in a common package-manager prefix;
            // only the resolved compiler's directory should determine adjacency.
            fs::File::create(bin.join("rustup"))?.sync_all()?;

            let mut env = Env::default();
            env.insert("PATH", bin.to_string_lossy().into_owned());
            env.insert(
                "CARGO_HOME",
                dir.path().join("cargo").to_string_lossy().into_owned(),
            );
            env.insert(
                "RUSTUP_HOME",
                dir.path().join("rustup").to_string_lossy().into_owned(),
            );
            let context = Context::new_with_shell_and_path(
                Default::default(),
                Shell::Unknown,
                Target::Main,
                dir.path().into(),
                dir.path().into(),
                env,
            );
            assert_eq!(
                matches!(rustc_source(&context), RustcSource::RustupProxy(_)),
                expected_proxy,
                "{binary_name}"
            );
            dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_find_toolchain_dir_rejects_path_based_toolchains() {
        for invalid in [
            "/opt/my-toolchain",
            "../my-toolchain",
            ".",
            "..",
            "../.",
            "/",
            "/.",
            "foo/",
            "foo/.",
            #[cfg(windows)]
            "C:",
            #[cfg(windows)]
            "C:.",
        ] {
            assert_eq!(
                find_toolchain_dir(invalid, None, Path::new("unused")),
                None,
                "should reject invalid or path-based toolchain: {invalid:?}",
            );
        }
    }

    #[test]
    fn test_scan_man_page_rust_version() {
        let man = r#".TH RUSTC "1" "April 2019" "rustc 1.97.0-beta.6 (b2282dd56 2026-07-01)" "User Commands"
.SH NAME
rustc \- The Rust compiler
"#;
        assert_eq!(
            scan_man_page_rust_version(man),
            Some("1.97.0-beta.6 (b2282dd56 2026-07-01)"),
            "should extract the version from the .TH header",
        );
    }

    #[test]
    fn test_scan_man_page_rust_version_placeholder() {
        let man = r#".TH RUSTC "1" "April 2019" "rustc <INSERT VERSION HERE>" "User Commands"
.SH NAME
rustc \- The Rust compiler
"#;
        assert_eq!(
            scan_man_page_rust_version(man),
            None,
            "should reject the placeholder shipped by older toolchains",
        );
    }

    #[test]
    fn test_scan_man_page_rust_version_bare_version() {
        let man = r#".TH RUSTC "1" "April 2019" "rustc 1.20.0" "User Commands"
.SH NAME
rustc \- The Rust compiler
"#;
        assert_eq!(
            scan_man_page_rust_version(man),
            None,
            "should reject a bare version without hash and date",
        );
    }
}
