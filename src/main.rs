use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use tracing_actix_web::TracingLogger;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::fs::File;

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

    env_logger::init_from_env(Env::default().default_filter_or("trace")); 

    /*
    let log_file = File::create("./app.log").unwrap();



    CombinedLogger::init(
        vec![
            // Write logs to file
            WriteLogger::new(
                LevelFilter::Debug,
                Config::default(),
                log_file,
            ),

            // Write logs to console
            WriteLogger::new(LevelFilter::Debug, Config::default(), std::io::stdout()),
        ]
    ).unwrap();*/

   

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(hello)
            .service(hi)
            //.wrap(Logger::default())
            
            
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}