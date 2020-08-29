use crate::api::requests::brand_requests::{EditBrandRequest, NewBrandRequest};
use crate::api::responses::pagination::PaginateQuery;
use crate::AppState;

//use slog::{info, debug};
use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_brands(
    paginate_params: web::Query<PaginateQuery>,
    _state: web::Data<AppState>,
) -> impl Responder {
    debug!("{:?}", paginate_params);
    HttpResponse::Ok()
}

pub async fn post_new_brand(new_brand: web::Json<NewBrandRequest>, _state: web::Data<AppState>) -> impl Responder {
    //debug!(state.log, "{:#?}", new_brand);

    HttpResponse::Ok()
}

pub async fn get_brand(slug: web::Path<String>, _state: web::Data<AppState>) -> impl Responder {
    //info!(state.log, "GET /brands/{}", slug);

    HttpResponse::Ok()
}

pub async fn edit_brand(
    slug: web::Path<String>,
    modified_brand: web::Json<EditBrandRequest>,
    _state: web::Data<AppState>,
) -> impl Responder {
    //info!(state.log, "PUT /brands/{}", slug);
    //debug!(state.log, "{:#?}", modified_brand);

    HttpResponse::Ok()
}
