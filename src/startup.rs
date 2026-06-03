use crate::routes::{health_check, subscribe};
use actix_web::{App, HttpServer, dev::Server, web};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> std::io::Result<Server> {
    let db_pool = web::Data::new(connection);

    // Capture `connection` from the surrounding environment with move
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
