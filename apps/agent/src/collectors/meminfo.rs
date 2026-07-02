use std::fs;

use crate::Collector;

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
