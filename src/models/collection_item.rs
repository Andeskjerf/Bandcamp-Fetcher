#[derive(Debug, Clone)]
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

    pub fn with_params(band: &str, name: &str, download_link: &str) -> Self {
        Self {
            band: band.to_string(),
            name: name.to_string(),
            download_link: download_link.to_string(),
        }
    }

    pub fn band(&self) -> String {
        self.band.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn download_link(&self) -> String {
        self.download_link.clone()
    }

    pub fn set_band(&mut self, band: &str) {
        self.band = band.to_string();
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_download_link(&mut self, download_link: &str) {
        self.download_link = download_link.to_string();
    }
}
