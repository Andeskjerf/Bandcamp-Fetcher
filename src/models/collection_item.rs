#[derive(Debug)]
pub struct CollectionItem {
    band: String,
    // album, single, EP, etc
    name: String,
    download_link: String,
}

impl CollectionItem {
    pub fn new() -> Self {
        Self {
            band: String::new(),
            name: String::new(),
            download_link: String::new(),
        }
    }

    pub fn set_band(&mut self, band: &str) {
        self.band = band.to_string();
    }

    pub fn set_name(&mut self, name: &str) {
        self.band = name.to_string();
    }

    pub fn set_download_link(&mut self, download_link: &str) {
        self.band = download_link.to_string();
    }
}
