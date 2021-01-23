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

/// Memory and Swap usage information in KiB
pub struct MemoryInfo {
    pub free: Option<usize>,
    pub avail: Option<usize>,
    pub total: usize,
    pub swap_total: usize,
    pub swap_free: usize,
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
impl MemoryInfo {
    fn new() -> Result<Self, ()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Your OS does not support the memory_usage module",
        ))
    }
}

#[cfg(test)]
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
