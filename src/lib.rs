use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use sqlx::PgConnection;

pub mod configuration;

async fn healtch_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(healtch_check))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
