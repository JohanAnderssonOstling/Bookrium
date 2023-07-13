use lazy_static::lazy_static;
use crate::library_cxx::library_ffi::*;
use std::sync::Mutex;
use file_io::file_io::LIBRARY_DIR;
use std::path::Path;
use library::model::LibraryModel;
use std::collections::HashMap;
use library_types::Nav;
lazy_static!(
    static ref LIBRARIES: Mutex<HashMap<String, LibraryModel>> = Mutex::new(HashMap::new());
);

fn open_library(uuid: &str) {
	let mut library = LIBRARIES.lock().unwrap();
	library.insert(uuid.to_string(), LibraryModel::open(uuid));
}

fn get_media_files(uuid: &str) -> Vec<MediaFile> {
	let mut library_lock = LIBRARIES.lock().unwrap();
	let library = library_lock.get_mut(uuid).unwrap();
	let files = library.get_books();

	let mut media_files = Vec::new();
	for file in files {
		media_files.push(MediaFile {
			uuid: file.uuid,
			path: file.path.to_str().unwrap().to_string(),
			title: file.title,
			description: file.desc,
			navigation: convert_navigation(file.contents),
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
	//(library.set_media_position(file_uuid, position));
}

fn get_media_position(library_uuid: &str, file_uuid: &str) -> String {
	let mut library_lock = LIBRARIES.lock().unwrap();
	let library = library_lock.get_mut(library_uuid).unwrap();
	//library.get_media_position(file_uuid)
  "hfdg".into()
}

fn has_cover(library_uuid: &str, file_uuid: &str) -> bool {
	Path::new(format!("{}/{}/{}/thumbnails", LIBRARY_DIR.as_str(), library_uuid, file_uuid).as_str()).exists()
}

fn get_cover_path(library_uuid: &str, file_uuid: &str) -> String {
	format!("{}/{}/{}/thumbnails/256.jpg", LIBRARY_DIR.as_str(), library_uuid, file_uuid)
}

fn convert_navigation(nav: Vec<Nav>) -> Vec<Navigation> {
	let mut nav_points_ffi = Vec::new();
	for nav_point in nav {
		nav_points_ffi.push(Navigation {
			name: nav_point.name,
			href: nav_point.href,
			childs: convert_navigation(nav_point.childs),
		});
	}
	nav_points_ffi
}

#[cxx::bridge]
mod library_ffi {
	pub struct MediaFile {
		pub uuid: String,
		pub path: String,
		pub title: String,
		pub description: String,
		pub navigation: Vec<Navigation>,
	}

	pub struct Navigation {
		pub name: String,
		pub href: String,
		pub childs: Vec<Navigation>,
	}

	extern "Rust" {
		fn get_media_files(uuid: &str) -> Vec<MediaFile>;
		fn scan_library(uuid: &str, path: &str);
		fn open_library(uuid: &str);
		fn set_media_position(library_uuid: &str, file_uuid: &str, position: &str);
		fn get_media_position(library_uuid: &str, file_uuid: &str) -> String;
		fn has_cover(library_uuid: &str, file_uuid: &str) -> bool;
		fn get_cover_path(library_uuid: &str, file_uuid: &str) -> String;
	}
}
