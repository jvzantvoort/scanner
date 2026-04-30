use crate::output::helpers::get_port_info;
use crate::scanner::host::HostScanResult;
use crate::scanner::port::PortState;
use console::style;

/// Format port state with colors
pub fn format_port_status(state: PortState) -> String {
    match state {
        PortState::Open => style("✓").green().bold().to_string(),
        PortState::Closed => style("✗").red().to_string(),
        PortState::Filtered => style("?").yellow().to_string(),
        PortState::Unknown => style("-").dim().to_string(),
    }
}

/// Generate dynamic table with port columns
pub fn format_table(results: &[HostScanResult], ports: &[u16]) -> String {
    if results.is_empty() {
        return "No results to display".to_string();
    }

    // Build simple table data
    let mut table_data = Vec::new();
    
    for result in results {
        let hostname = result
            .hostname
            .as_ref()
            .map(|h| {
                // Truncate long hostnames
                if h.len() > 30 {
                    format!("{}...", &h[..27])
                } else {
                    h.clone()
                }
            })
            .unwrap_or_else(|| "-".to_string());
        
        let status = if result.is_up { 
            style("up").green().bold().to_string()
        } else { 
            style("down").dim().to_string()
        };
        
        let mut port_states = Vec::new();
        for &port in ports {
            let state = result.ports.get(&port).copied().unwrap_or(PortState::Unknown);
            port_states.push(format_port_status(state));
        }
        
        table_data.push((
            result.ip.to_string(),
            hostname,
            status,
            port_states.clone(),
        ));
    }
    
    // Build header and rows manually for better control
    let mut output = String::new();
    
    // Header
    output.push_str(&format!("{:<17} {:<32} ", 
        style("IP Address").bold(), 
        style("Hostname").bold()
    ));
    
    for &port in ports {
        let info = get_port_info(port);
        let header = format!("{}:{}", info.service, port);
        output.push_str(&format!("{:<12} ", style(header).bold()));
    }
    output.push_str(&format!("{:<8}\n", style("Status").bold()));
    
    // Separator
    output.push_str(&format!("{} {} ", 
        "─".repeat(17),
        "─".repeat(32)
    ));
    for _ in ports {
        output.push_str(&format!("{} ", "─".repeat(12)));
    }
    output.push_str(&format!("{}\n", "─".repeat(8)));
    
    // Data rows
    for (ip, hostname, status, port_states) in table_data {
        output.push_str(&format!("{:<17} {:<32} ", ip, hostname));
        
        for port_state in port_states {
            // Account for ANSI codes by using fixed width after formatting
            let display_len = console::measure_text_width(&port_state);
            let padding = if display_len < 12 { 12 - display_len } else { 0 };
            output.push_str(&format!("{}{} ", port_state, " ".repeat(padding)));
        }
        
        let status_len = console::measure_text_width(&status);
        let status_padding = if status_len < 8 { 8 - status_len } else { 0 };
        output.push_str(&format!("{}{}\n", status, " ".repeat(status_padding)));
    }

    output
}

/// Generate scan summary
pub fn format_summary(
    results: &[HostScanResult],
    target: &str,
    duration_secs: f64,
) -> String {
    let total_hosts = results.len();
    let up_hosts = results.iter().filter(|r| r.is_up).count();
    let total_open_ports: usize = results.iter().map(|r| r.open_port_count()).sum();
    let up_percentage = if total_hosts > 0 {
        (up_hosts as f64 / total_hosts as f64) * 100.0
    } else {
        0.0
    };
    let scan_rate = if duration_secs > 0.0 {
        total_hosts as f64 / duration_secs
    } else {
        0.0
    };

    let mut output = String::new();
    output.push_str("\n");
    output.push_str(&style("Scan Summary").bold().to_string());
    output.push_str("\n");
    output.push_str(&"═".repeat(60));
    output.push_str("\n");
    output.push_str(&format!("Target Network:    {}\n", target));
    output.push_str(&format!("Hosts Scanned:     {}\n", total_hosts));
    output.push_str(&format!(
        "Hosts Up:          {} ({:.1}%)\n",
        up_hosts, up_percentage
    ));
    output.push_str(&format!("Total Open Ports:  {}\n", total_open_ports));
    output.push_str(&format!("Scan Duration:     {:.1}s\n", duration_secs));
    output.push_str(&format!("Scan Rate:         {:.1} hosts/sec\n", scan_rate));

    // Security notes
    let mut security_notes = Vec::new();
    for result in results {
        for (&port, &state) in &result.ports {
            if state == PortState::Open {
                let info = get_port_info(port);
                if let Some(note) = info.security_note {
                    if note.contains("⚠️") {
                        security_notes.push(format!(
                            "Host {} has {} open: {}",
                            result.ip, info.service, note
                        ));
                    }
                }
            }
        }
    }

    if !security_notes.is_empty() {
        output.push_str("\n");
        output.push_str(&style("Security Notes:").yellow().bold().to_string());
        output.push_str("\n");
        for note in security_notes.iter().take(5) {
            output.push_str(&format!("  • {}\n", note));
        }
    }

    output
}
