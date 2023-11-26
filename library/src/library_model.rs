use std::fmt::{Error, format};
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
	pub uuid: 	String,
	pub path: 	String,
	meta_path: 	String,
}
impl LibraryModel {
	pub fn open(uuid: &str, path: &str) -> Self {
		let meta_path 	= format!("{path}/.bookrium");
		match fs::create_dir_all(&meta_path) {
			Ok(_) 		=> {},
			Err(err) 	=> {println!("Error creating meta dir: {}", err)},
		}
		let db_path		= format!("{meta_path}/library.db");
		Self {
			db: 	LibraryDBConn::new(db_path.as_str()),
			uuid: 	uuid.to_string(),
			path: 	path.to_string(),
			meta_path,
		}
	}

	pub fn get_books(&self, folder_uuid: &str) 	-> Books 	{ self.db.get_books(folder_uuid) }
	pub fn get_book_path(&self, book_uuid: &str)-> String 	{ self.db.get_book_path(book_uuid, &self.path) }
	pub fn get_book_toc(&self, book_uuid: &str) -> Contents { self.db.get_book_toc(book_uuid) }
	pub fn get_dirs(&self, parent_uuid: &str) 	-> Dirs 	{ self.db.get_dirs(parent_uuid) }

	pub fn delete_book(&self, book_uuid: &str) -> Result<(), std::io::Error> {
		let book_path = self.get_book_path(book_uuid);
		self.db.delete_book(book_uuid).unwrap();
		fs::remove_file(book_path)?;
		fs::remove_dir_all(format!("{}/.bookrium/{}", self.path, book_uuid))?;
		Ok(())
	}

	pub fn delete_dir(&self, dir_uuid: &str) -> Result<(), std::io::Error> {
		let dir_path = self.db.get_dir_path(dir_uuid);
		self.db.delete_dir(dir_uuid).unwrap();
		fs::remove_dir_all(dir_path)?;
		Ok(())
	}

	pub fn scan_lib(&self, path: &str) {
		self.db.clear_dirs();
		let mut covers: Vec<(String, Vec<u8>)> = Vec::new();
		covers = self.scan_lib_aux(PathBuf::from(path), "root", covers);
		std::thread::spawn(move || {
			for (path, cover) in covers { create_thumbnails(path, cover); }
		});
	}

	fn scan_lib_aux(&self, scan_path: PathBuf, parent_uuid: &str, mut covers: Vec<(String, Vec<u8>)>)
		-> Vec<(String, Vec<u8>)>{

		let (dirs, files) = scan_dir(&scan_path);
		for file in files {
			let cover = self.scan_book(file, parent_uuid);
			if cover.is_some() {covers.push(cover.unwrap());}
		}
		for dir in dirs {
			let uuid = Uuid::new_v4().to_string();
			let name = dir.file_name().unwrap().to_str().unwrap();
			self.db.insert_dir(uuid.as_str(), name, parent_uuid);
			covers = self.scan_lib_aux(dir, uuid.as_str(), covers);
		}
		covers
	}

	fn scan_book(&self, file: PathBuf, parent_uuid: &str) -> Option<(String, Vec<u8>)>{
		let file_name 		= file.file_name().unwrap().to_str().unwrap();
		let existing_uuid 	= self.db.get_book_uuid(file_name);

		if let Some(uuid) = existing_uuid.or_else(|| get_uuid(file.as_path())) {
			if self.db.book_exists(&uuid) {
				self.db.insert_book_dir(&uuid, parent_uuid);
				return None;
			}
		}

		let (book, cover_option) = parse_book(&file, parent_uuid).unwrap();
		let book_uuid = book.book.uuid.clone();
		self.db.insert_book(book);
		self.db.insert_book_dir(book_uuid.as_str(), parent_uuid);

		if  let Some(cover) = cover_option {
			let out_path = format!("{}/{}", self.meta_path, book_uuid);
			return Some((out_path, cover));
		}
		None
	}
}

impl LibraryModel {
	pub fn set_pos(&self, uuid: &str, position: &str, progress: u8) {
		self.db.set_pos(uuid, position, progress);
	}

	pub fn get_pos(&self, uuid: &str) -> String {
		self.db.get_pos(uuid)
	}

	pub fn get_cover_path (&self, book_uuid: &str) -> String {
		let path_str = format!("{}/{book_uuid}/256.jpg", self.meta_path);
		let path = Path::new(&path_str);
		if !path.exists() {return "".into();}
		path.to_str().unwrap().to_string()
	}

	pub fn get_container_cover_path (&self, container_uuid: &str) -> String{
		let books = self.get_books(container_uuid);
		for book in books {
			let cover_path = self.get_cover_path(book.uuid.as_str());
			if !cover_path.is_empty() {return cover_path;}
		}
		"".into()
	}
}

