#[tokio::test]
async fn health_check() {
    spawn_app();

    let client = reqwest::Client::new();

    let resp = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(resp.content_length(), Some(0));
}

fn spawn_app() {
    let server = zero2prod::run().expect("failed to bind address");

    let _ = tokio::spawn(server);
}
