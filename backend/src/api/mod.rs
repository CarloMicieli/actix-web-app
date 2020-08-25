mod handlers;
pub mod middleware;
mod responses;
mod tokens;

use handlers::{account_handlers, brand_handlers, health_handlers};

use actix_web::web;

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
                        .route(web::get().to(brand_handlers::get_all_brands))
                        .route(web::post().to(brand_handlers::post_new_brand))
                    )
                    .service(
                        web::resource("/{brand}")
                            .route(web::get().to(brand_handlers::get_brand))
                )
            )
    );
}
