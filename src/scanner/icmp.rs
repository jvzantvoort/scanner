use std::net::Ipv4Addr;
use std::time::Duration;
use surge_ping::{Client, Config, PingIdentifier, PingSequence, ICMP};

/// Check if a host is reachable via ICMP ping
pub async fn ping_host(ip: Ipv4Addr, timeout_ms: u64) -> bool {
    let config = Config::builder().kind(ICMP::V4).build();

    let client = match Client::new(&config) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let mut pinger = client
        .pinger(ip.into(), PingIdentifier(rand::random()))
        .await;
    pinger.timeout(Duration::from_millis(timeout_ms));

    // Send a single ping
    match pinger.ping(PingSequence(0), &[0u8; 56]).await {
        Ok((_packet, _duration)) => true,
        Err(_) => false,
    }
}
