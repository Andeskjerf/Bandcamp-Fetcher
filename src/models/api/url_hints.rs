use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlHints {
    pub subdomain: String,
    pub custom_domain: Option<String>,
    pub custom_domain_verified: Option<bool>,
    pub slug: String,
    pub item_type: String,
}
