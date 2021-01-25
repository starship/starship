use super::MemoryInfo;
use super::{get_page_size, get_phys_pages, sysctl_by_name_wrapper, sysctl_wrapper};
use std::io::Result;

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/starship-bindings.rs"));
}

use bindings::{xswdev, XSWDEV_VERSION};

fn get_swap_info() -> Result<(u64, u64)> {
    let n_swap_devices: i32 = sysctl_by_name_wrapper("vm.nswapdev")?;
    if n_swap_devices <= 0 {
        return Ok((0, 0));
    }

    let mut mib = [0i32; 16];
    let name = std::ffi::CString::new("vm.swap_info").unwrap();
    let mut size = mib.len() - 1;
    let result = unsafe { libc::sysctlnametomib(name.as_ptr(), mib.as_mut_ptr(), &mut size) };
    if result == -1 {
        return Err(std::io::Error::last_os_error());
    }

    let mib = &mut mib[0..=size];
    let (mut swap_free, mut swap_total) = (0, 0);
    for i in 0..n_swap_devices {
        mib[size] = i;

        let swap_info: xswdev = sysctl_wrapper(mib)?;

        if swap_info.xsw_version != XSWDEV_VERSION {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "XSW version mismatch. Please recompile starship for your FreeBSD version.",
            ));
        }

        swap_free += (swap_info.xsw_nblks - swap_info.xsw_used) as u64;
        swap_total += (swap_info.xsw_nblks) as u64;
    }

    Ok((swap_free, swap_total))
}

impl MemoryInfo {
    pub fn new() -> Result<Self> {
        let page_size: u64 = get_page_size()?;
        let total: u64 = get_phys_pages()?;

        let inactive: u64 = sysctl_by_name_wrapper("vm.stats.vm.v_inactive_count")?;
        let cache: u64 = sysctl_by_name_wrapper("vm.stats.vm.v_cache_count")?;
        let free: u64 = sysctl_by_name_wrapper("vm.stats.vm.v_free_count")?;
        let laundry: u64 = sysctl_by_name_wrapper("vm.stats.vm.v_laundry_count").unwrap_or(0);
        let arc: u64 = sysctl_by_name_wrapper("kstat.zfs.misc.arcstats.size").unwrap_or(0);

        let (swap_free, swap_total) = get_swap_info()?;

        Ok(Self {
            total: (total * page_size / 1024) as usize,
            free: Some((free * page_size / 1024) as usize),
            avail: Some((((free + cache + inactive + laundry) * page_size + arc) / 1024) as usize),
            swap_free: (swap_free * page_size / 1024) as usize,
            swap_total: (swap_total * page_size / 1024) as usize,
        })
    }
}
