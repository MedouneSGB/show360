mod config;
mod error;
mod routes;
mod state;

use anyhow::Context;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::config::ApiConfig;
use crate::state::AppState;

#[derive(Debug, Parser)]
#[command(name = "show360-api", about = "API applicative Show360 (Axum)")]
struct Cli {
    /// Bind address (ex. 0.0.0.0:8080)
    #[arg(long, env = "SHOW360_API_BIND", default_value = "0.0.0.0:8080")]
    bind: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let config = ApiConfig::from_env().map_err(|e| anyhow::anyhow!("configuration API: {e}"))?;
    let state = AppState::new(config);

    let app = routes::router(state);
    let listener = tokio::net::TcpListener::bind(&cli.bind)
        .await
        .with_context(|| format!("bind {}", cli.bind))?;

    tracing::info!(%cli.bind, "show360-api écoute");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("serve")?;
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("arrêt demandé");
}
