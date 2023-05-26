use directories::ProjectDirs;
use lazy_static::lazy_static;
use db_wrapper::csvdb::client_db::*;
use library_types::Library;

use file_io::client_file_io::CLIENT_DB_PATH;


/// The model for the client
pub struct ClientModel {
    conn: ClientDBConn, // Connection to the database
}
pub fn init_db(){
    //client_db::init_db();
}
/// brief: The implementation of the ClientModel
/// note: This is the implementation of the ClientModel. It is responsible for all the logic that the client needs to do.
impl ClientModel {
    /// Creates a new ClientModel
    /// # Returns
    /// A new ClientModel
    /// # Notes
    /// This function is async because it will be used in the future to create a new client on the server
    /// This function is also a constructor
    pub async fn new() -> Self {
        //server::start_db_server();

        let conn = ClientDBConn::open("/home/johan/.local/share/media_library/home.csv");
        Self { conn }
    }

    /// Creates a new library in the database
    /// # Parameters
    /// `library` - The library to be created
    /// # Notes
    /// This function is async because it will be used in the future to create a library on the server
    pub async fn create_library(&self, library: &Library) {
        self.conn.insert_library(library);
    }

    /// Gets all the libraries in the database
    /// # Returns
    /// A vector of all the libraries in the database
    /// # Notes
    /// This function is async because it will be used in the future to get all the libraries on the server
    pub async fn get_libraries(&self) -> Vec<Library> {
        self.conn.select_libraries()
    }


    /// Deletes a library from the database
    /// # Parameters
    /// `uuid` - The uuid of the library to be deleted
    /// # Notes
    /// This function is async because it will be used in the future to delete a library on the server
    pub async fn delete_library(&self, uuid: &str) {
        //self.conn.delete_library(uuid).await.unwrap();
        todo!()
    }


    /// Gets a library from the database using the uuid
    /// # Parameters
    /// `uuid` - The uuid of the library to be retrieved
    /// # Returns
    /// The library with the given uuid
    /// # Notes
    /// This function is async because it will be used in the future to get a library on the server
    /// This function will panic if the library does not exist
    /// This function will panic if there are multiple libraries with the same uuid
    pub async fn get_library(&self, uuid: &str) -> Library {
        //self.conn.select_library(uuid).await.unwrap()
        todo!()
    }
}
