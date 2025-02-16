use api::bandcamp_api::BandcampAPI;
use services::{
    collection_page_scraper::CollectionPageScraper, download_page_scraper::DownloadPageScraper,
    files::Files,
};
use std::{collections::HashMap, env};

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
    env::args().enumerate().for_each(|(i, arg)| match i {
        1 => username = arg,
        2 => identity = arg,
        3 => download_path = arg,
        _ => (),
    });

    let api = BandcampAPI::new(&username, &identity);
    let files = Files::new(&download_path);
    let scraper = CollectionPageScraper::new(
        // TODO: handle errors gracefully
        &api.get_collection_summary_html()
            .await
            .expect("failed to get collection summary html")
            .text()
            .await
            .expect("failed to parse HTML DOM into text"),
    );

    let mut items = scraper.get_purchased_items();
    let mut artist_subdirs: HashMap<String, Vec<String>> = HashMap::new();

    // this should give us a collection of direct URLs to get our zips from
    for item in items.iter_mut() {
        // TODO: handle errors gracefully
        let html = api
            .get_download_page_html(&item.download_link())
            .await
            .expect("failed to get download page html")
            .text()
            .await
            .expect("failed to parse download page HTML into text");

        let scraper = DownloadPageScraper::new(&html);
        item.set_formats(scraper.get_download_formats());

        let dirs = artist_subdirs
            .entry(item.band())
            .or_insert_with(|| files.get_artist_subdirectories(&item.band()));

        if !dirs.iter().any(|elem| item.name() == *elem) {
            let album_dir = files.get_artist_album_folder(&item.band(), &item.name());
        }
    }

    println!("{:?}", items.first().unwrap().formats());

    Ok(())
}
