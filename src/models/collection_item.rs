pub struct CollectionItem {
    band: String,
    // album, single, EP, etc
    name: String,
    download_link: String,
}

impl CollectionItem {
    pub fn new(band: &str, name: &str, download_link: &str) -> Self {
        Self {
            band: band.to_string(),
            name: name.to_string(),
            download_link: download_link.to_string(),
        }
    }
}
