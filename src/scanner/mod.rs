pub mod host;
pub mod icmp;
pub mod port;
pub mod tcp;

use crate::error::Result;
use dns_lookup::lookup_addr;
use host::HostScanResult;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

pub struct ScanConfig {
    pub timeout_ms: u64,
    pub concurrency: usize,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 1000,
            concurrency: 100,
        }
    }
}

/// Scan multiple hosts and ports concurrently
pub async fn scan_network(
    targets: Vec<Ipv4Addr>,
    ports: Vec<u16>,
    config: ScanConfig,
) -> Result<Vec<HostScanResult>> {
    // First, ping all hosts to see which ones are up
    let mut alive_hosts = Vec::new();
    let mut ping_tasks = JoinSet::new();

    for ip in targets.iter().copied() {
        let timeout_ms = config.timeout_ms;
        ping_tasks.spawn(async move {
            let is_alive = icmp::ping_host(ip, timeout_ms).await;
            (ip, is_alive)
        });
    }

    // Collect alive hosts
    while let Some(task_result) = ping_tasks.join_next().await {
        if let Ok((ip, is_alive)) = task_result {
            if is_alive {
                alive_hosts.push(ip);
            }
        }
    }

    // If no hosts responded to ping, return empty results
    if alive_hosts.is_empty() {
        return Ok(Vec::new());
    }

    let semaphore = Arc::new(Semaphore::new(config.concurrency));
    let mut tasks = JoinSet::new();

    // Spawn tasks for each (IP, port) combination - only for alive hosts
    for ip in alive_hosts.iter().copied() {
        for port in ports.iter().copied() {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let addr = SocketAddr::new(IpAddr::V4(ip), port);
            let timeout_ms = config.timeout_ms;

            tasks.spawn(async move {
                let state = tcp::scan_port(addr, timeout_ms).await;
                drop(permit);
                (ip, port, state)
            });
        }
    }

    // Collect results
    let mut results: HashMap<Ipv4Addr, HostScanResult> = HashMap::new();

    while let Some(task_result) = tasks.join_next().await {
        if let Ok((ip, port, state)) = task_result {
            results
                .entry(ip)
                .or_insert_with(|| HostScanResult::new(ip))
                .add_port_result(port, state);
        }
    }

    // Resolve hostnames for alive hosts
    let mut hostname_tasks = JoinSet::new();
    for ip in alive_hosts.iter().copied() {
        hostname_tasks.spawn(async move {
            let hostname = lookup_addr(&IpAddr::V4(ip)).ok();
            (ip, hostname)
        });
    }

    while let Some(task_result) = hostname_tasks.join_next().await {
        if let Ok((ip, Some(hostname))) = task_result {
            if let Some(result) = results.get_mut(&ip) {
                result.set_hostname(hostname);
            }
        }
    }

    // Convert to sorted vector
    let mut host_results: Vec<HostScanResult> = results.into_values().collect();
    host_results.sort();

    Ok(host_results)
}
