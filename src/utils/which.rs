#[cfg(windows)]
use std::env;
#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::path::{Path, PathBuf};

#[cfg(not(windows))]
pub use which::which;

/// A wrapper around `which::which` that resolves Scoop shim executables on
/// Windows to their actual target executables, avoiding the overhead of
/// launching the shim process.
#[cfg(windows)]
pub fn which<T: AsRef<OsStr>>(binary_name: T) -> Result<PathBuf, which::Error> {
    which::which(binary_name).map(resolve_scoop_shim)
}

/// Resolve a Scoop shim executable to its target executable.
///
/// The path is returned unchanged unless it is an `.exe` inside a Scoop shims
/// directory whose `.shim` file points to an existing target and forwards no
/// extra arguments (a shim with `args` has to run as-is).
#[cfg(windows)]
pub fn resolve_scoop_shim(path: PathBuf) -> PathBuf {
    if let Some(resolved) = resolve_scoop_shim_in(&path, &scoop_shims_dirs()) {
        log::trace!("Resolved Scoop shim executable {path:?} to target {resolved:?}");
        resolved
    } else {
        log::trace!("No Scoop shim resolution for {path:?}");
        path
    }
}

/// Resolve a Scoop shim executable to its target executable, if applicable.
#[cfg(windows)]
fn resolve_scoop_shim_in(path: &Path, shims_dirs: &[PathBuf]) -> Option<PathBuf> {
    let is_exe = path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"));
    if !is_exe || !is_in_dirs(path, shims_dirs) {
        return None;
    }

    parse_shim_file(&path.with_extension("shim"))
        .and_then(|path| dunce::canonicalize(&path).ok())
        .filter(|target| target.is_file())
}

/// Retrieve the possible Scoop shims directories on the system.
#[cfg(windows)]
fn scoop_shims_dirs() -> Vec<PathBuf> {
    let user_root = env::var_os("SCOOP")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| Some(PathBuf::from(env::var_os("USERPROFILE")?).join("scoop")));

    let global_root = env::var_os("SCOOP_GLOBAL")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| Some(PathBuf::from(env::var_os("ProgramData")?).join("scoop")));

    [user_root, global_root]
        .into_iter()
        .flatten()
        .map(|root| root.join("shims"))
        .collect()
}

/// Check if a path resides directly inside one of the given directories.
#[cfg(windows)]
fn is_in_dirs(path: &Path, dirs: &[PathBuf]) -> bool {
    let Some(parent) = path.parent() else {
        return false;
    };
    let Ok(parent) = dunce::canonicalize(parent) else {
        return false;
    };
    dirs.iter()
        .any(|dir| dunce::canonicalize(dir).is_ok_and(|dir| dir == parent))
}

/// Parse a Scoop `.shim` file and return the target path.
///
/// Returns `None` if the shim forwards extra arguments or contains any
/// other unsupported directives.
#[cfg(windows)]
fn parse_shim_file(shim_path: &Path) -> Option<PathBuf> {
    let content = std::fs::read_to_string(shim_path).ok()?;
    let mut target_path = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        let (key, val) = line.split_once('=')?;
        let key = key.trim();
        let val = val.trim().trim_matches(['"', '\'']).trim();
        if key.eq_ignore_ascii_case("path") {
            target_path = Some(PathBuf::from(val));
        } else {
            return None;
        }
    }

    let parent = shim_path.parent()?;
    Some(parent.join(target_path?))
}

#[cfg(all(test, windows))]
mod tests {
    use crate::utils;

    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    fn write_shim(dir: &Path, name: &str, content: &str) -> PathBuf {
        let shim_exe = dir.join(name).with_extension("exe");
        utils::write_file(&shim_exe, "").unwrap();
        let shim_config = dir.join(name).with_extension("shim");
        utils::write_file(&shim_config, content).unwrap();
        shim_exe
    }

