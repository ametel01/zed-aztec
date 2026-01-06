use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

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

impl AztecExtension {
    fn language_server_binary(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<LspBinary> {
        // 1. Check worktree PATH for aztec CLI (highest priority for Aztec projects)
        // aztec lsp runs nargo lsp inside a Docker container
        if let Some(path) = worktree.which("aztec") {
            return Ok(LspBinary {
                path,
                args: vec!["lsp".to_string()],
                environment: Some(worktree.shell_env()),
            });
        }

        // 2. Check cached path
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(LspBinary {
                    path: path.clone(),
                    args: vec!["lsp".to_string()],
                    environment: None,
                });
            }
        }

        // 3. Check Aztec installation path (~/.aztec/bin/aztec)
        let home_vars = ["HOME", "USERPROFILE"];
        for var in home_vars {
            if let Ok(home) = std::env::var(var) {
                // Prefer aztec CLI over nargo for Aztec projects
                let aztec_cli_path = format!("{}/.aztec/bin/aztec", home);
                if fs::metadata(&aztec_cli_path).map_or(false, |stat| stat.is_file()) {
                    self.cached_binary_path = Some(aztec_cli_path.clone());
                    return Ok(LspBinary {
                        path: aztec_cli_path,
                        args: vec!["lsp".to_string()],
                        environment: None,
                    });
                }
            }
        }

        // 4. Fallback: Check for standalone nargo (for pure Noir projects)
        if let Some(path) = worktree.which("nargo") {
            return Ok(LspBinary {
                path,
                args: vec!["lsp".to_string()],
                environment: Some(worktree.shell_env()),
            });
        }

        // 5. Check ~/.aztec/bin/nargo as last resort
        for var in home_vars {
            if let Ok(home) = std::env::var(var) {
                let nargo_path = format!("{}/.aztec/bin/nargo", home);
                if fs::metadata(&nargo_path).map_or(false, |stat| stat.is_file()) {
                    self.cached_binary_path = Some(nargo_path.clone());
                    return Ok(LspBinary {
                        path: nargo_path,
                        args: vec!["lsp".to_string()],
                        environment: None,
                    });
                }
            }
        }

        // 6. Error with installation instructions
        Err(
            "Aztec CLI not found. Install Aztec tooling:\n\
            \n\
            bash -i <(curl -s https://install.aztec.network)\n\
            aztec-up latest\n\
            \n\
            Note: Docker must be running for 'aztec lsp' to work.\n\
            \n\
            For pure Noir (non-Aztec) projects, install nargo:\n\
            curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash\n\
            noirup"
                .into(),
        )
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
}

zed::register_extension!(AztecExtension);
