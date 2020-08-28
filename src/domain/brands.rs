use super::common::Address;

#[derive(Debug, Serialize)]
pub struct Brand {
    id: uuid::Uuid,
    name: String,
    company_name: Option<String>,
    logo_url: Option<String>,
    address: Option<Address>,
    mail_address: Option<String>,
    website_url: Option<String>,
    kind: String,
}

impl Brand {
    pub fn new(
        id: uuid::Uuid,
        name: &str,
        company_name: Option<String>,
        logo_url: Option<String>,
        address: Option<Address>,
        mail_address: Option<String>,
        website_url: Option<String>,
        kind: &str,
    ) -> Self {
        Brand {
            id,
            name: name.to_owned(),
            company_name,
            logo_url,
            address,
            mail_address,
            website_url,
            kind: kind.to_owned(),
        }
    }
}
