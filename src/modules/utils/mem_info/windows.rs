use super::MemoryInfo;
use std::io::{Error, Result};
use std::mem::MaybeUninit;
use winapi::um::psapi::{K32GetPerformanceInfo, PERFORMANCE_INFORMATION};

impl MemoryInfo {
    pub fn new() -> Result<Self> {
        let mut mem_info = MaybeUninit::<PERFORMANCE_INFORMATION>::uninit();
        let len = std::mem::size_of::<PERFORMANCE_INFORMATION>() as u32;

        let ret = unsafe { K32GetPerformanceInfo(mem_info.as_mut_ptr(), len) };
        if ret == 0 {
            return Err(Error::last_os_error());
        }

        let mem_info = unsafe { mem_info.assume_init() };

        Ok(Self {
            total: mem_info.PhysicalTotal * mem_info.PageSize / 1024,
            avail: Some(mem_info.PhysicalAvailable * mem_info.PageSize / 1024),
            free: None,
            swap_total: (mem_info.CommitLimit - mem_info.PhysicalTotal) * mem_info.PageSize / 1024,
            swap_free: (mem_info.CommitLimit - mem_info.CommitTotal - mem_info.PhysicalAvailable)
                * mem_info.PageSize
                / 1024,
        })
    }
}
