use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::{Error};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use library_types::*;
use super::*;

pub struct LibraryDBConn {
	pub uuid: String,
	pub db: &'static Surreal<Any>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
	#[allow(dead_code)]
	pub id: Thing,
}

impl LibraryDBConn {
	pub fn new(uuid: &str) -> Self{
		DB.version();
		let db_conn = Self { uuid: uuid.to_string(), db: &DB };
		//RUNTIME.block_on(db_conn.create_schema());
		db_conn
	}

	pub async fn create_schema(&self){
		self.db.signin(Root { username: "root", password: "root" });
		self.db.use_ns("library").use_db(self.uuid.as_str()).await.unwrap();
	}


	pub async fn insert_file(&self, file: &MediaFile) -> Result<Record, Error> {
		self.db.create(("media", &file.uuid)).content(file).await
	}

	pub async fn select_file(&self, uuid: &str) -> Result<MediaFile, Error> {
		self.db.select(("media", uuid)).await
	}

	pub async fn select_files(&self) -> Result<Vec<MediaFile>, Error> {
		self.db.select("media").await
	}

	pub async fn delete_file(&self, uuid: &str) -> Result<Record, Error> {
		self.db.delete(("media", uuid)).await
	}

	pub async fn clear_library(&self) -> Result<(Vec<MediaFile>), Error> {
		self.db.delete("media").await
	}

	// File Queries
	pub async fn set_media_position(&self, uuid: &str, position: &str) {
		let json: Value = json!({ "position": position });
		//let res: MediaFile = self.db.update(("media", uuid)).merge(json).await?;

		let query ="UPDATE media:$uuid SET position = $position";
		let response = self.db.query(query).
				bind(("uuid", uuid.to_string())).bind(("position", position.to_string())).
				await;
		let res = match response {
			Ok(res) => res,
			Err(e) => panic!("Error: {}", e)
		};
	}

	pub async fn get_media_position(&self, uuid: &str) -> String {
		let query = format!("SELECT position FROM media:{}", uuid);
		let mut response = self.db.query(query.as_str()).await.unwrap();
		let position: String = "hello".to_string();
		position
	}
}
