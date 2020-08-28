pub mod handlers;
pub mod middleware;
pub mod requests;
pub mod responses;
pub mod tokens;

use actix_web::HttpRequest;
use crate::api::requests::brand_requests::{EditBrandRequest, NewBrandRequest};
use actix_web::{error, web, FromRequest, HttpResponse};
use handlers::{account_handlers, health_handlers};
use handlers::catalog::brand_handlers;

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
                            cfg.error_handler(json_error_handler)
                        }))
                        .route(web::get().to(brand_handlers::get_all_brands))
                        .route(web::post().to(brand_handlers::post_new_brand))
                    )
                .service(
                    web::resource("/{brand}")
                        .app_data(web::Json::<EditBrandRequest>::configure(|cfg| {
                            cfg.error_handler(json_error_handler)
                        }))
                        .route(web::get().to(brand_handlers::get_brand))
                        .route(web::put().to(brand_handlers::edit_brand))
                )
            )
    );
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}