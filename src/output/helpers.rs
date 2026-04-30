use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub service: String,
    pub description: String,
    pub security_note: Option<String>,
}

impl PortInfo {
    pub fn unknown(port: u16) -> Self {
        Self {
            service: format!("port-{}", port),
            description: "Unknown service".to_string(),
            security_note: None,
        }
    }
}

/// Get information about a port
pub fn get_port_info(port: u16) -> PortInfo {
    match port {
        21 => PortInfo {
            service: "FTP".to_string(),
            description: "File Transfer Protocol".to_string(),
            security_note: Some("Often disabled, check if needed".to_string()),
        },
        22 => PortInfo {
            service: "SSH".to_string(),
            description: "Secure Shell - Remote access".to_string(),
            security_note: Some("Ensure key-based authentication".to_string()),
        },
        23 => PortInfo {
            service: "Telnet".to_string(),
            description: "Unencrypted remote access".to_string(),
            security_note: Some("⚠️ INSECURE - Use SSH instead".to_string()),
        },
        25 => PortInfo {
            service: "SMTP".to_string(),
            description: "Email server".to_string(),
            security_note: Some("Check relay configuration".to_string()),
        },
        53 => PortInfo {
            service: "DNS".to_string(),
            description: "Domain Name System".to_string(),
            security_note: Some("Verify resolver security".to_string()),
        },
        80 => PortInfo {
            service: "HTTP".to_string(),
            description: "Web server (unencrypted)".to_string(),
            security_note: Some("Consider migrating to HTTPS".to_string()),
        },
        443 => PortInfo {
            service: "HTTPS".to_string(),
            description: "Secure web server".to_string(),
            security_note: Some("Verify TLS configuration".to_string()),
        },
        3306 => PortInfo {
            service: "MySQL".to_string(),
            description: "Database server".to_string(),
            security_note: Some("⚠️ Ensure not publicly exposed".to_string()),
        },
        5432 => PortInfo {
            service: "PostgreSQL".to_string(),
            description: "Database server".to_string(),
            security_note: Some("⚠️ Ensure not publicly exposed".to_string()),
        },
        6379 => PortInfo {
            service: "Redis".to_string(),
            description: "In-memory database".to_string(),
            security_note: Some("⚠️ Often unsecured by default".to_string()),
        },
        8080 => PortInfo {
            service: "HTTP-Alt".to_string(),
            description: "Alternative HTTP port".to_string(),
            security_note: Some("Common for dev servers".to_string()),
        },
        8443 => PortInfo {
            service: "HTTPS-Alt".to_string(),
            description: "Alternative HTTPS port".to_string(),
            security_note: None,
        },
        27017 => PortInfo {
            service: "MongoDB".to_string(),
            description: "NoSQL database".to_string(),
            security_note: Some("⚠️ Check authentication enabled".to_string()),
        },
        _ => PortInfo::unknown(port),
    }
}
