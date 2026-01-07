use std::fs;
use zed::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed::settings::LspSettings;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, serde_json, Result};

struct AztecExtension {
    cached_binary_path: Option<String>,
}

/// Represents a binary that can run the LSP server.
/// For Aztec projects: `aztec lsp` (runs nargo lsp in Docker)
/// For pure Noir projects: `nargo lsp` (direct)
#[derive(Clone)]
struct LspBinary {
    path: String,
    args: Vec<String>,
    environment: Option<Vec<(String, String)>>,
}

/// Look up an environment variable by name in a list of (key, value) pairs.
fn get_env_var(env: &[(String, String)], name: &str) -> Option<String> {
    env.iter().find(|(k, _)| k == name).map(|(_, v)| v.clone())
}

/// Detect if this is an Aztec project by checking Nargo.toml for Aztec dependencies.
/// Returns true if aztec-related dependencies are found, false for pure Noir projects.
fn is_aztec_project(worktree: &zed::Worktree) -> bool {
    if let Ok(nargo_toml) = worktree.read_text_file("Nargo.toml") {
        // Check for aztec dependencies in the manifest
        // Common patterns: "aztec", "@aztec", "aztec-nr"
        return nargo_toml.contains("aztec");
    }
    // If we can't read Nargo.toml, assume Aztec project (safer default for this extension)
    true
}

impl AztecExtension {
    /// Create an LspBinary for the aztec CLI (runs nargo in Docker with Aztec macro support).
    fn aztec_binary(path: String, env: Vec<(String, String)>) -> LspBinary {
        // Wrap in shell to clean up stale Docker container before starting LSP
        // The container name "aztec-nargo-lsp" is hardcoded in aztec CLI
        LspBinary {
            path: "/bin/sh".to_string(),
            args: vec![
                "-c".to_string(),
                format!(
                    "docker rm -f aztec-nargo-lsp 2>/dev/null; exec {} lsp",
                    path
                ),
            ],
            environment: Some(env),
        }
    }

    /// Create an LspBinary for native nargo (faster, no Docker needed).
    fn nargo_binary(path: String, env: Vec<(String, String)>) -> LspBinary {
        LspBinary {
            path,
            args: vec!["lsp".to_string()],
            environment: Some(env),
        }
    }

    /// Try to find aztec CLI in PATH or ~/.aztec/bin.
    fn find_aztec(&mut self, worktree: &zed::Worktree, home: Option<&String>) -> Option<String> {
        // Check PATH first
        if let Some(path) = worktree.which("aztec") {
            return Some(path);
        }
        // Check ~/.aztec/bin/aztec
        if let Some(home) = home {
            let aztec_path = format!("{}/.aztec/bin/aztec", home);
            if fs::metadata(&aztec_path).is_ok_and(|stat| stat.is_file()) {
                self.cached_binary_path = Some(aztec_path.clone());
                return Some(aztec_path);
            }
        }
        None
    }

    /// Try to find nargo in PATH or ~/.aztec/bin.
    fn find_nargo(&mut self, worktree: &zed::Worktree, home: Option<&String>) -> Option<String> {
        // Check PATH first
        if let Some(path) = worktree.which("nargo") {
            return Some(path);
        }
        // Check ~/.aztec/bin/nargo
        if let Some(home) = home {
            let nargo_path = format!("{}/.aztec/bin/nargo", home);
            if fs::metadata(&nargo_path).is_ok_and(|stat| stat.is_file()) {
                self.cached_binary_path = Some(nargo_path.clone());
                return Some(nargo_path);
            }
        }
        None
    }

    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<LspBinary> {
        let shell_env = worktree.shell_env();

        // 0. Check user settings first (highest priority)
        // Allows users to override binary path via settings.json:
        // { "lsp": { "noir": { "binary": { "path": "/custom/aztec", "arguments": ["lsp"] } } } }
        if let Ok(settings) = LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
            if let Some(binary) = settings.binary {
                if let Some(path) = binary.path {
                    let args = binary.arguments.unwrap_or_else(|| vec!["lsp".to_string()]);
                    let env: Vec<(String, String)> = binary
                        .env
                        .map(|e| e.into_iter().collect())
                        .unwrap_or_else(|| shell_env.clone());
                    return Ok(LspBinary {
                        path,
                        args,
                        environment: Some(env),
                    });
                }
            }
        }

        let home =
            get_env_var(&shell_env, "HOME").or_else(|| get_env_var(&shell_env, "USERPROFILE"));

        // 1. Detect project type and choose appropriate binary
        // - Aztec projects need `aztec lsp` (Docker-based with macro support)
        // - Pure Noir projects prefer `nargo lsp` (faster, native)
        let is_aztec = is_aztec_project(worktree);

        if is_aztec {
            // Aztec project: prefer aztec CLI, fall back to nargo
            if let Some(path) = self.find_aztec(worktree, home.as_ref()) {
                return Ok(Self::aztec_binary(path, shell_env));
            }
            if let Some(path) = self.find_nargo(worktree, home.as_ref()) {
                return Ok(Self::nargo_binary(path, shell_env));
            }
        } else {
            // Pure Noir project: prefer nargo (faster), fall back to aztec
            if let Some(path) = self.find_nargo(worktree, home.as_ref()) {
                return Ok(Self::nargo_binary(path, shell_env));
            }
            if let Some(path) = self.find_aztec(worktree, home.as_ref()) {
                return Ok(Self::aztec_binary(path, shell_env));
            }
        }

