use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CollectionData {
    redownload_urls: HashMap<String, String>,
    last_token: String,
    item_count: u32,
    batch_size: u32,
    hidden_items_count: u32,
    small_collection: bool,
    small_wishlist: bool,
    purchase_infos: HashMap<String, String>, // idk what this is
    collectors: HashMap<String, String>,     // idk what this is
    sequence: Vec<String>,
    pending_sequence: Vec<String>,
}

impl CollectionData {
    pub fn redownload_urls(&self) -> &HashMap<String, String> {
        &self.redownload_urls
    }
}
