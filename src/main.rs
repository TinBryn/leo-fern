use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let port = match std::env::var("PORT") {
        Ok(port) => port,
        Err(_) => "8080".to_owned(),
    };

    println!("PORT: {}", port);

    let addr = format!("127.0.0.1:{}", port);

    HttpServer::new(|| App::new().service(index))
        .bind(addr)?
        .run()
        .await
}
