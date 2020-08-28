use crate::api::requests::brand_requests::{EditBrandRequest, NewBrandRequest};
use crate::api::responses::pagination::PaginateQuery;
use crate::api::responses::pagination::PaginatedResults;
use crate::db::DbContext;
use crate::domain::brands::Brand;
use crate::domain::common::Address;

use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_brands(
    paginate_params: web::Query<PaginateQuery>,
    _context: web::Data<DbContext>,
) -> impl Responder {
    let brands =
        PaginatedResults::new(vec![fake_brand(), fake_brand(), fake_brand()]);

    debug!("{:?}", paginate_params);

    HttpResponse::Ok().json(brands)
}

pub async fn post_new_brand(
    new_brand: web::Json<NewBrandRequest>,
    _context: web::Data<DbContext>,
) -> impl Responder {
    debug!("{:#?}", new_brand);

    HttpResponse::Ok()
}

pub async fn get_brand(
    slug: web::Path<String>,
    _context: web::Data<DbContext>,
) -> impl Responder {
    info!("GET /brands/{}", slug);

    let brand = fake_brand();

    debug!("{:#?}", brand);

    HttpResponse::Ok().json(brand)
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

fn fake_brand() -> Brand {
    let brand = Brand::new(
        uuid::Uuid::new_v4(),
        "ACME",
        None,
        None,
        Some(Address::new(
            String::from("22 acacia avenue"),
            None,
            String::from("London"),
            None,
            String::from("UK"),
            String::from("12345"),
        )),
        None,
        None,
        "Industrial",
    );
    brand
}
