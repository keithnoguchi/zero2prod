//! [zero2prod]
//!
//! [zero2prod]: https://github.com/LukeMathWalker/zero-to-production/

use std::net::TcpListener;

use sqlx::{Connection, PgConnection};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = zero2prod::get_config().expect("failed to read configuration");
    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&addr)?;
    let connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("failed to connect to database");
    zero2prod::run(listener, connection)?.await
}
