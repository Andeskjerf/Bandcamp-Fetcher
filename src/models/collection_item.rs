use std::collections::HashMap;

use super::bandcamp_format::BandcampFormat;

#[derive(Debug, Clone)]
pub struct CollectionItem {
    band: String,
    // album, single, EP, etc
    name: String,
    download_link: String,
    formats: HashMap<String, BandcampFormat>,
}

impl CollectionItem {
    pub fn new() -> Self {
        Self {
            band: String::new(),
            name: String::new(),
            download_link: String::new(),
            formats: HashMap::new(),
        }
    }

    pub fn with_params(band: &str, name: &str, download_link: &str) -> Self {
        Self {
            band: band.to_string(),
            name: name.to_string(),
            download_link: download_link.to_string(),
            formats: HashMap::new(),
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

    pub fn set_formats(&mut self, formats: HashMap<String, BandcampFormat>) {
        self.formats = formats;
    }

    pub fn formats(&self) -> HashMap<String, BandcampFormat> {
        self.formats.clone()
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

    pub fn get_format_by_encoding(&self, encoding: &str) -> Option<BandcampFormat> {
        self.formats
            .get(encoding)
            .or_else(|| {
                Some(
                    self.formats
                        .values()
                        .next()
                        .expect("failed to get fallback encoding, none exists!"),
                )
            })
            .cloned()
    }
}
