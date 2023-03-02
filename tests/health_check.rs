use std::net::{SocketAddr, TcpListener};

#[tokio::test]
async fn health_check() {
    let local_addr = spawn_app();
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("http://{local_addr}/health_check"))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(resp.status().is_success());
    assert_eq!(resp.content_length(), Some(0));
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
