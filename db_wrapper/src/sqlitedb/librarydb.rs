use std::time::{SystemTime, UNIX_EPOCH};
use library_types::*;
use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::*;
use serde_json::*;

include_sql!("src/library.sql");
include_sql!("src/create_library.sql");

static TABLE_NAMES: [&str; 5] = ["dir", "creator", "subject", "publisher", "language"];

pub struct LibraryDBConn {
	pub db: rusqlite::Connection,
}

impl LibraryDBConn {
	pub fn new(path: &str) -> Self {
		let db = rusqlite::Connection::open(path).unwrap();
		Self { db: create_schema(db) }
	}


	pub fn insert_dir(&self, uuid: &str, path: &str, parent: &str) {
		self.db.insert_dir(uuid, path, parent).unwrap();
	}

	// DIRS ----------------------------------------------------

	pub fn get_dirs(&self, parent_dir_uuid: &str) -> Dirs {
		let mut dirs: Vec<Dir> = Vec::new();
		self.db.get_dirs (parent_dir_uuid, |row| {
			dirs.push (Dir {uuid: row.get(0)?, name: row.get(1)?, parent: row.get(2)? });
			Ok(())
		}).expect ("Error getting folders");
		dirs
	}

	pub fn clear_dirs(&self) {
		self.db.execute ("DELETE FROM dir", []).unwrap();
		self.db.execute ("DELETE FROM book_dir", []).unwrap();
		self.insert_dir ("root", "None", "/");
	}

	pub fn get_dir_path(&self, dir_uuid: &str) -> String {
		let mut path: String = String::new();
		self.db.select_dir(dir_uuid, |row| {
			let dir_name: 	String = row.get(2).unwrap();
			let parent_uuid:String = row.get(1).unwrap();

			if parent_uuid.eq("root") {
				path = format!("{}/", dir_name);
				return Ok(());
			}

			let dir_path = self.get_dir_path(parent_uuid.as_str());
			path = format!("{}/{}", dir_path, dir_name);
			Ok(())
		}).expect("Error getting media position");
		path.replace("//", "/")
	}

	pub fn insert_book_dir(&self, book_uuid: &str, dir_uuid: &str) {
		if self.db.insert_book_dir(book_uuid, dir_uuid).is_ok() {  }
	}

