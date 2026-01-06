# zed-aztec

Zed editor extension for [Aztec-nr](https://docs.aztec.network/) smart contract development.

## Features

- **Syntax highlighting** for `.nr` files with Aztec-specific constructs
- **LSP support** via `aztec lsp` for:
  - Code completion
  - Go to definition
  - Hover documentation
  - Diagnostics
- **Auto-formatting** support using `aztec fmt` (see Configuration section)

## Prerequisites

### Required

- **Docker** - Must be installed and running (aztec lsp runs in Docker)
- **Aztec CLI** - Install via:

```bash
bash -i <(curl -s https://install.aztec.network)
aztec-up latest
```

### Verify Installation

```bash
docker --version    # Docker must be running
aztec --version     # Should show Aztec version
aztec lsp          # Test LSP (Ctrl+C to exit)
```

## Installation

### From Zed Extensions (when published)

1. Open Zed
2. Go to Extensions (`Cmd+Shift+X` on macOS, `Ctrl+Shift+X` on Linux)
3. Search for "Aztec"
4. Click Install

### Development Installation

1. Clone this repository
2. In Zed: Extensions > Install Dev Extension
3. Select the cloned directory

## Configuration

### Enable Auto-Format on Save (Recommended)

Add to your Zed settings (`Cmd+,` on macOS, `Ctrl+,` on Linux → Open Settings JSON):

```json
{
  "languages": {
    "Aztec": {
      "formatter": {
        "external": {
          "command": "aztec",
          "arguments": ["fmt", "--stdin-filepath", "{buffer_path}"]
        }
      },
      "format_on_save": "on"
    }
  }
}
```

This uses `aztec fmt` which runs `nargo fmt` in Docker for consistent formatting.

### Disable Auto-Format on Save

```json
{
  "languages": {
    "Aztec": {
      "format_on_save": "off"
    }
  }
}
```

### Use nargo Instead of aztec (for pure Noir)

If you prefer `nargo lsp` directly (without Docker), ensure `nargo` is in your PATH before `aztec`. The extension checks in this order:

1. `aztec` in PATH → uses `aztec lsp`
2. `~/.aztec/bin/aztec`
3. `nargo` in PATH → uses `nargo lsp`
4. `~/.aztec/bin/nargo`

For formatting with nargo directly:

```json
{
  "languages": {
    "Aztec": {
      "formatter": {
        "external": {
          "command": "nargo",
          "arguments": ["fmt", "--stdin-filepath", "{buffer_path}"]
        }
      },
      "format_on_save": "on"
    }
  }
}
```

## Troubleshooting

### LSP Not Starting

1. **Check Docker is running**: `docker ps`
2. **Test aztec lsp manually**: `aztec lsp` (should start without errors)
3. **Check Zed logs**: View → Toggle Developer Tools → Language Server logs

### "aztec not found"

Run the Aztec installer:

```bash
bash -i <(curl -s https://install.aztec.network)
aztec-up latest
```

### Formatting Not Working

Ensure Docker is running and test manually:

```bash
aztec fmt
```

## Note on File Association

Both `zed-noir` and `zed-aztec` extensions handle `.nr` files. If you have both installed:

- Disable `zed-noir` when working on Aztec projects, or
- Configure file types per-workspace in your Zed settings

## License

MIT
