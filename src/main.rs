use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hi")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("Greetings")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

   env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hi)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}