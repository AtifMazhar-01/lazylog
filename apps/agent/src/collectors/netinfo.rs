use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use crate::Collector;
use crate::Writer;

#[derive(Debug, Default)]
pub struct NetworkInfo {
    interface: String,

    // Receive(rx)
    rx_bytes: u64,
    rx_packets: u64,
    rx_errors: u64,
    rx_drop: u64,

    // Transmit (TX)
    tx_bytes: u64,
    tx_packets: u64,
    tx_errors: u64,
    tx_drop: u64,
}

impl NetworkInfo {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Collector for NetworkInfo {
    fn collect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let netfile = fs::read_to_string("/proc/net/dev")?;

        // Skip the two header lines
        for line in netfile.lines().skip(2) {
            let (iface, stats) = line.split_once(':').unwrap();

            // Ignore loopback interface
            if iface.trim() == "lo" {
                continue;
            }

            self.interface = iface.trim().to_string();

            let mut values = stats.split_whitespace();

            // Receive
            self.rx_bytes = values.next().unwrap().parse()?;
            self.rx_packets = values.next().unwrap().parse()?;
            self.rx_errors = values.next().unwrap().parse()?;
            self.rx_drop = values.next().unwrap().parse()?;

            // Skip fifo, frame, compressed, multicast
            for _ in 0..4 {
                values.next();
            }

            // Transmit
            self.tx_bytes = values.next().unwrap().parse()?;
            self.tx_packets = values.next().unwrap().parse()?;
            self.tx_errors = values.next().unwrap().parse()?;
            self.tx_drop = values.next().unwrap().parse()?;

            // Read only the first non-loopback interface
            break;
        }

        Ok(())
    }
}

impl Writer for NetworkInfo {
    fn write(&mut self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("csv/network.csv")?;

        writeln!(
            file,
            "interface,rx_bytes Receive Bytes(RX),rx_packets,rx_errors,rx_drop,tx_bytes Transmit Bytes(TX),tx_packets,tx_errors,tx_drop"
        )?;

        writeln!(
            file,
            "{},{},{},{},{},{},{},{},{}",
            self.interface,
            self.rx_bytes,
            self.rx_packets,
            self.rx_errors,
            self.rx_drop,
            self.tx_bytes,
            self.tx_packets,
            self.tx_errors,
            self.tx_drop
        )?;

        Ok(())
    }
}