	pub fn delete_dir(&self, dir_uuid: &str) -> rusqlite::Result<usize, rusqlite::Error>{
		self.db.delete_dir(dir_uuid)
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

	pub fn insert_book(&self, parsed_book: ParseBook) {
		let scan_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		let book_uuid = parsed_book.book.uuid.as_str();
		self.db.insert_book(
			book_uuid,
			parsed_book.name.as_str(),
			parsed_book.book.progress,
			parsed_book.mdata.pos.as_str(),
			to_string(&parsed_book.mdata.contents).unwrap().as_str(),
			parsed_book.book.title.as_str(),
			parsed_book.mdata.desc.as_str(),
			to_string(&parsed_book.mdata.ids).unwrap().as_str(),
			parsed_book.mdata.publ,
			scan_time,
		).unwrap();
		self.insert_containers("creator", 	book_uuid, parsed_book.authors, Container::Creator);
		self.insert_containers("subject", 	book_uuid, parsed_book.subjects, Container::Subject);
		self.insert_containers("publisher", book_uuid, parsed_book.publisher, Container::Publisher);
		self.insert_containers("language", 	book_uuid, vec![parsed_book.language], Container::Language);
	}

	pub fn delete_book(&self, book_uuid: &str) -> rusqlite::Result<usize, rusqlite::Error>{
		self.db.delete_book(book_uuid)
	}

	pub fn get_book_path(&self, uuid: &str, library_path: &String) -> String {
		let mut file_name: 	String = String::new();
		let mut dir_path: 	String = String::new();

		self.db.get_book_file_info(uuid, |row| {
			file_name 				= row.get(0).unwrap();
			let dir_uuid: String 	= row.get(1).unwrap();
			dir_path 				= self.get_dir_path(dir_uuid.as_str());
			Ok(())
		}).expect("Error getting book file name");

		format!("{}/{}/{}", library_path, dir_path, file_name).replace("/None", "").replace("//", "/")
	}

	pub fn book_exists(&self, uuid: &str) -> bool {
		let mut exists: bool = false;
		self.db.select_book(uuid, |_row| {
			exists = true;
			Ok(())
		}).expect("Error getting media position");
		exists
	}

	pub fn get_books(&self, dir_uuid: &str) -> Books {
		let mut books: Books = Vec::new();
		self.db.get_books(dir_uuid, |row| {
			let book = LibBook {uuid: row.get(0).unwrap(), title: row.get(1).unwrap(), progress: row.get(2).unwrap()};
			books.push(book);
			Ok(())
		}).expect("Error getting media");
		books
	}

	pub fn set_pos(&self, uuid: &str, pos: &str, progress: u8) {
		self.db.set_pos(uuid, pos, progress).unwrap();
	}

	pub fn get_pos(&self, uuid: &str) -> String {
		let mut position: String = String::new();
		self.db.get_pos(uuid, |row| {
			position = row.get(0).unwrap();
			Ok(())
		}).expect("Error getting media position");
		position
	}

	pub fn get_book_toc(&self, book_uuid: &str) -> Contents {
		let mut contents: Contents = Vec::new();
		self.db.get_book_toc(book_uuid, |row| {
			let json: String = row.get(0).unwrap();
			contents  = from_str(json.as_str()).unwrap();
			Ok(())
		}).expect("Error getting toc");
		contents
	}

	// CONTAINERS --------------------------------------------------------------

	fn insert_containers(&self, table_name:&str, book_uuid: &str, containers: Vec<String>, typ:Container) {
		for container in containers {
			let name = container.as_str();
			let uuid = match self.get_entry_uuid("container", name) {
				Some(uuid) 	=> uuid,
				None 		=> self.insert_container(name, typ)
			};
			self.insert_book_container( book_uuid, uuid.as_str());
		}
	}

	fn insert_container(&self, name: &str, container:Container) -> String {
		let query 		= format!("INSERT OR IGNORE INTO container (uuid, name, type) VALUES (?,?,?)");
		let uuid 		= uuid::Uuid::new_v4().to_string();
		let mut stmt 	= self.db.prepare(query.as_str()).unwrap();
		stmt.execute(params![uuid, name, container as u32]).unwrap();
		uuid
	}

	fn insert_book_container(&self, book_uuid: &str, container_uuid: &str ){
		let query		= format!("INSERT INTO book_container (book_uuid, container_uuid) VALUES (?,?)");
		let mut stmt 	= self.db.prepare(query.as_str()).expect("{book_uuid}, {container_uuid}");
		stmt.execute(params![book_uuid, container_uuid]).expect("{book_uuid}, {container_uuid}");
	}

	fn get_entries(&self, table_name:&str, parent:Option<&str>) {
		if parent.is_none() {
			let mut stmt = self.db.prepare("SELECT * FROM {table_name}");
			
		}

	}

	fn get_entry_uuid(&self, table_name: &str, entry_name: &str) -> Option<String>{
		let query		= format!("SELECT uuid FROM {table_name} WHERE name = ?");
		let mut stmt 	= self.db.prepare(query.as_str()).unwrap();
		match stmt.query_row(params![entry_name], |row| row.get(0)) {
			Ok(val) => Some(val),
			Err(_) 	=> None, // Some other error
		}
	}

	/*fn delete_entry(&self, table_name:&str, uuid:&str) -> Result<usize> {
		let query 		= format!("DELETE FROM {table_name} WHERE uuid = ?");
		let mut stmt	= self.db.prepare(query.as_str()).unwrap();
		stmt.execute(params![uuid])
	}*/
}


fn create_schema (mut db: Connection) -> Connection{
	db.create_book_table().unwrap();

	db.create_dir_table().unwrap();
	db.create_book_dir_table().unwrap();

	db.create_container_table().unwrap();
	db.create_book_container_table().unwrap();
	db.create_container_link_table().unwrap();
	db.create_container_alias_table().unwrap();
	db
}

