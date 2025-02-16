use std::{fs, path::Path};

pub struct Files {
    path: String,
}

impl Files {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    fn create_directory(&self, path: &str) {
        if let Err(e) = fs::create_dir(path) {
            // TODO: handle this gracefully
            panic!("unable to create directory at {path}! error == {e}");
        }
    }

    pub fn get_artist_folder(&self, artist: &str) -> String {
        let path_binding = format!("{}/{}", self.path, artist);
        let path = Path::new(&path_binding);
        if !path.exists() {
            self.create_directory(path.to_str().unwrap());
        }

        path_binding
    }

    pub fn get_artist_album_folder(&self, artist: &str, album: &str) -> String {
        let path_binding = format!("{}/{}", self.get_artist_folder(artist), album);
        let path = Path::new(&path_binding);
        if !path.exists() {
            self.create_directory(path.to_str().unwrap());
        }

        path_binding
    }

    pub fn get_artist_subdirectories(&self, artist: &str) -> Vec<String> {
        let entries =
            fs::read_dir(self.get_artist_folder(artist)).expect("failed to read artist directory!");

        let mut subdirs = vec![];
        // TODO: i'm sure there's a fancy functional way of doing this
        entries.for_each(|elem| {
            if let Ok(elem) = elem {
                if elem.path().is_dir() {
                    subdirs.push(elem.file_name().into_string().unwrap());
                }
            }
        });

        subdirs
    }
}
