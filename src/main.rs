use api::bandcamp_api::BandcampAPI;
use services::collection_page_scraper::CollectionPageScraper;
use std::env;

mod api;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), ()> {
    if env::args().len() < 4 {
        println!("error: not enough arguments.");
        println!("args:\n\t1: username\n\t2: identity cookie\n\t3: download path");
        return Err(());
    }

    let mut username: String = String::new();
    let mut identity: String = String::new();
    let mut download_path: String = String::new();
    env::args().enumerate().for_each(|(i, arg)| {
        match i {
            1 => username = arg,
            2 => identity = arg,
            3 => download_path = arg,
            // ...how do i ignore this altogether?
            _ => (),
        }
    });

    let api = BandcampAPI::new(&username, &identity);
    let scraper = CollectionPageScraper::new(
        &api.get_collection_summary_html()
            .await
            .expect("failed to get collection summary html")
            .text()
            .await
            .expect("failed to parse HTML DOM into text"),
    );
    scraper.get_purchased_items();

    Ok(())
}
