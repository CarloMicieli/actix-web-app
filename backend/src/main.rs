mod webapi;

use dotenv::dotenv;
use env_logger::Env;
use std::env;

use actix_web::middleware::Compress;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let port_number = env::var("PORT").unwrap_or("5000".to_owned());
    let bind_address = format!("0.0.0.0:{}", port_number);

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/health_check", web::get().to(health_check))
    })
    .bind(bind_address)?
    .run()
    .await
}
