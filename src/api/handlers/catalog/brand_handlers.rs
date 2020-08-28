use crate::api::requests::brand_requests::{EditBrandRequest, NewBrandRequest};
use crate::api::responses::pagination::PaginateQuery;
use crate::db::DbContext;

use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_brands(
    paginate_params: web::Query<PaginateQuery>,
    _context: web::Data<DbContext>,
) -> impl Responder {
    debug!("{:?}", paginate_params);
    HttpResponse::Ok()
}

pub async fn post_new_brand(new_brand: web::Json<NewBrandRequest>, _context: web::Data<DbContext>) -> impl Responder {
    debug!("{:#?}", new_brand);

    HttpResponse::Ok()
}

pub async fn get_brand(slug: web::Path<String>, _context: web::Data<DbContext>) -> impl Responder {
    info!("GET /brands/{}", slug);

    HttpResponse::Ok()
}

pub async fn edit_brand(
    slug: web::Path<String>,
    modified_brand: web::Json<EditBrandRequest>,
    _context: web::Data<DbContext>,
) -> impl Responder {
    info!("PUT /brands/{}", slug);

    debug!("{:#?}", modified_brand);

    HttpResponse::Ok()
}
