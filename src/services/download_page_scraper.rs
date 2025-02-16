use std::collections::HashMap;

use scraper::{Html, Selector};
use serde_json::Value;

use crate::models::bandcamp_format::BandcampFormat;

pub struct DownloadPageScraper {
    html: Html,
}

impl DownloadPageScraper {
    pub fn new(html: &str) -> Self {
        Self {
            html: Html::parse_document(html),
        }
    }

    pub fn get_download_formats(&self) -> HashMap<String, BandcampFormat> {
        let selector = Selector::parse(r#"div[id="pagedata"]"#).unwrap();
        let mut selected = self.html.select(&selector);

        // there should only be a single pagedata div on the page, so next to get that
        let elem = selected
            .next()
            .expect("failed to get next item from selected!");
        let blob = elem
            .attr("data-blob")
            .expect("failed to get attr data-blob from pagedata!");

        // TODO: make this... better
        let json: Value = serde_json::from_str(blob).unwrap();
        let mut map: HashMap<String, BandcampFormat> = HashMap::new();

        // we start at the top of the json. the data we want is nested, so we dig in
        if let Some(object) = json.as_object() {
            let download_items = object
                .iter()
                .find(|(k, _)| k.as_str() == "download_items")
                .expect("failed to get download_items, unable to find!");

            // we're only interested in the downloads, so find that and start iterating
            if let Some(obj_downloads) = download_items.1.as_array() {
                // ...why are we iterating over this again? does the array only contain a single item?
                for item in obj_downloads {
                    if let Some(object) = item.as_object() {
                        let downloads = object
                            .iter()
                            .find(|(k, _)| k.as_str() == "downloads")
                            .expect("failed to get inner downloads element!");

                        // we find all of our downloads at this point, not sure why we're iterating above
                        if let Some(object) = downloads.1.as_object() {
                            for (key, val) in object.iter() {
                                map.insert(
                                    key.clone(),
                                    serde_json::from_value(val.clone()).unwrap(),
                                );
                            }
                        }
                    }
                }
            }
        }

        map
    }
}
