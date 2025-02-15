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

    pub fn get_purchased_items(&self) -> Vec<CollectionItem> {
        // let selector =
        //     Selector::parse(r#"li[class="collection-item-container"]"#).unwrap();
        let selector_details =
            Selector::parse(r#"div[class="collection-title-details"]"#).unwrap();
        let selector_download_links =
            Selector::parse(r#"span[class="redownload-item"]"#).unwrap();
        let selection_details = self.html.select(&selector_details);
        let selection_download_links = self.html.select(&selector_download_links);

        selection_details.for_each(|elem| {
            println!("{:?}", elem.html());
        });

        // for s in selection {
        //     println!("yeah we iterate mothafuckaaa");
        //     println!("{:?}", s);
        // }

        // println!("why da fuck len 0?? {}", selection.);

        vec![]
    }
}