    #[test]
    fn test_parse_shim_file_simple_path() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        fs::write(
            &shim_path,
            "path = C:\\scoop\\apps\\git\\current\\bin\\git.exe\n",
        )
        .unwrap();
        assert_eq!(
            parse_shim_file(&shim_path),
            Some(PathBuf::from("C:\\scoop\\apps\\git\\current\\bin\\git.exe"))
        );
    }

    #[test]
    fn test_parse_shim_file_quotes() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        for quote in ["\"", "'"] {
            fs::write(
                &shim_path,
                format!("path = {quote}C:\\scoop\\apps\\git\\current\\bin\\git.exe{quote}\n"),
            )
            .unwrap();
            assert_eq!(
                parse_shim_file(&shim_path),
                Some(PathBuf::from("C:\\scoop\\apps\\git\\current\\bin\\git.exe"))
            );
        }
    }

    #[test]
    fn test_parse_shim_file_non_empty_args() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        fs::write(
            &shim_path,
            "path = C:\\scoop\\apps\\git\\current\\bin\\git.exe\nargs = --version\n",
        )
        .unwrap();
        assert_eq!(parse_shim_file(&shim_path), None);
    }

    #[test]
    fn test_parse_shim_file_other_directives() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        for directive in ["args = ", "args = \"\"", "workingdir = C:\\somewhere"] {
            fs::write(
                &shim_path,
                format!("path = C:\\scoop\\apps\\git\\current\\bin\\git.exe\n{directive}\n"),
            )
            .unwrap();
            assert_eq!(parse_shim_file(&shim_path), None);
        }
    }

    #[test]
    fn test_parse_shim_file_comments_and_spaces() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        fs::write(
            &shim_path,
            "# This is a comment\n  ; Another comment\npath   =   C:\\test.exe  \n",
        )
        .unwrap();
        assert_eq!(
            parse_shim_file(&shim_path),
            Some(PathBuf::from("C:\\test.exe"))
        );
    }

    #[test]
    fn test_resolve_scoop_shim() {
        let dir = tempdir().unwrap();

        // Mock a Scoop-like directory structure:
        // dir/
        //   shims/
        //     git.exe (empty placeholder)
        //     git.shim (path = ../apps/git/git.exe)
        //   apps/
        //     git/
        //       git.exe (empty placeholder)
        let shims_dir = dir.path().join("shims");
        let apps_dir = dir.path().join("apps").join("git");
        fs::create_dir_all(&shims_dir).unwrap();
        fs::create_dir_all(&apps_dir).unwrap();

        let shim_exe = write_shim(&shims_dir, "git", "path = ../apps/git/git.exe\n");
        let target_exe = apps_dir.join("git.exe");
        File::create(&target_exe).unwrap();

        let shims_dirs = [shims_dir];
        let resolved = resolve_scoop_shim_in(&shim_exe, &shims_dirs)
            .expect("The shim should resolve to the target executable");
        assert_eq!(
            dunce::canonicalize(&resolved).unwrap(),
            dunce::canonicalize(&target_exe).unwrap()
        );

        assert_eq!(
            resolve_scoop_shim_in(&target_exe, &shims_dirs),
            None,
            "A non-shim executable should not be resolved"
        );
    }

    #[test]
    fn test_resolve_scoop_shim_with_args() {
        let dir = tempdir().unwrap();
        let shims_dir = dir.path().to_path_buf();

        let shim_exe = write_shim(
            &shims_dir,
            "git",
            "path = ../apps/git/git.exe\nargs = status\n",
        );

        assert_eq!(
            resolve_scoop_shim_in(&shim_exe, &[shims_dir]),
            None,
            "The shim forwards extra arguments, so it is not resolved to the target"
        );
    }

    #[test]
    fn test_resolve_scoop_shim_missing_target() {
        let dir = tempdir().unwrap();
        let shims_dir = dir.path().to_path_buf();

        let shim_exe = write_shim(&shims_dir, "git", "path = ../apps/git/git.exe\n");

        assert_eq!(
            resolve_scoop_shim_in(&shim_exe, &[shims_dir]),
            None,
            "The shim target does not exist, so it is not resolved to the target"
        );
    }
}
