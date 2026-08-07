use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use expect_test::expect;
use tower::ServiceExt;

async fn app() -> Router {
    let pool = database::test_utils::test_pool().await;
    api::router(api::AppState { pool })
}

#[tokio::test]
async fn root_returns_hello_world() {
    let response = app()
        .await
        .oneshot(Request::new(Body::empty()))
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");

    assert_eq!(&body[..], b"<h1>Hello, World!</h1>");
}

#[tokio::test]
async fn health_returns_ok() {
    let response = app()
        .await
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");
    let body = std::str::from_utf8(&body).expect("body should be utf-8");

    let actual = format!("{status}\n{body}");
    expect![[r#"
        200 OK
        ok"#]]
    .assert_eq(&actual);
}

#[tokio::test]
async fn live_returns_ok() {
    let response = app()
        .await
        .oneshot(
            Request::builder()
                .uri("/live")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");
    let body = std::str::from_utf8(&body).expect("body should be utf-8");

    let actual = format!("{status}\n{body}");
    expect![[r#"
        200 OK
        ok"#]]
    .assert_eq(&actual);
}
