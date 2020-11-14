use super::MemoryUsage;
pub struct MemInfo(procfs::Meminfo);

impl MemoryUsage for MemInfo {
    fn create() -> Option<MemInfo> {
        let inner = procfs::Meminfo::new().ok()?;
        Some(MemInfo(inner))
    }

    fn used_memory(&self) -> u64 {
        let info = &self.0;
        let used_in_byte = info.mem_total
            - info.mem_free
            - info.buffers
            - info.cached
            - info.s_reclaimable.unwrap_or(0);
        used_in_byte >> 10
    }

    fn total_memory(&self) -> u64 {
        let total_in_byte = self.0.mem_total;
        total_in_byte >> 10
    }

    fn total_swap(&self) -> u64 {
        let total_in_byte = self.0.swap_total;
        total_in_byte >> 10
    }

    fn used_swap(&self) -> u64 {
        let used_in_byte = self.0.swap_total - self.0.swap_free;
        used_in_byte >> 10
    }
}
