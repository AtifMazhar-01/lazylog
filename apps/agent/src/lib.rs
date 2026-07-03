pub mod collectors;
pub use collectors::cpuinfo::{cpu_model, cpu_usage, logical_cpus};
pub use collectors::diskinfo::DiskCollector;
pub use collectors::meminfo::MemoryInfo;

use crate::collectors::cpuinfo::CpuStat;
pub fn just_print() {
    //memory
    let mut mem = MemoryInfo::new();
    if let Err(e) = mem.collect() {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = mem.write() {
        eprintln!("Error: {}", e);
    }

    //cpu
    let mut cpu_info = CpuStat::new();
    if let Err(e) = cpu_info.collect() {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = cpu_info.write() {
        eprintln!("Error: {}", e);
    };

    //disk
    let mut disk_info = DiskCollector::new();
    if let Err(e) = disk_info.collect() {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = disk_info.write() {
        eprintln!("Error: {}", e);
    };
}
pub trait Collector {
    fn collect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
pub trait Writer {
    fn write(&mut self) -> std::io::Result<()>;
}
