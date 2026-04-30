# Network Scanner

![disclaimer](disclaimer.png "Disclaimer image")


[![CI](https://github.com/jvzantvoort/scanner/workflows/CI/badge.svg)](https://github.com/jvzantvoort/scanner/actions/workflows/ci.yml)
[![Release](https://github.com/jvzantvoort/scanner/workflows/Release/badge.svg)](https://github.com/jvzantvoort/scanner/actions/workflows/release.yml)
[![Docker Build](https://github.com/jvzantvoort/scanner/workflows/Docker%20Build/badge.svg)](https://github.com/jvzantvoort/scanner/actions/workflows/docker.yml)

A pure Rust network port scanner that detects open ports and displays results in a beautifully formatted table with color-coded status indicators.

## Features

- 🚀 **Pure Rust** - No syscalls, minimal dependencies
- ⚡ **Fast** - Concurrent scanning with configurable parallelism
- 🎨 **Beautiful Output** - Formatted tables with green highlighting for open ports
- 📊 **Multiple Formats** - Export as table, JSON, or CSV
- 🔒 **Security Info** - Built-in port information database with security notes
- 🛡️ **Safe** - Input validation and reserved range protection
- 🔍 **ICMP Detection** - Filters hosts that don't respond to ping
- 🌐 **Hostname Resolution** - Automatically resolves DNS names for discovered hosts

## Installation

From crates.io:
```bash
cargo install portscan-rs
```

Or build from source:

```bash
cargo build --release
./target/release/scanner --help
```

## Quick Start

**Note:** ICMP ping requires elevated privileges (sudo/root) on most systems.

Scan a single host:
```bash
sudo scanner --target 192.168.1.1 --ports 22,80,443
```

Scan a network range:
```bash
sudo scanner --target 192.168.1.0/24 --ports 22,80,443
```

Scan with custom settings:
```bash
sudo scanner --target 192.168.1.0/24 --ports 1-1024 --timeout 500 --concurrency 200
```

Export to JSON:
```bash
sudo scanner --target 192.168.1.0/24 --ports 22,80,443 --format json --output scan.json
```

## Usage

```
Usage: scanner [OPTIONS] --target <TARGET>

Options:
  -t, --target <TARGET>          Target IP address or CIDR range (e.g., 192.168.1.0/24)
  -p, --ports <PORTS>            Ports to scan (e.g., 22,80,443 or 1-1024) [default: 22,80,443]
      --timeout <TIMEOUT>        Connection timeout in milliseconds [default: 1000]
  -c, --concurrency <CONCURRENCY> Maximum concurrent connections [default: 100]
  -f, --format <FORMAT>          Output format: table, json, csv [default: table]
  -o, --output <OUTPUT>          Output file (default: stdout)
  -h, --help                     Print help
  -V, --version                  Print version
```

## Example Output

```
┌─────────────────┬────────────────────┬────────────┬────────────┬────────────┬──────────┐
│ IP Address      │ Hostname           │ SSH (22)   │ HTTP (80)  │ HTTPS (443)│ Status   │
├─────────────────┼────────────────────┼────────────┼────────────┼────────────┼──────────┤
│ 192.168.1.1     │ router.local       │ open       │ open       │ open       │ up       │
│ 192.168.1.10    │ server01           │ open       │ closed     │ open       │ up       │
│ 192.168.1.15    │ workstation        │ closed     │ open       │ open       │ up       │
└─────────────────┴────────────────────┴────────────┴────────────┴────────────┴──────────┘

Scan Summary
════════════════════════════════════════════════════════════
Target Network:    192.168.1.0/24
Hosts Scanned:     254
Hosts Up:          12 (4.7%)
Total Open Ports:  18
Scan Duration:     24.3s
Scan Rate:         10.4 hosts/sec
```

## Port Information Database

The scanner includes built-in information for common ports:
- SSH (22), Telnet (23), HTTP (80), HTTPS (443)
- MySQL (3306), PostgreSQL (5432), MongoDB (27017), Redis (6379)
- And many more...

Security warnings are displayed for potentially dangerous configurations.

## Performance

- Scans /24 network (254 hosts, 3 ports) in ~30 seconds
- Memory usage: <50MB for 1000+ host scans
- Configurable concurrency (default: 100 parallel connections)
- Binary size: <5MB (release build, stripped)

## Legal Notice

⚠️ **Only scan networks you own or have explicit permission to scan.** Unauthorized network scanning may be illegal in your jurisdiction.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
