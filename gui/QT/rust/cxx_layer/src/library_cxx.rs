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
		let cover_path = library.get_cover_path(file.uuid.as_str());
		media_files.push(CXXBook {
	    uuid: file.uuid, 	title: file.title, 	progress: file.progress,
		cover_path,
	});
    }
    media_files
}

fn scan_library(uuid: &str, path: &str) {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(uuid).unwrap();
    library.scan_lib(path);
}

fn set_media_position(library_uuid: &str, file_uuid: &str, position: &str, progress: u8) {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    library.set_pos(file_uuid, position, progress);
}

fn get_media_position(library_uuid: &str, file_uuid: &str) -> String {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
    library.get_pos(file_uuid)
}

fn get_dirs(library_uuid: &str, parent_uuid: &str) -> Vec<Dir> {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(library_uuid).unwrap();
	library.get_dirs(parent_uuid).into_iter().map(|dir| {
		let cover_path = library.get_container_cover_path(dir.uuid.as_str());
		Dir { uuid: dir.uuid, name: dir.name, cover_path, }
	}).collect()
}

fn get_book_path(uuid: &str, book_uuid: &str) -> String {
    let mut library_lock = LIBRARIES.lock().unwrap();
    let library = library_lock.get_mut(uuid).unwrap();
    library.get_book_path(book_uuid)
}

fn get_book_toc(library_uuid: &str, book_uuid: &str) -> Vec<Nav>{
	let mut library_lock = LIBRARIES.lock().unwrap();
	let library = library_lock.get_mut(library_uuid).unwrap();
	let contents = library.get_book_toc(book_uuid);
	contents.into_iter().map(|nav| {
		Nav {name: nav.name, href: nav.href}
	}).collect()
}

fn delete_book(library_uuid: &str, book_uuid: &str) -> String {
	let mut library_lock = LIBRARIES.lock().unwrap();
	let library = library_lock.get_mut(library_uuid).unwrap();
	match library.delete_book(book_uuid) {
		Ok(_) => {},
		Err(err) => {return format!{"Error deleting book: {}", err};}
	}
}

fn delete_dir(library_uuid: &str, dir_uuid: &str) -> String {
	let mut library_lock = LIBRARIES.lock().unwrap();
	let library = library_lock.get_mut(library_uuid).unwrap();
	library.delete_dir(dir_uuid)
}

#[cxx::bridge]
mod library_ffi {
    pub struct CXXBook {
	pub uuid: String, pub title: String, pub progress: u8, pub cover_path: String,
    }

    pub struct Dir { pub uuid: String, pub name: String, pub cover_path: String, }
	pub struct Nav { pub name: String, pub href: String, }

    extern "Rust" {
	fn get_media_files(uuid: &str, folder_uuid: &str) -> Vec<CXXBook>;
	fn get_dirs(library_uuid: &str, parent_uuid: &str) -> Vec<Dir>;
	fn scan_library(uuid: &str, path: &str);
	fn open_library(uuid: &str, path: &str);
	fn get_book_path(library_uuid: &str, book_uuid: &str) -> String;
	fn get_book_toc(library_uuid: &str, book_uuid: &str) -> Vec<Nav>;
	fn set_media_position(library_uuid: &str, file_uuid: &str, position: &str, progress: u8);
	fn get_media_position(library_uuid: &str, file_uuid: &str) -> String;
    fn delete_book(library_uuid: &str, book_uuid: &str) -> String;
		fn delete_dir(library_uuid: &str, dir_uuid: &str) -> String;
	}
}
