use library_types::*;
use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::*;
use serde_json::*;
use chrono::NaiveDate;
include_sql!("src/library.sql");

pub struct LibraryDBConn {
	pub db: rusqlite::Connection,
}

impl LibraryDBConn {
	pub fn new(uuid: &str) -> Self {
		let db = rusqlite::Connection::open("/home/johan/.local/share/media_library/libraries/test/library.db").unwrap();
		Self { db }
	}

	pub fn insert_dir(&self, dir: &Dir) {
		self.db.insert_dir(
			dir.uuid.as_str(),
			dir.name.as_str(),
			dir.path.as_str(),
			dir.parent_uuid.as_str()).unwrap();
	}

	pub fn get_dirs(&self, parent_dir_uuid: &str) -> Vec<Dir> {
		let mut dirs: Vec<Dir> = Vec::new();
		self.db.get_dirs(parent_dir_uuid, |row| {
			dirs.push(self.deserialize_dir(row).unwrap());
			Ok(())
		}).expect("Error getting folders");
		dirs
	}

	pub fn insert_media(&self, media: &Book) {
		self.db.insert_media(
			media.uuid.as_str(),
			media.path.as_str(),
			media.duration.as_str(),
			media.position.as_str(),
			media.parent_dir_uuid.as_str(),
			to_string(&media.navigation).unwrap().as_str(),
			media.title.as_str(),
			media.desc.as_str()).unwrap();
	}

	pub fn fetch_books(&self, dir_uuid: &str) -> Vec<Book> {
		let mut media: Vec<Book> = Vec::new();
		self.db.get_media(dir_uuid, |row| {
			media.push(self.deserialize_media(row).unwrap());
			Ok(())
		}).expect("Error getting media");

		media
	}



	pub fn set_media_position(&self, uuid: &str, pos: &str) {
		self.db.set_media_position(uuid, pos).unwrap();
	}

	pub fn select_media_position(&self, uuid: &str) -> String {
		let mut position: String = String::new();
		self.db.select_media_position(uuid, |row| {
			position = row.get(0).unwrap();
			Ok(())
		}).expect("Error getting media position");
		position
	}


	fn deserialize_dir(&self, row: &Row) -> rusqlite::Result<Dir> {
		Ok(Dir {
			uuid: row.get(0)?,
			name: row.get(1)?,
			path: row.get(2)?,
			parent_uuid: row.get(3)?,
		})
	}

	fn deserialize_media(&self, row: &Row) -> rusqlite::Result<Book> {
		let uuid: String = row.get(0)?;
		let nav_json: String = row.get(5)?;
		let identifiers_json: String = row.get(8)?;
		Ok(Book {
			uuid: uuid.clone(),
			path: row.get(1)?,
			duration: row.get(2)?,
			position: row.get(3)?,
			parent_dir_uuid: row.get(4)?,

			navigation: from_str(nav_json.as_str()).unwrap(),
			title: row.get(6)?,
			desc: row.get(7)?,
			identifiers: from_str(identifiers_json.as_str()).unwrap(),
			published: deserialize_date(row.get(9)?),
			creators: self.deserialize_creators(uuid.as_str()),
			subjects: self.deserialize_subjects(uuid.as_str()),
		})
	}

	fn deserialize_creators(&self, uuid: &str) -> Vec<Creator> {
		let mut creators: Vec<Creator> = Vec::new();
		self.db.get_book_creators(uuid, |row| {
			let role: String = row.get(2)?;
			creators.push(Creator {
				uuid: row.get(0)?,
				name: row.get(1)?,
				role: from_str(role.as_str()).unwrap(),
			});
			Ok(())
		}).expect("Error getting creators");
		creators
	}

	fn deserialize_subjects(&self, uuid: &str) -> Vec<Subject> {
		let mut subjects: Vec<Subject> = Vec::new();
		self.db.get_book_subjects(uuid, |row| {
			subjects.push(Subject {
				uuid: row.get(0)?,
				name: row.get(1)?,
			});
			Ok(())
		}).expect("Error getting subjects");
		subjects
	}
}

fn deserialize_date(str_date: String) -> NaiveDate {
	NaiveDate::parse_from_str(str_date.as_str(), "%Y-%m-%d").unwrap()
}


