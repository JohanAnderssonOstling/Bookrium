use library_types::*;
use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::*;
use serde_json::*;
include_sql!("src/library.sql");
include_sql!("src/create_library.sql");
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
		self.db.get_dirs(parent_dir_uuid, |row| {
			dirs.push(deserialize_dir(row).unwrap());
			Ok(())
		}).expect("Error getting folders");
		dirs
	}

	pub fn clear_dirs(&self) {
		self.db.execute("DELETE FROM dir", []).unwrap();
		self.db.execute("DELETE FROM book_dir", []).unwrap();
		self.insert_dir("root", "None", "/");
	}

	pub fn get_dir_path(&self, dir_uuid: &str) -> String {
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
		path.replace("//", "/")
	}

	pub fn insert_book_dir(&self, book_uuid: &str, dir_uuid: &str) {
		if self.db.insert_book_dir(book_uuid, dir_uuid).is_ok() {  }
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

	pub fn insert_book(&self, parsed_book: ParseBook, scan_timestamp: u64) {
		let book_uuid = parsed_book.book.uuid.as_str();
		self.db.insert_book(
			book_uuid,
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
		self.insert_containers("creator", book_uuid, parsed_book.authors);
		self.insert_containers("subject", book_uuid, parsed_book.subjects);
		self.insert_containers("publisher", book_uuid, parsed_book.publisher);
	}

	pub fn delete_book(&self, book_uuid: &str) -> rusqlite::Result<usize, rusqlite::Error>{
		self.db.delete_book(book_uuid)
	}

	pub fn delete_dir(&self, dir_uuid: &str) -> rusqlite::Result<usize, rusqlite::Error>{
		self.db.delete_dir(dir_uuid)
	}

	fn insert_containers(&self, table_name: &str, book_uuid: &str, containers: Vec<String>) {
		for container in containers {
			let name = container.as_str();
			let uuid = match self.get_entry_uuid(table_name, name) {
				Some(uuid) 	=> uuid,
				None 		=> self.insert_container(table_name, name)
			};
			self.insert_book_container(table_name, book_uuid, uuid.as_str());
		}
	}

	fn get_entry_uuid(&self, table_name: &str, entry_name: &str)
		-> Option<String>{
		let query = format!("SELECT uuid FROM {table_name} WHERE name = ?");
		let mut stmt = self.db.prepare(query.as_str()).unwrap();
		match stmt.query_row(params![entry_name], |row| row.get(0)) {
			Ok(val) => Some(val),
			Err(rusqlite::Error::QueryReturnedNoRows) => None, // No matching row
			Err(e) => None, // Some other error
		}
	}

	fn insert_container(&self, table_name: &str, name: &str)
	-> String {
		let query = format!
		("INSERT INTO {table_name} (uuid, name) VALUES (?,?)");
		let uuid = uuid::Uuid::new_v4().to_string();
		let mut stmt = self.db.prepare(query.as_str()).unwrap();
		stmt.execute(params![uuid, name]).unwrap();
		uuid
	}

	fn insert_book_container(&self, table_name: &str, book_uuid: &str, container_uuid: &str ){
		let query = format!("INSERT INTO book_{table_name} \
			(book_uuid, container_uuid) VALUES (?,?)");
		let mut stmt = self.db.prepare(query.as_str()).unwrap();
		println!("{table_name} {book_uuid} {container_uuid}");
		match stmt.execute(params![book_uuid, container_uuid]) {
			Ok(_) => (),
			Err(_) => ()
		}
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
			book_file_name 			= row.get(0).unwrap();
			let dir_uuid: String 	= row.get(1).unwrap();
			dir_path 	= self.get_dir_path(dir_uuid.as_str());
			Ok(())
		}).expect("Error getting book file name");
		format!("{}/{}/{}", library_path, dir_path, book_file_name).replace("/None", "").replace("//", "/")
	}

	pub fn book_exists(&self, uuid: &str) -> bool {
		let mut exists: bool = false;
		self.db.select_book(uuid, |row| {
			exists = true;
			Ok(())
		}).expect("Error getting media position");
		exists
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
			contents  = deserialize_toc(row);
			Ok(())
		}).expect("Error getting toc");
		contents
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

fn deserialize_toc(row: &Row) -> Contents {
	let json: String = row.get(0).unwrap();
	from_str(json.as_str()).unwrap()
}

fn create_schema (db: Connection) -> Connection{
	db.create_book_table().unwrap();
	db.create_creator_table().unwrap();
	db.create_book_creator_table().unwrap();
	db.create_dir_table().unwrap();
	db.create_book_dir_table().unwrap();
	db.create_subject_table().unwrap();
	db.create_book_subject_table().unwrap();
	db.create_publisher_table().unwrap();
	db.create_book_publisher_table().unwrap();
	db.create_language_table().unwrap();
	db.create_book_language_table().unwrap();
	db
}
#[cfg(test)]
mod tests {

}

