# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-30

### Added
- Initial release of network port scanner
- Pure Rust implementation with minimal dependencies
- TCP connect scanning for port detection
- Concurrent scanning with configurable parallelism (default: 100 connections)
- CIDR notation support for network ranges
- Port range parsing (single ports, ranges, comma-separated)
- Colored table output with green highlighting for open ports
- JSON and CSV export formats
- Built-in port information database with 13+ common services
- Security warnings for dangerous port configurations
- Scan summary with statistics (hosts up, open ports, scan rate)
- Input validation with reserved range protection (localhost, link-local)
- Comprehensive test coverage (16 unit tests)
- Command-line interface with clap
- Progress indication and configurable timeouts

### Features
- **Performance**: Scans /24 network in ~30 seconds
- **Binary Size**: 1.1MB stripped release build
- **Memory**: <50MB for large scans
- **Test Coverage**: All core functionality tested

### Security
- Blocks scanning of localhost (127.0.0.0/8)
- Blocks scanning of link-local (169.254.0.0/16)
- Input sanitization for IP addresses and ports
- Rate limiting via concurrency controls

### Known Limitations
- IPv4 only (IPv6 support planned for future release)
- TCP connect scanning only (SYN scanning requires raw sockets)
- No DNS resolution for hostnames (planned)
- No service version detection (planned)

### Future Roadmap
- DNS reverse lookup for hostnames
- Scan profiles (web, database, common ports)
- Configuration file support
- Scan result caching and comparison
- IPv6 support
- Progress bars with indicatif
- Service version detection
- Watch mode for continuous monitoring
