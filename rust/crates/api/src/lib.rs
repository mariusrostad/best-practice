use axum::{Router, extract::State, http::StatusCode, response::Html, routing::get};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/health", get(health_handler))
        .route("/live", get(live_handler))
        .with_state(state)
}

pub async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn health_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "ok")
}

pub async fn live_handler(State(state): State<AppState>) -> (StatusCode, &'static str) {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => (StatusCode::OK, "ok"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "unavailable"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[tokio::test]
    async fn index_handler_returns_hello_world_html() {
        let Html(html) = index_handler().await;
        assert_eq!(html, "<h1>Hello, World!</h1>");
    }

    #[tokio::test]
    async fn health_handler_returns_ok() {
        let (status, body) = health_handler().await;
        let actual = format!("{status}\n{body}");
        expect![[r#"
            200 OK
            ok"#]]
        .assert_eq(&actual);
    }

    #[tokio::test]
    async fn live_handler_returns_ok() {
        let pool = database::test_utils::test_pool().await;
        let (status, body) = live_handler(State(AppState { pool })).await;
        let actual = format!("{status}\n{body}");
        expect![[r#"
            200 OK
            ok"#]]
        .assert_eq(&actual);
    }

    #[tokio::test]
    async fn live_handler_returns_service_unavailable_when_pool_is_closed() {
        let pool = database::test_utils::test_pool().await;
        pool.close().await;

        let (status, body) = live_handler(State(AppState { pool })).await;
        let actual = format!("{status}\n{body}");
        expect![[r#"
            503 Service Unavailable
            unavailable"#]]
        .assert_eq(&actual);
    }
}
