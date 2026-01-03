# test-mcpb

MCP (Model Context Protocol) server examples for testing.

## Servers

### hello

Simple MCP server using stdio transport with a personalized greeting tool.

### auth

MCP server with OAuth 2.0 authorization using HTTP transport.

Features:
- OAuth 2.0 authorization code flow
- Dynamic client registration
- RFC 8707 resource indicator support
- RFC 9728 protected resource metadata

## Testing with tool-cli

[tool-cli](https://github.com/zerocore-ai/tool-cli) is the recommended way to test these MCP servers.

### Install tool-cli

```bash
curl -fsSL https://raw.githubusercontent.com/zerocore-ai/tool-cli/main/install.sh | sh
```

### Test the servers

Each server directory contains a `manifest.json` that tool-cli uses to run and inspect the server.

**Inspect server capabilities:**

```bash
tool info ./hello              # Show tools, prompts, resources
tool info ./hello --tools      # Show only tools
tool info ./hello --json       # Output as JSON
```

**Call tools directly:**

```bash
tool call ./hello -m greet -p name="World"
tool call ./auth -m greet -p name="World"  # Will trigger OAuth flow
```

**Verbose mode (see MCP protocol messages):**

```bash
tool call ./hello -m greet -p name="World" --verbose
```

**Validate manifest:**

```bash
tool validate ./hello
tool validate ./hello --strict  # Treat warnings as errors
```
