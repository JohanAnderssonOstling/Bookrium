use db_wrapper::csvdb::client_db::*;
use library_types::home_types::Library;

pub struct ClientModel {
    conn: ClientDBConn, // Connection to the database
}
pub fn init_db(){
    //client_db::init_db();
}
impl ClientModel {

    pub fn new() -> Self {
        //server::start_db_server();

        let conn = ClientDBConn::open("/home/johan/.local/share/media_library/home.csv");
        Self { conn }
    }
   
    pub fn create_library(&self, library: &Library) {
        self.conn.insert_library(library);
    }


    pub fn get_libraries(&self) -> Vec<Library> {
        self.conn.select_libraries()
    }


    pub  fn delete_library(&self, uuid: &str) {
        //self.conn.delete_library(uuid).await.unwrap();
        todo!()
    }
   
    pub fn get_library(&self, uuid: &str) -> Library {
        //self.conn.select_library(uuid).await.unwrap()
        todo!()
    }
}
