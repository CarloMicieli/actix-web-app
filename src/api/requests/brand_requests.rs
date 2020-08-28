use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct NewBrandRequest {
    pub name: String,
    pub company_name: Option<String>,
    pub address: Option<AddressRequest>,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EditBrandRequest {
    pub name: String,
    pub company_name: Option<String>,
    pub address: Option<AddressRequest>,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddressRequest {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub region: Option<String>,
    pub zip_code: String,
    pub country: String,
}
