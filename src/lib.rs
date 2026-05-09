use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, http::Error, web};


async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


async fn health_check(_req: HttpRequest) -> impl Responder{
    HttpResponse::Ok().finish() // resp with empty body
}


pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| { // no params
        App::new()
        .route("/", web::get().to(greet))
        .route("/health_check", web::get().to(health_check))
         .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}