use lazy_static::lazy_static;
use crate::model::*;
use library_types::Library;

lazy_static!(
    static ref CLIENT: ClientModel = ClientModel::new();
);


/// Creates a library
/// # Parameters
/// 'path' - the path to the library
pub fn create_library(name: &str, path: &str, url: &str) -> Library {
    let new_library = Library::new(name, path, url);
    CLIENT.create_library(&new_library);
    new_library
}

/// Returns a vector of all the libraries
pub fn get_libraries() -> Vec<Library> {
    CLIENT.get_libraries()
}

/// Deletes a library
/// # Parameters
/// `uuid` - the uuid of the library to be deleted
pub fn delete_library(uuid: &str) {
   CLIENT.delete_library(uuid)
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
