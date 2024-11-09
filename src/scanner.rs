use super::*;
use futures::StreamExt;

pub struct Scanner {
    ip: IpAddr,
    port_start: u16,
    port_end: u16,
    timeout: Duration,
    workers: usize,
    batch_size: usize,
    scan_type: ScanType,
    service_map: HashMap<u16, String>,
}

impl Scanner {
    pub fn new(args: &Args) -> Result<Self, ScanError> {
        let ip = args
            .target
            .parse()
            .map_err(|_| ScanError::InvalidIpAddress(args.target.clone()))?;

        if args.port_start > args.port_end {
            return Err(ScanError::InvalidPortRange(
                "Start port cannot be greater than end port".into(),
            ));
        }

        let service_map = COMMON_SERVICES
            .iter()
            .map(|&(name, port)| (port, name.to_string()))
            .collect();

        Ok(Scanner {
            ip,
            port_start: args.port_start,
            port_end: args.port_end,
            timeout: Duration::from_millis(args.timeout_ms),
            workers: args.workers,
            batch_size: args.batch_size,
            scan_type: args.scan_type,
            service_map,
        })
    }

    async fn scan_single_port(&self, port: u16) -> ScanResult {
        let addr = SocketAddr::new(self.ip, port);
        let start = std::time::Instant::now();

        let is_open = matches!(
            timeout(self.timeout, TcpStream::connect(&addr)).await,
            Ok(Ok(_))
        );

        let latency = start.elapsed();
        let service = self.service_map.get(&port).cloned();

        ScanResult {
            port,
            is_open,
            latency,
            service,
        }
    }

    async fn scan_port_batch(
        &self,
        ports: Vec<u16>,
        results: Arc<Mutex<Vec<ScanResult>>>,
        progress: &ui::ScanProgress,
    ) -> Result<(), ScanError> {
        let futures = ports.into_iter().map(|port| self.scan_single_port(port));

        let batch_results: Vec<ScanResult> = futures::future::join_all(futures).await;
        let open_ports: Vec<ScanResult> = batch_results.into_iter().filter(|r| r.is_open).collect();

        if !open_ports.is_empty() {
            let mut results = results.lock().await;
            results.extend(open_ports);
            progress.inc_found();
        }

        progress.inc_scanned();
        Ok(())
    }

    pub async fn run(&self) -> Result<Vec<ScanResult>, ScanError> {
        let total_ports = self.port_end - self.port_start + 1;
        let total_batches = (total_ports as usize + self.batch_size - 1) / self.batch_size;

        let progress = ui::ScanProgress::new(total_batches as u64);
        let results = Arc::new(Mutex::new(Vec::new()));

        println!(
            "\nScanning {} ports on {} with {} workers (batch size: {})\n",
            total_ports, self.ip, self.workers, self.batch_size
        );

        let ports: Vec<Vec<u16>> = (self.port_start..=self.port_end)
            .collect::<Vec<u16>>()
            .chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        futures::stream::iter(ports)
            .map(|batch| {
                let results = results.clone();
                let progress = progress.clone();

                async move {
                    self.scan_port_batch(batch, results, &progress).await?;
                    Ok::<_, ScanError>(())
                }
            })
            .buffer_unordered(self.workers)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        progress.finish();

        let mut scan_results = results.lock().await;
        scan_results.sort_by_key(|r| r.port);

        Ok(scan_results.to_vec())
    }
}
