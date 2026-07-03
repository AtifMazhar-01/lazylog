use crate::Collector;
use crate::Writer;
use std::{error::Error, fs};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Debug)]
pub struct DiskStats {
    pub name: String,
    pub reads_completed: u64,
    pub reads_merged: u64,
    pub sectors_read: u64,
    pub time_reading_ms: u64,

    pub writes_completed: u64,
    pub writes_merged: u64,
    pub sectors_written: u64,
    pub time_writing_ms: u64,

    pub io_in_progress: u64,
    pub io_time_ms: u64,
    pub weighted_io_time_ms: u64,
}

pub struct DiskCollector {
    pub disks: Vec<DiskStats>,
}
impl DiskCollector {
    pub fn new() -> Self {
        Self { disks: Vec::new() }
    }
}

impl Default for DiskCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl Collector for DiskCollector {
    fn collect(&mut self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string("/proc/diskstats")?;

        self.disks.clear();

        for line in contents.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();

            if fields.len() < 14 {
                continue;
            }

            let disk = DiskStats {
                name: fields[2].to_string(),
                reads_completed: fields[3].parse().unwrap_or(0),
                reads_merged: fields[4].parse().unwrap_or(0),
                sectors_read: fields[5].parse().unwrap_or(0),
                time_reading_ms: fields[6].parse().unwrap_or(0),

                writes_completed: fields[7].parse().unwrap_or(0),
                writes_merged: fields[8].parse().unwrap_or(0),
                sectors_written: fields[9].parse().unwrap_or(0),
                time_writing_ms: fields[10].parse().unwrap_or(0),

                io_in_progress: fields[11].parse().unwrap_or(0),
                io_time_ms: fields[12].parse().unwrap_or(0),
                weighted_io_time_ms: fields[13].parse().unwrap_or(0),
            };

            self.disks.push(disk);
        }

        Ok(())
    }
}

impl Writer for DiskCollector {
    fn write(&mut self) -> std::io::Result<()> {
        let file = File::create("csv/diskstats.csv")?;
        let mut writer = BufWriter::new(file);

        writeln!(
            writer,
            "name,reads_completed,reads_merged,sectors_read,time_reading_ms,\
             writes_completed,writes_merged,sectors_written,time_writing_ms,\
             io_in_progress,io_time_ms,weighted_io_time_ms"
        )?;

        for disk in &self.disks {
            writeln!(
                writer,
                "{},{},{},{},{},{},{},{},{},{},{},{}",
                disk.name,
                disk.reads_completed,
                disk.reads_merged,
                disk.sectors_read,
                disk.time_reading_ms,
                disk.writes_completed,
                disk.writes_merged,
                disk.sectors_written,
                disk.time_writing_ms,
                disk.io_in_progress,
                disk.io_time_ms,
                disk.weighted_io_time_ms
            )?;
        }

        Ok(())
    }
}
