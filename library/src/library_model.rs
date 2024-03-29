use std::collections::LinkedList;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::time::Instant;
use threadpool::ThreadPool;

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

	pub fn get_books	(&self, folder_uuid: &str) 	-> Books 	{ self.db.get_books(folder_uuid) }
	pub fn get_book_path(&self, book_uuid: &str)	-> String 	{ self.db.get_book_path(book_uuid, &self.path) }
	pub fn get_book_toc	(&self, book_uuid: &str) 	-> Contents { self.db.get_book_toc(book_uuid) }
	pub fn get_dirs		(&self, parent_uuid: &str) 	-> Dirs 	{ self.db.get_dirs(parent_uuid) }

	pub fn delete_book(&self, book_uuid: &str) -> Result<(), std::io::Error> {
		let book_path = self.get_book_path(book_uuid);
		self.db.delete_entry("book", book_uuid).unwrap();
		fs::remove_file(book_path)?;
		fs::remove_dir_all(format!("{}/.bookrium/{}", self.path, book_uuid))?;
		Ok(())
	}

	pub fn delete_dir(&self, dir_uuid: &str) -> Result<(), std::io::Error> {
		let dir_path = self.db.get_dir_path(dir_uuid);
		let path = &self.path;
		println!("{dir_path}");
		self.db.delete_entry("dir", dir_uuid).unwrap();
		fs::remove_dir_all(format!("{path}/{dir_path}"))?;
		Ok(())
	}

	pub fn scan_lib(&mut self, path: &str) {
		self.db.clear_dirs();
		let (cover_sender, cover_receiver) 	= channel::<(String, Vec<u8>)>();
		let (file_sender, file_receiver) 	= channel::<(PathBuf, String)>();
		let (book_sender, book_receiver) 	= channel::<(ParseBook, String)>();
		let pool = ThreadPool::new(8);
		let book_pool = ThreadPool::new(16);//ThreadPool::new(8);
		let cover_pool = pool.clone();
		std::thread::spawn(move || {
			let before_cover = Instant::now();
			for item in cover_receiver {
				let pool = cover_pool.clone();
				pool.execute(move || {
					create_thumbnails(item.0.clone(), item.1);
					let path = item.0;
					println!("Creating cover {path}, {:?}", before_cover.elapsed());
				});
			}

		});

		let meta_paths = self.meta_path.clone();
		let before_scan = Instant::now();
		std::thread::spawn(move || {
			for file in file_receiver {
				let book_pool = book_pool.clone();
				let cover_sender_clone = cover_sender.clone();
				let book_sender_clone = book_sender.clone();
				let meta_paths = meta_paths.clone();
				book_pool.execute(move || {
					let start_parse = Instant::now();
					let parse_book_result = parse_book(&file.0);
					if let Some((book, cover_option)) = parse_book_result {
						let book_uuid = book.book.uuid.clone();
						if let Some(cover) = cover_option {
							let out_path = format!("{}/{}", meta_paths, book_uuid);
							cover_sender_clone.send((out_path, cover)).unwrap();
						}
						book_sender_clone.send((book, file.1)).unwrap(); // Use the cloned sender
						println!("Book delay {:?}, Parse delay: {:?}", before_scan.elapsed(), start_parse.elapsed());
					}
				});
			}
			println!("Parse: {:?}", before_scan.elapsed());
		});
		self.scan_lib_aux(PathBuf::from(path), "root", file_sender);
		let scan_duration = before_scan.elapsed();
		let before_insert = Instant::now();
		self.db.insert_books(book_receiver);
		let insert_duration = before_insert.elapsed();
		println!("Scan: {:?}, Insert: {:?}", scan_duration, insert_duration);
	}

	fn scan_lib_aux(&self, scan_path: PathBuf, parent_uuid: &str, book_sender:Sender<(PathBuf, String)>){
		let (dirs, files) = scan_dir(&scan_path);
		for file in files {
			self.scan_book(file, parent_uuid, book_sender.clone());
		}
		for dir in dirs {
			let uuid = Uuid::new_v4().to_string();
			let name = dir.file_name().unwrap().to_str().unwrap();
			self.db.insert_dir(uuid.as_str(), name, parent_uuid);
			self.scan_lib_aux(dir, uuid.as_str(), book_sender.clone());
		}
	}

	fn scan_book(&self, file: PathBuf, parent_uuid: &str, book_sender:Sender<(PathBuf, String)>){
		let file_name 		= file.file_name().unwrap().to_str().unwrap();
		/*let existing_uuid 	= self.db.get_entry_uuid("book", file_name);

		if let Some(uuid) = existing_uuid//.or_else(|| get_uuid(file.as_path())) {
		{
			if self.db.entry_exists("book", &uuid).is_some() {
				self.db.insert_book_dir(&uuid, parent_uuid);
				return;
			}
		}*/
		book_sender.send((file, parent_uuid.to_string())).unwrap();
	}
}

impl LibraryModel {
	pub fn set_pos(&self, uuid: &str, position: &str, progress: u8) { self.db.set_pos(uuid, position, progress); }
	pub fn get_pos(&self, uuid: &str) -> String { self.db.get_pos(uuid) }

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

	pub fn get_container_path(&self, container_uuid: &str){

	}
}

