use super::*;
use surrealdb::{Error, Surreal};
use library_types::Library;
use surrealdb::engine::any::{Any, connect};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::opt::auth::Root;
pub struct ClientDBConn {

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

pub async fn create_schema(conn: &Surreal<Any>) {
    conn.query("DEFINE NAMESPACE client");
    conn.query("DEFINE DATABASE client");
    conn.query("DEFINE TABLE library");
    conn.query("DEFINE FIELD uuid ON TABLE library TYPE string");
    conn.query("DEFINE FIELD name ON TABLE library TYPE string");
    conn.query("DEFINE FIELD path ON TABLE library TYPE string");
    conn.query("DEFINE FIELD url ON TABLE library TYPE string");
}

pub fn init_db(){
    DB.version();
}

impl ClientDBConn{
    pub fn open(path: &str) -> Result<ClientDBConn, Error> {
        //create_schema(&DB).await;
        DB.version();
        Ok(ClientDBConn { })
    }

    pub async fn insert_library(&self, library: &Library) -> Result<Record, Error> {
        DB.use_ns("client").use_db("client").await?;
        DB.create(("library", &library.uuid)).content(library).await
    }

    pub async fn select_libraries(&self) -> Result<Vec<Library>, Error>{
        DB.use_ns("client").use_db("client").await?;
        DB.select("library").await
    }

    pub async fn select_library(&self, uuid: &str) -> Result<Library, Error> {
        DB.use_ns("client").use_db("client").await?;
        DB.select(("library", uuid)).await
    }

    pub async fn delete_library(&self, uuid: &str) -> Result<Library, Error> {
        DB.use_ns("client").use_db("client").await?;
        DB.delete(("library", uuid)).await
    }

    pub async fn get_library_by_path(&self, path: &str) -> Library {
        todo!()
    }
    pub async fn delete_library_by_path(&self, path: &str) -> Result<Library, Error> {
        DB.use_ns("client").use_db("client").await?;
        let library: Library = DB.delete(("library", path)).await?;
        Ok(library)
    }
}
/*
#[cfg(test)]
mod tests {
use super::*;

    #[tokio::test]
    async fn test_insert_select_library(){
        let db = ClientDBConn::open("mem://").await.expect("Error opening database");
        let library = Library::default();

        db.insert_library(&library).await.expect("Error inserting library");
        let selected_library = db.select_libraries().await.expect("Error selecting libraries");
        assert_eq!(library, selected_library[0]);
    }
    #[tokio::test]
    async fn test_insert_delete_library(){
        let db = ClientDBConn::open("mem://").await.expect("Error opening database");
        let library = Library {
            uuid: "uuid1".to_string(),
            name: "name1".to_string(),
            path: "path1".to_string(),
            url: "url1".to_string(),
        };
        // make a copy of the original library
        let library_copy = library.clone();
        // insert library
        db.insert_library(&library).await.expect("Error inserting library");
        // delete library by uuid
        let deleted_library = db.delete_library("uuid1").await.expect("Failed to delete library 1");
        assert_eq!(library, deleted_library);
    }

    #[tokio::test]
    async fn test_insert_delete_multiple_libraries(){
        let db = ClientDBConn::open("mem://").await.expect("Error opening database");

        let library1 = Library {
            uuid: "uuid1".to_string(),
            name: "name1".to_string(),
            path: "path1".to_string(),
            url: "url1".to_string(),
        };

        let library2 = Library {
            uuid: "uuid2".to_string(),
            name: "name2".to_string(),
            path: "path2".to_string(),
            url: "url2".to_string(),
        };

        // insert libraries
        db.insert_library(&library1).await.expect("Error inserting library");
        db.insert_library(&library2).await.expect("Error inserting library");
        let selected_library = db.select_libraries().await.expect("Error selecting libraries");
        assert_eq!(library1, selected_library[0]);

        let deleted_library = db.delete_library("uuid1").await.expect("Failed to delete library 1");
        let selected_library = db.select_libraries().await.expect("Error selecting libraries");
        assert_eq!(library2, selected_library[0]);
    }

    #[tokio::test]
    async fn test_insert_delete_library_by_path(){
        let db = ClientDBConn::open("mem://").await.expect("Error opening database");
        let library = Library::default();

        let library_clone = library.clone();
        // insert a new library
        db.insert_library(&library).await.expect("Error inserting library");
        // remove by path
        let deleted_library = db.delete_library_by_path(&library.path).await.expect("Failed to delete library 2");
        assert_eq!(deleted_library, library_clone);
    }

}
*/