use crate::config::StarshipConfig;
use crate::context::{Context, Shell};
use std::path::Path;

/// Render a specific starship module by name
pub fn render_module(
    module_name: &str,
    path: &Path,
    config: Option<toml::Value>,
) -> Option<String> {
    let mut context = Context::new_with_dir(clap::ArgMatches::default(), path);
    context.config = StarshipConfig { config };
    context.shell = Shell::Unknown;

    fs_sync(&path);

    crate::print::get_module(module_name, context)
}

/// Syncs directory in filesystem to disk to ensure consistent tests
#[cfg(not(windows))]
fn fs_sync(path: &Path) {
    use nix::fcntl::{open, OFlag};
    use nix::sys::stat::Mode;
    use nix::unistd::fsync;
    let dir = open(
        path,
        OFlag::O_DIRECTORY | OFlag::O_RDONLY | OFlag::O_CLOEXEC,
        Mode::empty(),
    )
    .unwrap();
    fsync(dir).unwrap();
}

#[cfg(windows)]
fn fs_sync(path: &Path) {
    use std::iter;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::fileapi::{CreateFileW, FlushFileBuffers, OPEN_EXISTING};
    use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
    use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;
    use winapi::um::winnt::{
        FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_DELETE, FILE_SHARE_READ,
        FILE_SHARE_WRITE, GENERIC_WRITE,
    };

    let wpath: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect();

    let handle = unsafe {
        CreateFileW(
            wpath.as_ptr(),
            GENERIC_WRITE,
            FILE_SHARE_DELETE | FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_DIRECTORY | FILE_ATTRIBUTE_NORMAL | FILE_FLAG_BACKUP_SEMANTICS,
            std::ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        panic!("Failed to open dir for sync, {}", unsafe { GetLastError() });
    }

    let r = unsafe { FlushFileBuffers(handle) };
    if r == 0 {
        panic!("Failed to flush dir, {}", unsafe { GetLastError() });
    }

    unsafe { CloseHandle(handle) };
}
