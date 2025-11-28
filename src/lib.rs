use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn healtch_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new(|| App::new().route("/health_check", web::get().to(healtch_check)))
            .listen(listener)?
            .run();

    Ok(server)
}
