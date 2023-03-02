use std::net::TcpListener;

use actix_web::{dev, web, App, HttpServer};

use crate::routes;

pub fn run(listener: TcpListener) -> std::io::Result<dev::Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/health_check", web::get().to(routes::health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
