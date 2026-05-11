use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;
use crate::routes::*;


pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| { // no params
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