        // 2. Error with installation instructions
        Err(if is_aztec {
            "Aztec CLI not found. Install Aztec tooling:\n\
            \n\
            bash -i <(curl -s https://install.aztec.network)\n\
            aztec-up latest\n\
            \n\
            Note: Docker must be running for 'aztec lsp' to work."
        } else {
            "nargo not found. Install Noir tooling:\n\
            \n\
            curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash\n\
            noirup\n\
            \n\
            Or for Aztec projects, install Aztec tooling:\n\
            bash -i <(curl -s https://install.aztec.network)"
        }
        .into())
    }
}

impl zed::Extension for AztecExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary.path,
            args: binary.args,
            env: binary.environment.unwrap_or_default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Disable parsing cache to ensure diagnostics refresh on every file change.
        // The nargo LSP caches parsed files by content hash, which can sometimes
        // cause stale diagnostics when files are modified.
        Ok(Some(serde_json::json!({
            "enableParsingCache": false
        })))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Allow user to override workspace configuration via settings.json:
        // { "lsp": { "noir": { "settings": { "noir": { "enableCodeLens": true } } } } }
        if let Ok(settings) = LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
            if let Some(user_settings) = settings.settings {
                return Ok(Some(user_settings));
            }
        }

        // Default workspace configuration for nargo LSP
        Ok(Some(serde_json::json!({
            "noir": {
                "enableCodeLens": true,
                "enableParsing": true
            }
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        let label = &completion.label;
        let kind = completion.kind?;

        // Build a code string that will be syntax-highlighted
        let (code, highlight_range) = match kind {
            CompletionKind::Function | CompletionKind::Method => {
                // For functions: "fn name(params) -> type" or just "fn name"
                let detail = completion.detail.as_deref().unwrap_or("");
                if detail.is_empty() {
                    (format!("fn {}", label), 3..3 + label.len())
                } else {
                    // detail often contains "(params) -> ReturnType"
                    let code = format!("fn {}{}", label, detail);
                    (code, 3..3 + label.len())
                }
            }
            CompletionKind::Struct => (format!("struct {}", label), 7..7 + label.len()),
            CompletionKind::Enum => (format!("enum {}", label), 5..5 + label.len()),
            CompletionKind::Interface => {
                // Noir uses "trait" for interfaces
                (format!("trait {}", label), 6..6 + label.len())
            }
            CompletionKind::Module => (format!("mod {}", label), 4..4 + label.len()),
            CompletionKind::Constant => {
                let detail = completion.detail.as_deref().unwrap_or("");
                if detail.is_empty() {
                    (format!("let {}", label), 4..4 + label.len())
                } else {
                    (format!("let {}: {}", label, detail), 4..4 + label.len())
                }
            }
            CompletionKind::Variable => {
                let detail = completion.detail.as_deref().unwrap_or("");
                if detail.is_empty() {
                    (format!("let {}", label), 4..4 + label.len())
                } else {
                    (format!("let {}: {}", label, detail), 4..4 + label.len())
                }
            }
            CompletionKind::Field => {
                let detail = completion.detail.as_deref().unwrap_or("");
                if detail.is_empty() {
                    (label.clone(), 0..label.len())
                } else {
                    (format!("{}: {}", label, detail), 0..label.len())
                }
            }
            CompletionKind::Keyword => {
                // Keywords are already syntax-highlighted well
                (label.clone(), 0..label.len())
            }
            _ => {
                // Default: just use the label
                (label.clone(), 0..label.len())
            }
        };

        Some(CodeLabel {
            code,
            spans: vec![CodeLabelSpan::code_range(highlight_range)],
            filter_range: (0..label.len()).into(),
        })
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        let name = &symbol.name;
        let kind = symbol.kind;

        // Build a code string that will be syntax-highlighted
        let (code, highlight_range) = match kind {
            SymbolKind::Function | SymbolKind::Method => {
                (format!("fn {}", name), 3..3 + name.len())
            }
            SymbolKind::Struct => (format!("struct {}", name), 7..7 + name.len()),
            SymbolKind::Enum => (format!("enum {}", name), 5..5 + name.len()),
            SymbolKind::Interface => {
                // Noir uses "trait" for interfaces
                (format!("trait {}", name), 6..6 + name.len())
            }
            SymbolKind::Module | SymbolKind::Namespace => {
                (format!("mod {}", name), 4..4 + name.len())
            }
            SymbolKind::Constant => (format!("global {}", name), 7..7 + name.len()),
            SymbolKind::Variable => (format!("let {}", name), 4..4 + name.len()),
            SymbolKind::Field | SymbolKind::Property => (name.clone(), 0..name.len()),
            SymbolKind::Constructor => (format!("fn {}", name), 3..3 + name.len()),
            SymbolKind::TypeParameter => {
                // Generic type parameters like T, U
                (name.clone(), 0..name.len())
            }
            _ => {
                // Default: just use the name
                (name.clone(), 0..name.len())
            }
        };

        Some(CodeLabel {
            code,
            spans: vec![CodeLabelSpan::code_range(highlight_range)],
            filter_range: (0..name.len()).into(),
        })
    }
}

zed::register_extension!(AztecExtension);
