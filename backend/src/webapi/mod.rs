mod pagination;
mod brands_controller;
mod health_controller;

use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    #[rustfmt::skip]
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::resource("/health_check")
                    .route(web::get().to(health_controller::health_check))
            )
            .service(
                web::scope("/brands")
                    .service(
                    web::resource("")
                        .route(web::get().to(brands_controller::get_all_brands))
                        .route(web::post().to(brands_controller::post_new_brand))
                    )
                    .service(
                        web::resource("/{brand}")
                            .route(web::get().to(brands_controller::get_brand))
                )
            )
    );
}
