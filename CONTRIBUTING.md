# Contributing to Scanner

Thank you for your interest in contributing to Scanner! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and constructive in all interactions with the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/scanner.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run formatting: `cargo fmt`
7. Run clippy: `cargo clippy`
8. Commit your changes: `git commit -m "feat: add new feature"`
9. Push to your fork: `git push origin feature/your-feature-name`
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with verbose output
cargo test -- --nocapture
```

### Running Benchmarks

```bash
cargo bench
```

### Code Formatting

We use `rustfmt` for code formatting:

```bash
cargo fmt
```

### Linting

We use `clippy` for linting:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat:` - A new feature
- `fix:` - A bug fix
- `docs:` - Documentation only changes
- `style:` - Changes that do not affect the meaning of the code
- `refactor:` - A code change that neither fixes a bug nor adds a feature
- `perf:` - A code change that improves performance
- `test:` - Adding missing tests or correcting existing tests
- `chore:` - Changes to the build process or auxiliary tools

Examples:
```
feat: add support for IPv6 scanning
fix: correct hostname resolution timeout
docs: update README with new examples
```

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the CHANGELOG.md with a note describing your changes
3. Ensure all tests pass and there are no clippy warnings
4. Request review from maintainers
5. Once approved, a maintainer will merge your PR

## Testing Guidelines

- Write tests for new features
- Ensure existing tests pass
- Aim for high test coverage
- Include both unit tests and integration tests where appropriate

## Documentation

- Update documentation for any changed functionality
- Include doc comments for public APIs
- Keep README.md up to date

## Questions?

Feel free to open an issue for questions or discussions.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
