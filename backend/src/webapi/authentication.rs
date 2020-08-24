use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::{
    encode, EncodingKey, Header,
};

#[derive(Debug, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct Authentication {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    name: String,
    iat: u64,
}

pub async fn user_login(user: web::Json<Login>) -> impl Responder {
    debug!("User {} tried to login", user.username);

    let secret_key =
        std::env::var("SECRET_KEY").expect("Unable to find a SECRET_KEY");

    let my_claims = Claims {
        sub: "12345678".to_owned(),
        name: user.username.to_owned(),
        iat: 1,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap();

    HttpResponse::Ok().json(Authentication { token })
}

