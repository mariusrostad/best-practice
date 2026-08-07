//! Binary entrypoint for the `server` crate.

use anyhow::Result;

const DEFAULT_DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/app";

#[tokio::main]
async fn main() -> Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());
    let pool = database::setup(&database_url).await?;
    let state = api::AppState { pool };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let addr = listener.local_addr()?;
    let startup_banner = format!(
        r#"
 ____            _     ____                 _   _
| __ )  ___  ___| |_  |  _ \ _ __ __ _  ___| |_(_) ___ ___
|  _ \ / _ \/ __| __| | |_) | '__/ _` |/ __| __| |/ __/ _ \
| |_) |  __/\__ \ |_  |  __/| | | (_| | (__| |_| | (_|  __/
|____/ \___||___/\__| |_|   |_|  \__,_|\___|\__|_|\___\___| Rust Web App

  module: server
  url:  http://{addr}
"#
    );
    println!("{startup_banner}");
    server::run(listener, state).await
}
