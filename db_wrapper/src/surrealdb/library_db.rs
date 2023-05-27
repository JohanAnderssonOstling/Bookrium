use serde::{Deserialize, Serialize};
use surrealdb::{Error, Surreal};
use surrealdb::engine::any::{Any, connect};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use library_types::*;
use super::*;

pub struct LibraryDBConn {
	pub uuid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
	#[allow(dead_code)]
	pub id: Thing,
}

pub async fn create_schema(uuid: &str) {
	DB.signin(Root { username: "root", password: "root" });

	DB.query("DEFINE TABLE media_file");
	DB.query("DEFINE TABLE media");
	DB.query("DEFINE NAMESPACE library");
	DB.query("DEFINE DATABASE library");
	DB.use_ns("library").use_db(uuid).await.unwrap();
}

impl LibraryDBConn {
	pub fn new(uuid: &str) -> Result<Self, Error> {
		DB.version();
		RUNTIME.block_on(create_schema(uuid));
		Ok(Self { uuid: uuid.to_string() })
	}

	pub async fn set_db(&self) {
		DB.use_ns("library").use_db(self.uuid.as_str()).await.unwrap();
	}

	pub async fn insert_file(&self, file: &MediaFile) -> Result<Record, Error> {
		self.set_db().await;
		DB.create("media_file").content(file).await
	}

	pub async fn select_file(&self, uuid: &str) -> Result<MediaFile, Error> {
		self.set_db().await;
		DB.select(("media_file", uuid)).await
	}

	pub async fn select_files(&self) -> Result<Vec<MediaFile>, Error> {
		self.set_db().await;
		DB.select("media_file").await
	}

	pub async fn delete_file(&self, uuid: &str) -> Result<Record, Error> {
		self.set_db().await;
		DB.delete(("media_file", uuid)).await
	}

	pub async fn clear_library(&self) -> Result<(Vec<MediaFile>), Error> {
		self.set_db().await;
		DB.delete("media_file").await
	}

	// File Queries
}
