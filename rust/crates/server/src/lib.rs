//! Library entry for the `server` crate: bind a listener and serve the API.

use anyhow::Result;

pub async fn run(listener: tokio::net::TcpListener, state: api::AppState) -> Result<()> {
    axum::serve(listener, api::router(state)).await?;
    Ok(())
}
