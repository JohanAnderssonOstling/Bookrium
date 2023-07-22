use std::path::PathBuf;
use std::time::SystemTime;

use db_wrapper::sqlitedb::librarydb::LibraryDBConn;
use file_io::file_io::*;
use file_io::folder_scanner::scan_dir;
use file_io::media_parser::{get_uuid, parse_book};
use library_types::*;
use uuid::Uuid;

pub struct LibraryModel {
    db: LibraryDBConn,
    pub uuid: String,
    pub path: String,
}

impl LibraryModel {
    pub fn open(uuid: &str, path: &str) -> Self {
	Self {
	    db: LibraryDBConn::new(uuid),
	    uuid: uuid.to_string(),
	    path: path.to_string(),
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

	for file in dir.1 {
	    self.scan_book(file, parent_uuid);
	}
    }

    fn scan_book(&self, file: PathBuf, parent_uuid: &str) {
	let file_name = file.file_name().unwrap().to_str().unwrap();
	let existing_uuid = self.db.get_book_uuid(file_name);

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

	let book = parse_book(&file, parent_uuid);
	let scan_timestamp = SystemTime::now().duration_since(
	    SystemTime::UNIX_EPOCH).unwrap().as_secs();
	self.db.insert_book(&book.0, scan_timestamp);
	self.db.insert_book_dir(&book.0.book.uuid, parent_uuid);
	if book.1.is_some() {
	    let cover = book.1.unwrap();
	    create_thumbs(&self.uuid, &book.0.book.uuid, cover);
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
}

