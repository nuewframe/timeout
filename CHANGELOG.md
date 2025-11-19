# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Core Foundation**: Initial release of the `timeout` CLI tool with cross-platform support (Linux, macOS, Windows).
- **Windows Support**: Native Windows implementation using `Job Objects` for process tree termination and graceful degradation (warns on signals, waits for grace period).
- **Unix Signals**: Full support for named and numeric Unix signals (e.g., `SIGINT`, `TERM`, `9`) via the `--signal` flag.
- **Observability**: Structured logging via `tracing` and `tracing-subscriber` for detailed diagnostics (e.g., `timeout=500ms`).
- **Safety**: Async-signal-safe process spawning architecture using `rustix` and static error handling.
- **Documentation**: Comprehensive `README.md` with Unix/Windows references and `CONTRIBUTING.md` for developer workflows.
- **Testing**: Robust end-to-end test suite verifying timeout behavior, signal handling, and output formatting.
