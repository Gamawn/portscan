use clap::{Parser, ValueEnum};
use futures::stream::{self, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use num_cpus;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};

#[derive(Parser, Debug)]
#[command(name = "portscanner")]
#[command(about = "A fast port scanner with service detection", long_about = None)]
struct Args {
    /// Target IP address to scan
    #[arg(short = 'i', long = "ip")]
    target: String,

    /// Start port number (default: 1)
    #[arg(short = 's', long = "start", default_value = "1")]
    port_start: u16,

    /// End port number (default: 1024)
    #[arg(short = 'e', long = "end", default_value = "1024")]
    port_end: u16,

    /// Scan type (tcp/udp)
    #[arg(short = 't', long = "type", default_value = "tcp")]
    scan_type: ScanType,

    /// Timeout in milliseconds (default: 200)
    #[arg(short = 'm', long = "timeout", default_value = "200")]
    timeout_ms: u64,

    /// Number of concurrent workers (default: CPU cores * 16)
    #[arg(short = 'w', long = "workers", default_value_t = num_cpus::get() * 16)]
    workers: usize,

    /// Batch size for processing (default: 100)
    #[arg(short = 'b', long = "batch", default_value = "100")]
    batch_size: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ScanType {
    Tcp,
    Udp,
}

const COMMON_SERVICES: &[(&str, u16)] = &[
    ("SSH", 22),
    ("HTTP", 80),
    ("HTTPS", 443),
    ("FTP", 21),
    ("SMTP", 25),
    ("DNS", 53),
    ("MySQL", 3306),
    ("PostgreSQL", 5432),
    ("Redis", 6379),
    ("MongoDB", 27017),
    ("RDP", 3389),
    ("Telnet", 23),
    ("IMAP", 143),
    ("SNMP", 161),
    ("SMB", 445),
    ("LDAP", 389),
];

#[derive(Debug)]
struct ScanResult {
    port: u16,
    is_open: bool,
    latency: Duration,
}

async fn scan_port(ip: IpAddr, port: u16, timeout_ms: u64) -> ScanResult {
    let addr = SocketAddr::new(ip, port);
    let start = std::time::Instant::now();

    let is_open = matches!(
        timeout(Duration::from_millis(timeout_ms), TcpStream::connect(&addr)).await,
        Ok(Ok(_))
    );

    ScanResult {
        port,
        is_open,
        latency: start.elapsed(),
    }
}

async fn scan_port_batch(
    ip: IpAddr,
    ports: Vec<u16>,
    timeout_ms: u64,
    results: Arc<Mutex<Vec<ScanResult>>>,
    scan_pb: ProgressBar,
    results_pb: ProgressBar,
) {
    let futures = ports
        .into_iter()
        .map(|port| scan_port(ip, port, timeout_ms));

    let batch_results: Vec<ScanResult> = futures::future::join_all(futures).await;

    let open_ports: Vec<ScanResult> = batch_results.into_iter().filter(|r| r.is_open).collect();

    if !open_ports.is_empty() {
        let mut results = results.lock().await;
        results.extend(open_ports);
        results_pb.inc(1);
    }

    scan_pb.inc(1);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let ip: IpAddr = args.target.parse()?;

    let service_map: HashMap<u16, &str> = COMMON_SERVICES
        .iter()
        .map(|&(name, port)| (port, name))
        .collect();

    let mp = MultiProgress::new();
    let total_ports = args.port_end - args.port_start + 1;
    let total_batches = (total_ports as usize + args.batch_size - 1) / args.batch_size;

    let scan_pb = mp.add(ProgressBar::new(total_batches as u64));
    scan_pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} batches ({per_sec})")
            .unwrap(),
    );

    let results_pb = mp.add(ProgressBar::new(64));
    results_pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.green/white} {pos} open ports found")
            .unwrap(),
    );

    println!(
        "\nScanning {} ports on {} with {} workers (batch size: {})\n",
        total_ports, ip, args.workers, args.batch_size
    );

    let results = Arc::new(Mutex::new(Vec::new()));

    let ports: Vec<Vec<u16>> = (args.port_start..=args.port_end)
        .collect::<Vec<u16>>()
        .chunks(args.batch_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    stream::iter(ports)
        .map(|batch| {
            let results = results.clone();
            let scan_pb = scan_pb.clone();
            let results_pb = results_pb.clone();

            async move {
                scan_port_batch(ip, batch, args.timeout_ms, results, scan_pb, results_pb).await;
            }
        })
        .buffer_unordered(args.workers)
        .collect::<Vec<()>>()
        .await;

    scan_pb.finish();
    results_pb.finish();

    let mut scan_results = results.lock().await;
    scan_results.sort_by_key(|r| r.port);

    println!("\nResults:");
    println!(
        "{:<10} {:<15} {:<15} {}",
        "PORT", "STATE", "SERVICE", "LATENCY"
    );
    println!("{}", "-".repeat(50));

    for result in scan_results.iter() {
        let service = service_map.get(&result.port).unwrap_or(&"Unknown");
        println!(
            "{:<10} {:<15} {:<15} {:.2}ms",
            result.port,
            "open",
            service,
            result.latency.as_secs_f64() * 1000.0
        );
    }

    println!("\nScan completed!");
    println!("Found {} open ports", scan_results.len());

    Ok(())
}
