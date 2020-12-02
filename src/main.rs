mod api;
mod db;
mod domain;
mod config;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use dotenv::dotenv;
use slog::info;

use actix_cors::Cors;
use actix_web::middleware::Compress;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};

use crate::db::DbContext;
use crate::config::Config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to read .env file");

    let log = Config::configure_log();

    let bind_address = Config::bind_address_from_env();

    let db_context = DbContext::new_from_env();

    info!(log, "Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
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

pub struct AppState {
    db_context: DbContext,
    log: Logger,
}