use crate::tokens::Claims;
use actix_web::{web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Debug, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct Authentication {
    token: String,
}

pub async fn user_login(user: web::Json<Login>) -> impl Responder {
    debug!("User {} tried to login", user.username);

    let secret_key =
        std::env::var("SECRET_KEY").expect("Unable to find a SECRET_KEY");

    let iat = Utc::now();
    let exp = iat + Duration::minutes(30);

    let my_claims = Claims::new(&user.username, iat, exp);

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap();

    HttpResponse::Ok().json(Authentication { token })
}
