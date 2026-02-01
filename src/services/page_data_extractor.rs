use log::error;
use scraper::{Html, Selector};
use serde_json::Value;

use crate::models::{
    api::{collection_data::CollectionData, fan::Fan},
    bandcamp_sale::BandcampSale,
};

/// Utility struct to retrieve the PageData blob embedded in the DOM of each page
/// This blob contains critical data formatted as a json required to make requests
pub struct PageDataExtractor {
    json: Value,
}

impl PageDataExtractor {
    pub fn new(html_dom: &str) -> Self {
        Self {
            json: PageDataExtractor::get_page_data(Html::parse_document(html_dom)),
        }
    }

    fn get_page_data(html: Html) -> Value {
        let selector = Selector::parse(r#"div[id="pagedata"]"#).unwrap();
        let mut selected = html.select(&selector);

        // there should only be a single pagedata div on the page, so next to get that
        let elem = selected
            .next()
            .expect("failed to get next item from selected!");
        let blob = elem
            .attr("data-blob")
            .expect("failed to get attr data-blob from pagedata!");

        // TODO: make this... better
        serde_json::from_str(blob).unwrap()
    }

    /// Gets a key from the json object
    fn get_value(&self, json: &Value, key: &str) -> Option<Value> {
        let value = json.get(key);
        if value.is_none() {
            error!("Failed to find '{key}' object in 'pagedata' blob!");
            return None;
        }
        Some(value.unwrap().clone())
    }

    pub fn get_fan(&self) -> Option<Fan> {
        let identities = self.get_value(&self.json, "identities");
        let result = serde_json::from_value(self.get_value(&identities.unwrap(), "fan").unwrap());
        match result {
            Ok(res) => Some(res),
            Err(err) => {
                error!("Failed to parse 'fan' into model! {err:?}");
                None
            }
        }
    }

    pub fn get_collection_data(&self) -> Option<CollectionData> {
        let collection_data = self.get_value(&self.json, "collection_data");
        let result = serde_json::from_value(collection_data.unwrap());
        match result {
            Ok(res) => Some(res),
            Err(err) => {
                error!("Failed to parse 'collection_data' into model! {err:?}");
                None
            }
        }
    }

    /// Gets a list of items from the download page
    /// Appears to only contain a single item, the item we are currently viewing
    pub fn get_digital_items(&self) -> Option<Vec<BandcampSale>> {
        let val = self.get_value(&self.json, "digital_items").unwrap();
        let download_items = val.to_string();
        let result = serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(
            &download_items,
        ));
        match result {
            Ok(res) => Some(res),
            Err(err) => {
                error!("Failed to parse 'download_items' into model! {err:?}");
                None
            }
        }
    }
}
