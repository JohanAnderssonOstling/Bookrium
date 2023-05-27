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
