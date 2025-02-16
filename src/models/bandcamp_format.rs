use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BandcampFormat {
    description: String,
    encoding_name: String,
    size_mb: String,
    url: String,
}
