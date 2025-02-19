use api::bandcamp_api::BandcampAPI;
use services::{
    collection_page_scraper::CollectionPageScraper, download_page_scraper::DownloadPageScraper,
    files::Files,
};
use simplelog::{Config, TermLogger};
use std::{collections::HashMap, env};

mod api;
mod models;
mod services;

fn main() -> Result<(), ()> {
    TermLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("failed to init logger!");

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
            .expect("failed to get collection summary html")
            .text()
            .expect("failed to parse HTML DOM into text"),
    );

    log::info!("getting purchased items for user {}...", username);
    let mut items = scraper.get_purchased_items();
    let mut artist_subdirs: HashMap<String, Vec<String>> = HashMap::new();
    log::info!("found {} items!", items.len());

    let mut did_something = false;

    // TODO: this ain't pretty. too many things happening
    // this should give us a collection of direct URLs to get our zips from
    for item in items.iter_mut() {
        did_something = true;

        // TODO: handle errors gracefully
        let html = api
            .get_download_page_html(&item.download_link())
            .expect("failed to get download page html")
            .text()
            .expect("failed to parse download page HTML into text");

        let scraper = DownloadPageScraper::new(&html);
        item.set_formats(scraper.get_download_formats());

        let dirs = artist_subdirs
            .entry(item.band())
            .or_insert_with(|| files.get_artist_subdirectories(&item.band()));

        if !dirs.iter().any(|elem| item.name() == *elem) {
            log::info!(
                "'{}' by '{}' not found on filesystem",
                item.name(),
                item.band()
            );

            let album_dir = files.get_artist_album_folder(&item.band(), &item.name());
            // TODO: the user should be able to pick their preferred encodings
            // maybe multiple choices, so we can pick the next best option and so on?
            let bandcamp_format = item
                .get_format_by_encoding("flac") // try to get flac if possible, will fallback to something else if it fails
                .expect("failed to get encoding for album!");

            log::info!(
                "downloading album archive '{}' by '{}' with '{}' encoding ({})...",
                item.name(),
                item.band(),
                bandcamp_format.encoding_name(),
                bandcamp_format.size_mb(),
            );
            let file_path = api.download_file(&bandcamp_format.url(), &album_dir);

            // singles do not come in zips
            if file_path.contains(".zip") {
                log::info!(
                    "unzipping '{}' by '{}' to {}...",
                    item.name(),
                    item.band(),
                    album_dir
                );
                files.unzip_archive(&file_path);
                std::fs::remove_file(&file_path)
                    .expect("failed to remove archive after extraction");
            }
        }
    }

    if !did_something {
        log::info!("nothing to do, no new items since last run")
    }

    Ok(())
}
