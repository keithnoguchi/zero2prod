use std::net::TcpListener;

use actix_web::{dev, web, App, HttpServer};
use sqlx::PgConnection;

use crate::routes;

pub fn run(listener: TcpListener, connection: PgConnection) -> std::io::Result<dev::Server> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/health_check", web::get().to(routes::health_check))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
