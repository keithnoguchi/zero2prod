//! [zero2prod]
//!
//! [zero2prod]: https://github.com/LukeMathWalker/zero-to-production/

use std::io::Result;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
