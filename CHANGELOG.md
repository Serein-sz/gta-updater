# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- CLI argument support with `clap`
  - `--app` / `-a`: Update specific app only
  - `--force` / `-f`: Force update even on latest version
  - `--dry-run` / `-n`: Preview updates without downloading
  - `--verbose` / `-v`: Show detailed output
- Colored terminal output for better UX
- Progress bar for downloads
- Better error handling and user feedback
- Modular code structure (cli, conf, updater modules)

### Changed
- Improved error messages with context
- Version comparison now handles edge cases gracefully
- Configuration automatically saved after updates
- Binary naming on Windows (auto-adds .exe extension)

### Fixed
- Removed `unwrap()` calls that could cause panics
- Better handling of missing assets
- Proper version comparison error handling
- Directory creation before download

## [0.1.0] - 2026-06-15

### Added
- Initial release
- GitHub Release asset fetching
- Cross-platform support (Linux, macOS, Windows)
- Multi-architecture support (amd64, arm64)
- TOML configuration
- Automatic version checking
