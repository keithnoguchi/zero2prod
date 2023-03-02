//! [zero2prod]
//!
//! [zero2prod]: https://github.com/LukeMathWalker/zero-to-production/

mod routes;

use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/health_check", web::get().to(routes::health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
