mod api;
mod db;
mod domain;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use env_logger::Env;
use std::env;

use actix_cors::Cors;
use actix_web::middleware::Compress;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};

use crate::db::DbContext;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to read .env file");
    env_logger::from_env(Env::default().default_filter_or("debug")).init();

    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_owned());
    let port_number = env::var("APP_PORT").unwrap_or("5000".to_owned());
    let bind_address = format!("{}:{}", &app_host, &port_number);

    let db_context = DbContext::new_from_env();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                    ])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .data(db_context.clone())
            .data(web::JsonConfig::default().limit(4096))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(api::middleware::BearerAuthentication)
            .configure(api::config_services)
    })
    .bind(bind_address)?
    .run()
    .await
}
