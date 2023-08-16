use lazy_static::lazy_static;
use crate::library_cxx::library_ffi::*;
use std::sync::Mutex;
use std::path::Path;
use library::library_model::LibraryModel;
use std::collections::HashMap;
lazy_static!(
    static ref LIBRARIES: Mutex<HashMap<String, LibraryModel>> = Mutex::new(HashMap::new());
);

fn open_library(uuid: &str, path: &str) {
    let mut library = LIBRARIES.lock().unwrap();
    library.insert(uuid.to_string(), LibraryModel::open(uuid, path));
}

fn get_media_files(uuid: &str, folder_uuid: &str) -> Vec<CXXBook> {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(uuid).unwrap();
    let files = library.get_books(folder_uuid);
    let mut media_files = Vec::new();

    for file in files {
	media_files.push(CXXBook {
	    uuid: file.uuid,
	    title: file.title,
	    progress: file.progress,

	});
    }
    media_files
}

fn scan_library(uuid: &str, path: &str) {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(uuid).unwrap();
    library.scan_lib(path);
}

fn set_media_position(library_uuid: &str, file_uuid: &str, position: &str) {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    (library.set_pos(file_uuid, position));
}

fn get_media_position(library_uuid: &str, file_uuid: &str) -> String {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    library.get_pos(file_uuid)
}

fn has_cover(library_uuid: &str, file_uuid: &str) -> bool {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    library.get_cover_path(file_uuid).is_some()
}

fn get_cover_path(library_uuid: &str, file_uuid: &str) -> String {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    let cover_path = library.get_cover_path(file_uuid);
    if cover_path.is_none () { return "".into(); }
    cover_path.unwrap()
}

fn get_container_cover_path(library_uuid: &str, container_uuid: &str) -> String {
	let mut library_lock = LIBRARIES.lock().unwrap();
	let library = library_lock.get_mut(library_uuid).unwrap();
	let cover_path = library.get_container_cover_path(container_uuid);
	if cover_path.is_none () { return "".into(); }
	cover_path.unwrap()
}

fn convert_dir(dirs: Vec<library_types::Dir>) -> Vec<Dir> {
    dirs.into_iter().map(|dir| Dir {
	uuid: dir.uuid,
	name: dir.name,
    }).collect()
}

fn get_dirs(library_uuid: &str, parent_uuid: &str) -> Vec<Dir> {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    convert_dir(library.get_dirs(parent_uuid))
}

fn get_book_path(uuid: &str, book_uuid: &str) -> String {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(uuid).unwrap();
    library.get_book_path(book_uuid)
}

#[cxx::bridge]
mod library_ffi {
    pub struct CXXBook {
	pub uuid: String, pub title: String, pub progress: u8,
    }

    pub struct Dir {
	pub uuid: String, pub name: String,
    }

    extern "Rust" {
	fn get_media_files(uuid: &str, folder_uuid: &str) -> Vec<CXXBook>;
	fn get_dirs(library_uuid: &str, parent_uuid: &str) -> Vec<Dir>;
	fn scan_library(uuid: &str, path: &str);
	fn open_library(uuid: &str, path: &str);
	fn get_book_path(library_uuid: &str, book_uuid: &str) -> String;
	fn set_media_position(library_uuid: &str, file_uuid: &str, position: &str);
	fn get_media_position(library_uuid: &str, file_uuid: &str) -> String;
	fn has_cover(library_uuid: &str, file_uuid: &str) -> bool;
	fn get_cover_path(library_uuid: &str, file_uuid: &str) -> String;
	fn get_container_cover_path(library_uuid: &str, container_uuid: &str) -> String;
    }
}
