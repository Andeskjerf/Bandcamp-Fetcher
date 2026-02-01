use api::bandcamp_api::BandcampAPI;
use log::error;
use services::files::Files;
use simplelog::{Config, TermLogger};
use std::{collections::HashMap, env};

use crate::{
    models::api::collection_items::CollectionItems,
    services::{page_data_extractor::PageDataExtractor, sanitizer::Sanitizer},
};

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
    let collection_page = api
        .get_collection_summary_html()
        .expect("Failed to get collection summary HTML!")
        .text()
        .expect("Failed to parse HTML DOM into text!");
    let files = Files::new(&download_path);

    log::info!("getting purchased items for user {}...", username);

    let pagedata_extractor = PageDataExtractor::new(&collection_page);
    let fan = pagedata_extractor.get_fan().unwrap();
    let collection_data = pagedata_extractor.get_collection_data().unwrap();

    let collection_items: CollectionItems = api
        .get_collection_items_json(
            fan.id(),
            collection_data.last_token(),
            collection_data.item_count() - collection_data.batch_size(),
        )
        .unwrap()
        .json()
        .unwrap();

    let mut urls = collection_data.redownload_urls().clone();
    urls.extend(collection_items.redownload_urls().clone());

    let mut artist_subdirs: HashMap<String, Vec<String>> = HashMap::new();
    log::info!("found {} items!", urls.len());

    let mut did_something = false;

    let sanitizer = Sanitizer::new();
    for (_, url) in urls.iter_mut() {
        // TODO: handle errors gracefully
        // FIXME: we end up querying Bandcamp for every URL we find, even if we don't need to download an item...
        // this could end up becoming a lot of queries
        // imagine if this runs every 5 minutes on a collection that has some number of thousand items...
        let html = api
            .get_download_page_html(url)
            .expect("failed to get download page html")
            .text()
            .expect("failed to parse download page HTML into text");

        let download_page_extractor = PageDataExtractor::new(&html);

        let items = download_page_extractor.get_digital_items();
        let first_item = items.as_ref().unwrap().iter().next();
        if items.is_none() || first_item.is_none() {
            error!("Failed to get any download items!");
            continue;
        }
        let item = first_item.unwrap();

        let dirs = artist_subdirs
            .entry(item.artist.clone())
            .or_insert_with(|| files.get_artist_subdirectories(&item.artist));

        let sanitized_title = sanitizer.sanitize_path(&item.title);

        if !dirs.contains(&sanitized_title) {
            did_something = true;
            log::info!(
                "'{}' by '{}' not found on filesystem",
                sanitized_title,
                item.artist
            );

            let album_dir = files.get_artist_album_folder(&item.artist, &sanitized_title);
            // TODO: the user should be able to pick their preferred encodings
            // maybe multiple choices, so we can pick the next best option and so on?
            let bandcamp_format = item
                .get_format_by_encoding("flac") // try to get flac if possible, will fallback to something else if it fails
                .expect("failed to get encoding for album!");

            log::info!(
                "downloading '{}' by '{}' with '{}' encoding ({})...",
                sanitized_title,
                item.artist,
                bandcamp_format.encoding_name(),
                bandcamp_format.size_mb(),
            );
            let file_path = api.download_file(&bandcamp_format.url(), &album_dir);

            // singles do not come in zips
            if file_path.contains(".zip") {
                log::info!(
                    "unzipping '{}' by '{}' to {}...",
                    sanitized_title,
                    item.artist,
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
