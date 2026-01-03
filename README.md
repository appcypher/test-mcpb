# test-mcpb

MCP (Model Context Protocol) server examples for testing.

## Servers

### hello

Simple MCP server using stdio transport with a personalized greeting tool.

```bash
cd hello
cargo build --release
```

### auth

MCP server with OAuth 2.0 authorization using HTTP transport.

Features:
- OAuth 2.0 authorization code flow
- Dynamic client registration
- RFC 8707 resource indicator support
- RFC 9728 protected resource metadata

```bash
cd auth
cargo build --release
./target/release/auth --port 3000
```
