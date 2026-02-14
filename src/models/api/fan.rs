use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// The representation of a user in Bandcamp
pub struct Fan {
    id: u64,
    username: String,
    name: String,
    photo: Option<String>,
    private: bool,
    verified: bool,
    url: String,
}

impl Fan {
    pub fn id(&self) -> u64 {
        self.id
    }
}
