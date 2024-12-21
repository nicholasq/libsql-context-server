# Zed Context Server

This extension provides a connection to a Model Context Server for Libsql, for
use with the Zed AI assistant. It will add prompts from the connected server as
slash commands to the Assistant Panel.

> **Note:** This extension does not install a libsql model context server. You
> will need to provide your own. Here is a link to one:
> [Mcp Server LibSQL](https://github.com/nicholasq/mcp-server-libsql)

This extension assumes you are using a Model Context Server in
[stdio transport](https://modelcontextprotocol.io/docs/concepts/transports#standard-input-output-stdio).
It also assumes that the Model Context Server receives these arguments:
`<libsql-url> --auth-token <your-libsql-db-token>` (--auth-token is optional if
database is local)

## Configuration

Your Zed `settings.json`:

```json
{
  "context_servers": {
    "zed-context-server": {
      "settings": {
        "command": "uvx",
        "args": [
          "mcp-server-git",
          "--repository",
          "/Users/d/Projects/opensource/onetime/onetimesecret"
        ]
      }
    }
  }
}
```

## Usage

Assuming you have
[Mcp Server LibSQL](https://github.com/nicholasq/mcp-server-libsql) connected to
this extension:

- `/libsql-schema <table-name>`: Retrieve the schema for the table with the
  given name.
- `/libsql-schema all-tables`: Retrieve the schemas for all tables in the
  database.
- `/libsql-query <table-name>`: Retrieve all rows from the table with the given
  name(actually it only pulls max 500 rows for now).
