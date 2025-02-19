use std::{fs, io::Cursor, path::Path};

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

    // does it make sense to have this function here?
    pub fn download_zip(&self, url: &str, path: &str) -> Result<u64, std::io::Error> {
        let response = reqwest::blocking::get(url).expect("unable to make request!");
        let bytes = response.bytes().expect("unable to parse request as bytes!");

        let mut file = std::fs::File::create(path).expect("unable to create file for download!");
        let mut content = Cursor::new(bytes);

        // copy request bytes into file
        std::io::copy(&mut content, &mut file)
    }

    pub fn unzip_archive(&self, zip_path: &str, dest_path: &str) {
        let mut zip_archive = zip::ZipArchive::new(
            std::fs::File::open(zip_path).expect("failed to open zip file at specified path!"),
        )
        .expect("failed to open file as a zip archive!");

        for i in 0..zip_archive.len() {
            let mut file = zip_archive.by_index(i).unwrap();
            println!("filename: {}", file.name());
        }
    }
}
