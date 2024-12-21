use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const SERVER_COMMAND: &str = "/opt/homebrew/bin/uvx"; // Use full path
const SERVER_PACKAGE: &str = "mcp-server-git";

struct ZedModelContextExtension;

#[derive(Debug, Deserialize)]
struct GitContextServerSettings {
    repository: String,
}

impl zed::Extension for ZedModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("zed-context-server", project)?;

        let Some(settings) = settings.settings else {
            return Err("missing repository setting".into());
        };

        let settings: GitContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: SERVER_COMMAND.to_string(),
            args: vec![
                SERVER_PACKAGE.to_string(),
                "--repository".to_string(),
                settings.repository,
            ],
            env: vec![
                ("MCP_DEBUG".to_string(), "1".to_string()),
                ("RUST_LOG".to_string(), "debug".to_string()),
            ],
        })
    }
}

zed::register_extension!(ZedModelContextExtension);
