mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/mach-bindings.rs"));
}

use super::MemoryInfo;
use libc::{
    c_int, c_uint, c_void, size_t, sysconf, sysctl, xsw_usage, CTL_HW, CTL_VM, HW_MEMSIZE,
    VM_SWAPUSAGE, _SC_PAGE_SIZE,
};
use std::{
    io,
    io::Result,
    mem::{size_of, MaybeUninit},
    ptr::null_mut,
};

fn sysctl_wrapper<T>(mib: &mut [c_int]) -> Result<T> {
    let mut size = size_of::<T>() as size_t;
    let mut value = MaybeUninit::<T>::uninit();

    let result = unsafe {
        sysctl(
            mib.as_mut_ptr(),
            mib.len() as c_uint,
            value.as_mut_ptr() as *mut c_void,
            &mut size,
            null_mut(),
            0,
        )
    };

    if result == -1 {
        return Err(io::Error::last_os_error());
    }

    unsafe { Ok(value.assume_init()) }
}

impl MemoryInfo {
    pub fn new() -> Result<Self> {
        let swap_usage: xsw_usage = sysctl_wrapper(&mut [CTL_VM, VM_SWAPUSAGE])?;
        let total: u64 = sysctl_wrapper(&mut [CTL_HW, HW_MEMSIZE])?;

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

        let page_size = unsafe { sysconf(_SC_PAGE_SIZE) };
        if page_size < 0 {
            return Err(io::Error::last_os_error());
        }
        let page_size = page_size as usize;

        let stat = unsafe { stat.assume_init() };

        Ok(Self {
            avail: Some(
                (stat.free_count as usize + stat.inactive_count as usize) * page_size / 1024,
            ),
            free: Some(
                (stat.free_count as usize - stat.speculative_count as usize) * page_size / 1024,
            ),
            total: total as usize / 1024,
            swap_free: swap_usage.xsu_avail as usize / 1024,
            swap_total: swap_usage.xsu_total as usize / 1024,
        })
    }
}
