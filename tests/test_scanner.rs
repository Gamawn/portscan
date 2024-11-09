use portscan::{Args, ScanError, ScanType, Scanner};
use tokio::net::TcpListener;

pub async fn setup_test_servers(num_ports: u32) -> Vec<u16> {
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
async fn test_scanner_creation() {
    let args = Args {
        target: "127.0.0.1".to_string(),
        port_start: 1,
        port_end: 100,
        scan_type: ScanType::Tcp,
        timeout_ms: 200,
        workers: 10,
        batch_size: 50,
    };

    let scanner = Scanner::new(&args);
    assert!(scanner.is_ok());
}

#[tokio::test]
async fn test_scanner_invalid_ip() {
    let args = Args {
        target: "invalid_ip".to_string(),
        port_start: 1,
        port_end: 100,
        scan_type: ScanType::Tcp,
        timeout_ms: 200,
        workers: 10,
        batch_size: 50,
    };

    let scanner = Scanner::new(&args);
    assert!(matches!(
        scanner.err(),
        Some(ScanError::InvalidIpAddress(_))
    ));
}

#[tokio::test]
async fn test_scanner_invalid_port_range() {
    let args = Args {
        target: "127.0.0.1".to_string(),
        port_start: 100,
        port_end: 1, // Invalid: start > end
        scan_type: ScanType::Tcp,
        timeout_ms: 200,
        workers: 10,
        batch_size: 50,
    };

    let scanner = Scanner::new(&args);
    assert!(matches!(
        scanner.err(),
        Some(ScanError::InvalidPortRange(_))
    ));
}

#[tokio::test]
async fn test_scanner_single_open_port() {
    let open_ports = setup_test_servers(1).await;
    let port = open_ports[0];

    let args = Args {
        target: "127.0.0.1".to_string(),
        port_start: port,
        port_end: port,
        scan_type: ScanType::Tcp,
        timeout_ms: 1000,
        workers: 1,
        batch_size: 1,
    };

    let scanner = Scanner::new(&args).unwrap();
    let results = scanner.run().await.unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].port, port);
    assert!(results[0].is_open);
}

#[tokio::test]
async fn test_scanner_multiple_ports() {
    let open_ports = setup_test_servers(3).await;

    let min_port = *open_ports.iter().min().unwrap();
    let max_port = *open_ports.iter().max().unwrap();

    let args = Args {
        target: "127.0.0.1".to_string(),
        port_start: min_port,
        port_end: max_port,
        scan_type: ScanType::Tcp,
        timeout_ms: 1000,
        workers: 2,
        batch_size: 2,
    };

    let scanner = Scanner::new(&args).unwrap();
    let results = scanner.run().await.unwrap();

    let found_ports: Vec<u16> = results
        .iter()
        .filter(|r| r.is_open)
        .map(|r| r.port)
        .collect();

    assert_eq!(found_ports.len(), open_ports.len());
    for port in open_ports {
        assert!(found_ports.contains(&port));
    }
}
