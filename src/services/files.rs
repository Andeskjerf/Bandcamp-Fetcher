use std::{
    env, fs,
    path::{Path, PathBuf},
};

use super::sanitizer::Sanitizer;

pub struct Files {
    path: String,
    sanitizer: Sanitizer,
}

impl Files {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            sanitizer: Sanitizer::new(),
        }
    }

    fn expand_tilde(path: &str) -> PathBuf {
        if let Some(end) = path.strip_prefix("~/") {
            let home = env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(end)
        } else {
            PathBuf::from(path)
        }
    }

    fn create_directory(&self, path: &str) {
        if let Err(e) = fs::create_dir(path) {
            // TODO: handle this gracefully
            panic!("unable to create directory at {path}! error == {e}");
        }
    }

    pub fn get_artist_folder(&self, artist: &str) -> String {
        let path_binding = format!("{}{}", self.path, self.sanitizer.sanitize_path(artist));
        let path = Files::expand_tilde(&path_binding);
        if !path.exists() {
            self.create_directory(path.to_str().unwrap());
        }

        String::from(path.to_str().unwrap())
    }

    pub fn get_artist_album_folder(&self, artist: &str, album: &str) -> String {
        // we do need to sanitize the artist name, but we do it in the other function
        // also sanitize the album name for good measure
        let path_binding = format!(
            "{}/{}",
            self.get_artist_folder(artist),
            self.sanitizer.sanitize_path(album)
        );
        let path = Files::expand_tilde(&path_binding);
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

    pub fn unzip_archive(&self, zip_path: &str) {
        let mut zip_archive = zip::ZipArchive::new(
            std::fs::File::open(zip_path).expect("failed to open zip file at specified path!"),
        )
        .expect("failed to open file as a zip archive!");

        let mut split = zip_path.split('/').collect::<Vec<&str>>();
        split.pop().expect("unable to pop zip_path");
        let folder_to_extract_to = split.join("/");

        for i in 0..zip_archive.len() {
            let mut song = zip_archive.by_index(i).unwrap();
            let mut file =
                std::fs::File::create(format!("{}/{}", folder_to_extract_to, song.name()))
                    .expect("unable to create file for download!");
            std::io::copy(&mut song, &mut file).expect("failed to copy zip file to path");
        }
    }
}
