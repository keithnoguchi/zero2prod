//! [zero2prod]
//!
//! [zero2prod]: https://github.com/LukeMathWalker/zero-to-production/

use std::net::TcpListener;

use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    zero2prod::init_subscriber(zero2prod::get_subscriber("zero2prod", "info"));
    let config = zero2prod::get_config().expect("failed to read configuration");
    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&addr)?;
    let db_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("failed to connect to database");
    zero2prod::run(listener, db_pool)?.await
}
