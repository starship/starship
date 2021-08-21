use std::iter;
use std::os::windows::ffi::OsStrExt;
use std::{ffi::c_void, io::Error, path::Path, ptr};

mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::Foundation::PWSTR,
    Windows::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
    },
};

/// Attempt to get version information for an executable located in `path`
/// from the exe-file version metadata
pub fn from_metadata<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    log::debug!(
        "Getting version of {:?} from executable metadata",
        path.as_ref()
    );

    let mut path_wide: Vec<u16> = path
        .as_ref()
        .as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect();
    let path_pwstr = PWSTR(path_wide.as_mut_ptr());

    // handle is set to 0 and ignored later
    let mut handle = 0;

    // Get size of version information buffer for `GetFileVersionInfoW`
    let size = unsafe { GetFileVersionInfoSizeW(path_pwstr, &mut handle) };
    if size == 0 {
        log::error!("Failed to call GetFileVersionInfoSizeW");
        return Err(Error::last_os_error());
    }

    // Fill version information buffer
    let mut buffer = vec![0u8; size as usize];
    let rc = unsafe {
        GetFileVersionInfoW(
            path_pwstr,
            handle,
            size,
            buffer.as_mut_ptr().cast::<c_void>(),
        )
    };

    if rc.ok().is_err() {
        log::error!("Failed to call GetFileVersionW");
        return Err(Error::last_os_error());
    }

    // Will be set to point to VS_FIXEDFILEINFO struct in `buffer`
    let version_ptr: *mut VS_FIXEDFILEINFO = ptr::null_mut();
    let mut version_len = 0u32;

    // Query root (\) language-independent version information
    let rc = unsafe {
        VerQueryValueW(
            buffer.as_ptr().cast::<c_void>(),
            r"\",
            ptr::addr_of!(version_ptr) as *mut *mut c_void,
            &mut version_len,
        )
    };

    if rc.ok().is_err() || version_len == 0 || version_ptr.is_null() {
        log::error!("Failed to call VerQueryValueW: {:?}", rc.ok());
        return Err(Error::last_os_error());
    }

    let version = unsafe { &*version_ptr };

    let out = if (version.dwFileVersionLS) as u16 > 0 {
        format!(
            "{}.{}.{}.{}",
            (version.dwFileVersionMS >> 16) as u16,
            (version.dwFileVersionMS) as u16,
            (version.dwFileVersionLS >> 16) as u16,
            (version.dwFileVersionLS) as u16
        )
    } else {
        format!(
            "{}.{}.{}",
            (version.dwFileVersionMS >> 16) as u16,
            (version.dwFileVersionMS) as u16,
            (version.dwFileVersionLS >> 16) as u16,
        )
    };

    log::debug!("Version of \"{:?}\" resolved to \"{}\"", path.as_ref(), out);

    Ok(out)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    #[ignore]
    fn cli_and_metadata_same() {
        let output = utils::create_command(r"C:\Program Files\nodejs\node.exe")
            .unwrap()
            .arg("--version")
            .output()
            .unwrap();
        let cli_version = std::str::from_utf8(&output.stdout).unwrap().trim();

        assert_eq!(
            cli_version,
            format!(
                "v{}",
                from_metadata(r"C:\Program Files\nodejs\node.exe").unwrap()
            )
        )
    }
}
