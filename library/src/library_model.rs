use std::fs;
use std::path::{Path, PathBuf};
use std::time::*;

use db_wrapper::sqlitedb::librarydb::LibraryDBConn;
use library_types::*;
use uuid::Uuid;
use crate::book_parser::{get_uuid, parse_book};
use crate::library_io::{create_thumbnails, scan_dir};

pub struct LibraryModel {
    db: LibraryDBConn,
    pub uuid: String,
    pub path: String,
    meta_path: String,
}

impl LibraryModel {
    pub fn open(uuid: &str, path: &str) -> Self {
	let meta_path = format!("{path}/.bookrium");
	fs::create_dir_all(&meta_path).unwrap();
	let db_path = format!("{meta_path}/library.db");
	Self {
	    db: LibraryDBConn::new(db_path.as_str()),
	    uuid: uuid.to_string(),
	    path: path.to_string(),
	    meta_path,
	}
    }

    pub fn get_books(&self, folder_uuid: &str) -> Books {
	self.db.get_books(folder_uuid)
    }

    pub fn get_book_path(&self, book_uuid: &str) -> String {
	self.db.get_book_path(book_uuid, &self.path)
    }

    pub fn get_dirs(&self, parent_uuid: &str) -> Dirs {
	self.db.get_dirs(parent_uuid)
    }

    pub fn scan_lib(&self, path: &str) {
	let scan_path = PathBuf::from(path);
	self.db.clear_dirs();
	self.scan_lib_aux(scan_path, "root");
    }

    fn scan_lib_aux(&self, scan_path: PathBuf, parent_uuid: &str) {
	let dir = scan_dir(&scan_path);

	for dir in dir.0 {
	    let uuid = Uuid::new_v4().to_string();
	    let name = dir.file_name().unwrap().to_str().unwrap();

	    self.db.insert_dir(uuid.as_str(), name, parent_uuid);
	    self.scan_lib_aux(dir, uuid.as_str());
	}

	for file in dir.1 { self.scan_book(file, parent_uuid); }
    }

    fn scan_book(&self, file: PathBuf, parent_uuid: &str) {
	let file_name 		= file.file_name().unwrap().to_str().unwrap();
	let existing_uuid 	= self.db.get_book_uuid(file_name);

	if let Some(uuid) = existing_uuid {
	    self.db.insert_book_dir(&uuid, parent_uuid);
	    return;
	}

	if let Some(uuid) = get_uuid(file.as_path()) {
	    if self.db.book_exists(&uuid) {
		self.db.insert_book_dir(&uuid, parent_uuid);
		return;
	    }
	}

	let book_res = parse_book(&file, parent_uuid);
	if book_res.is_none() { return; }
	let book = book_res.unwrap();
	let scan_timestamp = SystemTime::now().duration_since(
	    UNIX_EPOCH).unwrap().as_secs();
	self.db.insert_book(&book.0, scan_timestamp);
	self.db.insert_book_dir(&book.0.book.uuid, parent_uuid);
	if  book.1.is_some() {
	    let cover = book.1.unwrap();
	    let output_path = format!("{}/{}", self.meta_path, book.0.book.uuid);
	    std::fs::create_dir_all(&output_path).unwrap();
	    create_thumbnails(output_path, cover);
	}
    }
}

impl LibraryModel {
    pub fn set_pos(&self, uuid: &str, position: &str) {
	self.db.set_pos(uuid, position);
    }

    pub fn get_pos(&self, uuid: &str) -> String {
	self.db.get_pos(uuid)
    }

    pub fn get_cover_path (&self, book_uuid: &str) -> Option<String> {
	let path_str = format!("{}/{book_uuid}/256.jpg", self.meta_path);
	println!("get_cover_path: {}", path_str);
	let path = Path::new(&path_str);
	if !path.exists() {return None;}
	Some(path.to_str().unwrap().to_string())
    }
}

