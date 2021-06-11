use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
        .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}