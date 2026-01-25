use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::api::bandcamp_item::BandcampItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionItems {
    items: Vec<BandcampItem>,
    redownload_urls: HashMap<String, String>,
}

impl CollectionItems {
    pub fn redownload_urls(&self) -> &HashMap<String, String> {
        &self.redownload_urls
    }
}
