#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(target_os = "freebsd")]
mod freebsd;

#[cfg(target_os = "freebsd")]
pub use freebsd::*;

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
fn sysctl_wrapper<T>(mib: &mut [libc::c_int]) -> std::io::Result<T> {
    use std::mem;

    let mut size = mem::size_of::<T>() as libc::size_t;
    let mut value = mem::MaybeUninit::<T>::uninit();

    let result = unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as libc::c_uint,
            value.as_mut_ptr() as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };

    if result == -1 {
        return Err(std::io::Error::last_os_error());
    }

    unsafe { Ok(value.assume_init()) }
}

#[cfg(target_os = "freebsd")]
fn sysctl_by_name_wrapper<T>(name: &str) -> std::io::Result<T> {
    use std::mem;

    let name = std::ffi::CString::new(name).unwrap();

    let mut size = mem::size_of::<T>() as libc::size_t;
    let mut value = mem::MaybeUninit::<T>::uninit();

    let result = unsafe {
        libc::sysctlbyname(
            name.as_ptr(),
            value.as_mut_ptr() as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };

    if result == -1 {
        return Err(std::io::Error::last_os_error());
    }

    unsafe { Ok(value.assume_init()) }
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
fn get_page_size() -> std::io::Result<u64> {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page_size < 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(page_size as u64)
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
fn get_phys_pages() -> std::io::Result<u64> {
    let pages = unsafe { libc::sysconf(libc::_SC_PHYS_PAGES) };
    if pages < 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(pages as u64)
}

/// Memory and Swap usage information in KiB
pub struct MemoryInfo {
    pub free: Option<usize>,
    pub avail: Option<usize>,
    pub total: usize,
    pub swap_total: usize,
    pub swap_free: usize,
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd"
)))]
impl MemoryInfo {
    pub fn new() -> std::io::Result<Self> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Your OS does not support the memory_usage module",
        ))
    }
}

#[cfg(all(
    test,
    any(target_os = "windows", target_os = "linux", target_os = "macos")
))]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let info = MemoryInfo::new().unwrap();
        assert_ne!(info.total, 0);
        assert!(info.total >= info.free.or(info.avail).unwrap());
        assert!(info
            .avail
            .zip(info.free)
            .map(|(avail, free)| avail >= free)
            .unwrap_or(true));
        assert!(info.swap_total >= info.swap_free);
    }
}
