use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BandcampFormat {
    description: String,
    encoding_name: String,
    size_mb: String,
    url: String,
}

impl BandcampFormat {
    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn encoding_name(&self) -> &str {
        &self.encoding_name
    }

    pub fn size_mb(&self) -> &str {
        &self.size_mb
    }
}
