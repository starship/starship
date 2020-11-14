use super::MemoryUsage;
use std::ops::Deref;
use sysinfo::{RefreshKind, System, SystemExt};
pub struct MemInfo(System);

impl MemoryUsage for MemInfo {
    fn create() -> Option<Self> {
        let system = System::new_with_specifics(RefreshKind::new().with_memory());
        Some(MemInfo(system))
    }

    fn used_memory(&self) -> u64 {
        self.0.get_used_memory()
    }

    fn total_memory(&self) -> u64 {
        self.0.get_total_memory()
    }

    fn total_swap(&self) -> u64 {
        self.0.get_total_swap()
    }

    fn used_swap(&self) -> u64 {
        self.0.get_used_swap()
    }
}
