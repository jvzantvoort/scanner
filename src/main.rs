use clap::Parser;
use scanner::{output, parser, scanner::ScanConfig, scanner::scan_network, Result, ScannerError};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(
    name = "scanner",
    version,
    about = "A pure Rust network port scanner",
    long_about = "Scan network hosts for open ports with beautiful formatted output.\n\
                  Highlights open ports in green and provides helpful security information."
)]
struct Args {
    /// Target IP address or CIDR range (e.g., 192.168.1.0/24)
    #[arg(short, long)]
    target: String,

    /// Ports to scan (e.g., 22,80,443 or 1-1024)
    #[arg(short, long, default_value = "22,80,443")]
    ports: String,

    /// Connection timeout in milliseconds
    #[arg(long, default_value = "1000")]
    timeout: u64,

    /// Maximum concurrent connections
    #[arg(short, long, default_value = "100")]
    concurrency: usize,

    /// Output format: table, json, csv
    #[arg(short, long, default_value = "table")]
    format: String,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Parse inputs
    let targets = parser::parse_cidr(&args.target)?;
    let ports = parser::parse_ports(&args.ports)?;

    println!("Starting scan of {} hosts on {} ports...", targets.len(), ports.len());
    
    let start = Instant::now();

    // Configure and run scan
    let config = ScanConfig {
        timeout_ms: args.timeout,
        concurrency: args.concurrency,
    };

    let results = scan_network(targets, ports.clone(), config).await?;
    
    let duration = start.elapsed().as_secs_f64();

    // Format output
    let output_str = match args.format.as_str() {
        "json" => output::format_json(&results, &args.target, duration)
            .map_err(|e| ScannerError::OutputError(e.to_string()))?,
        "csv" => output::format_csv(&results)
            .map_err(|e| ScannerError::OutputError(e.to_string()))?,
        _ => {
            // Table format
            let mut table_output = String::new();
            table_output.push_str(&output::format_table(&results, &ports));
            table_output.push_str(&output::format_summary(&results, &args.target, duration));
            table_output
        }
    };

    // Write output
    if let Some(output_path) = args.output {
        std::fs::write(&output_path, output_str)
            .map_err(|e| ScannerError::OutputError(e.to_string()))?;
        println!("Results written to {}", output_path.display());
    } else {
        println!("\n{}", output_str);
    }

    Ok(())
}
