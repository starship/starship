use super::MemoryInfo;
use super::{get_page_size, get_phys_pages, sysctl_by_name_wrapper, sysctl_wrapper};
use libc::{c_int, c_uint, dev_t};
use std::io::Result;

const XSWDEV_VERSION: u32 = 2;

#[repr(C)]
struct xswdev {
    xsw_version: c_uint,
    xsw_dev: dev_t,
    xsw_flags: c_int,
    xsw_nblks: c_int,
    xsw_used: c_int,
}

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
            log::info!("XSW version mismatch. Starship does not support swap information for your FreeBSD version.");
            return Ok((0, 0));
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
