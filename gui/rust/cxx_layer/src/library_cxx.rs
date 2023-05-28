use lazy_static::lazy_static;
use crate::library_cxx::library_ffi::MediaFile;
use std::sync::Mutex;
use file_io::file_io::LIBRARY_DIR;
use std::path::Path;
use tokio::runtime::Runtime;
use library::model::LibraryModel;
use std::collections::HashMap;
lazy_static!(
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref LIBRARIES: Mutex<HashMap<String, LibraryModel>> = Mutex::new(HashMap::new());
);

fn open_library(uuid: &str) {
    let mut library = LIBRARIES.lock().unwrap();
    library.insert(uuid.to_string(), LibraryModel::open(uuid));
}

fn get_media_files(uuid: &str) -> Vec<MediaFile>{
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(uuid).unwrap();
    let files = RUNTIME.block_on(library.fetch_files());

    let mut media_files = Vec::new();
    for file in files {
        media_files.push(MediaFile{ uuid: file.uuid, path: file.path, });
    }
    media_files
}

fn scan_library(uuid: &str, path: &str) {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(uuid).unwrap();
    RUNTIME.block_on(library.scan_library(path));
}

fn set_media_position(library_uuid: &str, file_uuid: &str, position: &str) {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    RUNTIME.block_on(library.set_media_position(file_uuid, position));
}

fn get_media_position(library_uuid: &str, file_uuid: &str) -> String {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    RUNTIME.block_on(library.get_media_position(file_uuid))
}

fn has_cover(library_uuid: &str, file_uuid: &str) -> bool {
    Path::new(format!("{}/{}/{}/thumbnails", LIBRARY_DIR.as_str(), library_uuid, file_uuid).as_str()).exists()
}

fn get_cover_path(library_uuid: &str, file_uuid: &str) -> String {
    format!("{}/{}/{}/thumbnails/256.jpg", LIBRARY_DIR.as_str(), library_uuid, file_uuid)
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
        fn set_media_position(library_uuid: &str, file_uuid: &str, position: &str);
        fn get_media_position(library_uuid: &str, file_uuid: &str) -> String;
        fn has_cover(library_uuid: &str, file_uuid: &str) -> bool;
        fn get_cover_path(library_uuid: &str, file_uuid: &str) -> String;
    }
}
