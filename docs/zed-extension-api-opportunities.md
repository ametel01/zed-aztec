# Zed Extension API Opportunities

Analysis of `zed_extension_api` v0.7.0 APIs that could improve the zed-aztec extension.

## Currently Used APIs

| API | Usage |
|-----|-------|
| `worktree.which()` | Find `aztec`/`nargo` in PATH |
| `worktree.shell_env()` | Get user's shell environment variables |
| `worktree.read_text_file()` | Detect Aztec vs Noir projects via Nargo.toml |
| `LspSettings::for_worktree()` | User-configurable binary path via settings |
| `Extension::language_server_command()` | Return LSP binary command |
| `Extension::language_server_initialization_options()` | Disable parsing cache |
| `Extension::language_server_workspace_configuration()` | Runtime LSP settings (code lens, parsing) |
| `Extension::label_for_completion()` | Syntax-highlighted completions |
| `Extension::label_for_symbol()` | Syntax-highlighted symbols |
| `set_language_server_installation_status()` | Show binary search progress in UI |

## Available APIs for Improvement

### High Priority

#### 1. ✅ LspSettings::for_worktree() - User-configurable binary path (IMPLEMENTED)

Users can override the binary path via Zed settings instead of relying solely on auto-detection.

User configuration in `~/.config/zed/settings.json`:
```json
{
  "lsp": {
    "noir": {
      "binary": {
        "path": "/custom/path/to/aztec",
        "arguments": ["lsp"]
      }
    }
  }
}
```

**Benefit**: Users with non-standard installations can configure the extension without modifying code.

---

#### 2. ✅ worktree.read_text_file() - Detect Aztec vs Noir projects (IMPLEMENTED)

Auto-detects project type by reading `Nargo.toml` to determine whether to prioritize `aztec` CLI or `nargo`.

- Aztec projects (Nargo.toml contains "aztec"): prefer `aztec lsp` (Docker-based with macro support)
- Pure Noir projects: prefer `nargo lsp` (faster, native)

**Benefit**: Smarter binary selection - no Docker overhead for pure Noir projects.

---

### Medium Priority

#### 3. ✅ language_server_workspace_configuration() (IMPLEMENTED)

Pass workspace-level settings to the LSP server at runtime.

User configuration in `~/.config/zed/settings.json`:
```json
{
  "lsp": {
    "noir": {
      "settings": {
        "noir": {
          "enableCodeLens": false,
          "enableParsing": false
        }
      }
    }
  }
}
```

Default settings if not configured:
- `enableCodeLens`: true (show run/test buttons in code)
- `enableParsing`: true (enable syntax parsing)

**Benefit**: Configure LSP behavior without requiring server restart.

---

#### 4. ✅ label_for_completion() - Enhanced completion UI (IMPLEMENTED)

Customize how completions appear in the autocomplete menu with syntax highlighting.

Formatting by completion kind:
- **Functions/Methods**: `fn name(params) -> Type`
- **Structs**: `struct Name`
- **Enums**: `enum Name`
- **Traits**: `trait Name`
- **Modules**: `mod name`
- **Variables/Constants**: `let name: Type`
- **Fields**: `name: Type`

**Benefit**: Richer IDE experience with syntax-highlighted completions that show type information.

---

#### 5. ✅ label_for_symbol() - Enhanced symbol UI (IMPLEMENTED)

Customize how symbols appear in the symbol picker and outline with syntax highlighting.

Formatting by symbol kind:
- **Functions/Methods/Constructor**: `fn name`
- **Structs**: `struct Name`
- **Enums**: `enum Name`
- **Traits (Interface)**: `trait Name`
- **Modules/Namespaces**: `mod name`
- **Constants**: `global name`
- **Variables**: `let name`

**Benefit**: Consistent, syntax-highlighted symbols in symbol picker (Cmd+T) and document outline.

---

### Not Implementable (API Limitations)

#### 6. ❌ KeyValueStore - Persistent storage (NOT AVAILABLE)

**Reason**: The KeyValueStore API is too limited for general extension use:
1. **Write-only**: Only `insert(key, value)` exists — no `get()` method to retrieve values
2. **Context-specific**: Only available as a parameter to `index_docs()` for documentation indexing
3. **Not extensible**: Cannot obtain a KeyValueStore instance for general caching

