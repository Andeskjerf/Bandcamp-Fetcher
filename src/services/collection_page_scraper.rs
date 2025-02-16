use scraper::{Html, Selector};

use crate::models::collection_item::CollectionItem;

pub struct CollectionPageScraper {
    html: Html,
}

impl CollectionPageScraper {
    pub fn new(html_dom: &str) -> Self {
        Self {
            html: Html::parse_document(html_dom),
        }
    }

    fn get_item_name(&self, html_dom: &str) -> String {
        html_dom
            // there's a newline after the item name, for some reason. only get what's before the \n
            .split("\n")
            .next()
            .unwrap()
            .to_string()
    }

    fn get_band_name(&self, html_dom: &str) -> String {
        html_dom
            // we know that the band / artist name is prefixed by 'by '
            .split("by ")
            // skip 1, the 'by ' part
            .nth(1)
            .unwrap()
            .to_string()
    }

    // TODO: does not get the actual download URL, but rather the page that loads the URL we need...
    /// Parses the DOM to get the download URL for all items on the collection page
    fn get_item_download_url(&self) -> Vec<String> {
        let selector = Selector::parse(r#"span[class="redownload-item"] a"#).unwrap();

        self.html
            .select(&selector)
            .fold(vec![], |mut acc: Vec<String>, elem| {
                acc.push(
                    elem.attr("href")
                        .expect("element does not have a href attribute, for some reason!")
                        .to_string(),
                );
                acc
            })
    }

    /// Parses the DOM to find the band name, and the item name (Album, single, EP, etc)
    fn get_item_details(&self) -> Vec<CollectionItem> {
        let selector = Selector::parse(r#"div[class="collection-title-details"] a"#).unwrap();
        let selection = self.html.select(&selector);

        let mut items: Vec<CollectionItem> = vec![];

        // get band name & item name from the DOM
        for child in selection {
            items.push(CollectionItem::new());
            for (i, child) in child.child_elements().enumerate() {
                let last_item = items.last_mut().unwrap();
                match i {
                    0 => last_item.set_name(&self.get_item_name(&child.inner_html())),
                    1 => last_item.set_band(&self.get_band_name(&child.inner_html())),
                    _ => log::warn!("found unknown element, i=={i}, while parsing DOM for purchased items! DOM may have changed and could cause the parser to break!"),
                };
            }
        }

        items
    }

    pub fn get_purchased_items(&self) -> Vec<CollectionItem> {
        let items = self.get_item_details();
        let urls = self.get_item_download_url();

        // somewhat naive, this expects that we're getting the details & urls in the same order
        // that should be the case, as the order of items is not random on the DOM
        // it could still be more robust!
        items
            .iter()
            .zip(urls.iter())
            .map(|(item, url)| CollectionItem::with_params(&item.band(), &item.name(), url))
            .collect::<Vec<CollectionItem>>()
    }
}
