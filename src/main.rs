//! [zero2prod]
//!
//! [zero2prod]: https://github.com/LukeMathWalker/zero-to-production/

use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = zero2prod::get_config().expect("failed to read configuration");
    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&addr)?;
    println!("listening on {}", addr);
    zero2prod::run(listener)?.await
}
