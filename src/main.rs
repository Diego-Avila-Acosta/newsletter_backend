use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn healtch_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(healtch_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
