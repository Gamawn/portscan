use portscan::COMMON_SERVICES;
use std::collections::HashSet;

#[test]
fn test_services_unique_ports() {
    let ports: HashSet<u16> = COMMON_SERVICES.iter().map(|&(_, port)| port).collect();

    assert_eq!(ports.len(), COMMON_SERVICES.len());
}

#[test]
fn test_services_valid_ports() {
    for &(_, port) in COMMON_SERVICES {
        assert!(port > 0);
    }
}

#[test]
fn test_services_common_ports() {
    let ports: HashSet<u16> = COMMON_SERVICES.iter().map(|&(_, port)| port).collect();

    assert!(ports.contains(&80));
    assert!(ports.contains(&443));
    assert!(ports.contains(&22));
    assert!(ports.contains(&21));
}
