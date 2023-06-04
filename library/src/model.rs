use std::path::PathBuf;

use db_wrapper::sqlitedb::librarydb::LibraryDBConn;
use file_io::file_io::*;
use file_io::folder_scanner::scan_dir;
use file_io::media_parser::parse_media;
use library_types::*;

pub struct LibraryModel {
	db: LibraryDBConn,
	pub uuid: String,
}

impl LibraryModel {
	pub fn open(uuid: &str) -> Self {

		Self { db: LibraryDBConn::new(uuid), uuid: uuid.to_string() }
	}

	fn add_file(&self, file: &Book) {
		self.db.insert_media(file);
	}

	pub fn fetch_files(&self) -> Vec<Book> {
		self.db.fetch_books("root")
	}

	pub fn scan_library(&self, path: &str) {
		//self.db.clear_library().await.unwrap();
		let scan_path = PathBuf::from(path);
		self.scan_library_aux(scan_path, "root");
	}

	fn scan_library_aux(&self, scan_path: PathBuf, parent_uuid: &str) {
		let scanned_dir: (Vec<PathBuf>, Vec<PathBuf>) = scan_dir(scan_path);

		for dir in scanned_dir.0 {
			let dir_file = parse_media(&dir, parent_uuid).0;
			self.add_file(&dir_file);
			self.scan_library_aux(dir, &dir_file.uuid);
		}

		for file in scanned_dir.1 {
			let parsed_file = parse_media(&file, parent_uuid);
			let file = parsed_file.0;
			self.add_file(&file);
			if parsed_file.1.is_some() {
				create_thumbnails_raw(&self.uuid, &file.uuid, parsed_file.1.unwrap());
			}
		}
	}
}

impl LibraryModel {
	pub fn set_media_position(&self, uuid: &str, position: &str) {
		self.db.set_media_position(uuid, position);
	}

	pub fn get_media_position(&self, uuid: &str) -> String {
		self.db.select_media_position(uuid)
	}
}
