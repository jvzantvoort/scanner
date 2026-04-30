use crate::error::{Result, ScannerError};
use std::collections::BTreeSet;

/// Parse port specification into a list of unique, sorted ports
/// Supports:
/// - Single port: "80"
/// - Comma-separated: "22,80,443"
/// - Ranges: "1-1024"
/// - Mixed: "22,80-443,8080"
pub fn parse_ports(input: &str) -> Result<Vec<u16>> {
    let mut ports = BTreeSet::new();

    for part in input.split(',') {
        let part = part.trim();
        
        if part.contains('-') {
            // Handle range
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return Err(ScannerError::InvalidPort(format!(
                    "Invalid port range format: {}",
                    part
                )));
            }

            let start: u16 = range_parts[0].trim().parse().map_err(|_| {
                ScannerError::InvalidPort(format!("Invalid port number: {}", range_parts[0]))
            })?;

            let end: u16 = range_parts[1].trim().parse().map_err(|_| {
                ScannerError::InvalidPort(format!("Invalid port number: {}", range_parts[1]))
            })?;

            if start == 0 || end == 0 {
                return Err(ScannerError::InvalidPort(
                    "Port numbers must be between 1 and 65535".to_string(),
                ));
            }

            if start > end {
                return Err(ScannerError::InvalidPort(format!(
                    "Invalid range: start ({}) is greater than end ({})",
                    start, end
                )));
            }

            for port in start..=end {
                ports.insert(port);
            }
        } else {
            // Handle single port
            let port: u16 = part.parse().map_err(|_| {
                ScannerError::InvalidPort(format!("Invalid port number: {}", part))
            })?;

            if port == 0 {
                return Err(ScannerError::InvalidPort(
                    "Port numbers must be between 1 and 65535".to_string(),
                ));
            }

            ports.insert(port);
        }
    }

    Ok(ports.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_port() {
        let ports = parse_ports("80").unwrap();
        assert_eq!(ports, vec![80]);
    }

    #[test]
    fn test_comma_separated() {
        let ports = parse_ports("22,80,443").unwrap();
        assert_eq!(ports, vec![22, 80, 443]);
    }

    #[test]
    fn test_range() {
        let ports = parse_ports("1-5").unwrap();
        assert_eq!(ports, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mixed_format() {
        let ports = parse_ports("22,80-82,443").unwrap();
        assert_eq!(ports, vec![22, 80, 81, 82, 443]);
    }

    #[test]
    fn test_duplicates_removed() {
        let ports = parse_ports("80,80,80").unwrap();
        assert_eq!(ports, vec![80]);
    }

    #[test]
    fn test_sorted_output() {
        let ports = parse_ports("443,22,80").unwrap();
        assert_eq!(ports, vec![22, 80, 443]);
    }

    #[test]
    fn test_invalid_port_zero() {
        let result = parse_ports("0");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_range() {
        let result = parse_ports("100-50");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format() {
        let result = parse_ports("abc");
        assert!(result.is_err());
    }
}
