use crate::scanner::host::HostScanResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonOutput {
    pub scan_info: ScanInfo,
    pub results: Vec<JsonHostResult>,
}

#[derive(Serialize)]
pub struct ScanInfo {
    pub target: String,
    pub scan_time: String,
    pub duration_seconds: f64,
    pub total_hosts: usize,
    pub hosts_up: usize,
}

#[derive(Serialize)]
pub struct JsonHostResult {
    pub ip: String,
    pub hostname: Option<String>,
    pub is_up: bool,
    pub ports: Vec<JsonPortResult>,
}

#[derive(Serialize)]
pub struct JsonPortResult {
    pub port: u16,
    pub state: String,
    pub service: String,
}

pub fn format_json(
    results: &[HostScanResult],
    target: &str,
    duration: f64,
) -> Result<String, serde_json::Error> {
    use crate::output::helpers::get_port_info;
    use chrono::Utc;

    let up_hosts = results.iter().filter(|r| r.is_up).count();

    let json_results: Vec<JsonHostResult> = results
        .iter()
        .map(|r| {
            let mut ports: Vec<JsonPortResult> = r
                .ports
                .iter()
                .map(|(&port, &state)| {
                    let info = get_port_info(port);
                    JsonPortResult {
                        port,
                        state: state.to_string(),
                        service: info.service,
                    }
                })
                .collect();
            ports.sort_by_key(|p| p.port);

            JsonHostResult {
                ip: r.ip.to_string(),
                hostname: r.hostname.clone(),
                is_up: r.is_up,
                ports,
            }
        })
        .collect();

    let output = JsonOutput {
        scan_info: ScanInfo {
            target: target.to_string(),
            scan_time: Utc::now().to_rfc3339(),
            duration_seconds: duration,
            total_hosts: results.len(),
            hosts_up: up_hosts,
        },
        results: json_results,
    };

    serde_json::to_string_pretty(&output)
}
