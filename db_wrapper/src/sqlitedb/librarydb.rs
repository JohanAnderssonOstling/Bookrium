use library_types::{Dir, MediaFile, Navigation, NavPoint};
use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;
use serde_json::json;
include_sql!("src/library.sql");

pub struct LibraryDBConn {
	pub db: rusqlite::Connection,
}

impl LibraryDBConn{
	pub fn new (uuid: &str) -> Self{
		let db = rusqlite::Connection::open("/home/johan/.local/share/media_library/libraries/test/library.db").unwrap();
		Self { db }
	}
	pub fn insert_dir(&self, dir: &Dir) {
		self.db.insert_dir(
			dir.uuid.as_str(), dir.name.as_str(), dir.path.as_str(), dir.parent_uuid.as_str()).unwrap();
	}

	pub fn get_dirs(&self, parent_dir_uuid: &str) -> Vec<Dir> {
		let mut dirs: Vec<Dir> = Vec::new();
		self.db.get_dirs(parent_dir_uuid, | row| {
			dirs.push(deserialize_dir(row).unwrap());
			Ok(())
		}).expect("Error getting folders");
		dirs
	}

	pub fn insert_media(&self, media: &MediaFile) {
		self.db.insert_media(
			media.uuid.as_str(),
			media.path.as_str(),
			media.duration.as_str(),
			media.position.as_str(),
			media.parent_dir_uuid.as_str(),
			serde_json::to_string(&media.navigation).unwrap().as_str(),
			media.title.as_str(),
			media.description.as_str()).unwrap();
	}

	pub fn fetch_media(&self, parent_dir_uuid: &str) -> Vec<MediaFile> {
		let mut media: Vec<MediaFile> = Vec::new();
		self.db.get_media(parent_dir_uuid, | row| {
			media.push(deserialize_media(row).unwrap());
			Ok(())
		}).expect("Error getting media");

		media
	}

	pub fn set_media_position(&self, uuid: &str, position: &str) {
		self.db.set_media_position(uuid, position).unwrap();
	}
	pub fn select_media_position(&self, uuid: &str) -> String {
		let mut position: String = String::new();
		self.db.select_media_position(uuid, | row| {
			position = row.get(0).unwrap();
			Ok(())
		}).expect("Error getting media position");
		position
	}
}

fn deserialize_dir(row: &rusqlite::Row) -> rusqlite::Result<Dir> {
	Ok(Dir {
		uuid: row.get(0)?, name: row.get(1)?, path: row.get(2)?, parent_uuid: row.get(3)?,
	})
}

fn deserialize_media(row: &rusqlite::Row) -> rusqlite::Result<MediaFile> {
	let navigation_json: String = row.get(5)?;
	Ok(MediaFile {
		uuid: row.get(0)?, path: row.get(1)?, duration: row.get(2)?,
		position: row.get(3)?, parent_dir_uuid: row.get(4)?,
		navigation: serde_json::from_str(navigation_json.as_str()).unwrap(),
		title: row.get(6)?, description: row.get(7)?,
	})
}



