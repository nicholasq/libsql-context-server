use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

struct LibsqlModelContextExtension;

#[derive(Debug, Deserialize)]
struct LibsqlContextServerSettings {
    database_url: String,
    server_path: String,
    auth_token: Option<String>,
}

impl zed::Extension for LibsqlModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let ctx_server_settings =
            ContextServerSettings::for_project("libsql-context-server", project)?;
        let Some(settings) = ctx_server_settings.settings else {
            Err("missing settings for libsql-context-server")?
        };
        let settings: LibsqlContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        if settings.database_url.is_empty() {
            Err("missing database_url in libsql-context-server settings")?;
        }

        if settings.server_path.is_empty() {
            Err("missing server_path in libsql-context-server settings")?;
        }

        let mut args = vec![settings.database_url];

        if let Some(auth_token) = settings.auth_token.filter(|t| !t.is_empty()) {
            args.extend_from_slice(&["--auth-token".to_string(), auth_token]);
        }

        Ok(Command {
            command: settings.server_path,
            args,
            env: vec![],
        })
    }
}

zed::register_extension!(LibsqlModelContextExtension);
