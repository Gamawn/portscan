use clap::ValueEnum;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub port: u16,
    pub is_open: bool,
    pub latency: Duration,
    pub service: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ScanType {
    Tcp,
    Udp,
}

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),

    #[error("Port range error: {0}")]
    InvalidPortRange(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] std::io::Error),

    #[error("Scan timeout")]
    Timeout,
}
