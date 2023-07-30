use std::path::{PathBuf};
use library_types::*;
use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::*;
use serde_json::*;
include_sql!("src/library.sql");

pub struct LibraryDBConn {
    pub db: rusqlite::Connection,
}

impl LibraryDBConn {
    pub fn new(uuid: &str) -> Self {
	let db = rusqlite::Connection::open("/home/johan/.local/share/media_library/libraries/test/library.db").unwrap();
	Self { db }
    }

    pub fn insert_dir(&self, uuid: &str, path: &str, parent: &str) {
	self.db.insert_dir(uuid, path, parent).unwrap();
    }

    // DIRS ----------------------------------------------------

    pub fn get_dirs(&self, parent_dir_uuid: &str) -> Dirs {
	let mut dirs: Vec<Dir> = Vec::new();
	self.db.get_dirs(parent_dir_uuid, |row| {
	    dirs.push(deserialize_dir(row).unwrap());
	    Ok(())
	}).expect("Error getting folders");
	dirs
    }

    pub fn clear_dirs(&self) {
	self.db.execute("DELETE FROM dir", []).unwrap();
	self.insert_dir("root", "None", "/");
    }

    fn get_dir_path(&self, dir_uuid: &str) -> String {
	let mut path: String = String::new();
	self.db.select_dir(dir_uuid, |row| {
	    let name: String = row.get(2).unwrap();
	    let parent_uuid: String = row.get(1).unwrap();

	    if parent_uuid.eq("root") {
		path = format!("{}/", name);
		return Ok(());
	    };
	    println!("get_dir_path: {} {}", parent_uuid, name);

	    let dir_path = self.get_dir_path(parent_uuid.as_str());
	    path = format!("{}/{}", dir_path, name);
	    Ok(())
	}).expect("Error getting media position");
	let p = path;
	p
     }

    pub fn insert_book_dir(&self, book_uuid: &str, dir_uuid: &str) {
	println!("insert_book_dir: {} {}", book_uuid, dir_uuid);
	self.db.insert_book_dir(book_uuid, dir_uuid).unwrap();
    }

    // BOOKS ---------------------------------------------------


    pub fn get_book_uuid(&self, file_name: &str) -> Option<String> {
	let mut uuid: Option<String> = None;
	self.db.select_book_uuid(file_name, |row| {
	    uuid = Some(row.get(0).unwrap());
	    Ok(())
	}).expect("get_book_uuid error");
	uuid
    }

    pub fn insert_book(&self, parsed_book: &ParseBook, scan_timestamp: u64) {
	self.db.insert_book(
	    parsed_book.book.uuid.as_str(),
	    parsed_book.name.as_str(),
	    parsed_book.book.progress.clone(),
	    parsed_book.mdata.pos.as_str(),
	    to_string(&parsed_book.mdata.contents).unwrap().as_str(),
	    parsed_book.book.title.as_str(),
	    parsed_book.mdata.desc.as_str(),
	    to_string(&parsed_book.mdata.ids).unwrap().as_str(),
	    parsed_book.mdata.publ.clone(),
	    scan_timestamp,
	).unwrap();
    }

    pub fn get_books(&self, dir_uuid: &str) -> Books {
	let mut books: Books = Vec::new();
	self.db.get_books(dir_uuid, |row| {
	    let book = deserialize_book(row).unwrap();
	    books.push(book);
	    Ok(())
	}).expect("Error getting media");
	books
    }

    pub fn get_book_path(&self, uuid: &str, library_path: &String) -> String {
	let mut book_file_name: String = String::new();
	let mut dir_path: String = String::new();
	self.db.get_book_file_info(uuid, |row| {
	    book_file_name = row.get(0).unwrap();
	    let dir_uuid: String = row.get(1).unwrap();
	    dir_path = self.get_dir_path(dir_uuid.as_str());
	    Ok(())
	}).expect("Error getting book file name");
	format!("{}/{}{}", library_path, dir_path, book_file_name).replace("/None", "")
    }

    pub fn book_exists(&self, uuid: &str) -> bool {
	let mut exists: bool = false;
	self.db.select_book(uuid, |row| {
	    exists = true;
	    Ok(())
	}).expect("Error getting media position");
	exists
    }


    pub fn set_pos(&self, uuid: &str, pos: &str) {
	self.db.set_pos(uuid, pos).unwrap();
    }

    pub fn get_pos(&self, uuid: &str) -> String {
	let mut position: String = String::new();
	self.db.get_pos(uuid, |row| {
	    position = row.get(0).unwrap();
	    Ok(())
	}).expect("Error getting media position");
	position
    }
}

// Deserialize functions ---------------------------------------

fn deserialize_dir(row: &Row) -> rusqlite::Result<Dir> {
    Ok(Dir { uuid: row.get(0)?, name: row.get(1)?, parent: row.get(2)? })
}

fn deserialize_book(row: &Row) -> rusqlite::Result<LibBook> {
    Ok(LibBook {
	uuid: row.get(0).unwrap(),
	title: row.get(1).unwrap(),
	progress: row.get(2).unwrap(),
    })
}

