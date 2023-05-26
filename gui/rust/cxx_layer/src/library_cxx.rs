use lazy_static::lazy_static;
use crate::library_cxx::library_ffi::MediaFile;
use library_app::LibraryApp;
use std::sync::Mutex;
use file_io::client_file_io::LIBRARY_DIR;

lazy_static!(
    static ref LIBRARY_APP: Mutex<LibraryApp> = Mutex::new(LibraryApp::new());
);

fn open_library(uuid: &str) {
    println!("open_library with uuid: {}", uuid);
    let mut library_app = LIBRARY_APP.lock().unwrap();
    library_app.open(uuid, "url");
}

fn get_media_files(uuid: &str) -> Vec<MediaFile>{
    println!("get_media_files from uuid: {}", uuid);
    let mut library_app = LIBRARY_APP.lock().unwrap();
    let files = library_app.get_files(uuid).unwrap();
    let mut media_files = Vec::new();
    for file in files {
        media_files.push(MediaFile{
            uuid: file.uuid,
            path: file.path,
        });
    }
    media_files
}

fn has_cover(library_uuid: &str, file_uuid: &str) -> bool {
    std::path::Path::new(format!("{}/{}/{}/thumbnails", LIBRARY_DIR.as_str(), library_uuid, file_uuid).as_str()).exists()
}

fn get_cover_path(library_uuid: &str, file_uuid: &str) -> String {
    format!("{}/{}/{}/thumbnails/256.jpg", LIBRARY_DIR.as_str(), library_uuid, file_uuid)
}

fn scan_library(uuid: &str, path: &str) {
    let mut library_app = LIBRARY_APP.lock().unwrap();
    library_app.add_files(uuid, path).unwrap();
}

#[cxx::bridge]
mod library_ffi {

    pub struct MediaFile{
        pub uuid: String,

        pub path: String,
    }
    extern "Rust" {

        fn get_media_files(uuid: &str) -> Vec<MediaFile>;
        fn scan_library(uuid: &str, path: &str) ;
        fn open_library(uuid: &str);
        fn has_cover(library_uuid: &str, file_uuid: &str) -> bool;
        fn get_cover_path(library_uuid: &str, file_uuid: &str) -> String;
    }
}
