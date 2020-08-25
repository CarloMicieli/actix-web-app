use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Address {
    line1: String,
    line2: Option<String>,
    city: String,
    region: Option<String>,
    country: String,
    postal_code: String,
}

impl Address {
    pub fn new(
        line1: String,
        line2: Option<String>,
        city: String,
        region: Option<String>,
        country: String,
        postal_code: String,
    ) -> Self {
        Address {
            line1,
            line2,
            city,
            region,
            country,
            postal_code,
        }
    }
}
