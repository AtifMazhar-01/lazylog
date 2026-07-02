pub mod collectors;
pub use collectors::meminfo::MemoryInfo;
pub fn just_print() {
    let mut mem = MemoryInfo::new();
    if let Err(e) = mem.collect() {
        eprintln!("Error: {}", e);
    }
    if let Err(e) = mem.write() {
        eprintln!("Error: {}", e);
    }
    let (total_mem, available_mem) = mem.get_memory_gb();
    let used_memory = total_mem - available_mem;
    println!("Total memory : {:.4} GB", total_mem);
    println!("Used memory : {:.4} GB", used_memory);
    println!("Available memory : {:.4} GB", available_mem);
}
pub trait Collector {
    fn collect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
pub trait Writer {
    fn write(&mut self) -> std::io::Result<()>;
}
