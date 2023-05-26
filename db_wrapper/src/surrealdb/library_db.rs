
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

    // only used for tests
  /*  pub async fn new_mem(path: &str) -> Result<Self, Error> {
        let DB = connect(path).await?;

        DB.use_ns("library").use_DB("library").await?;
        Ok(Self { DB })
    }*/
    pub fn insert_media_file(&self, mediafile: &MediaFile) -> Result<Record, Error> {
        RUNTIME.block_on(self.insert_media_file_async(mediafile))
    }
    pub async fn insert_media_file_async(&self, media_file: &MediaFile) -> Result<Record, Error>{
        println!("Changing namespace");
        DB.use_ns("library").use_db("library").await?;
        println!("inserting media fil");
        DB.create("media_file").content(media_file).await
    }

    pub fn select_media_file(&self, uuid: &str) -> Result<MediaFile, Error> {
        println!("selecting media file block on");
        RUNTIME.block_on(self.select_media_file_async(uuid))
    }
    pub async fn select_media_file_async(&self, uuid: &str) -> Result<MediaFile, Error> {
        DB.use_ns("library").use_db("library").await?;
        println!("selecting media file");
        DB.select(("media_file", uuid)).await
    }

    pub fn select_media_files(&self) -> Result<Vec<MediaFile>, Error> {
        RUNTIME.block_on(self.select_media_files_async())
    }
    pub async fn select_media_files_async(&self) -> Result<Vec<MediaFile>, Error> {

        DB.use_ns("library").use_db("library").await?;
        println!("selecting media files");
        DB.select("media_file").await

    }

    pub fn delete_media_file(&self, uuid: &str) -> Result<Record, Error> {
        RUNTIME.block_on(self.delete_media_file_async(uuid))
    }
    pub async fn delete_media_file_async(&self, uuid: &str) -> Result<Record, Error> {
        DB.use_ns("library").use_db("library").await?;
        DB.delete(("media_file", uuid)).await
    }
}
/*
#[cfg(test)]
mod tests {
    use library_types::*;
    use super::*;

    #[tokio::test]
    async fn test_insert_select_media(){
        let DB = LibraryDBConn::new_mem("mem://").await.unwrap();
        let media_file = MediaFile::default();

        DB.insert_media_file(&media_file).await.expect("Error inserting media file");
        let files = DB.select_media_files().await.expect("Error selecting media files");
        assert_eq!(files[0], media_file);
    }
}*/