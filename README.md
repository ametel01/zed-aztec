# zed-aztec

Zed editor extension for [Aztec-nr](https://docs.aztec.network/) smart contract development.

## Features

- **Syntax highlighting** for `.nr` files with Aztec-specific constructs
- **LSP support** via `aztec lsp` for:
  - Code completion
  - Go to definition
  - Hover documentation
  - Diagnostics
- **Manual formatting** via `aztec fmt` command (see Configuration section)

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

### LSP Binary Selection

The extension checks for LSP binaries in this order:

1. `aztec` in PATH → uses `aztec lsp` (runs nargo in Docker)
2. `~/.aztec/bin/aztec`
3. `nargo` in PATH → uses `nargo lsp` (direct, for pure Noir)
4. `~/.aztec/bin/nargo`

For pure Noir projects without Docker, ensure `nargo` is in your PATH before `aztec`.

### Formatting

**Note:** `nargo fmt` only supports in-place formatting of workspace files and does not accept stdin input. Therefore, **Zed's auto-format on save is not supported** for Noir/Aztec files.

To format your code, run manually in your project directory:

```bash
# For Aztec projects (runs nargo fmt in Docker)
aztec fmt

# For pure Noir projects
nargo fmt
```

**Tip:** You can bind a keyboard shortcut in Zed to run `aztec fmt` via a terminal task.

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

### "Permission denied" on fmt or compile

If you see `Failed to lock git dependencies cache: Permission denied`, the `~/nargo` cache directory was created by Docker as root. Fix with:

```bash
sudo chown -R $USER:$USER ~/nargo
```

## Note on File Association

Both `zed-noir` and `zed-aztec` extensions handle `.nr` files. If you have both installed:

- Disable `zed-noir` when working on Aztec projects, or
- Configure file types per-workspace in your Zed settings

## License

MIT
