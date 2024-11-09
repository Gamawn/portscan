use super::types::ScanType;
use clap::Parser;
use num_cpus;

#[derive(Parser, Debug)]
#[command(name = "portscanner")]
#[command(about = "A fast port scanner with service detection", long_about = None)]
pub struct Args {
    /// Target IP address to scan
    #[arg(short = 'i', long = "ip")]
    pub target: String,

    /// Start port number (default: 1)
    #[arg(short = 's', long = "start", default_value = "1")]
    pub port_start: u16,

    /// End port number (default: 1024)
    #[arg(short = 'e', long = "end", default_value = "1024")]
    pub port_end: u16,

    /// Scan type (tcp/udp)
    #[arg(short = 't', long = "type", default_value = "tcp")]
    pub scan_type: ScanType,

    /// Timeout in milliseconds (default: 200)
    #[arg(short = 'm', long = "timeout", default_value = "200")]
    pub timeout_ms: u64,

    /// Number of concurrent workers (default: CPU cores * 16)
    #[arg(short = 'w', long = "workers", default_value_t = num_cpus::get() * 16)]
    pub workers: usize,

    /// Batch size for processing (default: 100)
    #[arg(short = 'b', long = "batch", default_value = "100")]
    pub batch_size: usize,
}
