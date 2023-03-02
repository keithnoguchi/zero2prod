use std::net::{SocketAddr, TcpListener};

use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn health_check() {
    let local_addr = spawn_app();
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://{local_addr}/health_check"))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(resp.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_success() {
    let local_addr = spawn_app();
    let config = zero2prod::get_config().expect("failed to read config.yaml");
    let db_connection_string = config.database.connection_string();
    let _db_connection = PgConnection::connect(&db_connection_string)
        .await
        .expect("failed to connect to database");
    let client = reqwest::Client::new();

    let body = "name=test%20name&email=test%40gmail.com";
    let resp = client
        .post(&format!("http://{local_addr}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request.");

    assert_eq!(resp.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_wrong_body() {
    let local_addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=test%20name", "missing the email"),
        ("email=test%40gmail.com", "missing name"),
        ("", "missing both name and the email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let resp = client
            .post(&format!("http://{local_addr}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request.");

        assert_eq!(
            400,
            resp.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message,
        );
    }
}

fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
    let local_addr = listener
        .local_addr()
        .expect("failed to get the local address");
    let server = zero2prod::run(listener).expect("failed to listen");

    // Spawn thread to avoid the clippy error.
    //
    // https://rust-lang.github.io/rust-clippy/master/index.html#let_underscore_future
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = tokio::spawn(server).await;
        });
    });

    local_addr
}
