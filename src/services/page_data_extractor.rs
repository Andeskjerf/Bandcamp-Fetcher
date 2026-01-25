use log::error;
use scraper::{Html, Selector};
use serde_json::Value;

use crate::models::api::{collection_data::CollectionData, fan::Fan};

/// Utility struct to retrieve the PageData blob embedded in the DOM of each page
/// This blob contains critical data formatted as a json required to make requests
pub struct PageDataExtractor {
    html: Html,
    json: Value,
}

impl PageDataExtractor {
    pub fn new(html_dom: &str) -> Self {
        let parsed = Html::parse_document(html_dom);
        Self {
            html: parsed.clone(),
            json: PageDataExtractor::get_page_data(parsed),
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
}
