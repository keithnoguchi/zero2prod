//! [zero2prod]
//!
//! [zero2prod]: https://github.com/LukeMathWalker/zero-to-production/

use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

// We'll make those data private once we start to use it inside
// subscribe function.
#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
