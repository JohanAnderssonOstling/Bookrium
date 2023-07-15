use std::path::PathBuf;

use db_wrapper::sqlitedb::librarydb::LibraryDBConn;
use file_io::file_io::*;
use file_io::folder_scanner::scan_dir;
use file_io::media_parser::parse_book;
use library_types::*;
use uuid::Uuid;

pub struct LibraryModel {
  db: LibraryDBConn,
  pub uuid: String,
}

impl LibraryModel {
  pub fn open (uuid: &str) -> Self {
    Self { db: LibraryDBConn::new(uuid),
      uuid: uuid.to_string() } }

  fn add_book (&self, parsed_book: &ParseBook) {
    self.db.insert_book(parsed_book);
  }

  pub fn get_books (&self) -> Books {
    self.db.get_books("root")
  }

  pub fn scan_lib (&self, path: &str) {
    let scan_path = PathBuf::from(path);
    self.scan_lib_aux (scan_path, "root"); }

  fn scan_lib_aux (&self, scan_path: PathBuf, parent_uuid: &str) {
    let dir= scan_dir (&scan_path);

    for dir in dir.0 {
      let uuid = Uuid::new_v4().to_string();
      let path = scan_path.to_str().unwrap();

      self.db.insert_dir(uuid.as_str(), path, parent_uuid);
      self.scan_lib_aux (dir, uuid.as_str()); }

    for file in dir.1 {
      let file_name = file.file_name().unwrap().to_str().unwrap();
      let existing_uuid: Option<String> = self.db.get_book_uuid(file_name);
      if let Some(uuid) = existing_uuid {
	continue;
      }
      let book = parse_book (&file, parent_uuid);
      self.add_book (&book.0);
      if book.1.is_some() {
	create_thumbs(&self.uuid, &book.0.book.uuid, book.1.unwrap());
      }
    }
  }
}
/*
impl LibraryModel {
  pub fn set_pos (&self, uuid: &str, position: &str) {
    self.db.set_media_position (uuid, position);
  }

  pub fn get_pos (&self, uuid: &str) -> String {
    self.db.select_media_position (uuid)
  }
}
*/