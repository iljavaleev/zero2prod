use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, 
    dev::Server, web};
use std::net::TcpListener;


async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


async fn health_check(_req: HttpRequest) -> impl Responder{
    HttpResponse::Ok().finish() // resp with empty body
}

async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| { // no params
        App::new()
        .route("/", web::get().to(greet))
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
        .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}