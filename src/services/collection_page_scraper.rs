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

    fn get_band_name(&self, html_dom: &str) -> String {
        let split = html_dom
            .split('>')
            // not sure why clippy disagrees; I must skip 1?
            .skip(1)
            .next()
            .unwrap()
            .split("\n")
            .next()
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>()
            .join(" ");
        split.to_string()
    }

    pub fn get_purchased_items(&self) -> Vec<CollectionItem> {
        // let selector =
        //     Selector::parse(r#"li[class="collection-item-container"]"#).unwrap();
        let selector_details =
            Selector::parse(r#"div[class="collection-title-details"] a"#).unwrap();
        let selector_download_links = Selector::parse(r#"span[class="redownload-item"]"#).unwrap();
        let mut selection_details = self.html.select(&selector_details);
        let selection_download_links = self.html.select(&selector_download_links);

        let mut items: Vec<CollectionItem> = vec![];

        for child in selection_details {
            items.push(CollectionItem::new());
            for (i, child) in child.child_elements().enumerate() {
                match i {
                    1 => items
                        .last()
                        .unwrap()
                        .set_name(&self.get_band_name(&child.html())),
                    2 => (),
                    _ => (),
                };
            }
        }

        println!("{:?}", items);

        // selection_details.for_each(|elem| {
        //     println!("{:?}", elem.first_child().unwrap());
        // });

        // for s in selection {
        //     println!("yeah we iterate mothafuckaaa");
        //     println!("{:?}", s);
        // }

        // println!("why da fuck len 0?? {}", selection.);

        vec![]
    }
}
