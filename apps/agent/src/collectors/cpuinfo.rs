use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, thread, time::Duration};

use crate::{Collector, Writer};

#[derive(Debug, Default)]
pub struct CpuStat {
    idle: u64,
    total: u64,

    pub cpu_model: String,
    pub cores: usize,
    pub cpu_usage: f64,
}
impl CpuStat {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

fn read_cpu_times() -> (u64, u64) {
    let stat = fs::read_to_string("/proc/stat").unwrap();

    let cpu = stat.lines().next().unwrap();

    let values: Vec<u64> = cpu
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect();

    let idle = values[3] + values[4];
    let total: u64 = values.iter().sum();

    (idle, total)
}

pub fn cpu_usage() -> f64 {
    let (idle1, total1) = read_cpu_times();

    thread::sleep(Duration::from_secs(1));

    let (idle2, total2) = read_cpu_times();

    let idle = idle2 - idle1;
    let total = total2 - total1;

    100.0 * (1.0 - idle as f64 / total as f64)
}

pub fn cpu_model() -> Option<String> {
    let content = fs::read_to_string("/proc/cpuinfo").ok()?;

    for line in content.lines() {
        if let Some(model) = line.strip_prefix("model name\t: ") {
            return Some(model.to_string());
        }
    }

    None
}

pub fn logical_cpus() -> usize {
    let content = fs::read_to_string("/proc/cpuinfo").unwrap();

    content
        .lines()
        .filter(|line| line.starts_with("processor"))
        .count()
}

impl Collector for CpuStat {
    fn collect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.cpu_model = cpu_model().unwrap_or_else(|| "Unknown".to_string());
        self.cores = logical_cpus();
        self.cpu_usage = cpu_usage();
        (self.idle, self.total) = read_cpu_times();

        Ok(())
    }
}

impl Writer for CpuStat {
    fn write(&mut self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("csv/cpu.csv")?;
        writeln!(file, "cpu_model, no_of_cores, cpu_usage(%)")?;
        writeln!(
            file,
            "{},{:.4},{:.4}",
            self.cpu_model, self.cores, self.cpu_usage
        )?;

        Ok(())
    }
}
