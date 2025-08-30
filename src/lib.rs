use zed_extension_api as zed;

struct MatlabExt;

impl zed::Extension for MatlabExt {
    fn new() -> Self { Self }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        // Minimal: trust Zed settings lsp."matlab-language-server".binary
        if language_server_id.as_ref() != "matlab-language-server" {
            return Err("Unknown language server id".into());
        }

        // Read per-worktree LSP binary config, falling back to a clear error.
        // Expected settings (in ~/.zed/settings.json):
        // "lsp": { "matlab-language-server": { "binary": { "path": "node", "arguments": ["/path/to/out/index.js", "--stdio"], "env": { }}}}
        #[allow(deprecated)]
        let lsp = zed::settings::LspSettings::for_worktree("matlab-language-server", worktree)
            .map_err(|e| format!("Failed to read Zed settings for matlab-language-server: {}", e))?;

        if let Some(bin) = lsp.binary {
            let path = bin
                .path
                .ok_or_else(|| "lsp.matlab-language-server.binary.path is required".to_string())?;
            let mut args = bin.arguments.unwrap_or_default();
            let env_map = bin.env.unwrap_or_default();

            // Ensure --stdio is present (MATLAB LSP expects stdio transport)
            if !args.iter().any(|a| a == "--stdio") {
                args.push("--stdio".into());
            }

            // Bridge installPath: if not provided via CLI, use env if available
            let has_install_flag = args.iter().any(|a| a == "--matlabInstallPath");
            if !has_install_flag {
                if let Some(install) = env_map
                    .get("MATLAB_LSP_INSTALL_PATH")
                    .or_else(|| env_map.get("MATLAB_INSTALL_PATH"))
                {
                    if !install.trim().is_empty() {
                        args.push("--matlabInstallPath".into());
                        args.push(install.clone());
                    }
                }
            }

            // Bridge connection timing similarly (optional)
            let has_timing_flag = args.iter().any(|a| a == "--matlabConnectionTiming");
            if !has_timing_flag {
                if let Some(timing) = env_map.get("MATLAB_LSP_CONNECTION_TIMING") {
                    if !timing.trim().is_empty() {
                        args.push("--matlabConnectionTiming".into());
                        args.push(timing.clone());
                    }
                }
            }

            let env: Vec<(String, String)> = env_map.into_iter().collect();
            Ok(zed::Command { command: path, args, env })
        } else {
            Err(
                "Configure Zed: lsp.matlab-language-server.binary.path and .arguments (e.g. node + out/index.js --stdio)".into(),
            )
        }
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>, String> {
        Ok(None)
    }
}

zed::register_extension!(MatlabExt);
