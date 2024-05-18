# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this
project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.2.0] - 2024-05-18

### Added

In source code (src/main.rs):
- Implement logging with simple_logger
- Introduce the Encryption enum for encryption types and fmt::Display
- Add logging initialization with simple_logger

In Cargo.toml:
- Add license (GPL-3.0), categories, keywords, readme, and repo URL.
- Add new dependencies: log and simple_logger.

### Changed

In source code (src/main.rs):
- Improve configuration parsing with context-specific error messages
- Modularize code by introducing helper functions

In Cargo.toml:
- Modify release profile to have smaller binary files.
- Rename the package to "ciphercanvas"
- Bump version to 0.2.0 from 0.1.1

## [0.1.1] - 2024-03-07

### Added

  - Add option to export to PNG images (currently limited to more than 256x256).

## [0.1.0] - 2024-03-03

### Added

  - Add TOML configuration and document it.

### Changed

  - Change handling from StructOpt to Clap.
  - Rename project from `wifi-qrcode-rs` to `ciphercanvas-rs`.

## [0.0.2] - 2023-10-15

### Added

  - Instead of using manual user input, the program will instead take them from
    command-line arguments, handled by StructOpt.
  - Added project files:
      - CHANGELOG.md
      - CODE_OF_CONDUCT.md

### Fixed

  - Renamed LICENSE to LICENSE.md to reflect the right formatting.

## [0.0.1] - 2023-07-31

### Added

  - Initial release of `wifi-qrcode-rs`.
