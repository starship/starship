mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/starship-bindings.rs"));
}

use super::{get_page_size, get_phys_pages, sysctl_wrapper, MemoryInfo};
use libc::{xsw_usage, CTL_VM, VM_SWAPUSAGE};
use std::{io, io::Result, mem::MaybeUninit};

impl MemoryInfo {
    pub fn new() -> Result<Self> {
        // Get Swap Info
        let swap_usage: xsw_usage = sysctl_wrapper(&mut [CTL_VM, VM_SWAPUSAGE])?;
        // Get Total Memory
        let total = get_phys_pages()? as usize;

        // Get Usage Info
        let host_port = unsafe { bindings::mach_host_self() };
        let mut stat = MaybeUninit::<bindings::vm_statistics64>::uninit();
        let mut stat_count = bindings::STARSHIP_HOST_VM_INFO64_COUNT;

        let ret = unsafe {
            bindings::host_statistics64(
                host_port,
                bindings::HOST_VM_INFO64 as i32,
                stat.as_mut_ptr() as *mut i32,
                &mut stat_count,
            )
        };

        if ret != bindings::KERN_SUCCESS as i32 {
            return Err(io::Error::last_os_error());
        }
        let stat = unsafe { stat.assume_init() };

        // Get Page Size
        let page_size = get_page_size()? as usize;

        Ok(Self {
            avail: Some(
                (stat.free_count as usize + stat.inactive_count as usize) * page_size / 1024,
            ),
            free: Some(
                (stat.free_count as usize - stat.speculative_count as usize) * page_size / 1024,
            ),
            total: (total * page_size) as usize / 1024,
            swap_free: swap_usage.xsu_avail as usize / 1024,
            swap_total: swap_usage.xsu_total as usize / 1024,
        })
    }
}
