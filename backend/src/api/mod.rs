pub mod handlers;
pub mod middleware;
pub mod requests;
pub mod responses;
pub mod tokens;

use crate::api::requests::brand_requests::NewBrandRequest;
use actix_web::{error, web, FromRequest, HttpResponse};
use handlers::{account_handlers, brand_handlers, health_handlers};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::resource("/authenticate")
                    .route(web::post().to(account_handlers::authenticate))
            )
            .service(
                web::resource("/health_check")
                    .route(web::get().to(health_handlers::health_check))
            )
            .service(
                web::scope("/brands")
                .service(
                    web::resource("")
                        .app_data(web::Json::<NewBrandRequest>::configure(|cfg| {
                            cfg.limit(4096)
                                .error_handler(|err, _req| {
                                    error::InternalError::from_response(
                                        err, HttpResponse::BadRequest().finish()).into()
                                })
                        }))
                        .route(web::get().to(brand_handlers::get_all_brands))
                        .route(web::post().to(brand_handlers::post_new_brand))
                    )
                .service(
                    web::resource("/{brand}")
                        .route(web::get().to(brand_handlers::get_brand))
                        .route(web::put().to(brand_handlers::edit_brand))
                )
            )
    );
}
