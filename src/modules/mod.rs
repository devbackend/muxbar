pub mod styled;

use crate::utils::strings;
use crate::utils::system::{battery::BatteryInformation, cpu};
use chrono::{DateTime, Local};
use std::time::Duration;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use crate::utils::strings::mem_usage;
use crate::utils::system::memory::{get_memory_usage, get_swap_usage, MemoryUsageUnit};

// Those are only constructed in config.rs
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Module {
    Custom(&'static str),

    Time(&'static str),

    Battery,

    CpuPercentage(usize),
    MemoryPercentage(usize),
    MemoryUsage(MemoryUsageUnit),
    SwapPercentage(usize),
    SwapUsage(MemoryUsageUnit),

    // Host data
    Uptime,
    Hostname,

    // Tmux
    SessionName,
    WindowName,
    WindowIndex,
    PaneIndex,
}

impl Module {
    fn display(self) -> Result<String, ()> {
        match self {
            Module::Custom(s) => Ok(String::from(s)),
            Module::Time(format) => {
                let now: DateTime<Local> = Local::now();

                Ok(now.format(format).to_string())
            }
            Module::Battery => {
                BatteryInformation::new().map(|info| format!("{}%", info.percentages))
            }
            Module::SessionName => Ok(String::from("#S")),
            Module::WindowName => Ok(String::from("#W")),
            Module::WindowIndex => Ok(String::from("#I")),
            Module::PaneIndex => Ok(String::from("#P")),
            Module::Hostname => Ok(String::from("#H")),
            Module::CpuPercentage(rounding) => Ok(strings::round(cpu::get_total_average(), rounding)),
            Module::MemoryPercentage(rounding) => {
                let mut sys = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                sys.refresh_memory();

                let total_memory = sys.total_memory();
                let used_memory = sys.used_memory();

                let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
                Ok(strings::round(memory_usage_percent, rounding))
            }
            Module::Uptime => {
                let uptime = System::uptime();
                let uptime = Duration::from_secs(uptime);

                Ok(format!("{}", strings::PrettyDuration::new(uptime)))
            }
            Module::SwapPercentage(rounding) => {
                let mut sys = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                sys.refresh_memory();

                let total_swap = sys.total_swap();
                let used_swap = sys.used_swap();

                let swap_usage_percent = (used_swap as f64 / total_swap as f64) * 100.0;
                Ok(strings::round(swap_usage_percent, rounding))
            }
            Module::MemoryUsage(unit) => Ok(mem_usage(get_memory_usage(), unit)),
            Module::SwapUsage(unit) => Ok(mem_usage(get_swap_usage(), unit)),
        }
    }
}
