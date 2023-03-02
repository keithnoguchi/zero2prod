use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::DatabaseSettings;

#[tokio::test]
async fn health_check() {
    let app = TestApp::build().await;
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(resp.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_success() {
    let app = TestApp::build().await;
    let client = reqwest::Client::new();

    let body = "name=test%20name&email=test%40gmail.com";
    let resp = client
        .post(&format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request.");

    assert_eq!(resp.status().as_u16(), 200);

    // check the state in the database.
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("failed to fetch saved subscription");
    assert_eq!(saved.email, "test@gmail.com");
    assert_eq!(saved.name, "test name");
}

#[tokio::test]
async fn subscribe_wrong_body() {
    let app = TestApp::build().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=test%20name", "missing the email"),
        ("email=test%40gmail.com", "missing name"),
        ("", "missing both name and the email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let resp = client
            .post(&format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request.");

        assert_eq!(
            resp.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message,
        );
    }
}

struct TestApp {
    address: String,
    db_pool: PgPool,
}

impl TestApp {
    async fn init_database(database: &DatabaseSettings) -> PgPool {
        let mut conn = PgConnection::connect(&database.connection_string_without_db())
            .await
            .expect("failed to connect to database");
        conn.execute(format!(r#"CREATE DATABASE "{}";"#, database.database_name).as_ref())
            .await
            .expect("failed to create database");

        let db_pool = PgPool::connect(&database.connection_string())
            .await
            .expect("failed to connect to database");
        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("failed to migrate the dtabase");

        db_pool
    }

    async fn build() -> Self {
        let mut config = zero2prod::get_config().expect("failed to read config.yaml");
        config.database.database_name = Uuid::new_v4().to_string();
        let db_pool = Self::init_database(&config.database).await;
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
        let local_addr = listener
            .local_addr()
            .expect("failed to get the local address");
        let address = format!("http://{local_addr}");

        // Spawn thread to avoid the clippy error.
        //
        // https://rust-lang.github.io/rust-clippy/master/index.html#let_underscore_future
        let server = zero2prod::run(listener, db_pool.clone()).expect("failed to listen");
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = tokio::spawn(server).await;
            });
        });

        Self { address, db_pool }
    }
}
