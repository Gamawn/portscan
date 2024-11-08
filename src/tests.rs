use super::*;
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::TcpListener;

#[tokio::test]
async fn test_scan_port_open() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (_socket, _) = listener.accept().await.unwrap();
    });

    let result = scan_port(IpAddr::V4(Ipv4Addr::LOCALHOST), addr.port(), 1000).await;

    assert!(
        result.is_open,
        "result.is_open: {} but need true",
        result.is_open
    );
    assert_eq!(
        result.port,
        addr.port(),
        "testing eq of {} with {}",
        result.port,
        addr.port()
    );
}

#[tokio::test]
async fn test_scan_port_closed() {
    let port = 54321;
    let result = scan_port(IpAddr::V4(Ipv4Addr::LOCALHOST), port, 1000).await;

    assert!(!result.is_open);
    assert_eq!(result.port, port);
}

#[tokio::test]
async fn test_scan_port_timeout() {
    let result = scan_port(
        "10.255.255.255".parse().unwrap(),
        80,
        100, // Very short timeout
    )
    .await;

    assert!(!result.is_open);
    assert!(result.latency.as_millis() >= 100);
}

#[tokio::test]
async fn test_scan_port_batch() {
    let mp = MultiProgress::new();
    let scan_pb = mp.add(ProgressBar::new(1));
    let results_pb = mp.add(ProgressBar::new(64));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (_socket, _) = listener.accept().await.unwrap();
    });

    let results = Arc::new(Mutex::new(Vec::new()));
    let ports = vec![addr.port(), 54321];

    scan_port_batch(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        ports,
        1000,
        results.clone(),
        scan_pb,
        results_pb,
    )
    .await;

    let scan_results = results.lock().await;
    assert_eq!(scan_results.len(), 1);
    assert_eq!(scan_results[0].port, addr.port());
    assert!(scan_results[0].is_open);
}

#[test]
fn test_args_parsing() {
    let args = Args::parse_from(
        [
            "portscanner",
            "-i",
            "127.0.0.1",
            "-s",
            "1",
            "-e",
            "100",
            "-t",
            "tcp",
            "-m",
            "200",
            "-w",
            "32",
            "-b",
            "50",
        ]
        .iter(),
    );

    assert_eq!(args.target, "127.0.0.1");
    assert_eq!(args.port_start, 1);
    assert_eq!(args.port_end, 100);
    assert!(matches!(args.scan_type, ScanType::Tcp));
    assert_eq!(args.timeout_ms, 200);
    assert_eq!(args.workers, 32);
    assert_eq!(args.batch_size, 50);
}

#[test]
fn test_service_map() {
    let service_map: HashMap<u16, &str> = COMMON_SERVICES
        .iter()
        .map(|&(name, port)| (port, name))
        .collect();

    assert_eq!(service_map.get(&80), Some(&"HTTP"));
    assert_eq!(service_map.get(&443), Some(&"HTTPS"));
    assert_eq!(service_map.get(&22), Some(&"SSH"));
    assert_eq!(service_map.get(&12345), None);
}

async fn setup_test_server(num_ports: u32) -> Vec<u16> {
    let mut open_ports = Vec::new();

    for _ in 0..num_ports {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        open_ports.push(port);

        tokio::spawn(async move {
            let (_socket, _) = listener.accept().await.unwrap();
        });
    }

    open_ports
}

#[tokio::test]
async fn test_integration_multiple_ports() {
    let open_ports = setup_test_server(3).await;

    let mp = MultiProgress::new();
    let scan_pb = mp.add(ProgressBar::new(1));
    let results_pb = mp.add(ProgressBar::new(64));

    let results = Arc::new(Mutex::new(Vec::new()));

    let mut test_ports = open_ports.clone();
    test_ports.extend_from_slice(&[54321, 54322, 54323]);

    scan_port_batch(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        test_ports,
        1000,
        results.clone(),
        scan_pb,
        results_pb,
    )
    .await;

    let scan_results = results.lock().await;
    assert_eq!(scan_results.len(), open_ports.len());

    let mut found_ports: Vec<u16> = scan_results.iter().map(|r| r.port).collect();
    found_ports.sort();

    let mut expected_ports = open_ports;
    expected_ports.sort();

    assert_eq!(found_ports, expected_ports);
}
