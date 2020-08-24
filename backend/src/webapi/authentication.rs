use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::{
    encode, EncodingKey, Header,
};
use chrono::{DateTime, Utc, Duration, Timelike};

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
pub struct Claims {
    sub: String,
    name: String,
    #[serde(with = "jwt_numeric_date")]
    iat: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    exp: DateTime<Utc>,
}

impl Claims {
    pub fn new(name: &str, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = iat.date().and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
        let exp = exp.date().and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);
        Self {
            name: name.to_owned(),
            sub: "12345678".to_owned(),
            iat,
            exp
        }
    }
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

mod jwt_numeric_date {
    //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}
