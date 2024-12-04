# Libsql Context Server

This extension provides a Model Context Server for Libsql, for use with the Zed AI assistant. It adds a `/libsql-schema` slash command to the Assistant Panel.

## Configuration

To use the extension, you will need to point the context server at a Libsql database by setting the `database_url` in your Zed `settings.json`:

> **Note:** This extension does not install a libsql model context server. You will need to provide your own. Here is a link to one: [Mcp Server LibSQL](https://github.com/nicholasq/mcp-server-libsql)

```json
{
  "context_servers": {
    "libsql-context-server": {
      "settings": {
        "database_url": "libsql://db-name-user.turso.io",
        "server_path": "/path/to/libsql-server",
        "auth_token": "my_auth_token" // optional if running a local libsql instance
      }
    }
  }
}
```

## Usage

- `/libsql-schema`: Retrieve the schema for the table with the given name.
- `/libsql-schema all-tables`: Retrieve the schemas for all tables in the database.
