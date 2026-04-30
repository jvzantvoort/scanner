use super::port::PortState;
use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct HostScanResult {
    pub ip: Ipv4Addr,
    pub hostname: Option<String>,
    pub is_up: bool,
    pub ports: HashMap<u16, PortState>,
}

impl HostScanResult {
    pub fn new(ip: Ipv4Addr) -> Self {
        Self {
            ip,
            hostname: None,
            is_up: false,
            ports: HashMap::new(),
        }
    }

    pub fn add_port_result(&mut self, port: u16, state: PortState) {
        self.ports.insert(port, state);
        if state == PortState::Open {
            self.is_up = true;
        }
    }

    pub fn set_hostname(&mut self, hostname: String) {
        self.hostname = Some(hostname);
    }

    pub fn has_open_ports(&self) -> bool {
        self.ports.values().any(|s| *s == PortState::Open)
    }

    pub fn open_port_count(&self) -> usize {
        self.ports
            .values()
            .filter(|s| **s == PortState::Open)
            .count()
    }
}

impl Ord for HostScanResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ip.cmp(&other.ip)
    }
}

impl PartialOrd for HostScanResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HostScanResult {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip
    }
}

impl Eq for HostScanResult {}
