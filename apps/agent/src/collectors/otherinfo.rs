use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use crate::Collector;
use crate::Writer;

#[derive(Debug, Default)]
pub struct SystemInfo {
    processes: u64,
    running_processes: u64,
    blocked_processes: u64,

    uptime: f64,
    idle_time: f64,

    boot_time: u64,

    active_users: u32,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn uptime_hours(&self) -> f64 {
        self.uptime / 3600.0
    }

    pub fn uptime_days(&self) -> f64 {
        self.uptime / 86400.0
    }
}

impl Collector for SystemInfo {
    fn collect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stat = fs::read_to_string("/proc/stat")?;

        for line in stat.lines() {
            let mut parts = line.split_whitespace();

            match parts.next() {
                Some("processes") => {
                    self.processes = parts.next().unwrap().parse()?;
                }

                Some("procs_running") => {
                    self.running_processes = parts.next().unwrap().parse()?;
                }

                Some("procs_blocked") => {
                    self.blocked_processes = parts.next().unwrap().parse()?;
                }

                Some("btime") => {
                    self.boot_time = parts.next().unwrap().parse()?;
                }

                _ => {}
            }
        }

        let uptime = fs::read_to_string("/proc/uptime")?;
        let mut parts = uptime.split_whitespace();

        self.uptime = parts.next().unwrap().parse()?;
        self.idle_time = parts.next().unwrap().parse()?;

        // Active users
        // TODO: Read from /var/run/utmp
        self.active_users = 0;

        Ok(())
    }
}

impl Writer for SystemInfo {
    fn write(&mut self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("csv/system.csv")?;

        writeln!(
            file,
            "processes,running_processes,blocked_processes,uptime_seconds,idle_seconds,boot_time_seconds,active_users"
        )?;

        writeln!(
            file,
            "{},{},{},{:.2},{:.2},{},{}",
            self.processes,
            self.running_processes,
            self.blocked_processes,
            self.uptime,
            self.idle_time,
            self.boot_time,
            self.active_users
        )?;

        Ok(())
    }
}
