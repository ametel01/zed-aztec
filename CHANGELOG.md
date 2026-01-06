# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/ametel01/zed-aztec/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/ametel01/zed-aztec/releases/tag/v0.0.1
