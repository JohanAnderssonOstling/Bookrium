
use library_types::*;
use surrealdb::{Error, Surreal};
use surrealdb::engine::any::{Any, connect};
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use super::*;
pub struct LibraryDBConn {
    pub uuid: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

pub async fn create_schema(uuid: &str){
    DB.signin(Root { username: "root", password: "root"});

    DB.query("DEFINE TABLE media_file");
    DB.query("DEFINE TABLE media");
    DB.query("DEFINE NAMESPACE library");
    DB.query("DEFINE DATABASE library");
    DB.use_ns("library").use_db("library").await.unwrap();
    println!("created schema");
}

impl LibraryDBConn{
    pub fn new(uuid: &str) -> Result<Self, Error> {
        DB.version();
        RUNTIME.block_on(create_schema(uuid));
        Ok(Self { uuid: uuid.to_string() })
    }

    pub async fn insert_media_file(&self, media_file: &MediaFile) -> Result<Record, Error>{
        DB.use_ns("library").use_db("library").await?;
        DB.create("media_file").content(media_file).await
    }

    pub async fn select_media_file(&self, uuid: &str) -> Result<MediaFile, Error> {
        DB.use_ns("library").use_db("library").await?;
        DB.select(("media_file", uuid)).await
    }

    pub async fn select_media_files(&self) -> Result<Vec<MediaFile>, Error> {
        DB.use_ns("library").use_db("library").await?;
        DB.select("media_file").await
    }

    pub async fn delete_media_file(&self, uuid: &str) -> Result<Record, Error> {
        DB.use_ns("library").use_db("library").await?;
        DB.delete(("media_file", uuid)).await
    }
}
