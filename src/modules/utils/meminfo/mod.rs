#[cfg(target_os = "linux")]
mod meminfo_linux;
#[cfg(target_os = "linux")]
pub use meminfo_linux::MemInfo;

#[cfg(not(target_os = "linux"))]
mod meminfo_general;
#[cfg(not(target_os = "linux"))]
pub use meminfo_general::MemInfo;

/// Contains all the methods to calculate the memory usage
pub trait MemoryUsage: Sized {
    /// Returns the instance of memory information handle.
    fn create() -> Option<Self>;
    /// Returns the amound of used RAM in kiB.
    fn used_memory(&self) -> u64;
    /// Returns the RAM size in kiB.
    fn total_memory(&self) -> u64;
    /// Returns the SWAP size in kiB.
    fn total_swap(&self) -> u64;
    /// Returns the amount of used SWAP in kiB.
    fn used_swap(&self) -> u64;
}
