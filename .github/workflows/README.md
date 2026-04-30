# GitHub Workflows

This directory contains GitHub Actions workflows for continuous integration, deployment, and automation.

## Workflows

### CI (`ci.yml`)

Runs on every push to `main`/`develop` and on pull requests.

**Jobs:**
- **Test Suite**: Runs tests on multiple OS (Ubuntu, macOS, Windows) and Rust versions (stable, beta)
- **Rustfmt**: Checks code formatting
- **Clippy**: Runs the Rust linter
- **Security Audit**: Checks for known security vulnerabilities in dependencies
- **Coverage**: Generates code coverage reports and uploads to Codecov

### Release (`release.yml`)

Triggered when a new tag is pushed (e.g., `v0.1.0`).

**Jobs:**
- **Create Release**: Creates a GitHub release
- **Build Release**: Builds binaries for multiple platforms:
  - Linux (x86_64, x86_64-musl, aarch64)
  - macOS (x86_64, aarch64/Apple Silicon)
  - Windows (x86_64)
- **Publish Crate**: Publishes to crates.io (requires `CARGO_TOKEN` secret)

**Creating a Release:**
```bash
git tag v0.1.0
git push origin v0.1.0
```

### Benchmark (`benchmark.yml`)

Runs benchmarks on push to `main` and pull requests.

**Jobs:**
- Runs Criterion benchmarks
- Stores results for comparison over time

### Dependency Update (`dependency-update.yml`)

Scheduled to run weekly (Mondays at 9 AM UTC).

**Jobs:**
- Updates dependencies
- Runs tests with updated dependencies
- Creates a pull request if updates are available

### Docker Build (`docker.yml`)

Builds and publishes Docker images.

**Jobs:**
- Builds multi-platform Docker images (amd64, arm64)
- Pushes to GitHub Container Registry (ghcr.io)
- Tags images based on branch/tag names

**Using the Docker Image:**
```bash
docker pull ghcr.io/jvzantvoort/scanner:latest
docker run --rm ghcr.io/jvzantvoort/scanner:latest --help
```

## Required Secrets

Configure these in GitHub repository settings (Settings → Secrets and variables → Actions):

- `CARGO_TOKEN`: Token for publishing to crates.io (optional, only if publishing)
- `CODECOV_TOKEN`: Token for uploading coverage to Codecov (optional)
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions

## Dependabot

Dependabot is configured in `.github/dependabot.yml` to automatically:
- Update Rust dependencies weekly
- Update GitHub Actions weekly

## Badge URLs

Add these to your README.md:

```markdown
[![CI](https://github.com/jvzantvoort/scanner/workflows/CI/badge.svg)](https://github.com/jvzantvoort/scanner/actions/workflows/ci.yml)
[![Release](https://github.com/jvzantvoort/scanner/workflows/Release/badge.svg)](https://github.com/jvzantvoort/scanner/actions/workflows/release.yml)
[![Docker Build](https://github.com/jvzantvoort/scanner/workflows/Docker%20Build/badge.svg)](https://github.com/jvzantvoort/scanner/actions/workflows/docker.yml)
```

## Local Testing

Test workflows locally using [act](https://github.com/nektos/act):

```bash
# Install act
brew install act  # macOS
# or
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run CI workflow
act -j test

# Run specific job
act -j clippy
```
