use crate::error::{Result, ScannerError};
use ipnetwork::Ipv4Network;
use std::net::Ipv4Addr;

/// Parse CIDR notation into a list of IP addresses
pub fn parse_cidr(input: &str) -> Result<Vec<Ipv4Addr>> {
    // Try single IP first
    if let Ok(ip) = input.parse::<Ipv4Addr>() {
        // Validate single IP against reserved ranges
        if ip.octets()[0] == 127 {
            return Err(ScannerError::InvalidTarget(
                "Scanning localhost (127.0.0.0/8) is not allowed".to_string(),
            ));
        }

        if ip.octets()[0] == 169 && ip.octets()[1] == 254 {
            return Err(ScannerError::InvalidTarget(
                "Scanning link-local (169.254.0.0/16) is not allowed".to_string(),
            ));
        }

        return Ok(vec![ip]);
    }

    // Try CIDR notation
    let network: Ipv4Network = input
        .parse()
        .map_err(|_| ScannerError::InvalidTarget(format!("Invalid IP or CIDR: {}", input)))?;

    // Validate against reserved ranges
    let first_ip = network.network();

    // Block localhost
    if first_ip.octets()[0] == 127 {
        return Err(ScannerError::InvalidTarget(
            "Scanning localhost (127.0.0.0/8) is not allowed".to_string(),
        ));
    }

    // Block link-local
    if first_ip.octets()[0] == 169 && first_ip.octets()[1] == 254 {
        return Err(ScannerError::InvalidTarget(
            "Scanning link-local (169.254.0.0/16) is not allowed".to_string(),
        ));
    }

    Ok(network.iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ip() {
        let ips = parse_cidr("192.168.1.1").unwrap();
        assert_eq!(ips.len(), 1);
        assert_eq!(ips[0].to_string(), "192.168.1.1");
    }

    #[test]
    fn test_cidr_slash_24() {
        let ips = parse_cidr("192.168.1.0/24").unwrap();
        assert_eq!(ips.len(), 256);
        assert_eq!(ips[0].to_string(), "192.168.1.0");
        assert_eq!(ips[255].to_string(), "192.168.1.255");
    }

    #[test]
    fn test_cidr_slash_30() {
        let ips = parse_cidr("10.0.0.0/30").unwrap();
        assert_eq!(ips.len(), 4);
    }

    #[test]
    fn test_localhost_blocked() {
        let result = parse_cidr("127.0.0.1");
        assert!(result.is_err());
    }

    #[test]
    fn test_link_local_blocked() {
        let result = parse_cidr("169.254.1.0/24");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_cidr() {
        let result = parse_cidr("invalid");
        assert!(result.is_err());
    }
}
