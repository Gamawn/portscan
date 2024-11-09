use clap::Parser;
use portscan::{Args, ScanType};

#[test]
fn test_cli_default_values() {
    let args = Args::parse_from(["portscanner", "-i", "127.0.0.1"]);

    assert_eq!(args.target, "127.0.0.1");
    assert_eq!(args.port_start, 1);
    assert_eq!(args.port_end, 1024);
    assert!(matches!(args.scan_type, ScanType::Tcp));
    assert_eq!(args.timeout_ms, 200);
    assert_eq!(args.batch_size, 100);
}

#[test]
fn test_cli_custom_values() {
    let args = Args::parse_from([
        "portscanner",
        "-i",
        "192.168.1.1",
        "-s",
        "80",
        "-e",
        "443",
        "-t",
        "tcp",
        "-m",
        "500",
        "-w",
        "32",
        "-b",
        "50",
    ]);

    assert_eq!(args.target, "192.168.1.1");
    assert_eq!(args.port_start, 80);
    assert_eq!(args.port_end, 443);
    assert!(matches!(args.scan_type, ScanType::Tcp));
    assert_eq!(args.timeout_ms, 500);
    assert_eq!(args.workers, 32);
    assert_eq!(args.batch_size, 50);
}