```rust
// WIT interface definition (from zed extension_api):
resource key-value-store {
    insert: func(key: string, value: string) -> result<_, string>;
    // No get() method!
}

// Only available here:
fn index_docs(&self, provider: String, package: String, database: &KeyValueStore) -> Result<()>
```

**Alternative**: The `cached_binary_path` field in `AztecExtension` provides in-memory caching per session, but cannot persist across Zed restarts.

---

#### 7. ✅ set_language_server_installation_status() (IMPLEMENTED)

Show installation progress in Zed's UI when the extension is setting up.

```rust
// Show "checking" status while searching for binaries
zed::set_language_server_installation_status(
    language_server_id,
    &LanguageServerInstallationStatus::CheckingForUpdate,
);

// Clear status when binary found
zed::set_language_server_installation_status(
    language_server_id,
    &LanguageServerInstallationStatus::None,
);

// Show failure with installation instructions
zed::set_language_server_installation_status(
    language_server_id,
    &LanguageServerInstallationStatus::Failed("nargo not found. Install with: noirup".to_string()),
);
```

**Benefit**: Users see visual feedback in Zed's UI while the extension searches for LSP binaries, and clear error messages if installation is missing.

---

## Worktree API Reference

All methods available on `zed::Worktree`:

| Method | Signature | Description |
|--------|-----------|-------------|
| `id()` | `&self -> u64` | Returns the worktree ID |
| `root_path()` | `&self -> String` | Returns the project root path |
| `read_text_file()` | `&self, path: &str -> Result<String>` | Read file contents |
| `which()` | `&self, binary: &str -> Option<String>` | Find binary in PATH |
| `shell_env()` | `&self -> Vec<(String, String)>` | Get shell environment |

---

## Extension Trait Methods

All optional methods in `zed::Extension`:

### Language Server
- `language_server_command()` - LSP binary command *(currently used)*
- `language_server_initialization_options()` - Init params *(currently used)*
- `language_server_workspace_configuration()` - Workspace settings
- `language_server_additional_initialization_options()` - Cross-LSP init sharing
- `language_server_additional_workspace_configuration()` - Cross-LSP config sharing

### Code Presentation
- `label_for_completion()` - Completion UI customization
- `label_for_symbol()` - Symbol UI customization

### Slash Commands
- `complete_slash_command_argument()` - Argument suggestions
- `run_slash_command()` - Execute slash commands

### Context Servers
- `context_server_command()` - Context server launch command
- `context_server_configuration()` - Context server settings

### Documentation
- `suggest_docs_packages()` - Package suggestions for docs
- `index_docs()` - Index package documentation

### Debug Adapter Protocol
- `get_dap_binary()` - Debug adapter executable
- `dap_request_kind()` - Launch vs attach mode
- `dap_config_to_scenario()` - Debug configuration
- `dap_locator_create_scenario()` - Task to debug scenario
- `run_dap_locator()` - Locator resolution

---

## Implementation Recommendations

1. ~~**Phase 1**: Add `LspSettings::for_worktree()` support for user-configurable paths~~ ✅ Done
2. ~~**Phase 2**: Add `read_text_file("Nargo.toml")` for project type detection~~ ✅ Done
3. ~~**Phase 3**: Add `language_server_workspace_configuration()` for runtime LSP settings~~ ✅ Done
4. ~~**Phase 4**: Add `label_for_completion()` for UI polish~~ ✅ Done
5. ~~**Phase 5**: Add `label_for_symbol()` for symbol UI polish~~ ✅ Done

All high and medium priority APIs have been implemented!

## References

- [zed_extension_api docs](https://docs.rs/zed_extension_api/latest/zed_extension_api/)
- [Worktree struct](https://docs.rs/zed_extension_api/latest/zed_extension_api/struct.Worktree.html)
- [Extension trait](https://docs.rs/zed_extension_api/latest/zed_extension_api/trait.Extension.html)
- [LspSettings struct](https://docs.rs/zed_extension_api/latest/zed_extension_api/settings/struct.LspSettings.html)
