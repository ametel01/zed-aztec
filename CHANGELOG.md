# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Fix LSP connection reset by removing debug code that broke stdio communication
- Fix LSP restart failure by auto-removing stale Docker container before startup
- Fix Aztec CLI detection when Zed is launched from desktop launcher instead of terminal
- Fix docker not found when running `aztec lsp` by ensuring PATH includes /usr/bin
- Fix LSP crash when HOME environment variable is not passed by Zed
- Fix LSP crash when working directory is not under HOME (aztec CLI requirement)

## [0.2.0] - 2026-01-10

### Added
- Rich syntax highlighting with Rust-inspired patterns
  - Method calls distinguished from functions (`@function.method`)
  - Macro attributes highlighted (`#[...]` syntax)
  - Struct fields and properties (`@property`)
  - Generic type parameters (`@type.parameter`)
  - Complete operator coverage (arithmetic, logical, bitwise, comparison, assignment)
  - Module paths and namespaces (`crate::`, `super::`)
  - ALL_CAPS identifier convention for constants
- Troubleshooting guide for grammar compilation failures due to corrupted wasi-sdk

### Changed
- Enhance highlights.scm from 79 to 235 lines with semantic highlighting
- Reorganize syntax highlighting rules with clear category sections

### Fixed
- Fix syntax highlighting query errors for named nodes (`comptime`, `viewer`, `mutable`)

## [0.1.0] - 2026-01-08

### Added
- User-configurable LSP binary path via Zed settings
- Auto-detect Aztec vs pure Noir projects via Nargo.toml
- Runtime LSP configuration via workspace settings (code lens, parsing)
- Syntax-highlighted completions with kind-specific formatting (fn, struct, enum, etc.)
- Syntax-highlighted symbols in symbol picker and outline
- Visual status indicator during LSP binary search with failure messages

### Changed
- Prefer native nargo for pure Noir projects (faster startup, no Docker)
- Use worktree shell environment API for environment variable lookups

### Fixed
- Auto-remove stale `aztec-nargo-lsp` Docker container on LSP startup
- Disable LSP parsing cache to fix stale diagnostics after file changes
- Pass shell environment to all LSP binary paths for proper Docker/tool configuration

## [0.0.1] - 2026-01-06

Initial release of Aztec extension for Zed editor.

### Added
- Syntax highlighting for `.nr` files with Aztec-specific constructs
- LSP support via `aztec lsp` for code completion, go to definition, hover docs, and diagnostics
- Automatic LSP binary detection with fallback chain: `aztec` → `~/.aztec/bin/aztec` → `nargo` → `~/.aztec/bin/nargo`
- tree-sitter-noir grammar integration

### Known Limitations
- Format on save not supported (`nargo fmt` only supports in-place file formatting)
- Requires Docker for `aztec lsp` (runs nargo in container with Aztec macro support)

[Unreleased]: https://github.com/ametel01/zed-aztec/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/ametel01/zed-aztec/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ametel01/zed-aztec/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/ametel01/zed-aztec/releases/tag/v0.0.1
