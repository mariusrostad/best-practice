use expect_test::expect;

#[tokio::test]
async fn server_serves_root_health_and_live() {
    let pool = database::setup(database::test_utils::TEST_DATABASE_URL)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "failed to set up the test database; run `podman compose up -d` first: {error:#}"
            )
        });
    let state = api::AppState { pool };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("should bind ephemeral port");
    let addr = listener.local_addr().expect("should have local addr");

    tokio::spawn(async move {
        server::run(listener, state)
            .await
            .expect("server should run");
    });

    let base = format!("http://{addr}");
    let client = reqwest::Client::new();

    let root = client
        .get(format!("{base}/"))
        .send()
        .await
        .expect("GET / should succeed");
    let status = root.status();
    let body = root.text().await.expect("root body should be readable");
    let actual = format!("{status}\n{body}");
    expect![[r#"
        200 OK
        <h1>Hello, World!</h1>"#]]
    .assert_eq(&actual);

    let health = client
        .get(format!("{base}/health"))
        .send()
        .await
        .expect("GET /health should succeed");
    let status = health.status();
    let body = health.text().await.expect("health body should be readable");
    let actual = format!("{status}\n{body}");
    expect![[r#"
        200 OK
        ok"#]]
    .assert_eq(&actual);

    let live = client
        .get(format!("{base}/live"))
        .send()
        .await
        .expect("GET /live should succeed");
    let status = live.status();
    let body = live.text().await.expect("live body should be readable");
    let actual = format!("{status}\n{body}");
    expect![[r#"
        200 OK
        ok"#]]
    .assert_eq(&actual);
}
