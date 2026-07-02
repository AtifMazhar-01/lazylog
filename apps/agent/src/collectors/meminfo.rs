use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use crate::Collector;
use crate::Writer;

#[derive(Debug, Default)]
pub struct MemoryInfo {
    total_memory: f64,
    available_memory: f64,
}
impl MemoryInfo {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn get_memory_gb(&mut self) -> (f64, f64) {
        (
            self.total_memory / 1024.0 / 1024.0,
            self.available_memory / 1024.0 / 1024.0,
        )
    }
}

impl Collector for MemoryInfo {
    fn collect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let memfile = fs::read_to_string("/proc/meminfo")?;

        for line in memfile.lines() {
            let mut parts = line.split_whitespace();

            match parts.next() {
                Some("MemTotal:") => {
                    self.total_memory = parts.next().unwrap().parse::<f64>()?;
                }

                Some("MemAvailable:") => {
                    self.available_memory = parts.next().unwrap().parse::<f64>()?;
                }

                _ => {}
            }
        }

        Ok(())
    }
}

impl Writer for MemoryInfo {
    fn write(&mut self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("csv/memory.csv")?;
        writeln!(
            file,
            "total_memory(kb),used_memory(kb),available_memory(kb)"
        )?;
        writeln!(
            file,
            "{:.4},{:.4},{:.4}",
            self.total_memory,
            (self.total_memory - self.available_memory),
            self.available_memory
        )?;

        Ok(())
    }
}
