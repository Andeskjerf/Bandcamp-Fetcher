use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BandcampFormat {
    description: String,
    encoding_name: String,
    size_mb: String,
    url: String,
}
