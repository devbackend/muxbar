use sysinfo::{MemoryRefreshKind, RefreshKind, System};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum MemoryUsageUnit {
    Bytes,
    KB,  // 1000                bytes
    KiB, // 1024                bytes
    MB,  // 1000 * 1000         bytes
    MiB, // 1024 * 1024         bytes
    GB,  // 1000 * 1000 * 1000  bytes
    GiB, // 1024 * 1024 * 1024  bytes
}

pub fn get_memory_usage() -> (u64, u64) {
    let sys = get_sys();

    (sys.used_memory(), sys.total_memory())
}

pub fn get_swap_usage() -> (u64, u64) {
    let sys = get_sys();

    (sys.used_swap(), sys.total_swap())
}

fn get_sys() -> System {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
    );

    sys.refresh_memory();

    sys
}