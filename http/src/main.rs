use anyhow::Result;
use axum::{Router, routing::any_service};
use clap::Parser;
use http::Server;
use rmcp::transport::{
    StreamableHttpServerConfig,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};
use tracing_subscriber::{self, EnvFilter};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value = "3000")]
    port: u16,
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let args = Args::parse();
    let addr = format!("{}:{}", args.host, args.port);

    let mcp_service: StreamableHttpService<Server, LocalSessionManager> =
        StreamableHttpService::new(
            || Ok(Server::new()),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig::default(),
        );

    let app = Router::new()
        .route("/mcp", any_service(mcp_service));

    let tcp_listener = tokio::net::TcpListener::bind(&addr).await?;

    eprintln!("HTTP MCP server running on http://{}/mcp", addr);

    axum::serve(tcp_listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
        })
        .await?;

    Ok(())
}
