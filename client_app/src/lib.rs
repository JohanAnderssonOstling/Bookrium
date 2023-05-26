use lazy_static::lazy_static;
use client_model::{ClientModel, init_db};
use library_types::Library;
use std::thread;
use tokio::runtime::Runtime;

lazy_static!(
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref CLIENT: ClientModel = RUNTIME.block_on(ClientModel::new());
);
/// Start the SurrealDB server in a new thread using std thread
pub fn start_db(){
    init_db();
    /*thread::spawn(|| {
        //server::start_db_server();
    });*/

}

/// Creates a library
/// # Parameters
/// 'path' - the path to the library
pub fn create_library(name: &str, path: &str, url: &str) -> Library {
    let new_library = Library::new(name, path, url);
    RUNTIME.block_on(CLIENT.create_library(&new_library));
    new_library
}

/// Returns a vector of all the libraries
pub fn get_libraries() -> Vec<Library> {
    RUNTIME.block_on(CLIENT.get_libraries())
}

/// Deletes a library
/// # Parameters
/// `uuid` - the uuid of the library to be deleted
pub fn delete_library(uuid: &str) {
   RUNTIME.block_on(CLIENT.delete_library(uuid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_list_delete_library() {
        //List library
        //Create library
        //List libraries
        //Delete library
        //List library
    }
}
