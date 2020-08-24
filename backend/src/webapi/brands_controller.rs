use crate::models::brands::Brand;
use crate::models::common::Address;
use crate::webapi::pagination::PaginatedResults;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_brands() -> impl Responder {
    let brands =
        PaginatedResults::new(vec![fake_brand(), fake_brand(), fake_brand()]);

    HttpResponse::Ok().json(brands)
}

pub async fn post_new_brand() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn get_brand(slug: web::Path<String>) -> impl Responder {
    info!("GET /brands/{}", slug);

    let brand = fake_brand();

    debug!("{:#?}", brand);

    HttpResponse::Ok().json(brand)
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
