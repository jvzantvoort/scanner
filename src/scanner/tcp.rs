use super::port::PortState;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Scan a single port on a host using TCP connect
pub async fn scan_port(addr: SocketAddr, timeout_ms: u64) -> PortState {
    let duration = Duration::from_millis(timeout_ms);

    match timeout(duration, TcpStream::connect(addr)).await {
        Ok(Ok(_stream)) => {
            // Successfully connected - port is open
            PortState::Open
        }
        Ok(Err(e)) => {
            // Connection failed
            match e.kind() {
                std::io::ErrorKind::ConnectionRefused => PortState::Closed,
                std::io::ErrorKind::PermissionDenied => PortState::Filtered,
                _ => PortState::Filtered,
            }
        }
        Err(_) => {
            // Timeout occurred
            PortState::Filtered
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[tokio::test]
    async fn test_scan_closed_port() {
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            1, // Port 1 should be closed
        );
        let state = scan_port(addr, 100).await;
        // On localhost, closed ports typically return ConnectionRefused
        assert!(matches!(state, PortState::Closed | PortState::Filtered));
    }
}
